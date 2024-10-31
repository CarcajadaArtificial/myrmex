use bevy::prelude::*;
use bevy_egui::EguiContext;
use bevy_inspector_egui::bevy_inspector::hierarchy::SelectedEntities;
use bevy_window::PrimaryWindow;

#[derive(PartialEq, Clone, Default)]
pub(crate) enum PanelOption {
    #[default]
    Controls,
    Entities,
}

/// This function presents a list of panel options as selectable labels in the UI, allowing
/// the user to switch between different view modes, such as "Controls" and "Entities".
///
/// ## Parameters
///
/// - `ui`
///     A mutable reference to the `egui::Ui` context, providing UI elements for rendering the options.
///
/// - `panel_option`
///     A mutable reference to the currently selected `PanelOption`, allowing the function to update
///     the selected panel based on user interaction.
///
/// - `panel_labels`
///     A slice of labels and their associated `PanelOption` values, defining the selectable options
///     in the UI.
///
fn show_panel_options(
    ui: &mut egui::Ui,
    panel_option: &mut PanelOption,
    panel_labels: &[(&str, PanelOption)],
) {
    for (label, option) in panel_labels {
        if ui
            .selectable_label(*panel_option == *option, *label)
            .clicked()
        {
            *panel_option = option.clone();
        }
    }
}

/// This function provides an interactive, collapsible view of the entity hierarchy, enabling
/// users to explore and select entities within the `World` for inspection in the right panel.
///
/// ## Parameters
///
/// - `world`
///     A mutable reference to the Bevy `World`, allowing access to entities and components for
///     displaying the hierarchy.
///
/// - `ui`
///     A mutable reference to the `egui::Ui` context used to render the collapsible hierarchy section.
///
/// - `selected_entities`
///     A mutable reference to `SelectedEntities`, managing the selection state of entities within the hierarchy.
///
/// - `panel_option`
///     The current `PanelOption`, used to conditionally display the hierarchy view.
///
fn show_entity_hierarchy(
    world: &mut World,
    ui: &mut egui::Ui,
    selected_entities: &mut SelectedEntities,
    panel_option: &PanelOption,
) {
    if *panel_option == PanelOption::Entities {
        ui.collapsing("Hierarchy", |ui| {
            bevy_inspector_egui::bevy_inspector::hierarchy::hierarchy_ui(
                world,
                ui,
                selected_entities,
            );
        });
    }
}

/// This function configures the left panel, setting up a UI area for entity hierarchy exploration.
/// Users can toggle between "Controls" and "Entities" views, with "Entities" revealing a detailed
/// hierarchy for selection.
///
/// ## Parameters
///
/// - `world`
///     A mutable reference to the Bevy `World`, providing access to entities and components for
///     rendering the hierarchy.
///
/// - `selected_entities`
///     A mutable reference to `SelectedEntities`, tracking the selection state of entities.
///
/// - `egui_context`
///     A mutable reference to `EguiContext`, allowing the function to render the UI for the panel.
///
/// - `panel_option`
///     A mutable reference to the current `PanelOption`, enabling panel switching based on user input.
///
fn show_left_panel(
    world: &mut World,
    selected_entities: &mut SelectedEntities,
    egui_context: &mut EguiContext,
    panel_option: &mut PanelOption,
) {
    let panel_labels = [
        ("Controls", PanelOption::Controls),
        ("Entities", PanelOption::Entities),
    ];

    egui::SidePanel::left("menu")
        .default_width(200.0)
        .resizable(false)
        .show(egui_context.get_mut(), |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                ui.heading("Myrmex");

                show_panel_options(ui, panel_option, &panel_labels);
                show_entity_hierarchy(world, ui, selected_entities, panel_option);

                ui.allocate_space(ui.available_size());
            });
        });
}

/// This function configures the right panel, presenting inspection options for the selected
/// entity or shared components across multiple selected entities. Users can review and modify
/// entity data here.
///
/// ## Parameters
///
/// - `world`
///     A mutable reference to the Bevy `World`, allowing access to entities and components for
///     inspection in the right panel.
///
/// - `selected_entities`
///     A reference to `SelectedEntities`, representing the current entity selection.
///
/// - `egui_context`
///     A mutable reference to `EguiContext`, allowing the function to render the UI for the inspection panel.
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
pub fn inspector(
    world: &mut World,
    mut selected_entities: Local<SelectedEntities>,
    mut panel_option: Local<PanelOption>,
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
    show_right_panel(world, &selected_entities, &mut egui_context);
}
