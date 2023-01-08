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
fn render(ctx: &egui::Context, name: &str, is_open: &mut bool, inside_render: fn(&mut Ui)) {
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

//== == == == == == == == == == == == == == == == == == == == == == == == == == == == == == ==//
///
pub fn environment(ctx: &egui::Context, is_open: &mut bool) {
    render(ctx, "Environment", is_open, |ui| {
        ui.label(format!("{}", Utc::now().format("%a, %b %e - %I:%M:%S %P")));
        ui.label("x days passed");
        ui.separator();
        egui::Grid::new("Weather indicators")
            .num_columns(2)
            .show(ui, |ui| {
                ui.label("Light");
                ui.add(egui::ProgressBar::new(0.46).show_percentage());
                ui.end_row();
                ui.label("Temperature");
                ui.add(egui::ProgressBar::new(0.8).text("45 °C"));
                ui.end_row();
                ui.label("Humidity");
                ui.add(egui::ProgressBar::new(0.06).text("Mostly sunny"));
            });
    });
}

//== == == == == == == == == == == == == == == == == == == == == == == == == == == == == == ==//
///
pub fn property_filter(ctx: &egui::Context, is_open: &mut bool) {
    render(ctx, "Property Filter", is_open, |ui| {
        ui.label("This is the property filter widget.");
    });
}

//== == == == == == == == == == == == == == == == == == == == == == == == == == == == == == ==//
///
pub fn time_control(ctx: &egui::Context, is_open: &mut bool) {
    render(ctx, "Time Control", is_open, |ui| {
        ui.label("This is the time control widget.");
    });
}

//== == == == == == == == == == == == == == == == == == == == == == == == == == == == == == ==//
///
pub fn usage_indicator(ctx: &egui::Context, is_open: &mut bool) {
    render(ctx, "Usage Indicator", is_open, |ui| {
        ui.label("This is the usage indicator widget.");
    });
}
