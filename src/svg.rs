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

pub fn draw_control_points(t: &Transform, mut x: Vector, mut y: Vector, mut z: Vector) -> String {
    x = t.apply(x);
    y = t.apply(y);
    z = t.apply(z);

    let a = x.x;
    let b = x.y;
    let c = y.x;
    let d = y.y;
    let e = z.x;
    let f = z.y;

    let mut res = String::new();

    res += draw_control_point(a, b).as_str();
    res += draw_control_point(c, d).as_str();
    res += draw_control_point(e, f).as_str();

    res
}

pub fn draw_quadratic_curve(t: &Transform, mut x: Vector, mut y: Vector, mut z: Vector) -> String {
    x = t.apply(x);
    y = t.apply(y);
    z = t.apply(z);

    let a = x.x;
    let b = x.y;
    let c = y.x;
    let d = y.y;
    let e = z.x;
    let f = z.y;
    format!("C{a},{b} {c},{d} {e},{f} ")
}

pub fn path_begin() -> String {
    format!("<path d=\"")
}

pub fn path_end(fill: &str, stroke: &str, stroke_width: f32) -> String {
    format!("Z\" fill=\"{fill}\" stroke=\"{stroke}\" stroke-width=\"{stroke_width}\" />")
}

pub fn move_to(t: &Transform, mut v: Vector) -> String {
    v = t.apply(v);

    let a = v.x;
    let b = v.y;

    format!("M{a},{b} ")
}

pub fn svg_start(x: f32, y: f32, width: f32, height: f32) -> String {
    format!("<svg viewBox=\"{x} {y} {width} {height}\" xmlns=\"http://www.w3.org/2000/svg\">")
}

pub fn line_to(t: &Transform, mut v: Vector) -> String {
    v = t.apply(v);

    let x = v.x;
    let y = v.y;

    format!("L{x}, {y}")
}

pub fn draw_box(x: f32, y: f32, width: f32, height: f32, stroke: f32, color: &str) -> String {
    format!(
        "<path d=\"M{x},{y} l0,{height} l{width},0 l0,-{height} z\" \
        fill=\"none\" stroke=\"{color}\" stroke-width=\"{stroke}\"/>"
    )
}

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

pub fn svg_end() -> String {
    format!("</svg>")
}
