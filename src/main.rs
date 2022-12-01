pub mod svg;

fn main() {
    let width = 1.01;
    let height = 1.01;
    let stroke = 0.01;

    svg::svg_start(width, height);

    svg::path_begin();
    svg::move_to(0.00, 0.50);

    svg::draw_quadratic_curve(0.25, 0.50, 0.50, 0.50, 0.25, 0.25);
    svg::draw_quadratic_curve(0.00, 0.00, 1.00, 0.00, 0.75, 0.25);
    svg::draw_quadratic_curve(0.50, 0.50, 0.75, 0.50, 1.00, 0.50);

    svg::line_to(1.0, 1.0);
    svg::line_to(0.0, 1.0);
    svg::path_end("blue", "black", stroke);

    svg::draw_control_points(0.25, 0.50, 0.50, 0.50, 0.25, 0.25);
    svg::draw_control_points(0.00, 0.00, 1.00, 0.00, 0.75, 0.25);
    svg::draw_control_points(0.50, 0.50, 0.75, 0.50, 1.00, 0.50);

    svg::draw_box(0.0, 0.0, 1.0, 1.0, stroke);

    svg::svg_end();
}
