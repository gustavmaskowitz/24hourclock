use leptos::prelude::*;
use crate::modules::*;
use super::clock_segment::ClockSegment;
use super::now_highlight::NowHighlight;
use super::center_display::CenterDisplay;

#[component]
pub fn Clock(
    meetings: ReadSignal<Vec<Meeting>>,
    set_selected_slot: WriteSignal<Option<SelectedSlot>>,
    current_utc: ReadSignal<f64>,
    active_zones: ReadSignal<ActiveTimezones>,
    theme: Signal<&'static ThemeColors>,
) -> impl IntoView {
    view! {
        {move || {
            let t = *theme.get();
            let zones = active_zones.get();
            let n = zones.zones.len();
            let geos = compute_ring_geometries(n);
            let ref_offset = zones.zones[0].utc_offset;

            view! {
                <svg
                    width="100%"
                    height="auto"
                    viewBox="0 0 400 400"
                    class="drop-shadow-lg"
                    style="max-width: 600px; margin: 0 auto; display: block"
                >
                    // Background circle
                    <circle cx=CX cy=CY r=BG_R fill=t.svg_bg stroke=t.svg_border stroke-width="2" />

                    // Render all ring segments dynamically
                    {geos.iter().enumerate().flat_map(|(ring_idx, geo)| {
                        let tz = &zones.zones[ring_idx];
                        let tz_offset = tz.utc_offset;
                        let zones_clone = zones.zones.clone();
                        let outer_r = geo.outer_r;
                        let inner_r = geo.inner_r;
                        (0u32..24).map(move |h| {
                            let zones_for_segment = zones_clone.clone();
                            view! {
                                <ClockSegment
                                    hour=h
                                    outer_r=outer_r
                                    inner_r=inner_r
                                    ring_index=ring_idx
                                    tz_offset=tz_offset
                                    ref_offset=ref_offset
                                    theme=t
                                    meetings=meetings
                                    set_selected=set_selected_slot
                                    active_zones=zones_for_segment
                                />
                            }
                        }).collect::<Vec<_>>()
                    }).collect_view()}

                    // Ring dividers (N-1 dashed circles)
                    {geos.iter().take(n.saturating_sub(1)).map(|geo| {
                        view! {
                            <circle cx=CX cy=CY r=geo.inner_r fill="none" stroke=t.ring_divider stroke-width="1" stroke-dasharray="2,2" />
                        }
                    }).collect_view()}

                    // Now highlight
                    <NowHighlight current_utc=current_utc active_zones=active_zones theme=t />

                    // Center display
                    <CenterDisplay current_utc=current_utc active_zones=active_zones theme=t />
                </svg>
            }
        }}
    }
}
