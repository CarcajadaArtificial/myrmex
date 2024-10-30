use bevy::prelude::*;
use bevy_egui::EguiContext;
use bevy_inspector_egui::bevy_inspector::hierarchy::SelectedEntities;
use bevy_window::PrimaryWindow;

/// Displays the left panel with the entity hierarchy.
fn show_left_panel(
    world: &mut World,
    selected_entities: &mut SelectedEntities,
    egui_context: &mut EguiContext,
) {
    egui::SidePanel::left("menu")
        .default_width(200.0)
        .resizable(false)
        .show(egui_context.get_mut(), |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                ui.heading("Myrmex");
                ui.label("Press escape to toggle full screen");

                ui.collapsing("Hierarchy", |ui| {
                    bevy_inspector_egui::bevy_inspector::hierarchy::hierarchy_ui(
                        world,
                        ui,
                        selected_entities,
                    );
                });

                ui.allocate_space(ui.available_size());
            });
        });
}

/// Displays the right panel for inspecting selected entities or shared components.
fn show_right_panel(
    world: &mut World,
    selected_entities: &SelectedEntities,
    egui_context: &mut EguiContext,
) {
    egui::SidePanel::right("options")
        .default_width(350.0)
        .resizable(false)
        .show(egui_context.get_mut(), |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                ui.heading("Options");

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
    let mut egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .single(world)
        .clone();

    show_left_panel(world, &mut selected_entities, &mut egui_context);
    show_right_panel(world, &selected_entities, &mut egui_context);
}
