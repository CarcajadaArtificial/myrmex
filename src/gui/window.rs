//  __      ___         _
//  \ \    / (_)_ _  __| |_____ __ __
//   \ \/\/ /| | ' \/ _` / _ \ V  V /
//    \_/\_/ |_|_||_\__,_\___/\_/\_/
//
//=====================================================================================================//
use eframe::egui;
use egui::Ui;

use super::colors;

//== == == == == == == == == == == == == == == == == == == == == == == == == == == == == == == == == ==//
///
pub fn render_window(
    ctx: &egui::Context,
    name: &str,
    is_open: &mut bool,
    inside_render: fn(&mut Ui),
) {
    egui::Window::new(name)
        .frame(
            egui::Frame::none()
                .shadow(eframe::epaint::Shadow::small_light())
                .fill(colors::OBSIDIANA)
                .inner_margin(egui::style::Margin::same(12.0))
                .rounding(eframe::epaint::Rounding::same(4.0)),
        )
        .open(is_open)
        .show(ctx, |ui| {
            inside_render(ui);
        });
}
