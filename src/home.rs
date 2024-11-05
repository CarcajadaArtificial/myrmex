use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

/// Enum to represent the state of the app.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Component, Default)]
pub enum AppState {
    #[default]
    Home,
    CreatingUniverse,
    LoadedUniverse,
}

impl AppState {
    pub fn is_home(query: Query<&AppState>) -> bool {
        matches!(query.get_single().ok(), Some(AppState::Home))
    }

    pub fn is_creating_universe(query: Query<&AppState>) -> bool {
        matches!(query.get_single().ok(), Some(AppState::CreatingUniverse))
    }

    pub fn is_loaded_universe(query: Query<&AppState>) -> bool {
        matches!(query.get_single().ok(), Some(AppState::LoadedUniverse))
    }
}

/// Displays the GUI with a header and button to transition to the "CreatingUniverse" state.
pub fn home(mut egui_contexts: EguiContexts, mut query: Query<&mut AppState>) {
    if let Ok(mut app_state) = query.get_single_mut() {
        egui::CentralPanel::default().show(egui_contexts.ctx_mut(), |ui| {
            ui.heading("Myrmex");
            if ui.button("create universe").clicked() {
                *app_state = AppState::CreatingUniverse;
            }
        });
    }
}

/// Displays the GUI for creating a universe with inputs for dimensions and a creation button.
pub fn create_universe(mut egui_contexts: EguiContexts, mut query: Query<&mut AppState>) {
    if let Ok(mut app_state) = query.get_single_mut() {
        let mut is_universe_created = false;

        if let AppState::CreatingUniverse = &mut *app_state {
            egui::CentralPanel::default().show(egui_contexts.ctx_mut(), |ui| {
                ui.heading("Create Universe");

                ui.horizontal(|ui| {
                    ui.label("x:");
                    // ui.add(egui::DragValue::new(x).range(32..=256));
                    ui.label("y:");
                    // ui.add(egui::DragValue::new(y).range(32..=256));
                });

                if ui.button("create").clicked() {
                    println!("universe created");
                    is_universe_created = true;
                }
            });
        }

        if is_universe_created {
            *app_state = AppState::Home;
        }
    }
}
