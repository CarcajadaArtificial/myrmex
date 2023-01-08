# Changelog

## v0.0.20

### Changed fonts module to visuals module

[`~/src/gui/visuals.rs`](/src/gui/visuals.rs), [`~/src/gui/window.rs`](/src/gui/window.rs), [`~/src/gui/mod.rs`](/src/gui/mod.rs)
- Removed `fonts.rs`.
- Removed the frame override in `window.rs`.
- Added `set_visuals` to `gui/mod.rs`.

### Project notebook for misc notes.

[`~/docs/notebook.md`](/docs/notebook.md)
- Added notes for `visuals.rs`.

### Added the crono crate

[`~/Cargo.toml`](/Cargo.toml), [`~/src/gui/mod.rs`](/src/gui/mod.rs)
- Added the dependency to `Cargo.toml`.
- Added timedate implementation to `gui/mod.rs`.
