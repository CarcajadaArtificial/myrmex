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

///
struct MyrmexGui {
    world_properties: world_properties::WorldProperties,
    side_length: f32,
    height_length: f32,
    selected: String,
}

///
impl MyrmexGui {
    ///
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        visuals::set_app_style(cc);
        Self {
            world_properties: world_properties::WorldProperties::default(),
            side_length: 0.0,
            height_length: 0.0,
            selected: "First".to_string(),
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
            ui.heading("World Editor");
            ui.horizontal(|ui| {
                egui::ComboBox::from_label("Select existing world file")
                    .selected_text(format!("{:?}", self.selected))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.selected, "First".to_string(), "First");
                        ui.selectable_value(&mut self.selected, "Second".to_string(), "Second");
                        ui.selectable_value(&mut self.selected, "Third".to_string(), "Third");
                    });
                if ui.button("Load world").clicked() {}
            });
            ui.separator();
            ui.heading("Dimensions");
            ui.horizontal(|ui| {
                ui.label("Side (8-128 blocks):");
                ui.add(
                    egui::DragValue::new(&mut self.side_length)
                        .speed(0.1)
                        .fixed_decimals(0)
                        .clamp_range(std::ops::RangeInclusive::new(8, 128)),
                );
            });
            ui.horizontal(|ui| {
                ui.label("Height (16-128 blocks):");
                ui.add(
                    egui::DragValue::new(&mut self.height_length)
                        .speed(0.1)
                        .fixed_decimals(0)
                        .clamp_range(std::ops::RangeInclusive::new(16, 128)),
                );
            });
            ui.separator();
            ui.heading("Weather");
            ui.horizontal(|ui| {
                egui::ComboBox::from_label("Select weather file")
                    .selected_text(format!("{:?}", self.selected))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.selected, "First".to_string(), "First");
                        ui.selectable_value(&mut self.selected, "Second".to_string(), "Second");
                        ui.selectable_value(&mut self.selected, "Third".to_string(), "Third");
                    });
            });
            ui.separator();
            if ui.button("Load World").clicked() {}
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
