use leptos::prelude::*;
use crate::modules::*;

#[component]
pub fn NowHighlight(
    current_utc: ReadSignal<f64>,
    ring_assignments: ReadSignal<RingAssignments>,
    theme: ThemeColors,
) -> impl IntoView {
    let current_outer_hour = move || {
        get_timezone_hour(current_utc.get(), ring_assignments.get().outer)
    };

    let highlight_color = theme.now_highlight;

    view! {
        {move || {
            let outer_hour = current_outer_hour();
            let hour_int = outer_hour.floor() as u32;
            let exact_angle = get_hour_angle(outer_hour);

            // Outline paths for each ring at current hour
            let outer_path = segment_path(CX, CY, OUTER_OUTER_R, OUTER_INNER_R, hour_int as f64, (hour_int + 1) as f64);
            let middle_path = segment_path(CX, CY, MIDDLE_OUTER_R, MIDDLE_INNER_R, hour_int as f64, (hour_int + 1) as f64);
            let inner_path = segment_path(CX, CY, INNER_OUTER_R, INNER_INNER_R, hour_int as f64, (hour_int + 1) as f64);

            // Radial line from center to outer edge
            let (line_inner_x, line_inner_y) = polar_to_cartesian(CX, CY, CENTER_R, exact_angle);
            let (line_outer_x, line_outer_y) = polar_to_cartesian(CX, CY, OUTER_OUTER_R, exact_angle);

            view! {
                <g style="pointer-events: none">
                    <path d=outer_path fill="none" stroke=highlight_color stroke-width="3" />
                    <path d=middle_path fill="none" stroke=highlight_color stroke-width="3" />
                    <path d=inner_path fill="none" stroke=highlight_color stroke-width="3" />
                    <line
                        x1=line_inner_x
                        y1=line_inner_y
                        x2=line_outer_x
                        y2=line_outer_y
                        stroke="#ec4899"
                        stroke-width="3"
                        stroke-linecap="round"
                    />
                </g>
            }
        }}
    }
}
