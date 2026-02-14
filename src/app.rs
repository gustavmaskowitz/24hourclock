use leptos::prelude::*;
use gloo_timers::callback::Timeout;
use crate::components::*;
use crate::modules::*;

const STORAGE_KEY_ZONES: &str = "tz-clock-zones";
const STORAGE_KEY_THEME: &str = "tz-clock-theme";
const STORAGE_KEY_MODE: &str = "tz-clock-mode";

fn schedule_minute_update(set_current_utc: WriteSignal<f64>) {
    let now = js_sys::Date::new_0();
    let secs = now.get_utc_seconds() as u32;
    let ms = now.get_utc_milliseconds() as u32;
    let ms_to_next = ((60 - secs) * 1000).saturating_sub(ms).max(100);

    let timeout = Timeout::new(ms_to_next, move || {
        set_current_utc.set(get_current_utc_hour());
        schedule_minute_update(set_current_utc);
    });
    std::mem::forget(timeout);
}

fn load_zones() -> ActiveTimezones {
    let storage = web_sys::window()
        .and_then(|w| w.local_storage().ok().flatten());
    if let Some(storage) = storage {
        if let Ok(Some(raw)) = storage.get_item(STORAGE_KEY_ZONES) {
            let ids: Vec<&str> = raw.split(',').collect();
            let zones: Vec<TimezoneEntry> = ids.iter().filter_map(|id| {
                TIMEZONE_DATABASE.iter().find(|tz| tz.id == *id).cloned()
            }).collect();
            if !zones.is_empty() {
                return ActiveTimezones { zones };
            }
        }
    }
    ActiveTimezones::default()
}

fn save_zones(zones: &ActiveTimezones) {
    let storage = web_sys::window()
        .and_then(|w| w.local_storage().ok().flatten());
    if let Some(storage) = storage {
        let ids: Vec<&str> = zones.zones.iter().map(|z| z.id).collect();
        let _ = storage.set_item(STORAGE_KEY_ZONES, &ids.join(","));
    }
}

fn get_storage() -> Option<web_sys::Storage> {
    web_sys::window().and_then(|w| w.local_storage().ok().flatten())
}

fn load_theme() -> ThemeName {
    get_storage()
        .and_then(|s| s.get_item(STORAGE_KEY_THEME).ok().flatten())
        .and_then(|v| match v.as_str() {
            "Minimalist" => Some(ThemeName::Minimalist),
            "Bold" => Some(ThemeName::Bold),
            "Professional" => Some(ThemeName::Professional),
            "Playful" => Some(ThemeName::Playful),
            _ => None,
        })
        .unwrap_or(ThemeName::Minimalist)
}

fn save_theme(name: ThemeName) {
    if let Some(storage) = get_storage() {
        let _ = storage.set_item(STORAGE_KEY_THEME, name.label());
    }
}

fn load_mode() -> Mode {
    get_storage()
        .and_then(|s| s.get_item(STORAGE_KEY_MODE).ok().flatten())
        .and_then(|v| match v.as_str() {
            "Light" => Some(Mode::Light),
            "Dark" => Some(Mode::Dark),
            _ => None,
        })
        .unwrap_or(Mode::Dark)
}

fn save_mode(mode: Mode) {
    if let Some(storage) = get_storage() {
        let _ = storage.set_item(STORAGE_KEY_MODE, match mode {
            Mode::Light => "Light",
            Mode::Dark => "Dark",
        });
    }
}

#[component]
pub fn App() -> impl IntoView {
    // State
    let (meetings, set_meetings) = signal(Vec::<Meeting>::new());
    let (selected_slot, set_selected_slot) = signal(None::<SelectedSlot>);
    let (current_utc, set_current_utc) = signal(get_current_utc_hour());
    let (active_zones, set_active_zones) = signal(load_zones());
    let (theme_name, set_theme_name) = signal(load_theme());
    let (mode, set_mode) = signal(load_mode());

    // Derived: active theme colors
    let theme = Signal::derive(move || get_theme(theme_name.get(), mode.get()));

    // Minute-aligned timer: updates at each minute boundary
    schedule_minute_update(set_current_utc);

    // Persist zones and clear selected slot when active zones change
    Effect::new(move || {
        let zones = active_zones.get();
        save_zones(&zones);
        set_selected_slot.set(None);
    });

    // Persist theme and mode
    Effect::new(move || save_theme(theme_name.get()));
    Effect::new(move || save_mode(mode.get()));

    view! {
        <div style=move || format!(
            "min-height: 100vh; display: flex; flex-direction: column; overflow: auto; background-color: {}; transition: all 0.3s ease",
            theme.get().background
        )>
            <Header
                active_zones=active_zones
                theme=theme
            />

            <TimezoneConfigurator
                active_zones=active_zones
                set_active_zones=set_active_zones
                theme_name=theme_name
                set_theme_name=set_theme_name
                mode=mode
                set_mode=set_mode
                theme=theme
            />

            <div style="flex: 1; display: flex; flex-wrap: wrap; align-items: center; justify-content: center; gap: 32px; padding: 32px">
                <div style="flex-shrink: 0; width: 100%; max-width: 400px">
                    <Clock
                        meetings=meetings
                        set_selected_slot=set_selected_slot
                        current_utc=current_utc
                        active_zones=active_zones
                        theme=theme
                    />
                </div>

                <InfoPanels
                    meetings=meetings
                    set_meetings=set_meetings
                    selected_slot=selected_slot
                    active_zones=active_zones
                    theme=theme
                />
            </div>
        </div>
    }
}
