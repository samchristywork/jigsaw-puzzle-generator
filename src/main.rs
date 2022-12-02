pub mod svg;
pub mod transform;
pub mod vector;

use transform::Operation;
use vector::Vector;

fn make_piece(mut t: transform::Transform) {
    let stroke = 0.01;

    svg::path_begin();
    svg::move_to(&t, Vector { x: 0.00, y: 0.50 });

    let oldops = t.operations.clone();
    for i in 0..4 {
        t.operations = oldops.clone();

        t.operations.push(Operation {
            kind: transform::Kind::Offset,
            v: Vector { x: -0.5, y: -0.75 },
        });

        t.operations.push(Operation {
            kind: transform::Kind::Rotate,
            v: Vector {
                x: 3.141 / 2.0 * i as f32,
                y: 0.0,
            },
        });

        t.operations.push(Operation {
            kind: transform::Kind::Scale,
            v: Vector { x: 0.5, y: 0.5 },
        });

        t.operations.push(Operation {
            kind: transform::Kind::Offset,
            v: Vector { x: 0.5, y: 0.5 },
        });

        let mut inverted = 0.0;
        if i == 2 {
            inverted = 1.0;
        }
        svg::draw_quadratic_curve(
            &t,
            Vector { x: 0.25, y: 0.50 },
            Vector { x: 0.50, y: 0.50 },
            Vector {
                x: 0.25,
                y: 0.25 + 0.50 * inverted,
            },
        );
        svg::draw_quadratic_curve(
            &t,
            Vector {
                x: 0.00,
                y: 0.00 + inverted,
            },
            Vector {
                x: 1.00,
                y: 0.00 + inverted,
            },
            Vector {
                x: 0.75,
                y: 0.25 + 0.50 * inverted,
            }, //75
        );
        svg::draw_quadratic_curve(
            &t,
            Vector { x: 0.50, y: 0.50 },
            Vector { x: 0.75, y: 0.50 },
            Vector { x: 1.50, y: 0.50 },
        );
    }

    svg::path_end("blue", "black", stroke);

    //svg::draw_control_points(
    //    &t,
    //    Vector { x: 0.25, y: 0.50 },
    //    Vector { x: 0.50, y: 0.50 },
    //    Vector { x: 0.25, y: 0.25 },
    //);
    //svg::draw_control_points(
    //    &t,
    //    Vector { x: 0.00, y: 0.00 },
    //    Vector { x: 1.00, y: 0.00 },
    //    Vector { x: 0.75, y: 0.25 },
    //);
    //svg::draw_control_points(
    //    &t,
    //    Vector { x: 0.50, y: 0.50 },
    //    Vector { x: 0.75, y: 0.50 },
    //    Vector { x: 1.00, y: 0.50 },
    //);
}

fn main() {
    let width = 1.01;
    let height = 1.01;
    let stroke = 0.01;

    svg::svg_start(width, height);

    let mut t = transform::Transform {
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

    make_piece(transform::Transform {
        operations: t.operations.clone(),
    });

    //t.operations.push(Operation {
    //    kind: transform::Kind::Offset,
    //    v: Vector { x: 0.05, y: 0.00 },
    //});

    //make_piece(transform::Transform {
    //    operations: t.operations.clone(),
    //});

    svg::draw_box(0.0, 0.0, 1.0, 1.0, stroke);

    svg::svg_end();
}
