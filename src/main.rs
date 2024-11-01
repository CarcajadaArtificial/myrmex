use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_inspector_egui::DefaultInspectorConfigPlugin;

mod camera;
mod gui;
mod tilemap;

/// Enum to represent the state of the app.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Component, Default)]
enum AppState {
    #[default]
    Home,
    CreatingUniverse {
        x: i32,
        y: i32,
    },
    LoadedUniverse,
}

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
    // Set initial AppState component to Home
    commands.spawn(AppState::default());

    // commands.spawn(Camera2dBundle::default());

    // tilemap::setup(
    //     &mut commands,
    //     &asset_server,
    //     #[cfg(all(not(feature = "atlas"), feature = "render"))]
    //     &array_texture_loader,
    // );
}

/// Displays the GUI with a header and button to transition to the "CreatingUniverse" state.
fn gui_home(mut egui_contexts: EguiContexts, mut query: Query<&mut AppState>) {
    if let Ok(mut app_state) = query.get_single_mut() {
        egui::CentralPanel::default().show(egui_contexts.ctx_mut(), |ui| {
            ui.heading("Myrmex");
            if ui.button("create universe").clicked() {
                *app_state = AppState::CreatingUniverse { x: 32, y: 32 };
            }
        });
    }
}

/// Displays the GUI for creating a universe with inputs for dimensions and a creation button.
fn gui_create_universe(mut egui_contexts: EguiContexts, mut query: Query<&mut AppState>) {
    if let Ok(mut app_state) = query.get_single_mut() {
        if let AppState::CreatingUniverse { x, y } = &mut *app_state {
            egui::CentralPanel::default().show(egui_contexts.ctx_mut(), |ui| {
                ui.heading("Create Universe");

                // Input fields for x and y dimensions with a range of 32 to 256
                ui.horizontal(|ui| {
                    ui.label("x:");
                    ui.add(egui::DragValue::new(x).range(32..=256));
                    ui.label("y:");
                    ui.add(egui::DragValue::new(y).range(32..=256));
                });

                // Button to create the universe
                if ui.button("create").clicked() {
                    println!("universe created");
                }
            });
        }
    }
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
                        title: String::from("Myrmex - v0.0.43"),
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
        .add_systems(
            Update,
            gui_home.run_if(|query: Query<&AppState>| {
                matches!(query.get_single().ok(), Some(AppState::Home))
            }),
        )
        .add_systems(
            Update,
            gui_create_universe.run_if(|query: Query<&AppState>| {
                matches!(
                    query.get_single().ok(),
                    Some(AppState::CreatingUniverse { .. })
                )
            }),
        )
        // .add_system(camera::movement)
        // .add_system(
        //     gui::inspector.run_if(input_toggle_active(true, KeyCode::Escape)),
        // )
        .run();
}
