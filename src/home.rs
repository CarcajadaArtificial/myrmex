use super::state::GameState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

pub fn home(mut egui_contexts: EguiContexts, mut game_state: ResMut<GameState>) {
    egui::CentralPanel::default().show(egui_contexts.ctx_mut(), |ui| {
        ui.heading("Myrmex");

        ui.add_space(20.0);

        ui.heading("Create New Universe");
        ui.horizontal(|ui| {
            ui.label("Width:");
            ui.add(egui::DragValue::new(&mut game_state.universe_dimensions.0).range(32..=256));
            ui.label("Height:");
            ui.add(egui::DragValue::new(&mut game_state.universe_dimensions.1).range(32..=256));
        });

        if ui.button("Create Universe").clicked() {
            match game_state.save_universe() {
                Ok(universe_data) => {
                    println!("Universe created and saved with ID: {}", universe_data.id);
                    game_state.is_universe_loaded = true;
                }
                Err(e) => {
                    println!("Error saving universe: {}", e);
                }
            }
        }

        ui.add_space(20.0);

        ui.heading("Saved Universes");
    });
}
