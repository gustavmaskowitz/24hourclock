use leptos::prelude::*;
use gloo_timers::callback::Interval;
use crate::components::*;
use crate::modules::*;

#[component]
pub fn App() -> impl IntoView {
    // State
    let (meetings, set_meetings) = signal(vec![
        Meeting {
            id: 1,
            utc_hour: 16,
            title: "Sync Call".to_string(),
            essential: true,
        },
    ]);
    let (selected_slot, set_selected_slot) = signal(None::<SelectedSlot>);
    let (current_utc, set_current_utc) = signal(get_current_utc_hour());
    let (ring_assignments, set_ring_assignments) = signal(RingAssignments::default());
    let (theme_name, set_theme_name) = signal(ThemeName::Minimalist);
    let (mode, set_mode) = signal(Mode::Dark);

    // Derived: active theme colors
    let theme = Signal::derive(move || get_theme(theme_name.get(), mode.get()));

    // Timer: update current_utc every 60 seconds
    let interval = Interval::new(60_000, move || {
        set_current_utc.set(get_current_utc_hour());
    });
    // Leak the interval so it lives forever (like React's setInterval without cleanup concern)
    std::mem::forget(interval);

    view! {
        {move || {
            let t = *theme.get();
            view! {
                <div style=format!(
                    "min-height: 100vh; display: flex; flex-direction: column; overflow: auto; background-color: {}; transition: all 0.3s ease",
                    t.background
                )>
                    <Header
                        ring_assignments=ring_assignments
                        theme_name=theme_name
                        set_theme_name=set_theme_name
                        mode=mode
                        set_mode=set_mode
                        theme=theme
                    />

                    <RingControls
                        ring_assignments=ring_assignments
                        set_ring_assignments=set_ring_assignments
                        theme=theme
                    />

                    <div style="flex: 1; display: flex; flex-wrap: wrap; align-items: center; justify-content: center; gap: 32px; padding: 32px">
                        <div style="flex-shrink: 0; width: 100%; max-width: 400px">
                            <Clock
                                meetings=meetings
                                set_selected_slot=set_selected_slot
                                current_utc=current_utc
                                ring_assignments=ring_assignments
                                theme=theme
                            />
                        </div>

                        <InfoPanels
                            meetings=meetings
                            set_meetings=set_meetings
                            selected_slot=selected_slot
                            ring_assignments=ring_assignments
                            theme=theme
                        />
                    </div>
                </div>
            }
        }}
    }
}
