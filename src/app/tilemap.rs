use crate::save;
use bevy::prelude::*;
use bevy_ecs_tilemap::helpers::geometry::get_tilemap_center_transform;
use bevy_ecs_tilemap::prelude::*;

// Component to mark that tilemap is initialized
#[derive(Component)]
pub struct TilemapInitialized;

// Initial setup system
pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    #[cfg(all(not(feature = "atlas"), feature = "render"))] array_texture_loader: Res<
        ArrayTextureLoader,
    >,
    query: Query<Entity, With<TilemapInitialized>>,
) {
    // Only setup if tilemap doesn't exist
    if query.is_empty() {
        let texture_handle: Handle<Image> = asset_server.load("tiles.png");

        let (tilemap_entity, tile_storage, map_size, tile_size, grid_size, map_type) =
            create(&mut commands);

        commands.entity(tilemap_entity).insert((
            TilemapBundle {
                grid_size: grid_size.into(),
                map_type,
                size: map_size,
                storage: tile_storage,
                texture: TilemapTexture::Single(texture_handle.clone()),
                tile_size,
                transform: get_tilemap_center_transform(
                    &map_size,
                    &grid_size.into(),
                    &map_type,
                    0.0,
                ),
                ..Default::default()
            },
            TilemapInitialized,
        ));

        #[cfg(all(not(feature = "atlas"), feature = "render"))]
        {
            array_texture_loader.add(TilemapArrayTexture {
                texture: TilemapTexture::Single(texture_handle),
                tile_size,
                ..Default::default()
            });
        }
    }
}

// System to update tilemap size when needed
pub fn load_save_file(
    mut commands: Commands,
    save_file_data: Res<save::SaveFileData>,
    mut query: Query<(Entity, &mut TilemapSize, &mut Transform), With<TilemapInitialized>>,
) {
    if let Ok((entity, mut size, mut transform)) = query.get_single_mut() {
        if size.x != save_file_data.width || size.y != save_file_data.height {
            // Update the tilemap size
            *size = TilemapSize {
                x: save_file_data.width,
                y: save_file_data.height,
            };

            // Recreate tile storage with new size
            let mut tile_storage = TileStorage::empty(*size);

            // Refill tiles
            fill(
                TileTextureIndex(0),
                *size,
                TilemapId(entity),
                &mut commands,
                &mut tile_storage,
            );

            // Update transform based on the new grid size
            let grid_size: TilemapGridSize = TilemapTileSize { x: 16.0, y: 16.0 }.into();
            *transform = get_tilemap_center_transform(
                &size,
                &grid_size.into(),
                &TilemapType::default(),
                0.0,
            );

            // Update storage
            commands.entity(entity).insert(tile_storage);
        }
    }
}

fn create(
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

    fill(
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

fn fill(
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
