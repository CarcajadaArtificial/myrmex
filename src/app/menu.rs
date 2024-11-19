use super::height;
use crate::home;
use crate::save;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_window::PrimaryWindow;

#[derive(Resource, Default)]
pub struct MenuWindowVisibility {
    control: bool,
    time: bool,
    blocks: bool,
    height: bool,
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MenuWindowVisibility>().add_systems(
            Update,
            (
                render_right_panel,
                render_controls,
                render_time,
                render_blocks,
                render_height,
            )
                .run_if(input_toggle_active(true, KeyCode::Escape))
                .run_if(home::is_universe_loaded),
        );
    }
}

fn render_right_panel(
    mut egui_ctx: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut menu: ResMut<MenuWindowVisibility>,
) {
    let mut egui_context = egui_ctx.single_mut();

    egui::SidePanel::right("menu")
        .default_width(200.0)
        .resizable(false)
        .show(egui_context.get_mut(), |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                ui.heading("Myrmex");
                if ui.selectable_label(menu.control, "Control").clicked() {
                    menu.control = !menu.control;
                }
                if ui.selectable_label(menu.height, "Height").clicked() {
                    menu.height = !menu.height;
                }
                if ui.selectable_label(menu.time, "Time").clicked() {
                    menu.time = !menu.time;
                }
                if ui.selectable_label(menu.blocks, "Blocks").clicked() {
                    menu.blocks = !menu.blocks;
                }
                ui.allocate_space(ui.available_size());
            });
        });
}

fn render_controls(
    mut egui_ctx: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut menu: ResMut<MenuWindowVisibility>,
) {
    let mut egui_context = egui_ctx.single_mut();

    egui::Window::new("Controls")
        .open(&mut menu.control)
        .show(egui_context.get_mut(), |ui| {
            ui.heading("Controls");
            ui.separator();
            ui.label("Camera controls:");
            ui.monospace("W - Move up");
            ui.monospace("A - Move left");
            ui.monospace("S - Move backward");
            ui.monospace("D - Move right");
            ui.separator();
            ui.monospace("Esc - Toggle full screen");
        });
}

fn render_time(
    mut egui_ctx: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut menu: ResMut<MenuWindowVisibility>,
) {
    let mut egui_context = egui_ctx.single_mut();

    egui::Window::new("Time")
        .open(&mut menu.time)
        .show(egui_context.get_mut(), |ui| {
            ui.heading("Time");
            ui.separator();
        });
}

fn render_blocks(
    mut egui_ctx: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut menu: ResMut<MenuWindowVisibility>,
) {
    let mut egui_context = egui_ctx.single_mut();

    egui::Window::new("Blocks")
        .open(&mut menu.blocks)
        .show(egui_context.get_mut(), |ui| {
            ui.heading("Blocks");
            ui.separator();
        });
}

fn render_height(
    mut egui_ctx: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut menu: ResMut<MenuWindowVisibility>,
    mut height_data: ResMut<height::HeightData>,
    save_data: ResMut<save::SaveFileData>,
) {
    let mut egui_context = egui_ctx.single_mut();

    egui::Window::new("Height")
        .open(&mut menu.height)
        .show(egui_context.get_mut(), |ui| {
            ui.heading("Height");
            ui.separator();
            ui.label(format!("Z: {}", height_data.current_z));
            ui.horizontal(|ui| {
                if ui.button("+5").clicked() {
                    height_data.current_z += 5;
                }
                if ui.button("+1").clicked() {
                    height_data.current_z += 1;
                }
                if ui.button("-1").clicked() {
                    if height_data.current_z > 0 {
                        height_data.current_z -= 1;
                    }
                }
                if ui.button("-5").clicked() {
                    if height_data.current_z >= 5 {
                        height_data.current_z -= 5;
                    } else {
                        height_data.current_z = 0;
                    }
                }
            });

            ui.horizontal(|ui| {
                if ui.button("Floor").clicked() {
                    height_data.current_z = 0;
                }
                if ui.button("Half").clicked() {
                    height_data.current_z = save_data.z / 2;
                }
                if ui.button("Ceiling").clicked() {
                    height_data.current_z = save_data.z - 1;
                }
            });
        });
}
