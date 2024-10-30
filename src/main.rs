use bevy::prelude::*;
use bevy_ecs_tilemap::helpers::geometry::get_tilemap_center_transform;
use bevy_ecs_tilemap::prelude::*;

mod camera;

fn setup_tilemap(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    #[cfg(all(not(feature = "atlas"), feature = "render"))] array_texture_loader: &Res<
        ArrayTextureLoader,
    >,
) {
    let texture_handle: Handle<Image> = asset_server.load("tiles.png");

    let (tilemap_entity, tile_storage, map_size, tile_size, grid_size, map_type) =
        create_tilemap(commands);

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size: grid_size.into(),
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle.clone()),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size.into(), &map_type, 0.0),
        ..Default::default()
    });

    #[cfg(all(not(feature = "atlas"), feature = "render"))]
    {
        array_texture_loader.add(TilemapArrayTexture {
            texture: TilemapTexture::Single(texture_handle),
            tile_size,
            ..Default::default()
        });
    }
}

/// Creates the tilemap entity, initializes tile storage, and fills the tilemap with tiles.
fn create_tilemap(
    commands: &mut Commands,
) -> (
    Entity,
    TileStorage,
    TilemapSize,
    TilemapTileSize,
    Vec2,
    TilemapType,
) {
    let map_size = TilemapSize { x: 32, y: 32 };
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);

    // Fill the tilemap with tiles
    fill_tilemap(
        TileTextureIndex(0),
        map_size,
        TilemapId(tilemap_entity),
        commands,
        &mut tile_storage,
    );

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size: TilemapGridSize = tile_size.into();
    let map_type = TilemapType::default();

    (
        tilemap_entity,
        tile_storage,
        map_size,
        tile_size,
        grid_size.into(),
        map_type,
    )
}

/// Helper function to fill the tilemap with tiles.
fn fill_tilemap(
    tile_texture_index: TileTextureIndex,
    map_size: TilemapSize,
    tilemap_id: TilemapId,
    commands: &mut Commands,
    tile_storage: &mut TileStorage,
) {
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    texture_index: tile_texture_index,
                    tilemap_id,
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    #[cfg(all(not(feature = "atlas"), feature = "render"))] array_texture_loader: Res<
        ArrayTextureLoader,
    >,
) {
    commands.spawn(Camera2dBundle::default());

    setup_tilemap(
        &mut commands,
        &asset_server,
        #[cfg(all(not(feature = "atlas"), feature = "render"))]
        &array_texture_loader,
    );
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Basic Tilemap Example"),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, startup)
        .add_systems(Update, camera::movement)
        .run();
}
