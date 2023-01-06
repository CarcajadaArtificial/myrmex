# Changelog

## v0.0.16

### Rearranged gui app modules

[`~/src/gui.rs`](/src/gui.rs), [`~/src/menus.rs`](/src/menus.rs), [`~/src/main.rs`](/src/main.rs)
- Removed `menus.rs` module because global variables are in the app struct.
- Moved the app gui to its own module (`gui.rs`) and cleaned `main.rs`.
- Added the `widget_environment_is_open` property to the app's gui.