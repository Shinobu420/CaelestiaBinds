use eframe::egui;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

const DEFAULT_RADIUS: u8 = 6;
const DEFAULT_SPACING: i8 = 6;

pub fn is_dark_mode() -> bool {
    let path = scheme_path();
    if let Ok(contents) = fs::read_to_string(path) {
        if let Ok(scheme) = serde_json::from_str::<CaelestiaScheme>(&contents) {
            return scheme.colours.get("mode").is_none_or(|v| v == "dark");
        }
    }
    true
}

fn parse_hex_color(hex: &str) -> Option<egui::Color32> {
    let s = hex.trim();
    let hex = s.strip_prefix('#').unwrap_or(s);
    let (red, green, blue, alpha) = match hex.len() {
        6 => {
            let red = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let green = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let blue = u8::from_str_radix(&hex[4..6], 16).ok()?;
            (red, green, blue, 255)
        }
        8 => {
            let red = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let green = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let blue = u8::from_str_radix(&hex[4..6], 16).ok()?;
            let alpha = u8::from_str_radix(&hex[6..8], 16).ok()?;
            (red, green, blue, alpha)
        }
        _ => return None,
    };
    Some(egui::Color32::from_rgba_unmultiplied(
        red, green, blue, alpha,
    ))
}

#[derive(Deserialize)]
struct CaelestiaScheme {
    colours: std::collections::HashMap<String, String>,
}

pub fn scheme_path() -> PathBuf {
    std::env::var("HOME")
        .map(|home| PathBuf::from(home).join(".local/state/caelestia/scheme.json"))
        .unwrap_or_default()
}

// load and apply caelestia theme from local json
pub fn apply_theme(ctx: &egui::Context) -> Result<(), String> {
    let path = scheme_path();
    if !path.exists() {
        return Ok(());
    }

    let contents = fs::read_to_string(path).map_err(|e| format!("Failed to read JSON: {e}"))?;
    let scheme: CaelestiaScheme = serde_json::from_str(&contents).map_err(|e| format!("Failed to parse JSON: {e}"))?;
    let colours = scheme.colours;

    let bg = colours.get("base").and_then(|v| parse_hex_color(v));
    let fg = colours.get("text").and_then(|v| parse_hex_color(v));
    let panel = colours.get("surface").and_then(|v| parse_hex_color(v));
    let accent = colours.get("primary").and_then(|v| parse_hex_color(v));
    let stroke = colours.get("outline").and_then(|v| parse_hex_color(v));
    let selection = colours.get("surfaceVariant").and_then(|v| parse_hex_color(v));

    let mut style = (*ctx.style()).clone();
    let mut visuals = style.visuals.clone();

    if let Some(fg) = fg { visuals.override_text_color = Some(fg); }
    if let Some(bg) = bg { visuals.extreme_bg_color = bg; }
    if let Some(panel) = panel { visuals.panel_fill = panel; }
    if let Some(accent) = accent { visuals.hyperlink_color = accent; }
    if let Some(selection_bg) = selection { visuals.selection.bg_fill = selection_bg; }
    if let Some(stroke_c) = stroke { visuals.selection.stroke.color = stroke_c; }

    let apply_widget = |w: &mut egui::style::WidgetVisuals| {
        if let Some(fg) = fg { w.fg_stroke.color = fg; }
        if let Some(bg) = bg { w.weak_bg_fill = bg.gamma_multiply(0.6); }
        if let Some(panel) = panel { w.bg_fill = panel; }
        if let Some(stroke_c) = stroke { w.bg_stroke.color = stroke_c; }
        w.corner_radius = egui::CornerRadius::same(DEFAULT_RADIUS);
    };

    apply_widget(&mut visuals.widgets.noninteractive);
    apply_widget(&mut visuals.widgets.inactive);
    apply_widget(&mut visuals.widgets.hovered);
    apply_widget(&mut visuals.widgets.active);
    apply_widget(&mut visuals.widgets.open);

    style.spacing.item_spacing = egui::vec2(f32::from(DEFAULT_SPACING), f32::from(DEFAULT_SPACING));
    style.spacing.button_padding = egui::vec2(f32::from(DEFAULT_SPACING), f32::from(DEFAULT_SPACING));
    style.spacing.menu_margin = egui::Margin::same(DEFAULT_SPACING);
    style.spacing.window_margin = egui::Margin::same(DEFAULT_SPACING);

    style.visuals = visuals;
    ctx.set_style(style);

    Ok(())
}
