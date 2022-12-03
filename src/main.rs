pub mod svg;
pub mod transform;
pub mod vector;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use transform::Operation;
use vector::Vector;

fn draw_side_variant(t: transform::Transform, mut seed: i32) -> String {
    let mut res = String::new();

    let salt = 125;
    seed += salt;

    let mut hasher = DefaultHasher::new();
    seed.hash(&mut hasher);
    let hash = hasher.finish();

    let mut inverted = 0.0;
    if hash % 2 == 0 {
        inverted = 1.0;
    }

    res += svg::draw_quadratic_curve(
        &t,
        Vector { x: 0.25, y: 0.50 },
        Vector { x: 0.50, y: 0.50 },
        Vector {
            x: 0.25,
            y: 0.25 + 0.50 * inverted,
        },
    )
    .as_str();

    res += svg::draw_quadratic_curve(
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
        },
    )
    .as_str();

    res += svg::draw_quadratic_curve(
        &t,
        Vector { x: 0.50, y: 0.50 },
        Vector { x: 0.75, y: 0.50 },
        Vector { x: 1.50, y: 0.50 },
    )
    .as_str();

    res
}

fn make_piece(mut t: transform::Transform) -> String {
    let mut res = String::new();

    let stroke = 0.01;

    res += svg::path_begin().as_str();
    res += svg::move_to(&t, Vector { x: 0.00, y: 0.50 }).as_str();

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

        let s = transform::Transform {
            operations: t.operations.clone(),
        };
        res += draw_side_variant(s, i).as_str();
    }

    res += svg::path_end("blue", "black", stroke).as_str();

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

    res
}

fn main() {
    let width = 1.01;
    let height = 1.01;
    let stroke = 0.01;

    let mut s = String::new();
    s += svg::svg_start(width, height).as_str();

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

    s += make_piece(transform::Transform {
        operations: t.operations.clone(),
    })
    .as_str();

    //t.operations.push(Operation {
    //    kind: transform::Kind::Offset,
    //    v: Vector { x: 0.05, y: 0.00 },
    //});

    //make_piece(transform::Transform {
    //    operations: t.operations.clone(),
    //});

    s += svg::draw_box(0.0, 0.0, 1.0, 1.0, stroke).as_str();

    s += svg::svg_end().as_str();

    println!("{}", s);
}
