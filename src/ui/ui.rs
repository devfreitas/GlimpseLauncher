use crate::core::config::load;
use crate::core::indexer::AppEntry;
use crate::core::search::search_apps;
use eframe::egui;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use windows::core::{w, HSTRING, PCWSTR};
use windows::Win32::UI::Shell::ShellExecuteW;
use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;

use crossbeam_channel::Receiver;
use once_cell::sync::Lazy;

static APP_ICON: Lazy<Arc<egui::IconData>> = Lazy::new(|| {
    let icon_data = include_bytes!("../../public/icone.ico");
    let image = image::load_from_memory(icon_data)
        .expect("Failed to load icon")
        .into_rgba8();
    let (width, height) = image.dimensions();
    Arc::new(egui::IconData {
        rgba: image.into_raw(),
        width,
        height,
    })
});

pub struct LauncherApp {
    search_query: String,
    index: Vec<AppEntry>,
    filtered: Vec<AppEntry>,
    selected_index: usize,
    is_visible: Arc<AtomicBool>,
    show_settings: Arc<AtomicBool>,
    config: crate::core::config::Config,
    launched_paths: std::collections::HashSet<String>,
    was_visible_last_frame: bool,
    current_height: f32,
    index_receiver: Receiver<Vec<AppEntry>>,
    is_indexing: bool,
    is_dragging_mode: bool,
    tray_autostart_item: Option<tray_icon::menu::CheckMenuItem>,
}

impl LauncherApp {
    pub fn new(
        is_visible: Arc<AtomicBool>,
        show_settings: Arc<AtomicBool>,
        tray_autostart_item: Option<tray_icon::menu::CheckMenuItem>,
    ) -> Self {
        let (tx, rx) = crossbeam_channel::unbounded();

        let tx_clone = tx.clone();
        std::thread::spawn(move || {
            let index = crate::core::indexer::build_index(false);
            let _ = tx_clone.send(index);
            crate::core::indexer::start_watcher(tx_clone);
        });

        Self {
            search_query: String::new(),
            index: Vec::new(),
            filtered: Vec::new(),
            selected_index: 0,
            config: load(),
            is_visible,
            show_settings,
            was_visible_last_frame: false,
            current_height: 62.0,
            index_receiver: rx,
            is_indexing: true,
            is_dragging_mode: false,
            launched_paths: std::collections::HashSet::new(),
            tray_autostart_item,
        }
    }

    fn execute_selected(&mut self, ctx: &egui::Context) {
        let query = self.search_query.trim();

        if query.starts_with("g ")
            && query.len() > 2
            && self.config.enable_web_search.unwrap_or(true)
        {
            let search_term = &query[2..];
            let encoded_term = urlencoding::encode(search_term);
            let url = format!("https://www.google.com/search?q={}", encoded_term);
            let _ = webbrowser::open(&url);
            self.hide(ctx);
            return;
        }

        if query.starts_with("> ") && query.len() > 2 && self.config.enable_commands.unwrap_or(true)
        {
            let cmd = &query[2..];
            if cmd.trim() == "config" || cmd.trim() == "settings" {
                self.show_settings.store(true, Ordering::SeqCst);
                self.search_query.clear();
                return;
            }
            let _ = std::process::Command::new("cmd")
                .args(["/C", "start", "cmd.exe", "/K", cmd])
                .spawn();
            self.hide(ctx);
            return;
        }

        if let Some(app) = self.filtered.get(self.selected_index) {
            let path_str = app.path.to_str().unwrap_or("");

            if let Some(result) = path_str.strip_prefix("MATH:") {
                ctx.output_mut(|o| o.copied_text = result.to_string());
                self.hide(ctx);
                return;
            }

            if !self.launched_paths.contains(path_str) {
                self.launched_paths.insert(path_str.to_string());
                if let Some(app_id) = path_str.strip_prefix("UWP:") {
                    let shell_args = format!("shell:appsFolder\\{}", app_id);
                    let explorer = HSTRING::from("explorer.exe");
                    let args = HSTRING::from(shell_args);
                    unsafe {
                        ShellExecuteW(
                            None,
                            w!("open"),
                            &explorer,
                            &args,
                            PCWSTR::null(),
                            SW_SHOWNORMAL,
                        );
                    }
                } else {
                    if !crate::os::window::focus_window_if_running(path_str) {
                        let path = HSTRING::from(path_str);
                        unsafe {
                            ShellExecuteW(
                                None,
                                w!("open"),
                                &path,
                                PCWSTR::null(),
                                PCWSTR::null(),
                                SW_SHOWNORMAL,
                            );
                        }
                    }
                }
            }
            self.hide(ctx);
        }
    }

