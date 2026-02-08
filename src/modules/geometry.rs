use std::f64::consts::PI;

/// Convert 0-24 hour to angle in degrees (0h = top/12 o'clock, clockwise)
pub fn get_hour_angle(hour: f64) -> f64 {
    (hour / 24.0) * 360.0 - 90.0
}

/// Convert polar coordinates to Cartesian
pub fn polar_to_cartesian(cx: f64, cy: f64, r: f64, angle_deg: f64) -> (f64, f64) {
    let rad = angle_deg * PI / 180.0;
    (cx + r * rad.cos(), cy + r * rad.sin())
}

/// Generate SVG arc path for an hour segment
pub fn segment_path(cx: f64, cy: f64, outer_r: f64, inner_r: f64, start_hour: f64, end_hour: f64) -> String {
    let start_angle = get_hour_angle(start_hour);
    let end_angle = get_hour_angle(end_hour);

    let (p1x, p1y) = polar_to_cartesian(cx, cy, outer_r, start_angle);
    let (p2x, p2y) = polar_to_cartesian(cx, cy, outer_r, end_angle);
    let (p3x, p3y) = polar_to_cartesian(cx, cy, inner_r, end_angle);
    let (p4x, p4y) = polar_to_cartesian(cx, cy, inner_r, start_angle);

    format!(
        "M {} {} A {} {} 0 0 1 {} {} L {} {} A {} {} 0 0 0 {} {} Z",
        p1x, p1y,
        outer_r, outer_r, p2x, p2y,
        p3x, p3y,
        inner_r, inner_r, p4x, p4y
    )
}

/// Get the midpoint position for a label within a segment
pub fn label_position(cx: f64, cy: f64, outer_r: f64, inner_r: f64, hour: f64) -> (f64, f64) {
    let mid_angle = get_hour_angle(hour + 0.5);
    let label_r = (outer_r + inner_r) / 2.0;
    polar_to_cartesian(cx, cy, label_r, mid_angle)
}
