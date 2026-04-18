use super::types::{ColumnVisibility, Theme};
use crate::hyprland::SearchOptions;
use eframe::egui;

pub struct OptionsState<'a> {
    pub theme: &'a mut Theme,
    pub column_visibility: &'a mut ColumnVisibility,
    pub search_options: &'a mut SearchOptions,
    pub zen_mode: &'a mut bool,
    pub show_zen_info_modal: &'a mut bool,
    pub search_keybind: &'a mut egui::Key,
}

fn save_config(
    theme: Theme,
    column_visibility: &ColumnVisibility,
    search_options: &SearchOptions,
    zen_mode: bool,
    search_keybind: egui::Key,
) {
    let cfg = crate::config::UserConfig {
        theme,
        column_visibility: column_visibility.clone(),
        search_options: search_options.clone(),
        zen_mode,
        search_keybind,
    };
    let _ = crate::config::save(&cfg);
}

fn render_debug_info(ui: &mut egui::Ui, dark: bool) {
    ui.add_space(10.0);
    ui.heading("Debug Info");
    ui.label(format!("Dark Theme: {}", dark));
    ui.label(format!("Window BG: {:?}", ui.visuals().window_fill));
    ui.label(format!("Panel BG: {:?}", ui.visuals().panel_fill));
    ui.label(format!("Extreme BG: {:?}", ui.visuals().extreme_bg_color));
    ui.label(format!("Text Color: {:?}", ui.visuals().text_color()));
    ui.add_space(5.0);
}

fn render_keybind_section(
    ui: &mut egui::Ui,
    theme: Theme,
    column_visibility: &ColumnVisibility,
    search_options: &SearchOptions,
    zen_mode: bool,
    search_keybind: &mut egui::Key,
) {
    ui.separator();
    ui.add_space(10.0);

    ui.heading("⌨  Keybindings");
    ui.add_space(5.0);

    ui.horizontal(|ui| {
        ui.label("Focus Search Bar:");
        let mut changed = false;

        egui::ComboBox::from_id_salt("search_keybind_combo")
            .selected_text(format!("{:?}", search_keybind))
            .show_ui(ui, |ui| {
                changed |= ui.selectable_value(search_keybind, egui::Key::Backslash, "Backslash").changed();
                changed |= ui.selectable_value(search_keybind, egui::Key::Slash, "Slash").changed();
                changed |= ui.selectable_value(search_keybind, egui::Key::F, "F").changed();
                changed |= ui.selectable_value(search_keybind, egui::Key::S, "S").changed();
                changed |= ui.selectable_value(search_keybind, egui::Key::Space, "Space").changed();
            });

        if changed {
            save_config(theme, column_visibility, search_options, zen_mode, *search_keybind);
        }
    });
    ui.add_space(10.0);
}

fn render_column_visibility_section(
    ui: &mut egui::Ui,
    theme: Theme,
    column_visibility: &mut ColumnVisibility,
    search_options: &SearchOptions,
    zen_mode: bool,
    search_keybind: egui::Key,
) {
    ui.separator();
    ui.add_space(10.0);

    ui.heading("\u{f0db}  Visible Columns");
    ui.add_space(5.0);
    let r1 = ui.checkbox(&mut column_visibility.keybind, "\u{ea65}  Keybind");
    let r3 = ui.checkbox(&mut column_visibility.command, "\u{ebc4}  Command");
    if r1.changed() || r3.changed() {
        save_config(theme, column_visibility, search_options, zen_mode, search_keybind);
    }
    ui.add_space(10.0);
}

fn render_search_options_section(
    ui: &mut egui::Ui,
    theme: Theme,
    column_visibility: &ColumnVisibility,
    search_options: &mut SearchOptions,
    zen_mode: bool,
    search_keybind: egui::Key,
) {
    ui.separator();
    ui.add_space(10.0);

    ui.heading("\u{e68f}  Search Options");
    ui.add_space(5.0);
    ui.label("Search in:");
    let s1 = ui.checkbox(&mut search_options.keybind, "\u{ea65}  Keybind");
    let s3 = ui.checkbox(&mut search_options.command, "\u{ebc4}  Command");
    if s1.changed() || s3.changed() {
        save_config(theme, column_visibility, search_options, zen_mode, search_keybind);
    }
    ui.add_space(10.0);
}

fn render_zen_mode_section(
    ui: &mut egui::Ui,
    theme: Theme,
    column_visibility: &ColumnVisibility,
    search_options: &SearchOptions,
    zen_mode: &mut bool,
    show_zen_info_modal: &mut bool,
    search_keybind: egui::Key,
) {
    ui.separator();
    ui.add_space(10.0);

    ui.heading("\u{f06e}  ZEN Mode");
    ui.add_space(5.0);
    ui.label("Hide all distractions and focus on keybindings.");
    ui.add_space(5.0);
    if ui
        .button(egui::RichText::new("Enable ZEN Mode").size(14.0))
        .clicked()
    {
        *zen_mode = true;
        *show_zen_info_modal = true;
        save_config(theme, column_visibility, search_options, *zen_mode, search_keybind);
    }

    ui.add_space(10.0);
}

pub fn render_options_contents(ui: &mut egui::Ui, state: &mut OptionsState) {
    render_debug_info(ui, *state.theme == Theme::Dark);
    
    render_keybind_section(
        ui,
        *state.theme,
        state.column_visibility,
        state.search_options,
        *state.zen_mode,
        state.search_keybind,
    );

    render_column_visibility_section(
        ui,
        *state.theme,
        state.column_visibility,
        state.search_options,
        *state.zen_mode,
        *state.search_keybind,
    );
    render_search_options_section(
        ui,
        *state.theme,
        state.column_visibility,
        state.search_options,
        *state.zen_mode,
        *state.search_keybind,
    );
    render_zen_mode_section(
        ui,
        *state.theme,
        state.column_visibility,
        state.search_options,
        state.zen_mode,
        state.show_zen_info_modal,
        *state.search_keybind,
    );
}
