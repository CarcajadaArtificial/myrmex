# Changelog

## Working on: Observe all layers of the universe individually

- [x] Add tridimensional universe creation.
- [ ] Add a resource for the current z layer in view, start at half the
      universe's height.
- [ ] Add ui control for moving the z layers of the map.
  - [ ] Add a display for the current z layer in view.
  - [ ] Add buttons that move the view one layer up and down.
  - [ ] Add buttons that move the view to the top, bottom and half of the
        universe.

### Next up: Control the flow of time

- [ ] Add datepicker and time input for the current (starting) datetime of
      universe during creation.
- [ ] Add a resource for the current date time information, end-tick delay
- [ ] Add ui control for advancing time.
  - [ ] Add a display for the current date and time.
  - [ ] Add a button that moves to the following second.
  - [ ] Add a play/pause button with an input that sets the delay after
        computing a tick (default 800ms).

## Changes

### v0.0.66

- Added the app/height module.

### v0.0.65

- Refactored the loading module into a bevy plugin.

### v0.0.64

- Moved the menu and camera modules inside the app module.

### v0.0.63

- Added the third dimension to the universe creation.

### v0.0.62

- Added save file loading and implemented a resource for their content.

### v0.0.61

- Added the SaveFileData resource and updated it when loading a save file.

### v0.0.60

- Refacrtored the save file loading ui to the app module.

### v0.0.59

- Added the AppPlugin and the HomePlugin functions.

### v0.0.58

- Added a tilemap setup rendering for optimization.

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
