use crate::home;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;

mod camera;
mod height;
mod load;
mod menu;
mod tilemap;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, tilemap::setup)
            .init_resource::<menu::MenuWindowsState>()
            .add_plugins(load::LoadPlugin)
            .add_systems(
                Update,
                (
                    tilemap::load_save_file,
                    camera::movement,
                    menu::inspector.run_if(input_toggle_active(true, KeyCode::Escape)),
                )
                    .run_if(home::is_universe_loaded),
            );
    }
}
