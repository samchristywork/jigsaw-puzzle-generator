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

pub fn draw_control_points(a: f32, b: f32, c: f32, d: f32, e: f32, f: f32) {
    draw_control_point(a, b);
    draw_control_point(c, d);
    draw_control_point(e, f);
}

pub fn draw_quadratic_curve(a: f32, b: f32, c: f32, d: f32, e: f32, f: f32) {
    println!("C{a},{b} {c},{d} {e},{f} ");
}

pub fn path_begin() {
    println!("<path d=\"");
}

pub fn path_end(fill: &str, stroke: &str, stroke_width: f32) {
    println!("Z\" fill=\"{fill}\" stroke=\"{stroke}\" stroke-width=\"{stroke_width}\" />");
}

pub fn move_to(a: f32, b: f32) {
    println!("M{a},{b} ");
}

pub fn svg_start(width: f32, height: f32) {
    println!("<svg viewBox=\"0 0 {width} {height}\" xmlns=\"http://www.w3.org/2000/svg\">");
}

pub fn line_to(x: f32, y: f32) {
    println!("L{x}, {y}");
}

pub fn draw_box(x: f32, y: f32, width: f32, height: f32, stroke: f32) {
    println!("<path d=\"M{x},{y} l0,{height} l{width},0 l0,-{height} z\" fill=\"none\" stroke=\"black\" stroke-width=\"{stroke}\"/>");
}

pub fn svg_end() {
    println!("</svg>");
}
