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
    menu::{Menu, MenuEvent, MenuItem},
    Icon, TrayIconBuilder,
};
use ui::LauncherApp;

fn create_tray_icon() -> Option<tray_icon::TrayIcon> {
    let icon_data = include_bytes!("../public/icon.png");
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
    let quit_i = MenuItem::new("Sair do Launcher", true, None);
    let _ = tray_menu.append(&quit_i);

    let quit_id = quit_i.id().clone();

    let tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip("Native Launcher (Alt+S)")
        .with_icon(icon)
        .build()
        .ok()?;

    thread::spawn(move || {
        let menu_channel = MenuEvent::receiver();
        while let Ok(event) = menu_channel.recv() {
            if event.id == quit_id {
                std::process::exit(0);
            }
        }
    });

    Some(tray_icon)
}

fn main() -> Result<(), eframe::Error> {
    let (tx, rx) = unbounded();

    thread::spawn(move || {
        hotkey::listen_for_hotkey(tx);
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
    let app_visibility = is_visible.clone();

    eframe::run_native(
        "Native Launcher",
        options,
        Box::new(move |cc| {
            let mut fonts = egui::FontDefinitions::default();
            egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
            cc.egui_ctx.set_fonts(fonts);

            if let Some(tray) = create_tray_icon() {
                Box::leak(Box::new(tray));
            }

            let ctx = cc.egui_ctx.clone();
            let thread_visibility = is_visible.clone();

            thread::spawn(move || {
                while rx.recv().is_ok() {
                    thread_visibility.store(true, Ordering::SeqCst);
                    ctx.request_repaint();
                }
            });

            Box::new(LauncherApp::new(app_visibility))
        }),
    )
}