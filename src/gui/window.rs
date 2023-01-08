//  __      ___         _
//  \ \    / (_)_ _  __| |_____ __ __
//   \ \/\/ /| | ' \/ _` / _ \ V  V /
//    \_/\_/ |_|_||_\__,_\___/\_/\_/
//
//=====================================================================================================//
use eframe::egui;
use egui::Ui;

//== == == == == == == == == == == == == == == == == == == == == == == == == == == == == == ==//
///
pub fn render(ctx: &egui::Context, name: &str, is_open: &mut bool, inside_render: fn(&mut Ui)) {
    egui::Window::new(name)
        .open(is_open)
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| {
            inside_render(ui);
        });
}
