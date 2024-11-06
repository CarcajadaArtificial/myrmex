use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Component, Default)]
pub enum AppState {
    #[default]
    Home,
    Universe,
}

#[derive(Resource, Default)]
pub struct HomeState {
    universe_dimensions: (u32, u32),
}

impl AppState {
    pub fn is_home(query: Query<&AppState>) -> bool {
        matches!(query.get_single().ok(), Some(AppState::Home))
    }

    pub fn is_universe(query: Query<&AppState>) -> bool {
        matches!(query.get_single().ok(), Some(AppState::Universe))
    }
}

pub fn home(
    mut egui_contexts: EguiContexts,
    mut query: Query<&mut AppState>,
    mut home_state: ResMut<HomeState>,
) {
    if let Ok(mut app_state) = query.get_single_mut() {
        egui::CentralPanel::default().show(egui_contexts.ctx_mut(), |ui| {
            ui.heading("Myrmex");

            ui.add_space(20.0);

            // Universe Creation Section
            ui.heading("Create New Universe");
            ui.horizontal(|ui| {
                ui.label("Width:");
                ui.add(egui::DragValue::new(&mut home_state.universe_dimensions.0).range(32..=256));
                ui.label("Height:");
                ui.add(egui::DragValue::new(&mut home_state.universe_dimensions.1).range(32..=256));
            });

            if ui.button("Create Universe").clicked() {
                // Here you would initialize your universe with home_state.universe_dimensions
                *app_state = AppState::Universe;
            }

            ui.add_space(20.0);

            // Saved Universes Section (if you have this feature)
            ui.heading("Saved Universes");
            // Add your saved universes list/grid here
        });
    }
}
