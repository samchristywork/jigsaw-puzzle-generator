use crate::svg;
use crate::transform;
use crate::vector;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct Piece {
    pub svg_string: String,
    pub control_points: Vec<vector::Vector>,
}

impl Piece {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            svg_string: String::new(),
            control_points: Vec::new(),
        }
    }

    pub fn add_string(&mut self, svg_string: &str) {
        self.svg_string += svg_string;
    }

    pub fn add(&mut self, mut svg_string: Self) {
        self.svg_string += svg_string.svg_string.as_str();
        self.control_points.append(&mut svg_string.control_points);
    }
}

fn draw_side_variant(t: &transform::Transform, seedx: i32, seedy: i32, seedi: u16) -> Piece {
    let mut res = Piece::new();

    let salt = 125;
    let mut hasher = DefaultHasher::new();
    salt.hash(&mut hasher);
    if seedi == 0 {
        seedy.hash(&mut hasher);
    }
    if seedi == 1 {
        seedx.hash(&mut hasher);
    }
    if seedi == 2 {
        (seedy - 1).hash(&mut hasher);
    }
    if seedi == 3 {
        (seedx - 1).hash(&mut hasher);
    }
    let hash = hasher.finish();
    let mut inverted = if hash % 2 == 0 { 1.0 } else { -1.0 };
    if seedi == 2 || seedi == 3 {
        inverted *= -1.0;
    }

    let mut points = vec![
        vector::Vector { x: 0.25, y: 0.50 }, // 0
        vector::Vector { x: 0.50, y: 0.50 }, // 1
        vector::Vector {
            x: 0.25,
            y: 0.25f32.mul_add(inverted, 0.50),
        }, // 2
        vector::Vector {
            x: 0.00,
            y: 0.50f32.mul_add(inverted, 0.50),
        }, // 3
        vector::Vector {
            x: 1.00,
            y: 0.50f32.mul_add(inverted, 0.50),
        }, // 4
        vector::Vector {
            x: 0.75,
            y: 0.25f32.mul_add(inverted, 0.50),
        }, // 5
        vector::Vector { x: 0.50, y: 0.50 }, // 6
        vector::Vector { x: 0.75, y: 0.50 }, // 7
        vector::Vector { x: 1.50, y: 0.50 }, // 8
    ];

    let mut hasher = DefaultHasher::new();
    salt.hash(&mut hasher);
    seedx.hash(&mut hasher);
    seedi.hash(&mut hasher);
    let x = (hasher.finish() % 50) as f32 / 100.0 - 0.25;

    let mut hasher = DefaultHasher::new();
    salt.hash(&mut hasher);
    seedy.hash(&mut hasher);
    seedi.hash(&mut hasher);
    let y = (hasher.finish() % 50) as f32 / 100.0 - 0.25;

    let skew = vector::Vector {
        x: x * inverted,
        y: y * inverted,
    };
    points[1] += skew;
    points[2] += skew;
    points[3] += skew;
    points[4] += skew;
    points[5] += skew;
    points[6] += skew;

    res.add_string(svg::draw_quadratic_curve(t, points[0], points[1], points[2]).as_str());

    res.add_string(svg::draw_quadratic_curve(t, points[3], points[4], points[5]).as_str());

    res.add_string(svg::draw_quadratic_curve(t, points[6], points[7], points[8]).as_str());

    // if seed == 0 {
    //     res.control_points.append(&mut points);
    // }

    res
}

#[must_use]
pub fn make(origin: (i32, i32), mut t: &mut transform::Transform) -> Piece {
    let mut res = Piece::new();

    let stroke = 0.004;

    res.add_string(svg::path_begin().as_str());
    res.add_string(
        svg::move_to(
            t,
            vector::Vector {
                x: origin.0 as f32 * 2.0,
                y: origin.1 as f32 * 2.0 + 0.5,
            },
        )
        .as_str(),
    );

    println!("Piece {:?}", origin);

    t.operations.push(transform::Operation {
        kind: transform::Kind::Offset,
        v: vector::Vector { x: -0.5, y: -0.75 },
    });

    let oldops = t.operations.clone();
    for i in 0u16..4 {
        t.operations = oldops.clone();

        t.operations.push(transform::Operation {
            kind: transform::Kind::Rotate,
            v: vector::Vector {
                x: std::f32::consts::PI / 2.0 * f32::from(i),
                y: 0.0,
            },
        });

        t.operations.push(transform::Operation {
            kind: transform::Kind::Scale,
            v: vector::Vector { x: 0.5, y: 0.5 },
        });

        t.operations.push(transform::Operation {
            kind: transform::Kind::Offset,
            v: vector::Vector { x: 0.5, y: 0.5 },
        });

        t.operations.push(transform::Operation {
            kind: transform::Kind::Offset,
            v: vector::Vector {
                x: origin.0 as f32,
                y: origin.1 as f32,
            },
        });

        let s = transform::Transform {
            operations: t.operations.clone(),
        };
        res.add(draw_side_variant(&s, origin.0, origin.1, i));
    }

    res.add_string(svg::path_end("darkblue", "black", stroke).as_str());

    res
}
