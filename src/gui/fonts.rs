//   ___        _
//  | __|__ _ _| |_ ___
//  | _/ _ \ ' \  _(_-<
//  |_|\___/_||_\__/__/
//
//=====================================================================================================//
/// This module contains functions that make use of fonts.
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
