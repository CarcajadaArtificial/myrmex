# Changelog

## v0.0.25

### Moved the opening window functionality to `world_properties`.

[`~/src/gui/world_properties.rs`](/src/gui/world_properties.rs), [`~/src/gui/window.rs`](/src/gui/window.rs), [`~/src/gui/mod.rs`](/src/gui/mod.rs)
- Added public gate properties for each window.
- Removed the previous gate properties from the global state struct.
- Removed title bar from the `window` component.
