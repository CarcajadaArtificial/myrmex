//   _____         ___      _   _             ___               _
//  |_   _|__ _ __| _ ) ___| |_| |_ ___ _ __ | _ \__ _ _ _  ___| |
//    | |/ _ \ '_ \ _ \/ _ \  _|  _/ _ \ '  \|  _/ _` | ' \/ -_) |
//    |_|\___/ .__/___/\___/\__|\__\___/_|_|_|_| \__,_|_||_\___|_|
//           |_|
//=====================================================================================================//
//

pub fn bottom(ctx: &egui::Context) {
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
}
