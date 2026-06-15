#![windows_subsystem = "windows"]
#[cfg(feature = "use_mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod config;
mod hotkey;
mod indexer;
mod search;
mod ui;

use crossbeam_channel::unbounded;
use eframe::egui;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use tray_icon::{
    menu::{CheckMenuItem, Menu, MenuEvent, MenuItem, PredefinedMenuItem},
    Icon, TrayIconBuilder, TrayIconEvent,
};
use winreg::enums::{HKEY_CURRENT_USER, KEY_SET_VALUE};
use winreg::RegKey;

fn is_autostart_enabled() -> bool {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if let Ok(run) = hkcu.open_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Run") {
        let val: Result<String, _> = run.get_value("GlimpseLauncher");
        return val.is_ok();
    }
    false
}

fn toggle_autostart(enable: bool) {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if let Ok(run) = hkcu.open_subkey_with_flags(
        "Software\\Microsoft\\Windows\\CurrentVersion\\Run",
        KEY_SET_VALUE,
    ) {
        if enable {
            if let Ok(exe) = std::env::current_exe() {
                let _ = run.set_value("GlimpseLauncher", &exe.to_string_lossy().to_string());
            }
        } else {
            let _ = run.delete_value("GlimpseLauncher");
        }
    }
}
use ui::LauncherApp;

pub enum AppMsg {
    ShowLauncher,
    ShowSettings,
}

fn create_tray_icon(tx_focus: crossbeam_channel::Sender<AppMsg>) -> Option<tray_icon::TrayIcon> {
    let icon_data = include_bytes!("../public/icone.ico");
    let icon_result = image::load_from_memory(icon_data)
        .map(|img| img.into_rgba8())
        .map(|image| {
            let (width, height) = image.dimensions();
            let rgba = image.into_raw();
            Icon::from_rgba(rgba, width, height)
        });

    let icon = match icon_result {
        Ok(Ok(icon)) => icon,
        _ => return None,
    };

    let tray_menu = Menu::new();

    let settings_i = MenuItem::with_id("settings_menu", "Configurações", true, None);
    let theme_i = MenuItem::with_id("theme_toggle", "Alternar Modo Claro/Escuro", true, None);
    let autostart_i = CheckMenuItem::with_id(
        "autostart_toggle",
        "Iniciar junto ao Windows",
        true,
        is_autostart_enabled(),
        None,
    );
    let quit_i = MenuItem::with_id("quit_app", "Sair", true, None);

    let _ = tray_menu.append(&settings_i);
    let _ = tray_menu.append(&theme_i);
    let _ = tray_menu.append(&autostart_i);
    let _ = tray_menu.append(&PredefinedMenuItem::separator());
    let _ = tray_menu.append(&quit_i);

    let tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip("Glimpse Launcher (Alt+S)")
        .with_icon(icon)
        .build()
        .ok()?;

    thread::spawn(move || {
        let menu_channel = MenuEvent::receiver();
        let tray_channel = TrayIconEvent::receiver();
        loop {
            if let Ok(event) = menu_channel.try_recv() {
                if event.id == "quit_app" {
                    std::process::exit(0);
                } else if event.id == "settings_menu" {
                    let _ = tx_focus.send(AppMsg::ShowSettings);
                } else if event.id == "autostart_toggle" {
                    let is_checked = is_autostart_enabled();
                    toggle_autostart(!is_checked);
                } else if event.id == "theme_toggle" {
                    let mut config = crate::config::load();
                    let is_dark = config
                        .theme
                        .as_ref()
                        .and_then(|t| t.background_rgba)
                        .map_or(true, |rgba| rgba[0] < 100);
                    let new_theme = if is_dark {
                        // Light mode
                        crate::config::ThemeConfig {
                            background_rgba: Some([240, 240, 245, 230]),
                            blur_radius: None,
                        }
                    } else {
                        // Dark mode
                        crate::config::ThemeConfig {
                            background_rgba: Some([20, 20, 22, 200]),
                            blur_radius: None,
                        }
                    };
                    config.theme = Some(new_theme);
                    let _ = crate::config::save(&config);
                    let _ = tx_focus.send(AppMsg::ShowLauncher); // Focus to show the updated theme
                }
            }
            if let Ok(event) = tray_channel.try_recv() {
                if let tray_icon::TrayIconEvent::Click {
                    button: tray_icon::MouseButton::Left,
                    ..
                } = event
                {
                    let _ = tx_focus.send(AppMsg::ShowLauncher);
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    });

    Some(tray_icon)
}

fn main() -> Result<(), eframe::Error> {
    use interprocess::local_socket::{prelude::*, GenericNamespaced, ListenerOptions, Stream};
    use std::io::{prelude::*, BufReader};

    let socket_name = "glimpse_launcher_single_instance.sock"
        .to_ns_name::<GenericNamespaced>()
        .unwrap();

    let listener = match ListenerOptions::new()
        .name(socket_name.clone())
        .create_sync()
    {
        Ok(l) => l,
        Err(_) => {
            if let Ok(mut conn) = Stream::connect(socket_name) {
                let _ = conn.write_all(b"FOCAR_TELA\n");
            }
            std::process::exit(0);
        }
    };

    let (tx, rx) = unbounded::<AppMsg>();

    let tx_hotkey = tx.clone();
    thread::spawn(move || {
        hotkey::listen_for_hotkey(tx_hotkey);
    });

    let tx_ipc = tx.clone();
    thread::spawn(move || {
        for conn in listener.incoming() {
            if let Ok(conn) = conn {
                let mut conn = BufReader::new(conn);
                let mut buffer = String::new();
                if conn.read_line(&mut buffer).is_ok() && buffer.trim() == "FOCAR_TELA" {
                    let _ = tx_ipc.send(AppMsg::ShowLauncher);
                }
            }
        }
    });

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 400.0])
            .with_position([-10000.0, -10000.0])
            .with_decorations(false)
            .with_always_on_top()
            .with_transparent(true)
            .with_taskbar(false)
            .with_visible(true),
        ..Default::default()
    };

    let is_visible = Arc::new(AtomicBool::new(false));
    let show_settings = Arc::new(AtomicBool::new(false));
    let app_visibility = is_visible.clone();
    let app_settings = show_settings.clone();
    let tx_tray = tx.clone();

    eframe::run_native(
        "Glimpse Launcher",
        options,
        Box::new(move |cc| {
            let mut fonts = egui::FontDefinitions::default();
            egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
            cc.egui_ctx.set_fonts(fonts);

            if let Some(tray) = create_tray_icon(tx_tray.clone()) {
                Box::leak(Box::new(tray));
            }

            let ctx = cc.egui_ctx.clone();
            let thread_visibility = is_visible.clone();
            let thread_settings = show_settings.clone();

            thread::spawn(move || {
                while let Ok(msg) = rx.recv() {
                    match msg {
                        AppMsg::ShowLauncher => {
                            thread_visibility.store(true, Ordering::SeqCst);
                        }
                        AppMsg::ShowSettings => {
                            thread_settings.store(true, Ordering::SeqCst);
                        }
                    }
                    ctx.request_repaint();
                }
            });

            Box::new(LauncherApp::new(app_visibility, app_settings))
        }),
    )
}

