use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_window::PrimaryWindow;

fn show_panel_options(
    ui: &mut egui::Ui,
    panel_option: &mut MenuOption,
    panel_labels: &[(&str, MenuOption, fn(&mut egui::Ui))],
) {
    for (label, option, _) in panel_labels {
        if ui
            .selectable_label(*panel_option == *option, *label)
            .clicked()
        {
            *panel_option = option.clone();
        }
    }
}

fn show_options_window(egui_context: &mut EguiContext, panel_option: &MenuOption) {
    egui::Window::new("Options")
        .resizable(true)
        .show(egui_context.get_mut(), |ui| {
            if let Some((_, _, render_fn)) =
                MENU_OPTIONS.iter().find(|(_, opt, _)| opt == panel_option)
            {
                render_fn(ui);
            }
            // ui.allocate_space(ui.available_size());
        });
}

fn show_left_panel(egui_context: &mut EguiContext, panel_option: &mut MenuOption) {
    egui::SidePanel::left("menu")
        .default_width(200.0)
        .resizable(false)
        .show(egui_context.get_mut(), |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                ui.heading("Myrmex");
                show_panel_options(ui, panel_option, MENU_OPTIONS);
                ui.allocate_space(ui.available_size());
            });
        });
}

pub fn inspector(world: &mut World, mut panel_option: Local<MenuOption>) {
    let mut egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .single(world)
        .clone();

    show_left_panel(&mut egui_context, &mut panel_option);
    show_options_window(&mut egui_context, &panel_option);
}

#[derive(PartialEq, Clone, Default)]
pub(crate) enum MenuOption {
    #[default]
    Controls,
    Time,
    Blocks,
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

pub const MENU_OPTIONS: &[(&str, MenuOption, fn(&mut egui::Ui))] = &[
    ("Controls", MenuOption::Controls, render_controls),
    ("Time", MenuOption::Time, render_time),
    ("Blocks", MenuOption::Blocks, render_blocks),
];
