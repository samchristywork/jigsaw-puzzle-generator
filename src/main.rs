pub mod svg;
pub mod transform;
pub mod vector;

use transform::Operation;
use vector::Vector;


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

    svg::path_begin();
    svg::move_to(&t, Vector { x: 0.00, y: 0.50 });

    let oldops = t.operations.clone();
    for i in 0..4 {
        t.operations = oldops.clone();

        t.operations.push(Operation {
            kind: transform::Kind::Offset,
            v: Vector { x: -0.5, y: -0.5 },
        });

        t.operations.push(Operation {
            kind: transform::Kind::Rotate,
            v: Vector {
                x: 3.141 / 2.0 * i as f32,
                y: 0.0,
            },
        });

        t.operations.push(Operation {
            kind: transform::Kind::Offset,
            v: Vector { x: 0.5, y: 0.5 },
        });

        svg::draw_quadratic_curve(
            &t,
            Vector { x: 0.25, y: 0.50 },
            Vector { x: 0.50, y: 0.50 },
            Vector { x: 0.25, y: 0.25 },
        );
        svg::draw_quadratic_curve(
            &t,
            Vector { x: 0.00, y: 0.00 },
            Vector { x: 1.00, y: 0.00 },
            Vector { x: 0.75, y: 0.25 },
        );
        svg::draw_quadratic_curve(
            &t,
            Vector { x: 0.50, y: 0.50 },
            Vector { x: 0.75, y: 0.50 },
            Vector { x: 1.00, y: 0.50 },
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

    svg::draw_box(0.0, 0.0, 1.0, 1.0, stroke);

    svg::svg_end();
}
