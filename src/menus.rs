pub fn render_bottom_panel(ctx: &egui::Context) {
    egui::TopBottomPanel::bottom("bottom_panel").min_height(12.0).show(ctx, |ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
            ui.add(egui::Hyperlink::from_label_and_url("GitHub", "https://github.com/CarcajadaArtificial/Myrmex"));
            ui.label("Made by Oscar Alfonso Guerrero");
        });
    });
}


pub fn render_top_panel(ctx: &egui::Context) {
  egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
      egui::menu::bar(ui, |ui| {
          ui.menu_button("Widgets", |ui| {
              if ui.button("Environment").clicked() {
                  // …
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
}