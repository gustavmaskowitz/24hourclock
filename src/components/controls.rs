use leptos::prelude::*;
use crate::modules::*;

#[component]
pub fn Header(
    ring_assignments: ReadSignal<RingAssignments>,
    theme_name: ReadSignal<ThemeName>,
    set_theme_name: WriteSignal<ThemeName>,
    mode: ReadSignal<Mode>,
    set_mode: WriteSignal<Mode>,
    theme: Signal<&'static ThemeColors>,
) -> impl IntoView {
    let subtitle = move || {
        let a = ring_assignments.get();
        format!(
            "{} (Outer) \u{2022} {} (Middle) \u{2022} {} (Inner)",
            a.outer.name(), a.middle.name(), a.inner.name()
        )
    };

    view! {
        {move || {
            let t = *theme.get();
            let current_theme = theme_name.get();
            let current_mode = mode.get();

            view! {
                <div
                    class="header-bar"
                    style=format!(
                        "display: flex; flex-wrap: wrap; justify-content: space-between; align-items: center; padding: 12px 32px; border-bottom: 1px solid {}; gap: 8px",
                        t.card_border
                    )
                >
                    <div>
                        <h1 style=format!("font-size: 1.5rem; font-weight: 300; margin-bottom: 4px; color: {}", t.text_primary)>
                            "Timezone Meeting Clock"
                        </h1>
                        <p style=format!("font-size: 0.75rem; color: {}", t.text_secondary)>
                            {subtitle}
                        </p>
                    </div>
                    <div style="display: flex; gap: 16px; align-items: center">
                        // Theme buttons
                        <div style="display: flex; gap: 4px; flex-wrap: wrap">
                            {ThemeName::ALL.into_iter().map(|tn| {
                                let is_active = current_theme == tn;
                                let bg = if is_active { t.button_primary_bg } else { t.button_secondary_bg };
                                let color = if is_active { t.button_primary_text } else { t.text_secondary };
                                view! {
                                    <button
                                        style=format!(
                                            "background: {}; color: {}; border: none; border-radius: 6px; padding: 4px 8px; font-size: 10px; cursor: pointer; transition: all 0.3s ease",
                                            bg, color
                                        )
                                        on:click=move |_| set_theme_name.set(tn)
                                    >
                                        {tn.label()}
                                    </button>
                                }
                            }).collect_view()}
                        </div>

                        // Light/dark toggle
                        <button
                            style=format!(
                                "background: {}; border: 1px solid {}; border-radius: 20px; padding: 3px; cursor: pointer; width: 50px; position: relative; transition: all 0.3s ease",
                                t.card_bg, t.card_border
                            )
                            on:click=move |_| {
                                set_mode.set(if current_mode == Mode::Light { Mode::Dark } else { Mode::Light });
                            }
                        >
                            <div style=format!(
                                "width: 20px; height: 20px; border-radius: 50%; background: {}; transform: translateX({}px); transition: transform 0.3s ease",
                                t.button_primary_bg,
                                if current_mode == Mode::Dark { 24 } else { 0 }
                            ) />
                        </button>
                    </div>
                </div>
            }
        }}
    }
}

#[component]
pub fn RingControls(
    ring_assignments: ReadSignal<RingAssignments>,
    set_ring_assignments: WriteSignal<RingAssignments>,
    theme: Signal<&'static ThemeColors>,
) -> impl IntoView {
    // Ring swap logic: when selecting a timezone already used by another ring, swap them
    let handle_ring_change = move |ring: Ring, new_tz: Timezone| {
        let mut current = ring_assignments.get();
        let old_tz = match ring {
            Ring::Outer => current.outer,
            Ring::Middle => current.middle,
            Ring::Inner => current.inner,
        };

        // Find which ring currently has the new timezone and swap
        if current.outer == new_tz {
            current.outer = old_tz;
        } else if current.middle == new_tz {
            current.middle = old_tz;
        } else if current.inner == new_tz {
            current.inner = old_tz;
        }

        match ring {
            Ring::Outer => current.outer = new_tz,
            Ring::Middle => current.middle = new_tz,
            Ring::Inner => current.inner = new_tz,
        }

        set_ring_assignments.set(current);
    };

    let parse_tz = |val: &str| -> Timezone {
        match val {
            "dallas" => Timezone::Dallas,
            "connecticut" => Timezone::Connecticut,
            "london" => Timezone::London,
            _ => Timezone::Dallas,
        }
    };

    view! {
        {move || {
            let t = *theme.get();
            let a = ring_assignments.get();

            let select_style = format!(
                "border: 1px solid {}; background: {}; color: {}; border-radius: 6px; padding: 4px 6px; cursor: pointer; font-size: 12px",
                t.input_border, t.input_bg, t.text_primary
            );
            let label_style = format!("font-size: 11px; display: block; margin-bottom: 4px; white-space: nowrap; color: {}", t.text_secondary);

            let rings = vec![
                ("Outer Ring", a.outer, Ring::Outer),
                ("Middle Ring", a.middle, Ring::Middle),
                ("Inner Ring", a.inner, Ring::Inner),
            ];

            view! {
                <div
                    style=format!(
                        "display: flex; justify-content: center; gap: 16px; align-items: center; padding: 12px 32px; border-bottom: 1px solid {}; overflow-x: auto",
                        t.card_border
                    )
                >
                    {rings.into_iter().map(|(label, current_tz, ring)| {
                        let sstyle = select_style.clone();
                        let lstyle = label_style.clone();
                        let current_val = match current_tz {
                            Timezone::Dallas => "dallas",
                            Timezone::Connecticut => "connecticut",
                            Timezone::London => "london",
                        };
                        view! {
                            <div style="flex-shrink: 0">
                                <label style=lstyle>{label}</label>
                                <select
                                    style=sstyle
                                    prop:value=current_val
                                    on:change=move |ev| {
                                        let val = event_target_value(&ev);
                                        let tz = parse_tz(&val);
                                        handle_ring_change(ring, tz);
                                    }
                                >
                                    <option value="dallas" selected=move || current_tz == Timezone::Dallas>"Dallas"</option>
                                    <option value="connecticut" selected=move || current_tz == Timezone::Connecticut>"Connecticut"</option>
                                    <option value="london" selected=move || current_tz == Timezone::London>"London"</option>
                                </select>
                            </div>
                        }
                    }).collect_view()}
                </div>
            }
        }}
    }
}