    pub fn hide(&mut self, ctx: &egui::Context) {
        self.is_visible.store(false, Ordering::SeqCst);
        self.search_query.clear();
        self.selected_index = 0;
        self.filtered.clear();
        self.launched_paths.clear();
        self.current_height = 62.0;
        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(
            600.0,
            self.current_height,
        )));
        ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(egui::pos2(
            -10000.0, -10000.0,
        )));
        ctx.request_repaint();
    }
}

fn custom_switch(ui: &mut egui::Ui, on: &mut bool) -> egui::Response {
    let desired_size = egui::vec2(36.0, 20.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    if response.clicked() {
        *on = !*on;
        response.mark_changed();
    }

    if ui.is_rect_visible(rect) {
        let how_on = ui.ctx().animate_bool(response.id, *on);

        let off_bg_color = egui::Color32::from_rgb(100, 100, 100);
        let on_bg_color = egui::Color32::from_rgb(46, 204, 113); // Green

        let bg_color = egui::Color32::from_rgb(
            ((off_bg_color.r() as f32) * (1.0 - how_on) + (on_bg_color.r() as f32) * how_on) as u8,
            ((off_bg_color.g() as f32) * (1.0 - how_on) + (on_bg_color.g() as f32) * how_on) as u8,
            ((off_bg_color.b() as f32) * (1.0 - how_on) + (on_bg_color.b() as f32) * how_on) as u8,
        );

        let radius = rect.height() / 2.0;
        ui.painter().rect_filled(rect, radius, bg_color);

        let circle_x = egui::lerp(
            (rect.left() + radius + 2.0)..=(rect.right() - radius - 2.0),
            how_on,
        );
        let center = egui::pos2(circle_x, rect.center().y);
        ui.painter()
            .circle_filled(center, radius - 2.0, egui::Color32::WHITE);
    }

    response
}

impl eframe::App for LauncherApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Ok(new_index) = self.index_receiver.try_recv() {
            self.index = new_index;
            self.is_indexing = false;

            if !self.search_query.trim().starts_with("g ") {
                self.filtered = search_apps(self.search_query.trim(), &self.index);
            }
        }

        let current_visibility = self.is_visible.load(Ordering::SeqCst);
        let has_focus = ctx.input(|i| i.focused);
        if current_visibility && self.was_visible_last_frame && !has_focus && !self.is_dragging_mode
        {
            self.hide(ctx);
            return;
        }
        let just_opened = current_visibility && !self.was_visible_last_frame;
        if just_opened {
            self.config = crate::core::config::load();
        }
        self.was_visible_last_frame = current_visibility;

        if just_opened {
            self.current_height = 62.0;
            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(
                600.0,
                self.current_height,
            )));

            let center_pos = if let Some(monitor_size) = ctx.input(|i| i.viewport().monitor_size) {
                let x_pct = self.config.position_x.unwrap_or(0.5);
                let y_pct = self.config.position_y.unwrap_or(0.25);
                egui::pos2((monitor_size.x - 600.0) * x_pct, monitor_size.y * y_pct)
            } else {
                egui::pos2(100.0, 100.0)
            };

            ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(center_pos));

            if self.search_query.trim().is_empty() && !self.index.is_empty() {
                self.filtered.clear();
            }

            ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
        }

        if self.show_settings.load(Ordering::SeqCst) {
            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of("settings_viewport"),
                egui::ViewportBuilder::default()
                    .with_title("Configurações - Glimpse")
                    .with_icon(APP_ICON.clone())
                    .with_inner_size([800.0, 600.0]),
                |ctx, class| {
                    if class == egui::ViewportClass::Deferred {
                        return;
                    }

                    let mut config_changed = false;
                    let theme = self.config.theme.clone().unwrap_or_default();
                    let is_dark = theme.is_dark();

                    let visuals = if is_dark {
                        egui::Visuals::dark()
                    } else {
                        egui::Visuals::light()
                    };
                    let panel_fill = visuals.panel_fill;
                    ctx.set_visuals(visuals);

                    egui::CentralPanel::default()
                        .frame(egui::Frame::none().fill(panel_fill))
                        .show(ctx, |ui| {
                        let title_color = if is_dark { egui::Color32::from_rgb(255, 255, 255) } else { egui::Color32::from_rgb(0, 0, 0) };
                        let desc_color = if is_dark { egui::Color32::from_gray(160) } else { egui::Color32::from_gray(120) };

                        egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                            ui.add_space(24.0);

                            ui.horizontal(|ui| {
                                ui.add_space(24.0);
                                ui.heading(egui::RichText::new("Configurações").size(24.0).strong().color(title_color));
                            });

                            ui.add_space(24.0);

                            ui.horizontal(|ui| {
                                ui.add_space(24.0);
                                ui.vertical(|ui| {
                                    ui.set_width(ui.available_width() - 24.0);

                                    ui.label(egui::RichText::new("Funcionalidades").size(12.0).color(desc_color).strong());
                                    ui.add_space(12.0);

                                    // Item 1
                                    ui.horizontal(|ui| {
                                        ui.label(egui::RichText::new(egui_phosphor::regular::CALCULATOR).size(20.0).color(title_color));
                                        ui.add_space(12.0);
                                        ui.vertical(|ui| {
                                            ui.label(egui::RichText::new("Calculadora Embutida").size(15.0).color(title_color));
                                            ui.add_space(2.0);
                                            ui.label(egui::RichText::new("Avalia expressões matemáticas (ex: 2+2) diretamente na busca.").size(13.0).color(desc_color));
                                        });
                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                            let mut calc_enabled = self.config.enable_calculator.unwrap_or(true);
                                            if custom_switch(ui, &mut calc_enabled).changed() {
                                                self.config.enable_calculator = Some(calc_enabled);
                                                config_changed = true;
                                            }
                                        });
                                    });

                                    ui.add_space(16.0);

                                    // Item 2
                                    ui.horizontal(|ui| {
                                        ui.label(egui::RichText::new(egui_phosphor::regular::GLOBE).size(20.0).color(title_color));
                                        ui.add_space(12.0);
                                        ui.vertical(|ui| {
                                            ui.label(egui::RichText::new("Pesquisa Rápida na Web").size(15.0).color(title_color));
                                            ui.add_space(2.0);
                                            ui.label(egui::RichText::new("Busque no Google digitando 'g ' seguido do termo desejado.").size(13.0).color(desc_color));
                                        });
                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                            let mut web_enabled = self.config.enable_web_search.unwrap_or(true);
                                            if custom_switch(ui, &mut web_enabled).changed() {
                                                self.config.enable_web_search = Some(web_enabled);
                                                config_changed = true;
                                            }
                                        });
                                    });

                                    ui.add_space(16.0);

                                    // Item 3
                                    ui.horizontal(|ui| {
                                        ui.label(egui::RichText::new(egui_phosphor::regular::TERMINAL).size(20.0).color(title_color));
                                        ui.add_space(12.0);
                                        ui.vertical(|ui| {
                                            ui.label(egui::RichText::new("Comandos de Terminal").size(15.0).color(title_color));
                                            ui.add_space(2.0);
                                            ui.label(egui::RichText::new("Rode comandos de prompt digitando '> '.").size(13.0).color(desc_color));
                                        });
                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                            let mut cmd_enabled = self.config.enable_commands.unwrap_or(true);
                                            if custom_switch(ui, &mut cmd_enabled).changed() {
                                                self.config.enable_commands = Some(cmd_enabled);
                                                config_changed = true;
                                            }
                                        });
                                    });

                                    ui.add_space(32.0);
                                    ui.label(egui::RichText::new("Posição da Janela").size(12.0).color(desc_color).strong());
                                    ui.add_space(12.0);
                                    ui.horizontal(|ui| {
                                        ui.label(egui::RichText::new("Posição pré-definida:").color(title_color));
                                        if ui.button("Mover").clicked() {
                                            self.is_dragging_mode = true;
                                            self.show_settings.store(false, Ordering::SeqCst);
                                            self.is_visible.store(true, Ordering::SeqCst);
                                        }
                                        if ui.button("Centro").clicked() {
                                            self.config.position_x = Some(0.5);
                                            self.config.position_y = Some(0.25);
                                            config_changed = true;
                                        }
                                        if ui.button("Superior direito").clicked() {
                                            self.config.position_x = Some(0.9);
                                            self.config.position_y = Some(0.1);
                                            config_changed = true;
                                        }
                                        if ui.button("Superior esquerdo").clicked() {
                                            self.config.position_x = Some(0.1);
                                            self.config.position_y = Some(0.1);
                                            config_changed = true;
                                        }
                                        if ui.button("Inferior direito").clicked() {
                                            self.config.position_x = Some(0.98);
                                            self.config.position_y = Some(0.65);
                                            config_changed = true;
                                        }
                                        if ui.button("Inferior esquerdo").clicked() {
                                            self.config.position_x = Some(0.02);
                                            self.config.position_y = Some(0.65);
                                            config_changed = true;
                                        }
                                    });

                                    ui.add_space(32.0);
                                    ui.label(egui::RichText::new("Sistema").size(12.0).color(desc_color).strong());
                                    ui.add_space(12.0);

                                    // Tema Claro/Escuro
                                    ui.horizontal(|ui| {
                                        ui.label(egui::RichText::new(if is_dark { egui_phosphor::regular::MOON } else { egui_phosphor::regular::SUN }).size(20.0).color(title_color));
                                        ui.add_space(12.0);
                                        ui.vertical(|ui| {
                                            ui.label(egui::RichText::new("Tema Escuro").size(15.0).color(title_color));
                                            ui.add_space(2.0);
                                            ui.label(egui::RichText::new("Alterna entre o modo claro e escuro.").size(13.0).color(desc_color));
                                        });
                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                            let mut dark_enabled = is_dark;
                                            if custom_switch(ui, &mut dark_enabled).changed() {
                                                let new_theme = if dark_enabled {
                                                    crate::core::config::ThemeConfig::dark()
                                                } else {
                                                    crate::core::config::ThemeConfig::light()
                                                };
                                                self.config.theme = Some(new_theme);
                                                config_changed = true;
                                            }
                                        });
                                    });

                                    ui.add_space(16.0);

                                    // Iniciar com Windows
                                    ui.horizontal(|ui| {
                                        ui.label(egui::RichText::new(egui_phosphor::regular::WINDOWS_LOGO).size(20.0).color(title_color));
                                        ui.add_space(12.0);
                                        ui.vertical(|ui| {
                                            ui.label(egui::RichText::new("Iniciar com o Windows").size(15.0).color(title_color));
                                            ui.add_space(2.0);
                                            ui.label(egui::RichText::new("Abre o Glimpse automaticamente ao ligar o PC.").size(13.0).color(desc_color));
                                        });
                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                            let mut auto_start = self.config.start_with_windows.unwrap_or(false);
                                            if custom_switch(ui, &mut auto_start).changed() {
                                                crate::core::config::toggle_autostart(auto_start);
                                                self.config.start_with_windows = Some(auto_start);
                                                config_changed = true;
                                                if let Some(tray_item) = &self.tray_autostart_item {
                                                    tray_item.set_checked(auto_start);
                                                }
                                            }
                                        });
                                    });
                                }); // Close ui.vertical
                            }); // Close ui.horizontal
                        }); // Close ScrollArea
                    }); // Close CentralPanel

                    if config_changed {
                        let _ = crate::core::config::save(&self.config);
                    }
                    if ctx.input(|i| i.viewport().close_requested()) {
                        self.show_settings.store(false, Ordering::SeqCst);
                    }
                }
            ); // End show_viewport_immediate call
        }

        if !current_visibility {
            return;
        }

        let mut target_height = 70.0;
        if self.is_dragging_mode {
            target_height += 48.0;
        }
        if !self.filtered.is_empty() {
            target_height += 10.0;
            let items_to_show = self.filtered.len().min(3) as f32; // Show up to 3 items initially
            target_height += items_to_show * 44.0;
            target_height += 10.0;
        }

        if (self.current_height - target_height).abs() > 0.5 {
            self.current_height = target_height;
            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(
                600.0,
                self.current_height,
            )));
        }
        let theme = self.config.theme.clone().unwrap_or_default();
        let is_dark = theme.is_dark();
        let bg = theme.background_rgba.unwrap_or([20, 20, 22, 200]);

        let accent_color = if is_dark {
            let c = crate::constants::ACCENT_COLOR_DARK;
            egui::Color32::from_rgb(c[0], c[1], c[2])
        } else {
            let c = crate::constants::ACCENT_COLOR_LIGHT;
            egui::Color32::from_rgb(c[0], c[1], c[2])
        };

        let mut visuals = ctx.style().visuals.clone();

        // Ensure the OS window background is completely transparent so only our rounded frame is visible.
        visuals.window_fill = egui::Color32::TRANSPARENT;
        visuals.panel_fill = egui::Color32::TRANSPARENT;

        if is_dark {
            visuals.widgets.inactive.bg_fill =
                egui::Color32::from_rgba_unmultiplied(35, 35, 40, 190);
            visuals.widgets.hovered.bg_fill =
                egui::Color32::from_rgba_unmultiplied(45, 45, 50, 210);
            visuals.widgets.active.bg_fill = egui::Color32::from_rgba_unmultiplied(55, 55, 60, 220);
            visuals.widgets.inactive.bg_stroke =
                egui::Stroke::new(1.0, egui::Color32::from_rgba_unmultiplied(80, 80, 90, 120));
            visuals.override_text_color = Some(egui::Color32::WHITE);
        } else {
            visuals.widgets.inactive.bg_fill =
                egui::Color32::from_rgba_unmultiplied(235, 235, 240, 190);
            visuals.widgets.hovered.bg_fill =
                egui::Color32::from_rgba_unmultiplied(220, 220, 225, 210);
            visuals.widgets.active.bg_fill =
                egui::Color32::from_rgba_unmultiplied(210, 210, 215, 220);
            visuals.widgets.inactive.bg_stroke = egui::Stroke::new(
                1.0,
                egui::Color32::from_rgba_unmultiplied(200, 200, 210, 120),
            );
            visuals.override_text_color = Some(egui::Color32::BLACK);
        }
        visuals.window_rounding = egui::Rounding::same(12.0);
        ctx.set_visuals(visuals);

        let frame_stroke = if is_dark {
            egui::Color32::from_rgba_premultiplied(60, 60, 70, 255)
        } else {
            egui::Color32::from_rgba_premultiplied(200, 200, 215, 255)
        };

        let mut shadow = ctx.style().visuals.window_shadow;
        // Shadow offset depends on egui version, some use [f32; 2], some use Vec2. We'll just modify blur and color.
        shadow.blur = 30.0;
        shadow.color = egui::Color32::from_black_alpha(150);

        let frame_style = egui::Frame {
            fill: egui::Color32::from_rgba_unmultiplied(bg[0], bg[1], bg[2], (bg[3] as f32 * 0.8) as u8),
            rounding: egui::Rounding::same(16.0), // increased from 8.0 to 16.0 for modern look
            stroke: egui::Stroke::new(1.0, frame_stroke),
            inner_margin: egui::Margin {
                left: 8.0,
                right: 8.0,
                top: 8.0,
                bottom: 16.0,
            },
            shadow,
            ..Default::default()
        };
        let mut visuals = ctx.style().visuals.clone();
        visuals.widgets.inactive.bg_stroke =
            egui::Stroke::new(2.0, egui::Color32::from_rgba_unmultiplied(0, 0, 0, 80));
        ctx.set_visuals(visuals);

        egui::CentralPanel::default()
            .frame(frame_style)
            .show(ctx, |ui| {
                ui.style_mut().visuals.extreme_bg_color = egui::Color32::TRANSPARENT;

                if self.is_dragging_mode {
                    ui.add_space(4.0);
                    let frame = egui::Frame::none()
                        .fill(if is_dark {
                            egui::Color32::from_rgba_premultiplied(50, 50, 70, 150)
                        } else {
                            egui::Color32::from_rgba_premultiplied(200, 200, 220, 150)
                        })
                        .rounding(8.0)
                        .inner_margin(6.0);

                    frame.show(ui, |ui| {
                        ui.horizontal(|ui| {
                            let drag_rect = ui
                                .allocate_space(egui::vec2(ui.available_width() - 80.0, 24.0))
                                .1;
                            let drag_response = ui.interact(
                                drag_rect,
                                ui.id().with("drag_handle"),
                                egui::Sense::drag(),
                            );

                            ui.painter().text(
                                drag_rect.center(),
                                egui::Align2::CENTER_CENTER,
                                "↕ Arraste aqui para mover",
                                egui::FontId::proportional(14.0),
                                if is_dark {
                                    egui::Color32::WHITE
                                } else {
                                    egui::Color32::BLACK
                                },
                            );

                            if drag_response.drag_started() {
                                ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                            }

                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if ui.button("Salvar Posição").clicked() {
                                        self.is_dragging_mode = false;
                                        if let Some(outer_rect) =
                                            ctx.input(|i| i.viewport().outer_rect)
                                        {
                                            if let Some(monitor_size) =
                                                ctx.input(|i| i.viewport().monitor_size)
                                            {
                                                let w = monitor_size.x - 600.0;
                                                let h = monitor_size.y;
                                                let x_pct = if w > 0.0 {
                                                    outer_rect.min.x / w
                                                } else {
                                                    0.5
                                                };
                                                let y_pct = if h > 0.0 {
                                                    outer_rect.min.y / h
                                                } else {
                                                    0.25
                                                };
                                                self.config.position_x =
                                                    Some(x_pct.clamp(0.0, 1.0));
                                                self.config.position_y =
                                                    Some(y_pct.clamp(0.0, 1.0));
                                                let _ = crate::core::config::save(&self.config);
                                            }
                                        }
                                    }
                                },
                            );
                        });
                    });
                    ui.add_space(8.0);
                }

                let input_fill = if is_dark {
                    egui::Color32::from_rgba_premultiplied(35, 35, 40, 255)
                } else {
                    egui::Color32::from_rgba_premultiplied(245, 245, 250, 255)
                };
                // Borda do campo de busca colorida para indicar foco
                let input_stroke = accent_color;

                egui::Frame::none()
                    .fill(input_fill)
                    .stroke(egui::Stroke::new(1.0, input_stroke))
                    .rounding(10.0) // rounded search bar
                    .inner_margin(egui::Margin::symmetric(14.0, 10.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new(egui_phosphor::regular::MAGNIFYING_GLASS)
                                    .size(16.0)
                                    .color(if is_dark {
                                        egui::Color32::from_gray(150)
                                    } else {
                                        egui::Color32::from_gray(100)
                                    }),
                            );
                            ui.add_space(8.0);

                            let response = ui.add(
                                egui::TextEdit::singleline(&mut self.search_query)
                                    .hint_text("Pesquisar...")
                                    .font(egui::FontId::proportional(17.0))
                                    .frame(false)
                                    .desired_width(f32::INFINITY),
                            );

                            response.request_focus();

                            if response.changed() {
                                let query = self.search_query.trim();
                                if query.starts_with("g ") || query.starts_with("> ") {
                                    self.filtered.clear();
                                } else {
                                    self.filtered = search_apps(query, &self.index);

                                    if self.config.enable_calculator.unwrap_or(true) {
                                        if let Ok(result) = evalexpr::eval(query) {
                                            if let evalexpr::Value::Int(_)
                                            | evalexpr::Value::Float(_) = result
                                            {
                                                self.filtered.insert(
                                                    0,
                                                    crate::core::indexer::AppEntry {
                                                        name: result.to_string().into_boxed_str(),
                                                        path: std::path::PathBuf::from(format!(
                                                            "MATH:{}",
                                                            result
                                                        )),
                                                        priority: 255,
                                                        is_dir: false,
                                                    },
                                                );
                                            }
                                        }
                                    }
                                }
                                self.selected_index = 0;
                            }
                        });
                    });

                if self.is_indexing {
                    ui.add_space(4.0);
                    ui.horizontal(|ui| {
                        ui.add_space(16.0);
                        ui.add(egui::Spinner::new().size(12.0));
                        ui.add_space(6.0);
                        ui.label(
                            egui::RichText::new("A indexar...")
                                .size(12.0)
                                .color(egui::Color32::from_gray(100)),
                        );
                    });
                }

                if ui.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                    self.selected_index =
                        (self.selected_index + 1).min(self.filtered.len().saturating_sub(1));
                }
                if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                    self.selected_index = self.selected_index.saturating_sub(1);
                }
                if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.execute_selected(ctx);
                }
                if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                    self.hide(ctx);
                }

                if !self.filtered.is_empty() && !self.search_query.trim().starts_with("g ") {
                    egui::ScrollArea::vertical()
                        .max_height(f32::INFINITY)
                        .show(ui, |ui| {
                            ui.spacing_mut().item_spacing.y = 0.0;
                            for (i, app) in self.filtered.iter().enumerate() {
                                let is_selected = i == self.selected_index;

                                let item_frame = egui::Frame::none()
                                    .rounding(8.0) // rounded list items like Raycast/Spotlight
                                    .inner_margin(egui::Margin::symmetric(12.0, 8.0))
                                    .fill(if is_selected {
                                        if is_dark {
                                            egui::Color32::from_rgba_premultiplied(
                                                accent_color.r() / 3,
                                                accent_color.g() / 3,
                                                accent_color.b() / 3,
                                                40,
                                            )
                                        } else {
                                            egui::Color32::from_rgba_premultiplied(
                                                accent_color.r() / 8 + 200,
                                                accent_color.g() / 8 + 200,
                                                accent_color.b() / 8 + 200,
                                                40,
                                            )
                                        }
                                    } else {
                                        egui::Color32::TRANSPARENT
                                    })
                                    .stroke(egui::Stroke::NONE);

                                let response = item_frame.show(ui, |ui| {
                                    ui.set_width(ui.available_width());
                                    ui.horizontal(|ui| {
                                        let path_str = app.path.to_string_lossy();
                                        let is_file = !app.is_dir
                                            && !path_str.starts_with("MATH:")
                                            && !path_str.starts_with("UWP:")
                                            && !path_str.ends_with(".exe")
                                            && !path_str.ends_with(".EXE")
                                            && !path_str.ends_with(".lnk")
                                            && !path_str.ends_with(".LNK");

                                        if let Some(texture) = crate::core::icons::ICON_MANAGER.get_icon(ctx, &path_str) {
                                            ui.add(egui::Image::new(&texture).fit_to_exact_size(egui::vec2(22.0, 22.0)));
                                        } else {
                                            let icon = if path_str.starts_with("MATH:") {
                                                egui_phosphor::regular::CALCULATOR
                                            } else if app.is_dir {
                                                egui_phosphor::regular::FOLDER
                                            } else if is_file {
                                                egui_phosphor::regular::FILE_TEXT
                                            } else {
                                                egui_phosphor::regular::APP_WINDOW
                                            };
                                            ui.label(egui::RichText::new(icon).size(18.0));
                                        }

                                        ui.add_space(12.0);

                                        let text_color = if is_selected {
                                            if is_dark {
                                                egui::Color32::WHITE
                                            } else {
                                                egui::Color32::BLACK
                                            }
                                        } else {
                                            if is_dark {
                                                egui::Color32::from_gray(180)
                                            } else {
                                                egui::Color32::from_gray(80)
                                            }
                                        };

                                        ui.vertical(|ui| {
                                            ui.horizontal(|ui| {
                                                let mut text = egui::RichText::new(&*app.name)
                                                    .color(text_color)
                                                    .size(if is_selected { 16.5 } else { 15.5 });
                                                if is_selected {
                                                    text = text.strong();
                                                }
                                                ui.label(text);

                                                if is_selected {
                                                    ui.with_layout(
                                                        egui::Layout::right_to_left(
                                                            egui::Align::Center,
                                                        ),
                                                        |ui| {
                                                            let tag = if path_str
                                                                .starts_with("MATH:")
                                                            {
                                                                "CALC"
                                                            } else if path_str.starts_with("UWP:") {
                                                                "APP"
                                                            } else if app.is_dir {
                                                                "DIR"
                                                            } else if is_file {
                                                                "FILE"
                                                            } else {
                                                                "EXE"
                                                            };
                                                            let pill_bg = if is_dark {
                                                                egui::Color32::from_rgba_unmultiplied(100, 100, 100, 50)
                                                            } else {
                                                                egui::Color32::from_rgba_unmultiplied(150, 150, 150, 50)
                                                            };
                                                            let pill_text_color = if is_dark {
                                                                egui::Color32::from_gray(200)
                                                            } else {
                                                                egui::Color32::from_gray(60)
                                                            };

                                                            egui::Frame::none()
                                                                .fill(pill_bg)
                                                                .rounding(10.0)
                                                                .inner_margin(egui::Margin::symmetric(8.0, 3.0))
                                                                .show(ui, |ui| {
                                                                    ui.label(
                                                                        egui::RichText::new(tag)
                                                                            .size(10.0)
                                                                            .strong()
                                                                            .color(pill_text_color),
                                                                    );
                                                                });
                                                        },
                                                    );
                                                }
                                            });

                                            let subtitle_color = if is_selected {
                                                accent_color
                                            } else {
                                                if is_dark {
                                                    egui::Color32::from_gray(100)
                                                } else {
                                                    egui::Color32::from_gray(140)
                                                }
                                            };
                                            let subtitle = if path_str.starts_with("MATH:") {
                                                "Pressione Enter para copiar o resultado"
                                                    .to_string()
                                            } else if path_str.starts_with("UWP:") {
                                                "Aplicativo do Windows".to_string()
                                            } else {
                                                path_str.to_string()
                                            };
                                            ui.label(
                                                egui::RichText::new(subtitle)
                                                    .color(subtitle_color)
                                                    .size(12.0),
                                            );
                                        });
                                    });
                                });

                                if is_selected {
                                    let mut border_rect = response.response.rect;
                                    border_rect.max.x = border_rect.min.x + 3.0;
                                    ui.painter().rect_filled(
                                        border_rect,
                                        egui::Rounding { nw: 8.0, sw: 8.0, ne: 0.0, se: 0.0 },
                                        accent_color,
                                    );
                                    response.response.scroll_to_me(None);
                                }
                            }
                        });
                } else if self.search_query.trim().starts_with("g ") {
                    ui.add_space(8.0);
                    let g_fill = if is_dark {
                        egui::Color32::from_rgba_premultiplied(30, 35, 50, 255)
                    } else {
                        egui::Color32::from_rgba_premultiplied(230, 235, 245, 255)
                    };
                    egui::Frame::none()
                        .fill(g_fill)
                        .inner_margin(12.0)
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new(egui_phosphor::regular::GLOBE).size(16.0),
                                );
                                ui.add_space(8.0);
                                ui.label(
                                    egui::RichText::new(format!(
                                        "Google: {}",
                                        &self.search_query[2..]
                                    ))
                                    .color(egui::Color32::LIGHT_BLUE)
                                    .size(13.0),
                                );
                            });
                        });
                }
            });
    }
}
