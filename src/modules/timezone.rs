use super::types::{Timezone, WORK_START, WORK_END};

/// Convert UTC hour to specific timezone
pub fn get_timezone_hour(utc_hour: f64, tz: Timezone) -> f64 {
    (utc_hour + tz.offset() as f64 + 24.0) % 24.0
}

/// Convert hour from one timezone to another
pub fn convert_timezone(hour: f64, from: Timezone, to: Timezone) -> f64 {
    let utc = (hour - from.offset() as f64 + 24.0) % 24.0;
    get_timezone_hour(utc, to)
}

/// Check if hour is within work hours (9:00-18:00)
/// Uses raw float comparison to match original TypeScript behavior exactly
pub fn is_work_hour(hour: f64) -> bool {
    hour >= WORK_START && hour < WORK_END
}

/// Check if target timezone is working at the given reference hour
pub fn is_timezone_working(ref_hour: f64, ref_tz: Timezone, target_tz: Timezone) -> bool {
    let target_hour = convert_timezone(ref_hour, ref_tz, target_tz);
    is_work_hour(target_hour)
}

/// Check if all three timezones are working simultaneously
pub fn is_full_overlap(ref_hour: f64, ref_tz: Timezone) -> bool {
    Timezone::ALL
        .iter()
        .all(|&tz| is_timezone_working(ref_hour, ref_tz, tz))
}

/// Get current UTC hour as fractional (e.g. 14.5 = 14:30)
pub fn get_current_utc_hour() -> f64 {
    let now = js_sys::Date::new_0();
    now.get_utc_hours() as f64 + now.get_utc_minutes() as f64 / 60.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timezone_hour() {
        // UTC 16:00 in Dallas (UTC-6) = 10:00
        assert_eq!(get_timezone_hour(16.0, Timezone::Dallas), 10.0);
        // UTC 16:00 in London (UTC+0) = 16:00
        assert_eq!(get_timezone_hour(16.0, Timezone::London), 16.0);
    }

    #[test]
    fn test_convert_timezone() {
        // Dallas 10:00 = UTC 16:00 = London 16:00
        let london_hour = convert_timezone(10.0, Timezone::Dallas, Timezone::London);
        assert_eq!(london_hour, 16.0);
    }

    #[test]
    fn test_work_hours() {
        assert!(is_work_hour(9.0));
        assert!(is_work_hour(17.0));
        assert!(is_work_hour(9.5));
        assert!(!is_work_hour(8.99));
        assert!(!is_work_hour(18.0));
    }
}
