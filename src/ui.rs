use crate::indexer::{build_index, AppEntry};
use crate::search::search_apps;
use eframe::egui;
use crate::config::{ThemeConfig, load};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use windows::core::{w, HSTRING, PCWSTR};
use windows::Win32::UI::Shell::ShellExecuteW;
use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;
use crossbeam_channel::Receiver;

pub struct LauncherApp {
    search_query: String,
    index: Vec<AppEntry>,
    filtered: Vec<AppEntry>,
    selected_index: usize,
    is_visible: Arc<AtomicBool>,
    theme: ThemeConfig,
    was_visible_last_frame: bool,
    current_height: f32,
    index_receiver: Receiver<Vec<AppEntry>>,
    is_indexing: bool,
}

impl LauncherApp {
    pub fn new(is_visible: Arc<AtomicBool>) -> Self {
        let (tx, rx) = crossbeam_channel::unbounded();

        std::thread::spawn(move || {
            let index = build_index();
            let _ = tx.send(index);
        });

        Self {
            search_query: String::new(),
            index: Vec::new(),
            filtered: Vec::new(),
            selected_index: 0,
            theme: load().theme.unwrap_or_default(),
            is_visible,
            was_visible_last_frame: false,
            current_height: 60.0,
            index_receiver: rx,
            is_indexing: true,
        }
    }

    fn execute_selected(&mut self, ctx: &egui::Context) {
        let query = self.search_query.trim();

        if query.starts_with("g ") && query.len() > 2 {
            let search_term = &query[2..];
            let url = format!(
                "https://www.google.com/search?q={}",
                search_term.replace(' ', "+")
            );
            let path = HSTRING::from(url);
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
            self.hide(ctx);
            return;
        }

        if let Some(app) = self.filtered.get(self.selected_index) {
            let path_str = app.path.to_str().unwrap_or("");

            if path_str.starts_with("UWP:") {
                let app_id = &path_str[4..];
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
            self.hide(ctx);
        }
    }

    pub fn hide(&mut self, ctx: &egui::Context) {
        self.is_visible.store(false, Ordering::SeqCst);
        self.search_query.clear();
        self.selected_index = 0;
        self.filtered.clear();
        self.current_height = 60.0;
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

impl eframe::App for LauncherApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.is_indexing {
            if let Ok(new_index) = self.index_receiver.try_recv() {
                self.index = new_index;
                self.is_indexing = false;

                if !self.search_query.trim().starts_with("g ") {
                    self.filtered = search_apps(self.search_query.trim(), &self.index);
                }
            }
        }

        let current_visibility = self.is_visible.load(Ordering::SeqCst);
        let has_focus = ctx.input(|i| i.focused);
        if current_visibility && self.was_visible_last_frame && !has_focus {
            self.hide(ctx);
            return;
        }
        let just_opened = current_visibility && !self.was_visible_last_frame;
        self.was_visible_last_frame = current_visibility;

        if just_opened {
            self.current_height = 60.0;
            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(
                600.0,
                self.current_height,
            )));

            let center_pos = if let Some(monitor_size) = ctx.input(|i| i.viewport().monitor_size) {
                egui::pos2((monitor_size.x - 600.0) / 2.0, monitor_size.y * 0.25)
            } else {
                egui::pos2(100.0, 100.0)
            };

            ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(center_pos));

