use bevy::prelude::*;
use bevy_fast_tilemap::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FastTileMapPlugin::default())
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut map_assets: ResMut<Assets<Map>>,
) {
    commands.spawn(Camera2dBundle::default());

    let texture_handle = asset_server.load("tiles.png");

    let map_builder = Map::builder(UVec2::new(16, 16), texture_handle, Vec2::new(16.0, 16.0))
        .with_n_tiles(Some(UVec2::new(6, 1))); // Assuming your tileset has 6 tiles in a row

    let map = map_builder.build_and_initialize(|map_indexer| {
        // Initialize the tiles with indices from 0 to 5
        for y in 0..map_indexer.size().y {
            for x in 0..map_indexer.size().x {
                let tile_index = ((x + y) % 6) as u32;
                map_indexer.set(x, y, tile_index);
            }
        }
    });

    // Use MapBundleManaged to automatically manage the mesh
    commands.spawn(MapBundleManaged {
        material: map_assets.add(map),
        ..default()
    });
}
