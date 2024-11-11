use bevy::prelude::*;

pub struct HeightPlugin;

impl Plugin for HeightPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HeightData>();
        // .add_systems(Update, save_files.run_if(not(home::is_universe_loaded)));
    }
}

#[derive(Resource)]
pub struct HeightData {
    pub _current_z: u32,
}

impl Default for HeightData {
    fn default() -> Self {
        Self { _current_z: 0 }
    }
}

pub fn render_gui(ui: &mut egui::Ui) {
    ui.heading("Height");
}
