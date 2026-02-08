use leptos::prelude::*;
use crate::modules::*;

#[component]
pub fn CenterDisplay(
    current_utc: ReadSignal<f64>,
    ring_assignments: ReadSignal<RingAssignments>,
    theme: ThemeColors,
) -> impl IntoView {
    let minutes = move || {
        let now = js_sys::Date::new_0();
        format!("{:02}", now.get_utc_minutes())
    };

    let outer_time = move || {
        let a = ring_assignments.get();
        let h = get_timezone_hour(current_utc.get(), a.outer);
        format!("{:02}:{} {}", h.floor() as u32, minutes(), a.outer.short_name())
    };

    let middle_time = move || {
        let a = ring_assignments.get();
        let h = get_timezone_hour(current_utc.get(), a.middle);
        format!("{:02}:{} {}", h.floor() as u32, minutes(), a.middle.short_name())
    };

    let inner_time = move || {
        let a = ring_assignments.get();
        let h = get_timezone_hour(current_utc.get(), a.inner);
        format!("{:02}:{} {}", h.floor() as u32, minutes(), a.inner.short_name())
    };

    let center_bg = theme.center_circle_bg;
    let text_muted = theme.text_muted;
    let text_outer = theme.clock_text_outer;
    let text_middle = theme.clock_text_middle;
    let text_inner = theme.clock_text_inner;

    view! {
        <g>
            <circle cx=CX cy=CY r=CENTER_R fill=center_bg stroke="#ec4899" stroke-width="3" />
            <text x=CX y={CY - 22.0} text-anchor="middle" font-size="9" fill=text_muted>
                "NOW"
            </text>
            <text x=CX y={CY - 4.0} text-anchor="middle" font-size="11" font-weight="600" fill=text_outer>
                {outer_time}
            </text>
            <text x=CX y={CY + 10.0} text-anchor="middle" font-size="11" font-weight="600" fill=text_middle>
                {middle_time}
            </text>
            <text x=CX y={CY + 24.0} text-anchor="middle" font-size="11" font-weight="600" fill=text_inner>
                {inner_time}
            </text>
        </g>
    }
}
