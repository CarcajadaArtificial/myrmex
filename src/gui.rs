use bevy::prelude::*;
use bevy_egui::EguiContext;
use bevy_inspector_egui::bevy_inspector::hierarchy::SelectedEntities;
use bevy_window::PrimaryWindow;

/// This function sets up an interactive UI for inspecting entities within the Bevy `World`. It uses
/// `egui` to display two side panels: one for the entity hierarchy and another for inspecting selected
/// entities or their shared components.
///
/// ## Parameters
///
/// - `world`
///     A mutable reference to the Bevy `World`, allowing access to all entities and components.
///
/// - `selected_entities`
///     A local state that keeps track of the entities currently selected in the hierarchy UI.
///
/// ## UI Structure
///
/// - **Left Panel (Hierarchy)**
///     Displays a collapsible view of the entity hierarchy. The user can select entities from this
///     view, which will then be available for inspection in the right panel.
///
/// - **Right Panel (Options)**
///     Displays the components and data for the currently selected entity (or shared components for
///     multiple selected entities). The user can inspect and modify entity data here.
pub fn inspector(world: &mut World, mut selected_entities: Local<SelectedEntities>) {
    // Retrieve the Egui context from the primary window.
    let mut egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .single(world)
        .clone();

    // Define the left side panel for displaying the entity hierarchy.
    egui::SidePanel::left("menu")
        .default_width(200.0)
        .show(egui_context.get_mut(), |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                ui.heading("Myrmex");
                ui.label("Press escape to toggle full screen");

                ui.collapsing("Hierarchy", |ui| {
                    bevy_inspector_egui::bevy_inspector::hierarchy::hierarchy_ui(
                        world,
                        ui,
                        &mut selected_entities,
                    );
                });

                ui.allocate_space(ui.available_size());
            });
        });

    // Define the right side panel for displaying options and inspecting entity components.
    egui::SidePanel::right("options")
        .default_width(250.0)
        .show(egui_context.get_mut(), |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                ui.heading("Options");

                // Display the UI for either a single selected entity or shared components for multiple selected entities.
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

                ui.allocate_space(ui.available_size());
            });
        });
}
