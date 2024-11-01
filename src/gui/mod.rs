use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiContexts};
use bevy_inspector_egui::bevy_inspector::hierarchy::SelectedEntities;
use bevy_window::PrimaryWindow;
mod menu;
use menu::{MenuOption, MENU_OPTIONS};

/// Enum to represent the state of the app.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Component, Default)]
pub enum AppState {
    #[default]
    Home,
    CreatingUniverse {
        x: i32,
        y: i32,
    },
    LoadedUniverse,
}

/// Presents a list of panel options as selectable labels in the UI, allowing
/// the user to toggle between different views, such as "Controls" and "Entities".
///
/// ## Parameters
///
/// - `ui`
///     A mutable reference to the `egui::Ui` context, used to render selectable labels for panel options.
///
/// - `panel_option`
///     A mutable reference to the currently selected `MenuOption`, enabling the function to update
///     the active panel based on user selection.
///
/// - `panel_labels`
///     A slice of tuples containing labels and associated `MenuOption` values,
///     representing the selectable options in the UI.
///
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

/// Provides an interactive, collapsible view of the entity hierarchy, allowing
/// users to explore and select entities within the `World` for inspection.
///
/// ## Parameters
///
/// - `world`
///     A mutable reference to the Bevy `World`, allowing access to entities and components for
///     displaying the hierarchy structure.
///
/// - `ui`
///     A mutable reference to the `egui::Ui` context used to render the collapsible hierarchy section.
///
/// - `selected_entities`
///     A mutable reference to `SelectedEntities`, managing the state of selected entities in the hierarchy.
///
/// - `panel_option`
///     The current `MenuOption`, determining if the hierarchy view is displayed.
///
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

/// Configures the left panel, which provides a UI area for toggling views and exploring
/// the entity hierarchy. When "Entities" is selected, it displays a detailed hierarchy for selection.
///
/// ## Parameters
///
/// - `world`
///     A mutable reference to the Bevy `World`, allowing access to entities and components for
///     rendering the hierarchy view.
///
/// - `selected_entities`
///     A mutable reference to `SelectedEntities`, tracking the selected entities for inspection.
///
/// - `egui_context`
///     A mutable reference to `EguiContext`, enabling the function to render the UI.
///
/// - `panel_option`
///     A mutable reference to the current `MenuOption`, enabling panel switching based on user input.
///
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

/// Configures the right panel, presenting inspection options for the selected
/// entity or shared components across multiple selected entities, allowing users to review and modify entity data.
///
/// ## Parameters
///
/// - `world`
///     A mutable reference to the Bevy `World`, providing access to entities and components for
///     inspection in the right panel.
///
/// - `selected_entities`
///     A reference to `SelectedEntities`, representing the entities currently selected in the hierarchy view.
///
/// - `egui_context`
///     A mutable reference to `EguiContext`, enabling the function to render the UI for the inspection panel.
///
/// - `panel_option`
///     A reference to the current `MenuOption`, determining which panel is currently active.
///
fn show_right_panel(
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

/// Sets up an interactive UI for inspecting entities within the Bevy `World`, using
/// `egui` to display side panels for entity hierarchy exploration and inspection of selected entities.
///
/// ## Parameters
///
/// - `world`
///     A mutable reference to the Bevy `World`, allowing access to all entities and components.
///
/// - `selected_entities`
///     Local state that keeps track of the entities currently selected in the hierarchy UI.
///
/// - `panel_option`
///     Local state representing the active `MenuOption`, enabling users to switch between different views.
///
/// ## UI Structure
///
/// - **Left Panel (Hierarchy)**:
///     Displays a toggleable view of the entity hierarchy. Users can select entities from this
///     panel, which are then available for inspection in the right panel.
///
/// - **Right Panel (Options)**:
///     Displays detailed data for the currently selected entity (or shared components for
///     multiple selected entities). Users can review and modify entity data here.
///
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
    show_right_panel(world, &selected_entities, &mut egui_context, &panel_option);
}

/// Displays the GUI with a header and button to transition to the "CreatingUniverse" state.
pub fn home(mut egui_contexts: EguiContexts, mut query: Query<&mut AppState>) {
    if let Ok(mut app_state) = query.get_single_mut() {
        egui::CentralPanel::default().show(egui_contexts.ctx_mut(), |ui| {
            ui.heading("Myrmex");
            if ui.button("create universe").clicked() {
                *app_state = AppState::CreatingUniverse { x: 32, y: 32 };
            }
        });
    }
}

/// Displays the GUI for creating a universe with inputs for dimensions and a creation button.
pub fn create_universe(mut egui_contexts: EguiContexts, mut query: Query<&mut AppState>) {
    if let Ok(mut app_state) = query.get_single_mut() {
        if let AppState::CreatingUniverse { x, y } = &mut *app_state {
            egui::CentralPanel::default().show(egui_contexts.ctx_mut(), |ui| {
                ui.heading("Create Universe");

                // Input fields for x and y dimensions with a range of 32 to 256
                ui.horizontal(|ui| {
                    ui.label("x:");
                    ui.add(egui::DragValue::new(x).range(32..=256));
                    ui.label("y:");
                    ui.add(egui::DragValue::new(y).range(32..=256));
                });

                // Button to create the universe
                if ui.button("create").clicked() {
                    println!("universe created");
                }
            });
        }
    }
}
