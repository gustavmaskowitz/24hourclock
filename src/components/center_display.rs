use leptos::prelude::*;
use crate::modules::*;

#[component]
pub fn CenterDisplay(
    current_utc: ReadSignal<f64>,
    active_zones: ReadSignal<ActiveTimezones>,
    theme: ThemeColors,
) -> impl IntoView {
    let center_bg = theme.center_circle_bg;
    let text_muted = theme.text_muted;

    view! {
        {move || {
            let zones = active_zones.get();
            let n = zones.zones.len();
            let now = js_sys::Date::new_0();
            let utc_min = now.get_utc_minutes() as f64;
            let utc_hour = current_utc.get();

            let first_y = CY - 8.0;
            let last_y = CY + 28.0;
            let spacing = if n > 1 { (last_y - first_y) / (n as f64 - 1.0) } else { 0.0 };
            let font_size = if n <= 3 { 11 } else if n == 4 { 9 } else { 8 };

            let time_lines: Vec<_> = zones.zones.iter().enumerate().map(|(i, tz)| {
                let h = utc_to_local(utc_hour, tz.utc_offset);
                let hour_part = h.floor() as u32 % 24;
                let frac = tz.utc_offset % 1.0;
                let display_mins = if frac.abs() < 0.01 {
                    format!("{:02}", utc_min as u32)
                } else {
                    let local_min = (utc_min + frac * 60.0 + 60.0) % 60.0;
                    format!("{:02}", local_min.floor() as u32)
                };
                let text = format!("{:02}:{} {}", hour_part, display_mins, tz.short_name);
                let color = theme.ring_text_colors[i.min(4)];
                let y = if n == 1 { CY + 6.0 } else { first_y + spacing * i as f64 };
                (text, color, y)
            }).collect();

            view! {
                <g>
                    <circle cx=CX cy=CY r=CENTER_R fill=center_bg stroke="#ec4899" stroke-width="3" />
                    <text x=CX y={CY - 22.0} text-anchor="middle" font-size="9" fill=text_muted>
                        "NOW"
                    </text>
                    {time_lines.into_iter().map(|(text, color, y)| {
                        view! {
                            <text x=CX y=y text-anchor="middle" font-size=font_size font-weight="600" fill=color>
                                {text}
                            </text>
                        }
                    }).collect_view()}
                </g>
            }
        }}
    }
}
