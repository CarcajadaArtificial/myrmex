use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_inspector_egui::bevy_inspector::hierarchy::SelectedEntities;
use bevy_window::PrimaryWindow;

fn show_panel_options(
    ui: &mut egui::Ui,
    panel_option: &mut MenuOption,
    panel_labels: &[(
        &str,
        MenuOption,
        fn(&mut World, &SelectedEntities, &mut egui::Ui),
    )],
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

/// Shows options in a floating window instead of a side panel
fn show_options_window(
    world: &mut World,
    selected_entities: &SelectedEntities,
    egui_context: &mut EguiContext,
    panel_option: &MenuOption,
) {
    egui::Window::new("Options")
        .default_pos([940.0, 40.0]) // Position it on the right side
        .default_size([200.0, 300.0])
        .resizable(true)
        .show(egui_context.get_mut(), |ui| {
            if let Some((_, _, render_fn)) =
                MENU_OPTIONS.iter().find(|(_, opt, _)| opt == panel_option)
            {
                render_fn(world, selected_entities, ui);
            }
            ui.allocate_space(ui.available_size());
        });
}

fn show_entity_hierarchy(
    world: &mut World,
    ui: &mut egui::Ui,
    selected_entities: &mut SelectedEntities,
    panel_option: &MenuOption,
) {
    if *panel_option == MenuOption::Entities {
        ui.collapsing("Hierarchy", |ui| {
            bevy_inspector_egui::bevy_inspector::hierarchy::hierarchy_ui(
                world,
                ui,
                selected_entities,
            );
        });
    }
}

fn show_left_panel(
    world: &mut World,
    selected_entities: &mut SelectedEntities,
    egui_context: &mut EguiContext,
    panel_option: &mut MenuOption,
) {
    egui::SidePanel::left("menu")
        .default_width(200.0)
        .resizable(false)
        .show(egui_context.get_mut(), |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                ui.heading("Myrmex");

                show_panel_options(ui, panel_option, MENU_OPTIONS);
                show_entity_hierarchy(world, ui, selected_entities, panel_option);

                ui.allocate_space(ui.available_size());
            });
        });
}

fn _show_right_panel(
    world: &mut World,
    selected_entities: &SelectedEntities,
    egui_context: &mut EguiContext,
    panel_option: &MenuOption,
) {
    egui::SidePanel::right("options")
        .default_width(350.0)
        .resizable(false)
        .show(egui_context.get_mut(), |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                if let Some((_, _, render_fn)) =
                    MENU_OPTIONS.iter().find(|(_, opt, _)| opt == panel_option)
                {
                    render_fn(world, selected_entities, ui);
                }

                ui.allocate_space(ui.available_size());
            });
        });
}

pub fn inspector(
    world: &mut World,
    mut selected_entities: Local<SelectedEntities>,
    mut panel_option: Local<MenuOption>,
) {
    let mut egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .single(world)
        .clone();

    show_left_panel(
        world,
        &mut selected_entities,
        &mut egui_context,
        &mut panel_option,
    );
    // show_right_panel(world, &selected_entities, &mut egui_context, &panel_option);

    // Replace show_right_panel with show_options_window
    show_options_window(world, &selected_entities, &mut egui_context, &panel_option);
}

#[derive(PartialEq, Clone, Default)]
pub(crate) enum MenuOption {
    /// The default menu option that displays camera and control options.
    #[default]
    Controls,
    /// A menu option that displays information related to entities in the world.
    Entities,
}

fn render_controls(_: &mut World, _: &SelectedEntities, ui: &mut egui::Ui) {
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

fn _render_entities(world: &mut World, selected_entities: &SelectedEntities, ui: &mut egui::Ui) {
    ui.heading("Entity Options");
    match selected_entities.as_slice() {
        &[entity] => {
            bevy_inspector_egui::bevy_inspector::ui_for_entity(world, entity, ui);
        }
        entities => {
            bevy_inspector_egui::bevy_inspector::ui_for_entities_shared_components(
                world, entities, ui,
            );
        }
    }
}

pub const MENU_OPTIONS: &[(
    &str,
    MenuOption,
    fn(&mut World, &SelectedEntities, &mut egui::Ui),
)] = &[
    ("Controls", MenuOption::Controls, render_controls),
    // ("Entities", MenuOption::Entities, render_entities),
];
