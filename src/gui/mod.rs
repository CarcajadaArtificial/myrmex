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
mod world_properties;

#[derive(Default)]

///
struct MyrmexGui {
    world_properties: world_properties::WorldProperties,
}

///
impl MyrmexGui {
    ///
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        visuals::set_app_style(cc);
        Self {
            world_properties: world_properties::WorldProperties::default(),
        }
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
                        self.world_properties.window_environment_is_open =
                            !self.world_properties.window_environment_is_open
                    } else if ui.button("Property Filter").clicked() {
                        self.world_properties.window_property_filter_is_open =
                            !self.world_properties.window_property_filter_is_open
                    } else if ui.button("Time Control").clicked() {
                        self.world_properties.window_time_control_is_open =
                            !self.world_properties.window_time_control_is_open
                    } else if ui.button("Usage Indicator").clicked() {
                        self.world_properties.window_usage_indicator_is_open =
                            !self.world_properties.window_usage_indicator_is_open
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

        self.world_properties.environment(ctx);
        self.world_properties.property_filter(ctx);
        self.world_properties.time_control(ctx);
        self.world_properties.usage_indicator(ctx);
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
