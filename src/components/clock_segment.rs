use leptos::prelude::*;
use crate::modules::*;

#[component]
pub fn ClockSegment(
    hour: u32,
    outer_r: f64,
    inner_r: f64,
    ring_tz: Timezone,
    reference_tz: Timezone,
    theme: ThemeColors,
    meetings: ReadSignal<Vec<Meeting>>,
    set_selected: WriteSignal<Option<SelectedSlot>>,
    is_outer_ring: bool,
) -> impl IntoView {
    let display_hour = convert_timezone(hour as f64, reference_tz, ring_tz);
    let is_working = is_work_hour(display_hour);
    let fill = if is_working { "#22c55e" } else { theme.segment_default };

    let path = segment_path(CX, CY, outer_r, inner_r, hour as f64, (hour + 1) as f64);
    let (lx, ly) = label_position(CX, CY, outer_r, inner_r, hour as f64);

    let label_text = format!("{:02}", display_hour.floor() as u32);

    // Compute UTC hour for meeting lookup
    let utc_hour = ((hour as f64 - reference_tz.offset() as f64 + 24.0) % 24.0).round() as u32;

    let meeting_dot = if is_outer_ring {
        let meetings_val = meetings.get_untracked();
        meetings_val.iter().find(|m| m.utc_hour == utc_hour).map(|m| {
            let dot_color = if m.essential { theme.meeting_essential } else { theme.meeting_non_essential };
            let bg = theme.background;
            (dot_color, bg)
        })
    } else {
        None
    };

    let on_click = move |_| {
        let dallas_hour = convert_timezone(hour as f64, reference_tz, Timezone::Dallas);
        let connecticut_hour = convert_timezone(hour as f64, reference_tz, Timezone::Connecticut);
        let london_hour = convert_timezone(hour as f64, reference_tz, Timezone::London);
        set_selected.set(Some(SelectedSlot {
            utc_hour,
            dallas_hour,
            connecticut_hour,
            london_hour,
        }));
    };

    view! {
        <g>
            <path
                d=path
                fill=fill
                stroke=theme.segment_stroke
                stroke-width="1"
                cursor="pointer"
                on:click=on_click
            />
            <text
                x=lx
                y=ly
                text-anchor="middle"
                dominant-baseline="middle"
                font-size="8"
                font-weight="500"
                fill=theme.text_primary
                style="pointer-events: none"
            >
                {label_text}
            </text>
            {meeting_dot.map(|(dot_color, bg)| view! {
                <circle
                    cx={lx + 10.0}
                    cy=ly
                    r="4"
                    fill=dot_color
                    stroke=bg
                    stroke-width="1"
                />
            })}
        </g>
    }
}
