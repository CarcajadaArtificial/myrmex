use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_egui::EguiPlugin;

mod app;
mod home;
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
                        title: String::from("Myrmex - v0.0.65"),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(EguiPlugin)
        .add_plugins(TilemapPlugin)
        .add_plugins(app::AppPlugin)
        .add_plugins(home::HomePlugin)
        .add_systems(Startup, startup)
        .run();
}
