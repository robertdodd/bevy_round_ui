# Changelog

## [Unreleased]

**BREAKING CHANGES**: This release contains breaking changes.

### Added

- `BevyRoundUiDefaultPlugins` plugin to include default plugins for all enabled features.
- `round_rect` and `superellipse` Cargo features (both enabled by default).
- `SuperellipseUiMaterial` for rendering an approximate superellipse (requires `superellipse` Cargo feature).
- `superellipse` and `compare` examples.

### Removed

- `RoundUiPlugin` - Use `BevyRoundUiDefaultPlugins` instead, or add specific plugins individually.

### Changed

- Renamed `RoundUiMaterial` to `RoundRectUiMaterial`.
- Default value for `RoundRectUiMaterial::border` to `Color::None`.
- Updated `buttons` example to include superellipse and round-rect materials.

## 1.1.0

### Added

- `top`, `bottom`, `left` and `right` factory methods to `RoundUiBorder`

## 1.0.0

### Removed

- `autosize` feature
- `RoundUiMaterial::size` property
