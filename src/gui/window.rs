//  __      ___         _
//  \ \    / (_)_ _  __| |_____ __ __
//   \ \/\/ /| | ' \/ _` / _ \ V  V /
//    \_/\_/ |_|_||_\__,_\___/\_/\_/
//
//=====================================================================================================//
use chrono::prelude::Utc;
use eframe::egui;
use egui::Ui;

use super::colors;

//== == == == == == == == == == == == == == == == == == == == == == == == == == == == == == ==//
///
pub fn render(ctx: &egui::Context, name: &str, is_open: &mut bool, inside_render: fn(&mut Ui)) {
    egui::Window::new(name)
        .open(is_open)
        .frame(
            egui::Frame::default()
                .shadow(eframe::epaint::Shadow::small_light())
                .stroke(eframe::epaint::Stroke::NONE)
                .fill(colors::CHAPOPOTE)
                .inner_margin(egui::style::Margin::same(12.0))
                .rounding(eframe::epaint::Rounding::same(4.0)),
        )
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| {
            inside_render(ui);
        });
}
