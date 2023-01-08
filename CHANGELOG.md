# Changelog

## v0.0.21

### Changed app style override

[`~/src/gui/visuals.rs`](/src/gui/visuals.rs), [`~/src/gui/mod.rs`](/src/gui/mod.rs)
- Combined `insert_fonts()` and `set_visuals()` into `set_app_style()`;

### Moved windows renders to the `window.rs` module

[`~/src/gui/window.rs`](/src/gui/window.rs)
- Added window render functions.
- Finished the environment window.
