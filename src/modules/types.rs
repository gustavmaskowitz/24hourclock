#[derive(Debug, Clone, PartialEq)]
pub struct Meeting {
    pub id: u32,
    pub utc_hour: u32,
    pub title: String,
    pub essential: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TimezoneEntry {
    pub id: &'static str,
    pub name: &'static str,
    pub short_name: &'static str,
    pub utc_offset: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ActiveTimezones {
    pub zones: Vec<TimezoneEntry>,
}

impl Default for ActiveTimezones {
    fn default() -> Self {
        use crate::modules::timezone_db::TIMEZONE_DATABASE;
        Self {
            zones: vec![
                TIMEZONE_DATABASE.iter().find(|t| t.id == "america_chicago").unwrap().clone(),
                TIMEZONE_DATABASE.iter().find(|t| t.id == "america_new_york").unwrap().clone(),
                TIMEZONE_DATABASE.iter().find(|t| t.id == "europe_london").unwrap().clone(),
            ],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectedSlot {
    pub utc_hour: u32,
    pub local_hours: Vec<f64>,
}

#[derive(Debug, Clone, Copy)]
pub struct RingGeometry {
    pub outer_r: f64,
    pub inner_r: f64,
}

// SVG constants matching original exactly (viewBox 0 0 400 400)
pub const CX: f64 = 200.0;
pub const CY: f64 = 200.0;
pub const CENTER_R: f64 = 54.0;
pub const BG_R: f64 = 196.0;
pub const WORK_START: f64 = 9.0;
pub const WORK_END: f64 = 18.0;

pub const RING_GAP: f64 = 4.0;
pub const CENTER_GAP: f64 = 16.0;
pub const MAX_RINGS: usize = 5;
