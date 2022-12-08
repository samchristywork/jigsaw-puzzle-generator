pub mod piece;
pub mod svg;
pub mod transform;
pub mod vector;

use std::fs::File;
use std::io::Write;
use transform::Operation;
use vector::Vector;

fn main() {
    let zoom = 6.0;
    let width = 1.0 + zoom;
    let height = 1.0 + zoom;

    let mut piece = piece::Piece::new();

    piece.add_string(svg::start(-zoom / 2.0, -zoom / 2.0, width, height).as_str());

    let width = 9;

    for x in -width / 2..width / 2 + 1 {
        for y in -width / 2..width / 2 + 1 {
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

            piece.add(piece::make(
                (x, y),
                &mut transform::Transform {
                    operations: t.operations,
                },
                0.5,
                width,
            ));
        }
    }

    //for (i, point) in piece.control_points.clone().iter().enumerate() {
    //    let sz = 0.01;
    //    piece.add_string(
    //        svg::draw_box_with_label(
    //            point.x,
    //            point.y,
    //            sz,
    //            sz,
    //            0.01,
    //            "red",
    //            i.to_string().as_str(),
    //        )
    //        .as_str(),
    //    );
    //}

    //t.operations.push(Operation {
    //    kind: transform::Kind::Offset,
    //    v: Vector { x: 0.05, y: 0.00 },
    //});

    //make(transform::Transform {
    //    operations: t.operations.clone(),
    //});

    piece.add_string(svg::draw_box(0.0, 0.0, 1.0, 1.0, 0.01, "black").as_str());
    piece.add_string(svg::draw_box(0.25, 0.25, 0.5, 0.5, 0.0025, "green").as_str());

    piece.add_string(svg::end().as_str());

    let filename = "out.svg";
    let mut output =
        File::create("out.svg").unwrap_or_else(|_| panic!("Could not create file {}.", filename));
    write!(output, "{}", piece.svg_string).expect("Could not write to file.");
}
