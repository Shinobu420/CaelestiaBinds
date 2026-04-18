mod app;
mod cli;
mod config;
mod hyprland;
mod ui;

#[cfg(test)]
mod tests;

use app::KeybindsApp;
use cli::CliAction;
use eframe::egui;
use ui::styling::fonts::setup_custom_fonts;

fn main() -> Result<(), eframe::Error> {
    match cli::parse_args() {
        CliAction::RunGui => run_gui(),
    }
}

fn run_gui() -> Result<(), eframe::Error> {
    let icon_data = load_icon();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 700.0])
            .with_title("CaelestiaBinds")
            .with_icon(icon_data)
            .with_app_id("CaelestiaBinds"),
        ..Default::default()
    };

    eframe::run_native(
        "CaelestiaBinds",
        options,
        Box::new(|cc| {
            setup_custom_fonts(&cc.egui_ctx);
            if let Err(e) = ui::styling::css::apply_theme(&cc.egui_ctx) {
                eprintln!("Failed to load Caelestia theme: {e}");
            }
            Ok(Box::new(KeybindsApp::new()))
        }),
    )
}

fn load_icon() -> egui::IconData {
    let icon_bytes_light = include_bytes!("../assets/Logo_W.png");
    let icon_bytes_dark: &[u8] = include_bytes!("../assets/Logo_B.png");
    let icon_bytes: &[u8];
    
    if crate::ui::styling::css::is_dark_mode() {
        icon_bytes = icon_bytes_dark;
    }else {
        icon_bytes = icon_bytes_light;
    }

    let image = image::load_from_memory(icon_bytes)
        .expect("Failed to load icon")
        .into_rgba8();
    let (width, height) = image.dimensions();

    egui::IconData {
        rgba: image.into_raw(),
        width,
        height,
    }
}
