#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

mod gui;
mod core;
mod tor;
mod network;
mod config;
mod utils;
mod resources;

use eframe::egui;
use std::sync::Arc;
use tokio::runtime::Runtime;

use crate::gui::app::OnionymousApp;
use crate::core::state::AppState;
use crate::config::settings::Settings;

fn main() -> Result<(), eframe::Error> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_secs()
        .init();

    log::info!("Starting Onionymous v{}", env!("CARGO_PKG_VERSION"));

    if let Err(e) = resources::ensure_extracted() {
        log::error!("Failed to extract embedded resources: {}", e);
    }

    let runtime = Runtime::new().expect("Failed to create Tokio runtime");
    let runtime = Arc::new(runtime);

    let settings = Settings::load().unwrap_or_else(|err| {
        log::warn!("Could not load settings ({}), using defaults", err);
        Settings::default()
    });

    let start_minimized =
        crate::network::autostart::launched_via_autostart() && settings.start_minimized;

    let app_state = Arc::new(AppState::new(settings));
    let mut viewport = egui::ViewportBuilder::default()
        .with_inner_size([1280.0, 800.0])
        .with_min_inner_size([900.0, 600.0])
        .with_decorations(true)
        .with_transparent(true)
        .with_title("Onionymous")
        .with_resizable(true);

    if let Some(icon) = load_window_icon() {
        viewport = viewport.with_icon(icon);
    }

    let native_options = eframe::NativeOptions {
        viewport,
        centered: true,
        vsync: true,
        ..Default::default()
    };

    let state_for_app = app_state.clone();
    let runtime_for_app = runtime.clone();

    eframe::run_native(
        "Onionymous",
        native_options,
        Box::new(move |cc| {
            gui::theme::apply_theme(&cc.egui_ctx);
            gui::theme::load_fonts(&cc.egui_ctx);

            #[cfg(windows)]
            gui::blur::enable_blur_for_window(cc);

            Ok(Box::new(OnionymousApp::new(
                state_for_app,
                runtime_for_app,
                cc,
                start_minimized,
            )))
        }),
    )
}

fn load_window_icon() -> Option<egui::IconData> {
    let bytes = crate::resources::LOGO_ICO;
    let img = image::load_from_memory(bytes).ok()?;
    let rgba = img.to_rgba8();
    let (w, h) = rgba.dimensions();
    Some(egui::IconData {
        rgba: rgba.into_raw(),
        width: w,
        height: h,
    })
}
