//    ___ _   _ ___
//   / __| | | |_ _|
//  | (_ | |_| || |
//   \___|\___/|___|
//
//=====================================================================================================//
// This module has the main methods of rendering the application's gui. Here egui can be configured, the global state is defined and the entire app is rendered.
use chrono::prelude::*;
use eframe::egui;
mod colors;
mod visuals;
mod window;
#[derive(Default)]
//== == == == == == == == == == == == == == == == == == == == == == == == == == == == == == ==//
///
struct MyrmexGui {
    widget_environment_is_open: bool,
    widget_property_filter_is_open: bool,
    widget_time_control_is_open: bool,
    widget_usage_indicator_is_open: bool,
}

//== == == == == == == == == == == == == == == == == == == == == == == == == == == == == == ==//
///
impl MyrmexGui {
    ///
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        visuals::insert_fonts(cc);
        visuals::set_visuals(cc);
        Self::default()
    }
}

//== == == == == == == == == == == == == == == == == == == == == == == == == == == == == == ==//
///
impl eframe::App for MyrmexGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Widgets", |ui| {
                    if ui.button("Environment").clicked() {
                        self.widget_environment_is_open = true
                    } else if ui.button("Property Filter").clicked() {
                        self.widget_property_filter_is_open = true
                    } else if ui.button("Time Control").clicked() {
                        self.widget_time_control_is_open = true
                    } else if ui.button("Usage Indicator").clicked() {
                        self.widget_usage_indicator_is_open = true
                    }
                });
                ui.menu_button("Debug", |ui| {
                    if ui.button("Test").clicked() {
                        // …
                    }
                });
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
        });
        egui::TopBottomPanel::bottom("bottom_panel")
            .min_height(12.0)
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    ui.add(egui::Hyperlink::from_label_and_url(
                        "GitHub",
                        "https://github.com/CarcajadaArtificial/Myrmex",
                    ));
                    ui.label("Made by: Oscar Alfonso Guerrero");
                });
            });

        render_environment_window(ctx, &mut self.widget_environment_is_open);

        window::render(
            ctx,
            "Property Filter",
            &mut self.widget_property_filter_is_open,
            |ui| {
                ui.label("This is the property filter widget.");
            },
        );

        window::render(
            ctx,
            "Time Control",
            &mut self.widget_time_control_is_open,
            |ui| {
                ui.label("This is the time control widget.");
            },
        );

        window::render(
            ctx,
            "Usage Indicator",
            &mut self.widget_usage_indicator_is_open,
            |ui| {
                ui.label("This is the usage indicator widget.");
            },
        );
    }
}

//== == == == == == == == == == == == == == == == == == == == == == == == == == == == == == ==//
/// This function is called by `main.rs`. Renders the entirety of the app's gui.
pub fn render_app() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Myrmex",
        native_options,
        Box::new(|cc| Box::new(MyrmexGui::new(cc))),
    );
}

//== == == == == == == == == == == == == == == == == == == == == == == == == == == == == == ==//
fn render_environment_window(ctx: &egui::Context, is_open: &mut bool) {
    window::render(ctx, "Environment", is_open, |ui| {
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
