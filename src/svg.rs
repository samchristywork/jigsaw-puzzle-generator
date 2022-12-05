use crate::transform::Transform;
use crate::vector::Vector;

fn draw_control_point(x: f32, y: f32) -> String {
    let stroke = 0.01;
    format!(
        "<path d=\"M{x},{y} l0,.01 l.01,0 l0,-.01 z\" \
        fill=\"none\" \
        stroke=\"red\" \
        stroke-width=\"{stroke}\"\
        />"
    )
}

#[must_use]
pub fn draw_control_points(t: &Transform, mut a: Vector, mut b: Vector, mut c: Vector) -> String {
    a = t.apply(a);
    b = t.apply(b);
    c = t.apply(c);

    let mut res = String::new();

    res += draw_control_point(a.x, a.y).as_str();
    res += draw_control_point(b.x, b.y).as_str();
    res += draw_control_point(c.x, c.y).as_str();

    res
}

#[must_use]
pub fn draw_quadratic_curve(t: &Transform, mut x: Vector, mut y: Vector, mut z: Vector) -> String {
    x = t.apply(x);
    y = t.apply(y);
    z = t.apply(z);

    format!("C{},{} {},{} {},{} ", x.x, x.y, y.x, y.y, z.x, z.y)
}

#[must_use]
pub fn path_begin() -> String {
    "<path d=\"".to_string()
}

#[must_use]
pub fn path_end(fill: &str, stroke: &str, stroke_width: f32) -> String {
    format!("Z\" fill=\"{fill}\" stroke=\"{stroke}\" stroke-width=\"{stroke_width}\" />")
}

#[must_use]
pub fn move_to(t: &Transform, mut v: Vector) -> String {
    v = t.apply(v);

    let a = v.x;
    let b = v.y;

    format!("M{a},{b} ")
}

#[must_use]
pub fn start(x: f32, y: f32, width: f32, height: f32) -> String {
    format!("<svg viewBox=\"{x} {y} {width} {height}\" xmlns=\"http://www.w3.org/2000/svg\">")
}

#[must_use]
pub fn line_to(t: &Transform, mut v: Vector) -> String {
    v = t.apply(v);

    let x = v.x;
    let y = v.y;

    format!("L{x}, {y}")
}

#[must_use]
pub fn draw_box(x: f32, y: f32, width: f32, height: f32, stroke: f32, color: &str) -> String {
    format!(
        "<path d=\"M{x},{y} l0,{height} l{width},0 l0,-{height} z\" \
        fill=\"none\" stroke=\"{color}\" stroke-width=\"{stroke}\"/>"
    )
}

#[must_use]
pub fn draw_box_with_label(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    stroke: f32,
    color: &str,
    label: &str,
) -> String {
    format!(
        "<path d=\"M{x},{y} l0,{height} l{width},0 l0,-{height} z\" \
        fill=\"none\" stroke=\"{color}\" stroke-width=\"{stroke}\"/> \
        <text x=\"{x}\" y=\"{y}\" font-size=\".04\">{label}</text>"
    )
}

#[must_use]
pub fn end() -> String {
    "</svg>".to_string()
}
