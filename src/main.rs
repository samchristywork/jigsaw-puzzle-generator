pub mod svg;
pub mod transform;
pub mod vector;

use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::Write;
use transform::Operation;
use vector::Vector;

struct Piece {
    svg_string: String,
    control_points: Vec<Vector>,
}

impl Piece {
    pub fn new() -> Self {
        Self {
            svg_string: String::new(),
            control_points: Vec::new(),
        }
    }

    pub fn add_string(&mut self, svg_string: String) {
        self.svg_string += svg_string.as_str();
    }

    pub fn add(&mut self, mut svg_string: Self) {
        self.svg_string += svg_string.svg_string.as_str();
        self.control_points.append(&mut svg_string.control_points);
    }
}

fn draw_side_variant(t: transform::Transform, seed: i32) -> Piece {
    let mut res = Piece::new();

    let salt = 129;

    let mut hasher = DefaultHasher::new();
    (seed + salt).hash(&mut hasher);
    let hash = hasher.finish();

    let mut inverted = 0.0;
    if hash % 2 == 0 {
        inverted = 1.0;
    }

    let mut points = vec![
        Vector { x: 0.25, y: 0.50 }, // 0
        Vector { x: 0.50, y: 0.50 }, // 1
        Vector {
            x: 0.25,
            y: 0.25 + 0.50 * inverted,
        }, // 2
        Vector {
            x: 0.00,
            y: 0.00 + inverted,
        }, // 3
        Vector {
            x: 1.00,
            y: 0.00 + inverted,
        }, // 4
        Vector {
            x: 0.75,
            y: 0.25 + 0.50 * inverted,
        }, // 5
        Vector { x: 0.50, y: 0.50 }, // 6
        Vector { x: 0.75, y: 0.50 }, // 7
        Vector { x: 1.50, y: 0.50 }, // 8
    ];

    let skew = Vector { x: 0.2, y: -0.1 };
    points[1] += skew;
    points[2] += skew;
    points[3] += skew;
    points[4] += skew;
    points[5] += skew;
    points[6] += skew;

    res.add_string(svg::draw_quadratic_curve(
        &t, points[0], points[1], points[2],
    ));

    res.add_string(svg::draw_quadratic_curve(
        &t, points[3], points[4], points[5],
    ));

    res.add_string(svg::draw_quadratic_curve(
        &t, points[6], points[7], points[8],
    ));

    if seed == 0 {
        res.control_points.append(&mut points);
    }

    res
}

fn make_piece(mut t: transform::Transform) -> Piece {
    let mut res = Piece::new();

    let stroke = 0.01;

    res.add_string(svg::path_begin());
    res.add_string(svg::move_to(&t, Vector { x: 0.00, y: 0.50 }));

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
        res.add(draw_side_variant(s, i));
    }

    res.add_string(svg::path_end("blue", "black", stroke));

    res
}

fn main() {
    let width = 1.2;
    let height = 1.2;
    let stroke = 0.01;

    let mut piece = Piece::new();

    piece.add_string(svg::svg_start(-0.1, -0.1, width, height));

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

    piece.add(make_piece(transform::Transform {
        operations: t.operations.clone(),
    }));

    for point in piece.control_points.clone() {
        let sz = 0.01;
        piece.add_string(svg::draw_box(point.x, point.y, sz, sz, stroke));
    }

    //t.operations.push(Operation {
    //    kind: transform::Kind::Offset,
    //    v: Vector { x: 0.05, y: 0.00 },
    //});

    //make_piece(transform::Transform {
    //    operations: t.operations.clone(),
    //});

    piece.add_string(svg::draw_box(0.0, 0.0, 1.0, 1.0, stroke));

    piece.add_string(svg::svg_end());

    let mut output = File::create("out.svg").unwrap();
    write!(output, "{}", piece.svg_string).unwrap();
}
