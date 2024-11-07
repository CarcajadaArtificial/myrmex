use super::state::GameState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use std::fs;
use std::path::Path;

// Constants for universe dimensions and UI spacing
const MIN_DIMENSION: i32 = 32;
const MAX_DIMENSION: i32 = 256;
const SECTION_SPACING: f32 = 20.0;

/// Main home screen system that renders the entire UI interface.
/// Manages the layout of three main sections:
pub fn home(mut egui_contexts: EguiContexts, mut game_state: ResMut<GameState>) {
    egui::CentralPanel::default().show(egui_contexts.ctx_mut(), |ui| {
        render_header(ui);
        render_universe_creator(ui, &mut game_state);
        render_saved_universes(ui, &mut game_state);
    });
}

/// Renders the application header containing the title "Myrmex"
/// and adds spacing below it for visual separation.
fn render_header(ui: &mut egui::Ui) {
    ui.heading("Myrmex");
    ui.add_space(SECTION_SPACING);
}

/// Renders the universe creation section of the UI.
/// This section includes:
fn render_universe_creator(ui: &mut egui::Ui, game_state: &mut GameState) {
    ui.heading("Create New Universe");
    ui.horizontal(|ui| {
        ui.label("Width:");
        ui.add(
            egui::DragValue::new(&mut game_state.input_universe_dimensions.0)
                .range(MIN_DIMENSION..=MAX_DIMENSION),
        );
        ui.label("Height:");
        ui.add(
            egui::DragValue::new(&mut game_state.input_universe_dimensions.1)
                .range(MIN_DIMENSION..=MAX_DIMENSION),
        );
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
    ui.add_space(SECTION_SPACING);
}

/// Displays a list of previously saved universes from the "saves" directory.
fn render_saved_universes(ui: &mut egui::Ui, game_state: &mut GameState) {
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
fn display_universe_entries(ui: &mut egui::Ui, entries: fs::ReadDir, game_state: &mut GameState) {
    for entry in entries.flatten() {
        if let Some(file_name) = entry.file_name().to_str() {
            if ui.link(file_name).clicked() {
                load_universe_file(file_name, game_state);
            }
        }
    }
}

/// Attempts to load a universe file from the saves directory.
fn load_universe_file(file_name: &str, game_state: &mut GameState) {
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
