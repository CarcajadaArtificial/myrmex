//    ___ _   _ ___
//   / __| | | |_ _|
//  | (_ | |_| || |
//   \___|\___/|___|
//
//=====================================================================================================//
use eframe::egui;
#[derive(Default)]
//== == == == == == == == == == == == == == == == == == == == == == == == == == == == == == == == == ==//
///
struct MyrmexGui {
    widget_environment_is_open: bool,
}

//== == == == == == == == == == == == == == == == == == == == == == == == == == == == == == == == == ==//
///
impl MyrmexGui {
    ///
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

//== == == == == == == == == == == == == == == == == == == == == == == == == == == == == == == == == ==//
///
impl eframe::App for MyrmexGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Widgets", |ui| {
                    if ui.button("Environment").clicked() {
                        self.widget_environment_is_open = true
                    } else if ui.button("Property Filter").clicked() {
                        // …
                    } else if ui.button("Time Control").clicked() {
                        // …
                    } else if ui.button("Usage Indicator").clicked() {
                        // …
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
                    ui.label("Made by Oscar Alfonso Guerrero");
                });
            });

        egui::Window::new("Environment")
            .frame(
                egui::Frame::none()
                    .shadow(eframe::epaint::Shadow::small_light())
                    .fill(egui::Color32::BLACK)
                    .inner_margin(egui::style::Margin::same(12.0))
                    .rounding(eframe::epaint::Rounding::same(4.0)),
            )
            .open(&mut self.widget_environment_is_open)
            .show(ctx, |ui| {
                ui.label("This is the environment widget.");
            });
    }
}

//== == == == == == == == == == == == == == == == == == == == == == == == == == == == == == == == == ==//
/// This function is called by `main.rs`. Renders the entirety of the app's gui.
pub fn render_app() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Myrmex",
        native_options,
        Box::new(|cc| Box::new(MyrmexGui::new(cc))),
    );
}
