use crate::camera;
use crate::home;
use crate::menu;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
mod tilemap;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<tilemap::TilemapConfig>()
            .add_systems(Startup, tilemap::setup)
            .init_resource::<menu::MenuWindowsState>()
            .add_systems(
                Update,
                (
                    tilemap::update_tilemap_size,
                    camera::movement,
                    menu::inspector.run_if(input_toggle_active(true, KeyCode::Escape)),
                )
                    .run_if(home::is_universe_loaded),
            );
    }
}
