# Changelog

### v0.0.57

- Added closable windows for the menu module.

### v0.0.56

- Refactored the menu module.

### v0.0.55

- Added windows as menu options instead of a right panel.

### v0.0.54

- Refactored the state and save file modules.

### v0.0.53

- Renamed GameState to HomeState, and UniverseData to SaveFileData.

### v0.0.52

- Refacred the home module.

### v0.0.51

- Added a change of scene after selecting a save file.
- Renamed GameState::universe_dimensions to input_universe_dimensions.

### v0.0.50

- Added the functionality to list and read save files.

### v0.0.49

- Added save file creation functionality.
- Refactored the app state to its own file.

### v0.0.48

- Removed the AppState functionality in favor of the `GameState` resource.

### v0.0.47

- Refactored the AppState functionality to remove the `CreatingUniverse` scene.

### v0.0.46

- Refactored the AppState functionality to the `home.rs` module.
- Refactored the `run_universe()` function to the `app/mod.rs` module.
- Refactored the `gui/mod.rs` functionality to the `menu.rs` module.
- Updated the `v0.1.md` scope document.

### v0.0.45

- Added back the gui and camera testing implementations to the `LoadedUniverse`
  app state.

### v0.0.44

- Refactored the functionality from `gui_home()`, `gui_create_universe()`, and
  `AppState` to the `gui` module.

### v0.0.43

- Removed testing implementations of tilemaps and gui via comments.
- Added Home view and universcre creation view for creating and viewing save
  files.

### v0.0.42

- Added documentation to the menu options of the gui.

### v0.0.41

- Refactored the menu selection functionality its own directory and the options
  to the `menu.rs` file.

### v0.0.40

- Refactored the menu selection functionality to the `PANEL_LABELS` const.

### v0.0.39

- Added selectable labels to the left panel for menu options.
