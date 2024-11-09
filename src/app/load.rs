use crate::home;
use crate::save::SaveFileData;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use std::fs;
use std::path::Path;

pub fn save_files(
    mut egui_contexts: EguiContexts,
    mut game_state: ResMut<home::HomeState>,
    mut save_data: ResMut<SaveFileData>,
) {
    egui::TopBottomPanel::bottom("bottom_panel").show(egui_contexts.ctx_mut(), |ui| {
        ui.heading("Saved Universes");
        match fs::read_dir("saves") {
            Ok(entries) => {
                display_universe_entries(ui, entries, &mut game_state, &mut save_data);
            }
            Err(_) => {
                ui.label("Could not read saves directory");
            }
        }
    });
}

fn display_universe_entries(
    ui: &mut egui::Ui,
    entries: fs::ReadDir,
    game_state: &mut home::HomeState,
    save_data: &mut SaveFileData,
) {
    for entry in entries.flatten() {
        if let Some(file_name) = entry.file_name().to_str() {
            if ui.link(file_name).clicked() {
                load_universe_file(file_name, game_state, save_data);
            }
        }
    }
}

fn load_universe_file(
    file_name: &str,
    game_state: &mut home::HomeState,
    save_data: &mut SaveFileData,
) {
    let file_path = Path::new("saves").join(file_name);
    match fs::read_to_string(file_path) {
        Ok(contents) => match serde_json::from_str::<SaveFileData>(&contents) {
            Ok(loaded_data) => {
                *save_data = loaded_data;
                game_state.is_universe_loaded = true;
                println!("Successfully loaded save file: {}", file_name);
            }
            Err(e) => {
                println!("Error deserializing save file: {}", e);
            }
        },
        Err(e) => {
            println!("Error reading file: {}", e);
        }
    }
}
