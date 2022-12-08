# Changelog

## [1.1.0] - 2022-12-07

- Can create whole boards of puzzle pieces
- Every piece fits together with the pieces surrounding it
- Configurable zoom levels and board size
- Deformation is constrained so that pieces don't have intersecting curves
- The board has flat borders
- Amount of space between the pieces is configurable

## [1.0.0] - 2022-12-03

### Added

- Render differentiated puzzle pieces as SVG
- Rich composable transformation infrastructure
- Pieces composed of Bézier curves
- Control points can be shown for debugging
- SVG module gives access to useful drawing primitives like curves and text
- Piece differentiation generated from deterministic hashes
- Shapes are enclosed vector paths. This is an important property for some algorithms
- Vector struct with functions for composability
- SVG is built as a string that can be written to a file

### Changed

- None (initial release)
