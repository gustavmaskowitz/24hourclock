use leptos::prelude::*;
use crate::modules::*;
use super::slot_detail::SlotDetail;

#[component]
pub fn InfoPanels(
    meetings: ReadSignal<Vec<Meeting>>,
    set_meetings: WriteSignal<Vec<Meeting>>,
    selected_slot: ReadSignal<Option<SelectedSlot>>,
    ring_assignments: ReadSignal<RingAssignments>,
    theme: Signal<&'static ThemeColors>,
) -> impl IntoView {
    let overlap_slots = move || {
        let a = ring_assignments.get();
        (0u32..24).filter(|&h| is_full_overlap(h as f64, a.outer)).collect::<Vec<_>>()
    };

    let meetings_in_overlap = move || {
        let a = ring_assignments.get();
        let m = meetings.get();
        m.iter().filter(|meeting| {
            let outer_hour = get_timezone_hour(meeting.utc_hour as f64, a.outer);
            is_full_overlap(outer_hour, a.outer)
        }).cloned().collect::<Vec<_>>()
    };

    let meetings_outside_overlap = move || {
        let a = ring_assignments.get();
        let m = meetings.get();
        m.iter().filter(|meeting| {
            let outer_hour = get_timezone_hour(meeting.utc_hour as f64, a.outer);
            !is_full_overlap(outer_hour, a.outer)
        }).cloned().collect::<Vec<_>>()
    };

    view! {
        <div style="display: flex; flex-direction: column; gap: 12px; width: 100%; max-width: 28rem; padding: 0 8px">
            // Overlap summary
            {move || {
                let t = *theme.get();
                let slots = overlap_slots();
                let a = ring_assignments.get();
                let in_overlap_count = meetings_in_overlap().len();
                let hours_str = slots.iter()
                    .map(|h| format!("{:02}:00", h))
                    .collect::<Vec<_>>()
                    .join(", ");
                let tz_name = format!("{}", a.outer.name());

                view! {
                    <div style=format!(
                        "padding: 12px 16px; background: {}; border: 1px solid {}; border-radius: 8px; transition: all 0.3s ease",
                        t.success_bg, t.success_border
                    )>
                        <h3 style=format!("font-weight: 600; font-size: 0.75rem; margin-bottom: 4px; color: {}", t.success_text_dark)>
                            {format!("Full Overlap: {} hours available", slots.len())}
                        </h3>
                        <p style=format!("font-size: 0.75rem; word-break: break-word; color: {}", t.success_text)>
                            {hours_str} " " {tz_name}
                        </p>
                        <p style=format!("font-size: 0.75rem; margin-top: 8px; color: {}", t.success_text)>
                            {format!("{} meetings in overlap window", in_overlap_count)}
                        </p>
                    </div>
                }
            }}

            // Meetings outside overlap warning
            {move || {
                let t = *theme.get();
                let outside = meetings_outside_overlap();
                let a = ring_assignments.get();

                if outside.is_empty() {
                    return view! { <div /> }.into_any();
                }

                view! {
                    <div style=format!(
                        "padding: 12px 16px; background: {}; border: 1px solid {}; border-radius: 8px; transition: all 0.3s ease",
                        t.warning_bg, t.warning_border
                    )>
                        <h3 style=format!("font-weight: 600; font-size: 0.75rem; margin-bottom: 8px; color: {}", t.warning_text_dark)>
                            "\u{26A0} Meetings Outside Overlap"
                        </h3>
                        {outside.iter().map(|m| {
                            let outer_hour = get_timezone_hour(m.utc_hour as f64, a.outer);
                            let label = format!(
                                "{} \u{2014} {:02}:00 {}",
                                m.title,
                                outer_hour.floor() as u32,
                                a.outer.short_name()
                            );
                            let meeting_id = m.id;
                            view! {
                                <div style=format!(
                                    "display: flex; align-items: center; justify-content: space-between; font-size: 0.75rem; padding: 6px 0; gap: 8px; color: {}",
                                    t.warning_text
                                )>
                                    <span style="overflow: hidden; text-overflow: ellipsis; white-space: nowrap">{label}</span>
                                    <button
                                        style=format!("font-size: 0.75rem; flex-shrink: 0; cursor: pointer; background: none; border: none; color: {}", t.warning_text_dark)
                                        on:click=move |_| {
                                            set_meetings.update(|m| m.retain(|meeting| meeting.id != meeting_id));
                                        }
                                    >
                                        "Remove"
                                    </button>
                                </div>
                            }
                        }).collect_view()}
                    </div>
                }.into_any()
            }}

            // Selected slot detail
            <SlotDetail
                selected_slot=selected_slot
                meetings=meetings
                set_meetings=set_meetings
                ring_assignments=ring_assignments
                theme=theme
            />
        </div>
    }
}
