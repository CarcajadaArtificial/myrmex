use eframe::egui;
use egui::Ui;

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
                .fill(egui::Color32::BLACK)
                .inner_margin(egui::style::Margin::same(12.0))
                .rounding(eframe::epaint::Rounding::same(4.0)),
        )
        .open(is_open)
        .show(ctx, |ui| {
            inside_render(ui);
        });
}
