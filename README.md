![Banner](https://s-christy.com/status-banner-service/arithmetic-expression-parser/banner-slim.svg)

## Overview

This is a program that generates puzzle pieces, and renders them to SVG images
that can be viewed in a web browser or processed by other applications. The
puzzle pieces are created by connecting Bézier curves. The shape of these curves
are determined by the position of control points. Altering the X and Y offsets
of sets of control points is the primary way that variation in the shape of the
pieces is generated.

## Example

<div>
<img alt="Puzzle piece with control points." src="./res/example.svg">
</div>

## Features

- Render differentiated puzzle pieces as SVG
- Rich composable transformation infrastructure
- Pieces composed of Bézier curves
- Control points can be shown for debugging
- SVG module gives access to useful drawing primitives like curves and text
- Piece differentiation generated from deterministic hashes
- Shapes are enclosed vector paths. This is an important property for some algorithms
- Vector struct with functions for composability
- SVG is built as a string that can be written to a file

## License

This work is licensed under the GNU General Public License version 3 (GPLv3).

[<img src="https://s-christy.com/status-banner-service/GPLv3_Logo.svg" width="150" />](https://www.gnu.org/licenses/gpl-3.0.en.html)
