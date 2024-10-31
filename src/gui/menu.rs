use bevy::prelude::*;
use bevy_inspector_egui::bevy_inspector::hierarchy::SelectedEntities;

#[derive(PartialEq, Clone, Default)]
pub(crate) enum MenuOption {
    #[default]
    Controls,
    Entities,
}

/// Displays the control options in the UI.
fn render_controls(_: &mut World, _: &SelectedEntities, ui: &mut egui::Ui) {
    ui.heading("Controls");
    ui.separator();
    ui.label("W - Move Forward");
    ui.label("A - Move Left");
    ui.label("S - Move Backward");
    ui.label("D - Move Right");
    ui.separator();
    ui.label("Press Esc to toggle the UI.");
}

/// Displays entity options, showing details for either a single selected entity or shared components.
fn render_entities(world: &mut World, selected_entities: &SelectedEntities, ui: &mut egui::Ui) {
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
    ("Entities", MenuOption::Entities, render_entities),
];
