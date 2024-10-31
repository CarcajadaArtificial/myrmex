use bevy::prelude::*;
use bevy_inspector_egui::bevy_inspector::hierarchy::SelectedEntities;

#[derive(PartialEq, Clone, Default)]
pub(crate) enum MenuOption {
    /// The default menu option that displays camera and control options.
    #[default]
    Controls,
    /// A menu option that displays information related to entities in the world.
    Entities,
}

/// Renders the controls panel in the UI, providing an overview of camera controls
/// and key bindings.
///
/// ## Parameters
///
/// - `world`
///     A mutable reference to the `World` object in Bevy, used to access game state data if needed
///     (currently unused in this function).
///
/// - `selected_entities`
///     A reference to the selected entities within the UI, allowing UI rendering logic
///     based on entity selections (currently unused in this function).
///
/// - `ui`
///     A mutable reference to the `egui::Ui` context, used to render the control options.
///
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

/// Renders the entities panel in the UI, displaying detailed information about selected entities.
/// If a single entity is selected, it displays individual details; if multiple entities are selected,
/// it displays shared components.
///
/// ## Parameters
///
/// - `world`
///     A mutable reference to the `World` object, enabling access to entity data within the game
///     world, which can be used for displaying detailed information about entities.
///
/// - `selected_entities`
///     A reference to the currently selected entities in the inspector UI, allowing
///     this function to display either individual or shared entity data based on the selection.
///
/// - `ui`
///     A mutable reference to the `egui::Ui` context, enabling UI elements to display entity data.
///
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

/// A constant array of tuples representing the menu options available in the UI.
/// Each tuple contains a label, a `MenuOption` variant, and a function pointer to
/// the corresponding render function.
///
/// - The `Controls` option renders camera and key binding information.
/// - The `Entities` option displays entity-related details for selected entities.
///
pub const MENU_OPTIONS: &[(
    &str,
    MenuOption,
    fn(&mut World, &SelectedEntities, &mut egui::Ui),
)] = &[
    ("Controls", MenuOption::Controls, render_controls),
    ("Entities", MenuOption::Entities, render_entities),
];
