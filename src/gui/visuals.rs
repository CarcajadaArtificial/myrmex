//  __   ___              _
//  \ \ / (_)____  _ __ _| |___
//   \ V /| (_-< || / _` | (_-<
//    \_/ |_/__/\_,_\__,_|_/__/
//
//=====================================================================================================//
/// This module contains functions that configure eguis visuals.
use super::colors;
use egui::{FontData, FontDefinitions, FontFamily};

//== == == == == == == == == == == == == == == == == == == == == == == == == == == == == == ==//
/// This function edits eframe's CreationContext inserting FiraCode-Regular and setting it as the default main font and callback for monospace. Implementation taken from Docs.rs: https://docs.rs/egui/latest/egui/struct.FontDefinitions.html.
pub fn insert_fonts(cc: &eframe::CreationContext<'_>) {
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "FiraCode".to_owned(),
        FontData::from_static(include_bytes!("../../static/fonts/FiraCode-Regular.ttf")),
    );
    fonts
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "FiraCode".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .get_mut(&FontFamily::Monospace)
        .unwrap()
        .push("FiraCode".to_owned());
    cc.egui_ctx.set_fonts(fonts);
}

//== == == == == == == == == == == == == == == == == == == == == == == == == == == == == == ==//
/// This function sets up the visual theme for the application.
pub fn set_visuals(cc: &eframe::CreationContext<'_>) {
    let visuals = &mut egui::style::Visuals::default();
    // Colors
    visuals.hyperlink_color = colors::MOTA;
    visuals.panel_fill = colors::GRAFITO;
    visuals.window_fill = colors::CHAPOPOTE;
    visuals.override_text_color = Some(colors::BLANCO);
    visuals.faint_bg_color = colors::GRAFITO;
    visuals.extreme_bg_color = colors::HIERRO;
    visuals.error_fg_color = colors::FRESA;
    visuals.warn_fg_color = colors::FLAN;
    // Shadows
    visuals.popup_shadow = eframe::epaint::Shadow::small_light();
    visuals.window_shadow = eframe::epaint::Shadow::small_light();
    // Strokes
    visuals.window_stroke = eframe::epaint::Stroke::NONE;
    cc.egui_ctx.set_visuals(visuals.clone());
}
