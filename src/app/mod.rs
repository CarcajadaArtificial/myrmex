use crate::home;
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
            .add_plugins(load::LoadPlugin)
            .add_plugins(menu::MenuPlugin)
            .add_plugins(height::HeightPlugin)
            .add_systems(
                Update,
                (tilemap::load_save_file, camera::movement).run_if(home::is_universe_loaded),
            );
    }
}