            // When the launcher just opened with an empty query we show no results initially.
            if self.search_query.trim().is_empty() && !self.index.is_empty() {
                self.filtered.clear();
            }

            ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
        }

        if !current_visibility {
            return;
        }

        let mut target_height = 60.0;
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

        // ---- Glassmorphism visual tweaks ----
        // Adjust global visuals for a translucent, blurred look.
        // Ensure the native window is created with `transparent: true` in eframe options.
        let mut visuals = ctx.style().visuals.clone();
        // Semi‑transparent window background (glass effect)
        visuals.window_fill = egui::Color32::from_rgba_unmultiplied(25, 25, 30, 180);
        visuals.window_rounding = egui::Rounding::same(12.0);
        // Subtle widget backgrounds with translucency
        visuals.widgets.inactive.bg_fill = egui::Color32::from_rgba_unmultiplied(35, 35, 40, 190);
        visuals.widgets.hovered.bg_fill = egui::Color32::from_rgba_unmultiplied(45, 45, 50, 210);
        visuals.widgets.active.bg_fill = egui::Color32::from_rgba_unmultiplied(55, 55, 60, 220);
        // Light inner glow for depth
        visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgba_unmultiplied(80, 80, 90, 120));
        ctx.set_visuals(visuals);

        // Background fill for the result panel (already semi‑transparent)
        let bg = self.theme.background_rgba.unwrap_or([20, 20, 22, 200]);
        let frame_style = egui::Frame {
            fill: egui::Color32::from_rgba_unmultiplied(bg[0], bg[1], bg[2], bg[3]),
            rounding: egui::Rounding::same(8.0), // smoother corners for glass panels
            stroke: egui::Stroke::new(1.0, egui::Color32::from_rgba_premultiplied(60, 60, 70, 255)),
            inner_margin: egui::Margin::same(0.0),
            ..Default::default()
        };
        // Optional: Load custom font (Inter) for a modern look.
        // To enable, place the Inter-Regular.ttf file in the `assets` directory
        // and uncomment the code below.
        // let mut fonts = egui::FontDefinitions::default();
        // fonts.font_data.insert(
        //     "Inter".to_owned(),
        //     egui::FontData::from_static(include_bytes!("../assets/Inter-Regular.ttf")),
        // );
        // if let Some(family) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
        //     family.insert(0, "Inter".to_owned());
        // }
        // ctx.set_fonts(fonts);
        // Apply a subtle drop shadow to the result frame for depth
        let mut visuals = ctx.style().visuals.clone();
        visuals.widgets.inactive.bg_stroke = egui::Stroke::new(2.0, egui::Color32::from_rgba_unmultiplied(0, 0, 0, 80));
        ctx.set_visuals(visuals);
        

        egui::CentralPanel::default()
            .frame(frame_style)
            .show(ctx, |ui| {
                ui.style_mut().visuals.extreme_bg_color = egui::Color32::TRANSPARENT;

                egui::Frame::none()
                    .fill(egui::Color32::from_rgba_premultiplied(35, 35, 40, 255))
                    .stroke(egui::Stroke::new(
                        1.0,
                        egui::Color32::from_rgba_premultiplied(80, 80, 100, 255),
                    ))
                    .rounding(0.0)
                    .inner_margin(egui::Margin::symmetric(16.0, 12.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new("🔍")
                                    .size(16.0)
                                    .color(egui::Color32::from_gray(150)),
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
                                if query.starts_with("g ") {
                                    self.filtered.clear();
                                } else {
                                    self.filtered = search_apps(query, &self.index);
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
                                    .rounding(0.0)
                                    .inner_margin(egui::Margin::symmetric(16.0, 10.0))
                                    .fill(if is_selected {
                                        egui::Color32::from_rgba_premultiplied(50, 50, 70, 255)
                                    } else {
                                        egui::Color32::TRANSPARENT
                                    })
                                    .stroke(if is_selected {
                                        egui::Stroke::new(
                                            1.0,
                                            egui::Color32::from_rgba_unmultiplied(
                                                100, 100, 150, 100,
                                            ),
                                        )
                                    } else {
                                        egui::Stroke::NONE
                                    });

                                let response = item_frame.show(ui, |ui| {
                                    ui.set_width(ui.available_width());
                                    ui.horizontal(|ui| {
                                        // Todos os itens agora exibem o ícone de lupa
                                        if app.is_dir {
                                            ui.label(egui::RichText::new("📁").size(18.0));
                                        } else {
                                            ui.label(egui::RichText::new("🔍").size(18.0));
                                        }

                                        ui.add_space(12.0);

                                        let text_color = if is_selected {
                                            egui::Color32::WHITE
                                        } else {
                                            egui::Color32::from_gray(180)
                                        };

                                        ui.label(
                                            egui::RichText::new(&app.name)
                                                .color(text_color)
                                                .size(14.0),
                                        );

                                        if is_selected {
                                            ui.with_layout(
                                                egui::Layout::right_to_left(egui::Align::Center),
                                                |ui| {
                                                    let tag = if app
                                                        .path
                                                        .to_string_lossy()
                                                        .starts_with("UWP:")
                                                    {
                                                        "APP"
                                                    } else {
                                                        "EXE"
                                                    };
                                                    ui.label(
                                                        egui::RichText::new(tag)
                                                            .size(9.0)
                                                            .color(egui::Color32::from_gray(80)),
                                                    );
                                                },
                                            );
                                        }
                                    });
                                });

                                if is_selected {
                                    response.response.scroll_to_me(None);
                                }
                            }
                        });
                } else if self.search_query.trim().starts_with("g ") {
                    ui.add_space(8.0);
                    egui::Frame::none()
                        .fill(egui::Color32::from_rgba_premultiplied(30, 35, 50, 255))
                        .inner_margin(12.0)
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new("🌐").size(16.0));
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
