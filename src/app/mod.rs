use crate::home;
use bevy::prelude::*;
mod tilemap;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<tilemap::TilemapConfig>()
            .add_systems(Startup, tilemap::setup)
            .add_systems(
                Update,
                tilemap::update_tilemap_size.run_if(home::is_universe_loaded),
            );
    }
}
