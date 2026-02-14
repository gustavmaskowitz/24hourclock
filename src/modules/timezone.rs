use crate::modules::types::{TimezoneEntry, WORK_START, WORK_END};

/// Convert UTC hour to local time given UTC offset
pub fn utc_to_local(utc_hour: f64, utc_offset: f64) -> f64 {
    (utc_hour + utc_offset + 48.0) % 24.0
}

/// Convert local hour from one timezone offset to another
pub fn convert_between(hour: f64, from_offset: f64, to_offset: f64) -> f64 {
    let utc = (hour - from_offset + 48.0) % 24.0;
    utc_to_local(utc, to_offset)
}

/// Check if hour is within work hours (9:00-18:00)
pub fn is_work_hour(hour: f64) -> bool {
    hour >= WORK_START && hour < WORK_END
}

/// Check if all timezones in the list are simultaneously in working hours at the given UTC hour
pub fn is_full_overlap_utc(utc_hour: f64, zones: &[TimezoneEntry]) -> bool {
    zones.iter().all(|tz| is_work_hour(utc_to_local(utc_hour, tz.utc_offset)))
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
    fn test_utc_to_local() {
        // UTC 16:00 in Dallas (UTC-6) = 10:00
        assert_eq!(utc_to_local(16.0, -6.0), 10.0);
        // UTC 16:00 in London (UTC+0) = 16:00
        assert_eq!(utc_to_local(16.0, 0.0), 16.0);
        // UTC 16:00 in India (UTC+5.5) = 21.5
        assert_eq!(utc_to_local(16.0, 5.5), 21.5);
    }

    #[test]
    fn test_convert_between() {
        // Dallas 10:00 = UTC 16:00 = London 16:00
        let london_hour = convert_between(10.0, -6.0, 0.0);
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
