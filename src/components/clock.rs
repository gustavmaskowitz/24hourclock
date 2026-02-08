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
    ring_assignments: ReadSignal<RingAssignments>,
    theme: Signal<&'static ThemeColors>,
) -> impl IntoView {
    view! {
        {move || {
            let t = *theme.get();
            let assignments = ring_assignments.get();
            let ref_tz = assignments.outer;

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

                    // Outer ring segments
                    {(0u32..24).map(|h| view! {
                        <ClockSegment
                            hour=h
                            outer_r=OUTER_OUTER_R
                            inner_r=OUTER_INNER_R
                            ring_tz=assignments.outer
                            reference_tz=ref_tz
                            theme=t
                            meetings=meetings
                            set_selected=set_selected_slot
                            is_outer_ring=true
                        />
                    }).collect_view()}

                    // Middle ring segments
                    {(0u32..24).map(|h| view! {
                        <ClockSegment
                            hour=h
                            outer_r=MIDDLE_OUTER_R
                            inner_r=MIDDLE_INNER_R
                            ring_tz=assignments.middle
                            reference_tz=ref_tz
                            theme=t
                            meetings=meetings
                            set_selected=set_selected_slot
                            is_outer_ring=false
                        />
                    }).collect_view()}

                    // Inner ring segments
                    {(0u32..24).map(|h| view! {
                        <ClockSegment
                            hour=h
                            outer_r=INNER_OUTER_R
                            inner_r=INNER_INNER_R
                            ring_tz=assignments.inner
                            reference_tz=ref_tz
                            theme=t
                            meetings=meetings
                            set_selected=set_selected_slot
                            is_outer_ring=false
                        />
                    }).collect_view()}

                    // Ring dividers
                    <circle cx=CX cy=CY r=OUTER_INNER_R fill="none" stroke=t.ring_divider stroke-width="1" stroke-dasharray="2,2" />
                    <circle cx=CX cy=CY r=MIDDLE_INNER_R fill="none" stroke=t.ring_divider stroke-width="1" stroke-dasharray="2,2" />

                    // Now highlight
                    <NowHighlight current_utc=current_utc ring_assignments=ring_assignments theme=t />

                    // Center display
                    <CenterDisplay current_utc=current_utc ring_assignments=ring_assignments theme=t />
                </svg>
            }
        }}
    }
}
