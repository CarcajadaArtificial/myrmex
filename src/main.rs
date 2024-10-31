use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::DefaultInspectorConfigPlugin;
mod camera;
mod gui;
mod tilemap;

/// This function spawns a 2D camera and sets up the tilemap system. The tilemap setup is delegated
/// to the `tilemap::setup` function, which handles all necessary components and configuration for
/// rendering the tilemap.
///
/// ## Parameters
///
/// - `commands`
///     A Bevy `Commands` object used to spawn entities and issue component modifications.
///
/// - `asset_server`
///     A Bevy resource responsible for loading assets, such as the tilemap texture.
///
/// - `array_texture_loader`
///     An optional resource (based on feature flags) for loading array textures used when the 'atlas'
///     feature is disabled and the 'render' feature is enabled.
///
fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    #[cfg(all(not(feature = "atlas"), feature = "render"))] array_texture_loader: Res<
        ArrayTextureLoader,
    >,
) {
    commands.spawn(Camera2dBundle::default());

    tilemap::setup(
        &mut commands,
        &asset_server,
        #[cfg(all(not(feature = "atlas"), feature = "render"))]
        &array_texture_loader,
    );
}

/// Entry point for the Bevy application.
///
/// This function configures the Bevy app by adding necessary plugins and systems, including:
/// - Default plugins, with custom window settings.
/// - A tilemap plugin for rendering and managing tilemaps.
/// - Systems for camera movement and game startup.
///
/// # Plugins
/// - `DefaultPlugins`
///     Provides default settings for windows, assets, and input.
///
/// - `TilemapPlugin`
///     Adds support for rendering and managing tilemaps.
///
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Myrmex - v0.0.42"),
                        ..Default::default()
                    }),
                    ..default()
                })
                // Nearest neighbor filtering is applied to avoid pixel blurring for pixel art.
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(EguiPlugin)
        .add_plugins(DefaultInspectorConfigPlugin)
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, startup)
        .add_systems(Update, camera::movement)
        .add_systems(
            Update,
            gui::inspector.run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        .run();
}
