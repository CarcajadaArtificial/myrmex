use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

mod tilemap;

pub fn run_universe(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    #[cfg(all(not(feature = "atlas"), feature = "render"))] array_texture_loader: Res<
        ArrayTextureLoader,
    >,
) {
    tilemap::setup(
        &mut commands,
        &asset_server,
        #[cfg(all(not(feature = "atlas"), feature = "render"))]
        &array_texture_loader,
    );
}
