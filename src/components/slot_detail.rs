use leptos::prelude::*;
use crate::modules::*;

#[component]
pub fn SlotDetail(
    selected_slot: ReadSignal<Option<SelectedSlot>>,
    meetings: ReadSignal<Vec<Meeting>>,
    set_meetings: WriteSignal<Vec<Meeting>>,
    active_zones: ReadSignal<ActiveTimezones>,
    theme: Signal<&'static ThemeColors>,
) -> impl IntoView {
    let (new_title, set_new_title) = signal(String::new());
    let (new_essential, set_new_essential) = signal(false);

    view! {
        {move || {
            let slot = selected_slot.get()?;
            let t = *theme.get();
            let z = active_zones.get();

            // Build time label dynamically
            let time_label = slot.local_hours.iter().zip(z.zones.iter()).map(|(h, tz)| {
                let hour_part = h.floor() as u32 % 24;
                let frac = tz.utc_offset % 1.0;
                if frac.abs() < 0.01 {
                    format!("{:02}:00 {}", hour_part, tz.short_name)
                } else {
                    let mins = (frac.abs() * 60.0).round() as u32;
                    format!("{:02}:{:02} {}", hour_part, mins, tz.short_name)
                }
            }).collect::<Vec<_>>().join(" = ");

            // Check overlap
            let full_overlap = is_full_overlap_utc(slot.utc_hour as f64, &z.zones);

            let overlap_msg = if full_overlap {
                (format!("\u{2713} All timezones in working hours \u{2014} ideal!"), t.success_text)
            } else {
                let outside: Vec<&str> = z.zones.iter().filter_map(|tz| {
                    let local = utc_to_local(slot.utc_hour as f64, tz.utc_offset);
                    if !is_work_hour(local) { Some(tz.name) } else { None }
                }).collect();
                (format!("\u{26A0} {} outside working hours", outside.join(", ")), t.warning_text)
            };

            let slot_meetings: Vec<Meeting> = meetings.get().into_iter()
                .filter(|m| m.utc_hour == slot.utc_hour)
                .collect();

            let utc_hour_for_add = slot.utc_hour;

            Some(view! {
                <div style=format!(
                    "padding: 12px 16px; background: {}; border: 1px solid {}; border-radius: 8px; transition: all 0.3s ease",
                    t.card_bg, t.card_border
                )>
                    <h3 style=format!("font-weight: 600; font-size: 0.75rem; margin-bottom: 4px; color: {}", t.text_primary)>
                        {time_label}
                    </h3>
                    <p style=format!("font-size: 0.75rem; margin-bottom: 12px; color: {}", overlap_msg.1)>
                        {overlap_msg.0}
                    </p>

                    // Existing meetings at this slot
                    {slot_meetings.iter().map(|m| {
                        let meeting_id = m.id;
                        let dot_color = if m.essential { t.meeting_essential } else { t.meeting_non_essential };
                        let title = m.title.clone();
                        view! {
                            <div style=format!(
                                "display: flex; align-items: center; justify-content: space-between; padding: 8px; border-radius: 4px; margin-bottom: 6px; gap: 8px; background: {}; border: 1px solid {}",
                                t.background, t.card_border
                            )>
                                <span style=format!("font-size: 0.75rem; display: flex; align-items: center; gap: 8px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; color: {}", t.text_primary)>
                                    <span style=format!("width: 8px; height: 8px; border-radius: 50%; flex-shrink: 0; background: {}", dot_color) />
                                    {title}
                                </span>
                                <button
                                    style=format!("font-size: 0.75rem; flex-shrink: 0; cursor: pointer; background: none; border: none; color: {}", t.warning_text)
                                    on:click=move |_| {
                                        set_meetings.update(|m| m.retain(|meeting| meeting.id != meeting_id));
                                    }
                                >
                                    "\u{2715}"
                                </button>
                            </div>
                        }
                    }).collect_view()}

                    // Add meeting form
                    <div style="margin-top: 8px; display: flex; flex-direction: column; gap: 8px">
                        <input
                            type="text"
                            placeholder="Meeting title"
                            style=format!(
                                "font-size: 0.75rem; padding: 6px 8px; border-radius: 4px; border: 1px solid {}; background: {}; color: {}; outline: none",
                                t.input_border, t.input_bg, t.text_primary
                            )
                            prop:value=move || new_title.get()
                            on:input=move |ev| set_new_title.set(event_target_value(&ev))
                        />
                        <label style=format!("font-size: 0.75rem; display: flex; align-items: center; gap: 6px; color: {}", t.text_secondary)>
                            <input
                                type="checkbox"
                                prop:checked=move || new_essential.get()
                                on:change=move |ev| set_new_essential.set(event_target_checked(&ev))
                            />
                            "Essential for cross-timezone collaboration?"
                        </label>
                        <button
                            style=format!(
                                "padding: 8px; font-size: 0.75rem; border: none; border-radius: 6px; cursor: pointer; transition: all 0.3s ease; background: {}; color: {}",
                                t.button_primary_bg, t.button_primary_text
                            )
                            on:click=move |_| {
                                let title = new_title.get();
                                if !title.is_empty() {
                                    let id = js_sys::Date::now() as u32;
                                    set_meetings.update(|m| m.push(Meeting {
                                        id,
                                        utc_hour: utc_hour_for_add,
                                        title,
                                        essential: new_essential.get(),
                                    }));
                                    set_new_title.set(String::new());
                                    set_new_essential.set(false);
                                }
                            }
                        >
                            "+ Add Meeting at This Time"
                        </button>
                    </div>
                </div>
            })
        }}
    }
}
