use crate::save;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

#[derive(Resource, Default)]
pub struct HomeState {
    pub is_universe_loaded: bool,
    pub input_universe_dimensions: (u32, u32, u32),
}

pub struct HomePlugin;

impl Plugin for HomePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HomeState>()
            .add_systems(Update, home.run_if(not(is_universe_loaded)));
    }
}

const MIN_DIMENSION: i32 = 32;
const MAX_DIMENSION: i32 = 256;
const SECTION_SPACING: f32 = 20.0;

pub fn is_universe_loaded(game_state: Res<HomeState>) -> bool {
    game_state.is_universe_loaded
}

pub fn home(mut egui_contexts: EguiContexts, mut game_state: ResMut<HomeState>) {
    egui::CentralPanel::default().show(egui_contexts.ctx_mut(), |ui| {
        render_header(ui);
        render_universe_creator(ui, &mut game_state);
    });
}

fn render_header(ui: &mut egui::Ui) {
    ui.heading("Myrmex");
    ui.add_space(SECTION_SPACING);
}

fn render_universe_creator(ui: &mut egui::Ui, game_state: &mut HomeState) {
    ui.heading("Create New Universe");
    ui.horizontal(|ui| {
        ui.label("x (width):");
        ui.add(
            egui::DragValue::new(&mut game_state.input_universe_dimensions.0)
                .range(MIN_DIMENSION..=MAX_DIMENSION),
        );
        ui.label("y (length):");
        ui.add(
            egui::DragValue::new(&mut game_state.input_universe_dimensions.1)
                .range(MIN_DIMENSION..=MAX_DIMENSION),
        );
        ui.label("z (height):");
        ui.add(
            egui::DragValue::new(&mut game_state.input_universe_dimensions.2)
                .range(MIN_DIMENSION..=MAX_DIMENSION),
        );
    });
    if ui.button("Create Universe").clicked() {
        match save::save_universe(game_state.input_universe_dimensions) {
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
