pub mod piece;
pub mod svg;
pub mod transform;
pub mod vector;

use std::fs::File;
use std::io::Write;
use transform::Operation;
use vector::Vector;

fn main() {
    let zoom = 0.5;
    let width = 1.0 + zoom;
    let height = 1.0 + zoom;

    let mut piece = piece::Piece::new();

    piece.add_string(svg::svg_start(-zoom / 2.0, -zoom / 2.0, width, height));

    let t = transform::Transform {
        operations: vec![
            Operation {
                kind: transform::Kind::Scale,
                v: Vector { x: 0.5, y: 0.5 },
            },
            Operation {
                kind: transform::Kind::Offset,
                v: Vector { x: 0.25, y: 0.0 },
            },
        ],
    };

    piece.add(piece::make_piece(transform::Transform {
        operations: t.operations,
    }));

    for (i, point) in piece.control_points.clone().iter().enumerate() {
        let sz = 0.01;
        piece.add_string(svg::draw_box_with_label(
            point.x,
            point.y,
            sz,
            sz,
            0.01,
            "red",
            i.to_string().as_str(),
        ));
    }

    //t.operations.push(Operation {
    //    kind: transform::Kind::Offset,
    //    v: Vector { x: 0.05, y: 0.00 },
    //});

    //make_piece(transform::Transform {
    //    operations: t.operations.clone(),
    //});

    piece.add_string(svg::draw_box(0.0, 0.0, 1.0, 1.0, 0.01, "black"));
    piece.add_string(svg::draw_box(0.25, 0.25, 0.5, 0.5, 0.0025, "green"));

    piece.add_string(svg::svg_end());

    let mut output = File::create("out.svg").unwrap();
    write!(output, "{}", piece.svg_string).unwrap();
}
