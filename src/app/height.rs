use bevy::prelude::*;

pub struct HeightPlugin;

impl Plugin for HeightPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HeightData>();
    }
}

#[derive(Resource)]
pub struct HeightData {
    pub current_z: u32,
}

impl Default for HeightData {
    fn default() -> Self {
        Self { current_z: 0 }
    }
}
