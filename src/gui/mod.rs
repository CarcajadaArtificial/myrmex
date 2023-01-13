//    ___ _   _ ___
//   / __| | | |_ _|
//  | (_ | |_| || |
//   \___|\___/|___|
//
//=====================================================================================================//
// This module has the main methods of rendering the application's gui. Here egui can be configured, the global state is defined and the entire app is rendered.
use eframe::egui;
mod colors;
mod top_bottom_panel;
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
        visuals::set_app_style(cc);
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
        top_bottom_panel::bottom(ctx);

        window::environment(ctx, &mut self.widget_environment_is_open);
        window::property_filter(ctx, &mut self.widget_property_filter_is_open);
        window::time_control(ctx, &mut self.widget_time_control_is_open);
        window::usage_indicator(ctx, &mut self.widget_usage_indicator_is_open);
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
