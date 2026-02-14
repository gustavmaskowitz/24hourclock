use leptos::prelude::*;
use crate::modules::*;

#[component]
pub fn ClockSegment(
    hour: u32,
    outer_r: f64,
    inner_r: f64,
    ring_index: usize,
    tz_offset: f64,
    ref_offset: f64,
    theme: ThemeColors,
    meetings: ReadSignal<Vec<Meeting>>,
    set_selected: WriteSignal<Option<SelectedSlot>>,
    active_zones: Vec<TimezoneEntry>,
) -> impl IntoView {
    let display_hour = convert_between(hour as f64, ref_offset, tz_offset);
    let is_working = is_work_hour(display_hour);
    let fill = if is_working { "#22c55e" } else { theme.ring_defaults[ring_index] };

    let path = segment_path(CX, CY, outer_r, inner_r, hour as f64, (hour + 1) as f64);
    let (lx, ly) = label_position(CX, CY, outer_r, inner_r, hour as f64);

    // Format label: show HH:MM for fractional timezones, HH for integer
    let frac = display_hour % 1.0;
    let label_text = if frac.abs() < 0.01 || (1.0 - frac).abs() < 0.01 {
        format!("{:02}", display_hour.floor() as u32 % 24)
    } else {
        let mins = (frac * 60.0).round() as u32;
        format!("{:02}:{:02}", display_hour.floor() as u32 % 24, mins)
    };

    // Dynamic font size based on ring thickness
    let font_size = ((outer_r - inner_r) / 5.0).clamp(5.0, 8.0);

    // Compute UTC hour for meeting lookup
    let utc_hour = ((hour as f64 - ref_offset + 48.0) % 24.0).round() as u32 % 24;

    // Meeting dot only on outermost ring (ring_index == 0)
    let meeting_dot = if ring_index == 0 {
        let meetings_val = meetings.get_untracked();
        meetings_val.iter().find(|m| m.utc_hour == utc_hour).map(|m| {
            let dot_color = if m.essential { theme.meeting_essential } else { theme.meeting_non_essential };
            let bg = theme.background;
            (dot_color, bg)
        })
    } else {
        None
    };

    let on_click = {
        let active_zones = active_zones.clone();
        move |_| {
            let local_hours: Vec<f64> = active_zones.iter()
                .map(|tz| utc_to_local(utc_hour as f64, tz.utc_offset))
                .collect();
            set_selected.set(Some(SelectedSlot {
                utc_hour,
                local_hours,
            }));
        }
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
                font-size=font_size
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
