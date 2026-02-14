use leptos::prelude::*;
use crate::modules::*;

#[component]
pub fn NowHighlight(
    current_utc: ReadSignal<f64>,
    active_zones: ReadSignal<ActiveTimezones>,
    theme: ThemeColors,
) -> impl IntoView {
    let highlight_color = theme.now_highlight;

    view! {
        {move || {
            let zones = active_zones.get();
            let n = zones.zones.len();
            let geos = compute_ring_geometries(n);
            let ref_offset = zones.zones[0].utc_offset;
            let outer_hour = utc_to_local(current_utc.get(), ref_offset);
            let hour_int = outer_hour.floor() as u32;
            let exact_angle = get_hour_angle(outer_hour);

            let highlight_paths: Vec<String> = geos.iter().map(|geo| {
                segment_path(CX, CY, geo.outer_r, geo.inner_r, hour_int as f64, (hour_int + 1) as f64)
            }).collect();

            let (line_inner_x, line_inner_y) = polar_to_cartesian(CX, CY, CENTER_R, exact_angle);
            let (line_outer_x, line_outer_y) = polar_to_cartesian(CX, CY, geos[0].outer_r, exact_angle);

            view! {
                <g style="pointer-events: none">
                    {highlight_paths.into_iter().map(|path| {
                        view! {
                            <path d=path fill="none" stroke=highlight_color stroke-width="3" />
                        }
                    }).collect_view()}
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
