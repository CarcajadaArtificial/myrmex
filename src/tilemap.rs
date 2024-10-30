use bevy::prelude::*;
use bevy_ecs_tilemap::helpers::geometry::get_tilemap_center_transform;
use bevy_ecs_tilemap::prelude::*;

/// This function handles the initialization of the tilemap system, including loading the texture for
/// the tiles, creating and configuring the tilemap entity, and inserting it into the Bevy ECS.
/// It also supports adding array textures if the relevant feature is enabled.
///
/// ## Parameters
/// - `commands`
///     A mutable reference to the Bevy `Commands` object used to create entities and issue commands.
///
/// - `asset_server`
///     A reference to the Bevy `AssetServer` used to load textures and other assets.
///
/// - `array_texture_loader`
///     Conditionally included, this is a reference to the array texture loader used when specific
///     feature flags are enabled for tilemap rendering.
pub fn setup(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    #[cfg(all(not(feature = "atlas"), feature = "render"))] array_texture_loader: &Res<
        ArrayTextureLoader,
    >,
) {
    let texture_handle: Handle<Image> = asset_server.load("tiles.png");

    let (tilemap_entity, tile_storage, map_size, tile_size, grid_size, map_type) = create(commands);

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
