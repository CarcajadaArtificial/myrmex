use crate::home;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use std::fs;
use std::path::Path;

pub fn save_files(mut egui_contexts: EguiContexts, mut game_state: ResMut<home::HomeState>) {
    egui::TopBottomPanel::bottom("bottom_panel").show(egui_contexts.ctx_mut(), |ui| {
        render_saved_universes(ui, &mut game_state);
    });
}

/// Displays a list of previously saved universes from the "saves" directory.
fn render_saved_universes(ui: &mut egui::Ui, game_state: &mut home::HomeState) {
    ui.heading("Saved Universes");

    match fs::read_dir("saves") {
        Ok(entries) => {
            display_universe_entries(ui, entries, game_state);
        }
        Err(_) => {
            ui.label("Could not read saves directory");
        }
    }
}

/// Displays a list of saved universe files as clickable links.
/// Each entry in the directory is rendered as a link that, when clicked,
/// triggers the loading of that universe file.
fn display_universe_entries(
    ui: &mut egui::Ui,
    entries: fs::ReadDir,
    game_state: &mut home::HomeState,
) {
    for entry in entries.flatten() {
        if let Some(file_name) = entry.file_name().to_str() {
            if ui.link(file_name).clicked() {
                load_universe_file(file_name, game_state);
            }
        }
    }
}

/// Attempts to load a universe file from the saves directory.
fn load_universe_file(file_name: &str, game_state: &mut home::HomeState) {
    let file_path = Path::new("saves").join(file_name);
    match fs::read_to_string(file_path) {
        Ok(contents) => {
            println!("File contents: {}", contents);
            game_state.is_universe_loaded = true;
        }
        Err(e) => {
            println!("Error reading file: {}", e);
        }
    }
}
