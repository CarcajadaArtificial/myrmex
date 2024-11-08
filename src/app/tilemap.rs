use bevy::prelude::*;
use bevy_ecs_tilemap::helpers::geometry::get_tilemap_center_transform;
use bevy_ecs_tilemap::prelude::*;

// Component to mark that tilemap is initialized
#[derive(Component)]
pub struct TilemapInitialized;

// Resource to store tilemap configuration
#[derive(Resource)]
pub struct TilemapConfig {
    pub size: TilemapSize,
    pub tile_size: TilemapTileSize,
}

impl Default for TilemapConfig {
    fn default() -> Self {
        Self {
            size: TilemapSize { x: 32, y: 32 },
            tile_size: TilemapTileSize { x: 16.0, y: 16.0 },
        }
    }
}

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
pub fn update_tilemap_size(
    mut commands: Commands,
    config: Res<TilemapConfig>,
    mut query: Query<(Entity, &mut TilemapSize, &mut Transform), With<TilemapInitialized>>,
) {
    if let Ok((entity, mut size, mut transform)) = query.get_single_mut() {
        if size.x != config.size.x || size.y != config.size.y {
            // Update size
            *size = config.size;

            // Recreate tile storage with new size
            let mut tile_storage = TileStorage::empty(config.size);

            // Refill tiles
            fill(
                TileTextureIndex(0),
                config.size,
                TilemapId(entity),
                &mut commands,
                &mut tile_storage,
            );

            // Update transform
            let grid_size: TilemapGridSize = config.tile_size.into();
            *transform = get_tilemap_center_transform(
                &config.size,
                &grid_size.into(),
                &TilemapType::default(),
                0.0,
            );

            // Update storage
            commands.entity(entity).insert(tile_storage);
        }
    }
}

// Existing create and fill functions remain the same...

/// This function spawns an empty tilemap entity and sets up the storage for the tiles. It then
/// populates the tilemap by calling the `fill` function, which places tiles at every position within
/// the tilemap's dimensions.
///
/// ## Parameters
/// - `commands`:
///     A mutable reference to the Bevy `Commands` object used to spawn the tilemap entity.
///
/// ## Returns
/// A tuple containing:
/// - `Entity`: The created tilemap entity.
/// - `TileStorage`: The storage used to manage tile entities.
/// - `TilemapSize`: The dimensions of the tilemap.
/// - `TilemapTileSize`: The size of individual tiles.
/// - `Vec2`: The grid size representing the dimensions of the grid.
/// - `TilemapType`: The type of the tilemap.
///
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

/// This function iterates over the dimensions of the tilemap and spawns a tile entity at each
/// position. The spawned tiles are assigned the provided texture index and are added to the
/// `TileStorage`.
///
/// # Parameters
/// - `tile_texture_index`: The index representing the texture to be used for each tile.
/// - `map_size`: The dimensions of the tilemap.
/// - `tilemap_id`: The ID of the tilemap entity being populated.
/// - `commands`: A mutable reference to the Bevy `Commands` object used to spawn tile entities.
/// - `tile_storage`: A mutable reference to the `TileStorage` where tile entities are stored.
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
