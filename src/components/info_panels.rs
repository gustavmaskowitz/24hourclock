use leptos::prelude::*;
use crate::modules::*;
use super::slot_detail::SlotDetail;

#[component]
pub fn InfoPanels(
    meetings: ReadSignal<Vec<Meeting>>,
    set_meetings: WriteSignal<Vec<Meeting>>,
    selected_slot: ReadSignal<Option<SelectedSlot>>,
    active_zones: ReadSignal<ActiveTimezones>,
    theme: Signal<&'static ThemeColors>,
) -> impl IntoView {
    let overlap_slots = move || {
        let z = active_zones.get();
        (0u32..24).filter(|&h| is_full_overlap_utc(h as f64, &z.zones)).collect::<Vec<_>>()
    };

    let meetings_in_overlap = move || {
        let z = active_zones.get();
        let m = meetings.get();
        m.iter().filter(|meeting| {
            is_full_overlap_utc(meeting.utc_hour as f64, &z.zones)
        }).cloned().collect::<Vec<_>>()
    };

    let meetings_outside = move || {
        let z = active_zones.get();
        let m = meetings.get();
        m.iter().filter(|meeting| {
            !is_full_overlap_utc(meeting.utc_hour as f64, &z.zones)
        }).cloned().collect::<Vec<_>>()
    };

    view! {
        {move || {
            let t = *theme.get();
            let z = active_zones.get();
            let slots = overlap_slots();
            let in_overlap = meetings_in_overlap();
            let outside = meetings_outside();

            let ref_offset = z.zones[0].utc_offset;
            let overlap_hours_str = slots.iter().map(|&h| {
                let local = utc_to_local(h as f64, ref_offset);
                format!("{:02}:00", local.floor() as u32 % 24)
            }).collect::<Vec<_>>().join(", ");

            view! {
                <div style="flex: 1; min-width: 250px; max-width: 400px; display: flex; flex-direction: column; gap: 12px">
                    // Overlap summary
                    <div style=format!(
                        "padding: 12px 16px; background: {}; border: 1px solid {}; border-radius: 8px; transition: all 0.3s ease",
                        t.success_bg, t.success_border
                    )>
                        <h3 style=format!("font-weight: 600; font-size: 0.75rem; margin-bottom: 4px; color: {}", t.success_text_dark)>
                            {format!("Overlap Window: {} hours", slots.len())}
                        </h3>
                        <p style=format!("font-size: 0.75rem; color: {}", t.success_text)>
                            {if slots.is_empty() {
                                "No overlap hours found".to_string()
                            } else {
                                format!("{} ({} timezone)", overlap_hours_str, z.zones[0].short_name)
                            }}
                        </p>
                        <p style=format!("font-size: 0.75rem; margin-top: 4px; color: {}", t.success_text)>
                            {format!("{} meeting(s) in overlap window", in_overlap.len())}
                        </p>
                    </div>

                    // Meetings outside overlap (conditional)
                    {if !outside.is_empty() {
                        Some(view! {
                            <div style=format!(
                                "padding: 12px 16px; background: {}; border: 1px solid {}; border-radius: 8px; transition: all 0.3s ease",
                                t.warning_bg, t.warning_border
                            )>
                                <h3 style=format!("font-weight: 600; font-size: 0.75rem; margin-bottom: 8px; color: {}", t.warning_text_dark)>
                                    {format!("{} meeting(s) outside overlap", outside.len())}
                                </h3>
                                {outside.iter().map(|m| {
                                    let meeting_id = m.id;
                                    let local_hour = utc_to_local(m.utc_hour as f64, ref_offset);
                                    let title = m.title.clone();
                                    view! {
                                        <div style=format!(
                                            "display: flex; align-items: center; justify-content: space-between; padding: 4px 0; gap: 8px; font-size: 0.75rem; color: {}",
                                            t.warning_text
                                        )>
                                            <span style="overflow: hidden; text-overflow: ellipsis; white-space: nowrap">
                                                {format!("{:02}:00 {} - {}", local_hour.floor() as u32 % 24, z.zones[0].short_name, title)}
                                            </span>
                                            <button
                                                style=format!("flex-shrink: 0; cursor: pointer; background: none; border: none; color: {}", t.warning_text)
                                                on:click=move |_| {
                                                    set_meetings.update(|m| m.retain(|meeting| meeting.id != meeting_id));
                                                }
                                            >
                                                "\u{2715}"
                                            </button>
                                        </div>
                                    }
                                }).collect_view()}
                            </div>
                        })
                    } else {
                        None
                    }}

                    // Selected slot detail
                    <SlotDetail
                        selected_slot=selected_slot
                        meetings=meetings
                        set_meetings=set_meetings
                        active_zones=active_zones
                        theme=theme
                    />
                </div>
            }
        }}
    }
}
