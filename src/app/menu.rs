use super::height;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_window::PrimaryWindow;
use std::collections::HashSet;

#[derive(Resource, Default)]
pub struct MenuWindowsState {
    open_windows: HashSet<MenuOption>,
}

#[derive(PartialEq, Clone, Default, Hash, Eq)]
pub(crate) enum MenuOption {
    #[default]
    Controls,
    Time,
    Blocks,
    Height,
}

fn show_panel_options(
    ui: &mut egui::Ui,
    window_states: &mut MenuWindowsState,
    panel_labels: &[(&str, MenuOption, fn(&mut egui::Ui))],
) {
    for (label, option, _) in panel_labels {
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
}

fn show_options_windows(egui_context: &mut EguiContext, window_states: &mut MenuWindowsState) {
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

fn show_left_panel(egui_context: &mut EguiContext, window_states: &mut MenuWindowsState) {
    egui::SidePanel::left("menu")
        .default_width(200.0)
        .resizable(false)
        .show(egui_context.get_mut(), |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                ui.heading("Myrmex");
                show_panel_options(ui, window_states, MENU_OPTIONS);
                ui.allocate_space(ui.available_size());
            });
        });
}

pub fn inspector(world: &mut World) {
    let mut egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .single(world)
        .clone();

    let mut window_states = world.resource_mut::<MenuWindowsState>();

    show_left_panel(&mut egui_context, &mut window_states);
    show_options_windows(&mut egui_context, &mut window_states);
}

pub const MENU_OPTIONS: &[(&str, MenuOption, fn(&mut egui::Ui))] = &[
    ("Controls", MenuOption::Controls, render_controls),
    ("Time", MenuOption::Time, render_time),
    ("Blocks", MenuOption::Blocks, render_blocks),
    ("Height", MenuOption::Height, height::render_gui),
];

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
