use leptos::prelude::*;
use crate::modules::*;

#[component]
pub fn Header(
    active_zones: ReadSignal<ActiveTimezones>,
    theme: Signal<&'static ThemeColors>,
) -> impl IntoView {
    let subtitle = move || {
        let z = active_zones.get();
        z.zones.iter().map(|tz| tz.name.to_string()).collect::<Vec<_>>().join(" \u{2022} ")
    };

    view! {
        {move || {
            let t = *theme.get();

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
                </div>
            }
        }}
    }
}

#[component]
pub fn TimezoneConfigurator(
    active_zones: ReadSignal<ActiveTimezones>,
    set_active_zones: WriteSignal<ActiveTimezones>,
    theme_name: ReadSignal<ThemeName>,
    set_theme_name: WriteSignal<ThemeName>,
    mode: ReadSignal<Mode>,
    set_mode: WriteSignal<Mode>,
    theme: Signal<&'static ThemeColors>,
) -> impl IntoView {
    let (expanded, set_expanded) = signal(false);

    view! {
        {move || {
            let t = *theme.get();
            let zones = active_zones.get();
            let n = zones.zones.len();
            let can_add = n < MAX_RINGS;
            let can_remove = n > 1;
            let is_expanded = expanded.get();
            let current_theme = theme_name.get();
            let current_mode = mode.get();

            let toggle_label = if is_expanded { "\u{25B2} Config" } else { "\u{25BC} Config" };

            let select_style = format!(
                "border: 1px solid {}; background: {}; color: {}; border-radius: 6px; padding: 4px 6px; cursor: pointer; font-size: 12px",
                t.input_border, t.input_bg, t.text_primary
            );

            view! {
                <div style=format!("border-bottom: 1px solid {}; transition: all 0.3s ease", t.card_border)>
                    // Toggle bar
                    <div
                        style=format!(
                            "display: flex; justify-content: flex-start; align-items: center; padding: 4px 32px; cursor: pointer; user-select: none; color: {}; font-size: 11px",
                            t.text_secondary
                        )
                        on:click=move |_| set_expanded.set(!is_expanded)
                    >
                        {toggle_label}
                    </div>

                    // Collapsible content
                    {if is_expanded {
                        let select_style = select_style.clone();
                        Some(view! {
                            <div style="display: flex; flex-direction: column; gap: 8px; padding: 4px 32px 12px">
                                // Theme + mode row
                                <div style="display: flex; justify-content: center; gap: 16px; align-items: center; flex-wrap: wrap">
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

                                // Timezone selectors row
                                <div style="display: flex; justify-content: center; gap: 12px; align-items: center; overflow-x: auto; flex-wrap: wrap">
                                    {zones.zones.iter().enumerate().map(|(i, tz)| {
                                        let current_id = tz.id;
                                        let sstyle = select_style.clone();
                                        let ring_label = format!("Ring {}", i + 1);
                                        let label_style = format!("font-size: 11px; display: block; margin-bottom: 4px; white-space: nowrap; color: {}", t.text_secondary);

                                        view! {
                                            <div style="flex-shrink: 0; display: flex; align-items: flex-end; gap: 4px">
                                                <div>
                                                    <label style=label_style>{ring_label}</label>
                                                    <select
                                                        style=sstyle
                                                        prop:value=current_id
                                                        on:change=move |ev| {
                                                            let val = event_target_value(&ev);
                                                            let mut current = active_zones.get();
                                                            if let Some(new_tz) = TIMEZONE_DATABASE.iter().find(|t| t.id == val) {
                                                                if let Some(existing_idx) = current.zones.iter().position(|z| z.id == new_tz.id) {
                                                                    if existing_idx != i {
                                                                        current.zones.swap(i, existing_idx);
                                                                    }
                                                                } else {
                                                                    current.zones[i] = new_tz.clone();
                                                                }
                                                                set_active_zones.set(current);
                                                            }
                                                        }
                                                    >
                                                        {TIMEZONE_DATABASE.iter().map(|tz_opt| {
                                                            let tz_id = tz_opt.id;
                                                            let display = format!("{} ({})", tz_opt.name, format_offset(tz_opt.utc_offset));
                                                            view! {
                                                                <option value=tz_id selected=move || current_id == tz_id>
                                                                    {display}
                                                                </option>
                                                            }
                                                        }).collect_view()}
                                                    </select>
                                                </div>
                                                {if can_remove {
                                                    Some(view! {
                                                        <button
                                                            style=format!(
                                                                "background: none; border: 1px solid {}; border-radius: 4px; color: {}; cursor: pointer; padding: 4px 6px; font-size: 11px",
                                                                t.card_border, t.warning_text
                                                            )
                                                            on:click=move |_| {
                                                                let mut current = active_zones.get();
                                                                current.zones.remove(i);
                                                                set_active_zones.set(current);
                                                            }
                                                        >
                                                            "\u{2715}"
                                                        </button>
                                                    })
                                                } else {
                                                    None
                                                }}
                                            </div>
                                        }
                                    }).collect_view()}

                                    {if can_add {
                                        let add_style = format!(
                                            "background: {}; color: {}; border: none; border-radius: 6px; padding: 4px 12px; font-size: 12px; cursor: pointer",
                                            t.button_primary_bg, t.button_primary_text
                                        );
                                        Some(view! {
                                            <button
                                                style=add_style
                                                on:click=move |_| {
                                                    let mut current = active_zones.get();
                                                    let used_ids: Vec<&str> = current.zones.iter().map(|z| z.id).collect();
                                                    if let Some(new_tz) = TIMEZONE_DATABASE.iter().find(|t| !used_ids.contains(&t.id)) {
                                                        current.zones.push(new_tz.clone());
                                                        set_active_zones.set(current);
                                                    }
                                                }
                                            >
                                                "+ Add"
                                            </button>
                                        })
                                    } else {
                                        None
                                    }}
                                </div>
                            </div>
                        })
                    } else {
                        None
                    }}
                </div>
            }
        }}
    }
}

fn format_offset(offset: f64) -> String {
    let abs = offset.abs();
    let hours = abs.floor() as i32;
    let minutes = ((abs % 1.0) * 60.0).round() as i32;
    let sign = if offset < 0.0 { "-" } else { "+" };
    if minutes == 0 {
        format!("UTC{}{}", sign, hours)
    } else {
        format!("UTC{}{}:{:02}", sign, hours, minutes)
    }
}
