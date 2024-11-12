use super::height;
use crate::home;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_window::PrimaryWindow;
use std::collections::HashSet;

#[derive(Resource, Default)]
pub struct MenuData {
    open_windows: HashSet<MenuOption>,
}

#[derive(PartialEq, Clone, Hash, Eq)]
pub(crate) enum MenuOption {
    Controls,
    Time,
    Blocks,
    Height,
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MenuData>()
            // .add_systems(Update, render_hotkeys)
            .add_systems(
                Update,
                (render_left_panel, render_options_windows)
                    .run_if(input_toggle_active(true, KeyCode::Escape))
                    .run_if(home::is_universe_loaded),
            );
    }
}

const MENU_OPTIONS: &[(&str, MenuOption, fn(&mut egui::Ui))] = &[
    ("Controls", MenuOption::Controls, render_controls),
    ("Time", MenuOption::Time, render_time),
    ("Blocks", MenuOption::Blocks, render_blocks),
    ("Height", MenuOption::Height, height::render_gui),
];

fn render_options_windows(world: &mut World) {
    let mut egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .single(world)
        .clone();

    let mut window_states = world.resource_mut::<MenuData>();

    for option in &window_states.open_windows.clone() {
        if let Some((label, _, render_fn)) = MENU_OPTIONS.iter().find(|(_, opt, _)| opt == option) {
            let mut window_open = true;

            egui::Window::new(*label)
                .open(&mut window_open)
                .show(egui_context.get_mut(), |ui| {
                    render_fn(ui);
                });

            if !window_open {
                window_states.open_windows.remove(option);
            }
        }
    }
}

fn render_left_panel(world: &mut World) {
    let mut egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .single(world)
        .clone();

    let mut window_states = world.resource_mut::<MenuData>();

    egui::SidePanel::left("menu")
        .default_width(200.0)
        .resizable(false)
        .show(egui_context.get_mut(), |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                ui.heading("Myrmex");
                for (label, option, _) in MENU_OPTIONS {
                    if ui
                        .selectable_label(window_states.open_windows.contains(option), *label)
                        .clicked()
                    {
                        if window_states.open_windows.contains(option) {
                            window_states.open_windows.remove(option);
                        } else {
                            window_states.open_windows.insert(option.clone());
                        }
                    }
                }
                ui.allocate_space(ui.available_size());
            });
        });
}

fn render_controls(ui: &mut egui::Ui) {
    ui.heading("Controls");
    ui.separator();
    ui.label("Camera controls:");
    ui.monospace("W - Move up");
    ui.monospace("A - Move left");
    ui.monospace("S - Move backward");
    ui.monospace("D - Move right");
    ui.separator();
    ui.monospace("Esc - Toggle full screen");
}

fn render_time(ui: &mut egui::Ui) {
    ui.heading("Time");
}

fn render_blocks(ui: &mut egui::Ui) {
    ui.heading("Blocks");
}
