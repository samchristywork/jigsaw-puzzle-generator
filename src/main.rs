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

    let t = transform::Transform { operations: vec![] };

    svg::path_begin();
    svg::move_to(&t, Vector { x: 0.00, y: 0.50 });

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

    svg::line_to(&t, Vector { x: 1.0, y: 1.0 });
    svg::line_to(&t, Vector { x: 0.0, y: 1.0 });
    svg::path_end("blue", "black", stroke);

    svg::draw_control_points(
        &t,
        Vector { x: 0.25, y: 0.50 },
        Vector { x: 0.50, y: 0.50 },
        Vector { x: 0.25, y: 0.25 },
    );
    svg::draw_control_points(
        &t,
        Vector { x: 0.00, y: 0.00 },
        Vector { x: 1.00, y: 0.00 },
        Vector { x: 0.75, y: 0.25 },
    );
    svg::draw_control_points(
        &t,
        Vector { x: 0.50, y: 0.50 },
        Vector { x: 0.75, y: 0.50 },
        Vector { x: 1.00, y: 0.50 },
    );

    svg::draw_box(0.0, 0.0, 1.0, 1.0, stroke);

    svg::svg_end();
}
