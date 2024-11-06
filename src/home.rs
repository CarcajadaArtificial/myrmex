// home.rs
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

#[derive(Resource, Default)]
pub struct GameState {
    pub is_universe_loaded: bool,
    pub universe_dimensions: (u32, u32),
}

pub fn is_home(game_state: Res<GameState>) -> bool {
    !game_state.is_universe_loaded
}

pub fn is_universe(game_state: Res<GameState>) -> bool {
    game_state.is_universe_loaded
}

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
            game_state.is_universe_loaded = true;
        }

        ui.add_space(20.0);

        ui.heading("Saved Universes");
        // Add saved universes list here
    });
}
