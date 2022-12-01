use crate::transform::Transform;
use crate::vector::Vector;

fn draw_control_point(x: f32, y: f32) {
    let stroke = 0.01;
    println!(
        "<path d=\"M{x},{y} l0,.01 l.01,0 l0,-.01 z\" \
        fill=\"none\" \
        stroke=\"red\" \
        stroke-width=\"{stroke}\"\
        />"
    );
}

pub fn draw_control_points(t: &Transform, mut x: Vector, mut y: Vector, mut z: Vector) {
    x = t.apply(x);
    y = t.apply(y);
    z = t.apply(z);

    let a = x.x;
    let b = x.y;
    let c = y.x;
    let d = y.y;
    let e = z.x;
    let f = z.y;
    draw_control_point(a, b);
    draw_control_point(c, d);
    draw_control_point(e, f);
}

pub fn draw_quadratic_curve(t: &Transform, mut x: Vector, mut y: Vector, mut z: Vector) {
    x = t.apply(x);
    y = t.apply(y);
    z = t.apply(z);

    let a = x.x;
    let b = x.y;
    let c = y.x;
    let d = y.y;
    let e = z.x;
    let f = z.y;
    println!("C{a},{b} {c},{d} {e},{f} ");
}

pub fn path_begin() {
    println!("<path d=\"");
}

pub fn path_end(fill: &str, stroke: &str, stroke_width: f32) {
    println!("Z\" fill=\"{fill}\" stroke=\"{stroke}\" stroke-width=\"{stroke_width}\" />");
}

pub fn move_to(t: &Transform, mut v: Vector) {
    v = t.apply(v);

    let a = v.x;
    let b = v.y;

    println!("M{a},{b} ");
}

pub fn svg_start(width: f32, height: f32) {
    println!("<svg viewBox=\"0 0 {width} {height}\" xmlns=\"http://www.w3.org/2000/svg\">");
}

pub fn line_to(t: &Transform, mut v: Vector) {
    v = t.apply(v);

    let x = v.x;
    let y = v.y;

    println!("L{x}, {y}");
}

pub fn draw_box(x: f32, y: f32, width: f32, height: f32, stroke: f32) {
    println!("<path d=\"M{x},{y} l0,{height} l{width},0 l0,-{height} z\" fill=\"none\" stroke=\"black\" stroke-width=\"{stroke}\"/>");
}

pub fn svg_end() {
    println!("</svg>");
}
