use super::state::GameState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use std::fs;
use std::path::Path;

pub fn home(mut egui_contexts: EguiContexts, mut game_state: ResMut<GameState>) {
    egui::CentralPanel::default().show(egui_contexts.ctx_mut(), |ui| {
        ui.heading("Myrmex");
        ui.add_space(20.0);

        // Create New Universe section
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
                }
                Err(e) => {
                    println!("Error saving universe: {}", e);
                }
            }
        }

        ui.add_space(20.0);

        // Saved Universes section
        ui.heading("Saved Universes");

        // Read and display saved files
        if let Ok(entries) = fs::read_dir("saves") {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(file_name) = entry.file_name().to_str() {
                        // Create a clickable label for each file
                        if ui.link(file_name).clicked() {
                            // Read and print file contents when clicked
                            let file_path = Path::new("saves").join(file_name);
                            match fs::read_to_string(file_path) {
                                Ok(contents) => {
                                    println!("File contents: {}", contents);
                                }
                                Err(e) => {
                                    println!("Error reading file: {}", e);
                                }
                            }
                        }
                    }
                }
            }
        } else {
            ui.label("Could not read saves directory");
        }
    });
}
