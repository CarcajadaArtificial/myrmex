use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::DefaultInspectorConfigPlugin;

mod app;
mod camera;
mod home;
mod menu;
mod save;

fn startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Myrmex - v0.0.57"),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(EguiPlugin)
        .add_plugins(DefaultInspectorConfigPlugin)
        .add_plugins(TilemapPlugin)
        .init_resource::<home::HomeState>()
        .init_resource::<menu::MenuWindowsState>()
        .add_systems(Startup, startup)
        .add_systems(Update, home::home.run_if(not(home::is_universe_loaded)))
        .add_systems(
            Update,
            (
                camera::movement,
                menu::inspector.run_if(input_toggle_active(true, KeyCode::Escape)),
                app::run_universe,
            )
                .run_if(home::is_universe_loaded),
        )
        .run();
}
