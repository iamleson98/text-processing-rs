//! Time tagger for Vietnamese.
//!
//! Converts spoken Vietnamese time expressions to written form:
//! - "mười bốn giờ ba mươi" → "14:30"
//! - "tám giờ" → "8:00"
//! - "nửa đêm" → "0:00"
//! - "trưa" → "12:00"
//! - "chín giờ linh năm" → "9:05"
//! - "hai giờ chiều" → "14:00" (PM modifier)
//! - "mười hai giờ đêm" → "0:00" (midnight, "đêm" = night)
//! - "mười một giờ đêm" → "23:00" (11 PM)

use super::cardinal::words_to_number;

/// Parse a spoken Vietnamese time expression to written form `HH:MM` (24-hour).
pub fn parse(input: &str) -> Option<String> {
    let input_lower = input.trim().to_lowercase();

    // Special base times.
    if input_lower == "nửa đêm" || input_lower == "đêm khuya" {
        return Some("0:00".to_string());
    }
    if input_lower == "trưa" || input_lower == "giữa trưa" {
        return Some("12:00".to_string());
    }

    // PM / night modifiers add 12 to the hour (1-11 PM → 13-23).
    // "đêm" (night) is special: "12 giờ đêm" = midnight (0:00), "X giờ đêm"
    // for X=1..11 = (X+12):00.
    let (cleaned, add_12, is_night) = strip_time_modifiers(&input_lower);

    if cleaned.is_empty() {
        return None;
    }

    // "X giờ Y" or "X giờ" patterns.
    if let Some((hour_part, minute_part)) = cleaned.split_once(" giờ ") {
        let hour = words_to_number(hour_part)? as i64;
        let minute = words_to_number(minute_part)? as i64;
        return format_hour_minute(hour, minute, add_12, is_night);
    }

    if let Some(hour_part) = cleaned.strip_suffix(" giờ") {
        let hour = words_to_number(hour_part)? as i64;
        return format_hour_minute(hour, 0, add_12, is_night);
    }

    None
}

/// Strip Vietnamese time-of-day modifiers, returning the cleaned phrase,
/// whether a PM modifier was present, and whether the "đêm" (night) modifier
/// was present (which maps 12 → 0).
fn strip_time_modifiers(input: &str) -> (String, bool, bool) {
    // Order matters: longest phrases first.
    let pm_markers = [" buổi chiều", " buổi tối", " chiều", " tối"];
    let am_markers = [" buổi sáng", " sáng"];
    let night_markers = [" đêm"];
    // "trưa" (noon) is a no-op modifier — "12 giờ trưa" = "12 giờ" = 12:00.
    let noon_markers = [" trưa"];

    let mut cleaned = input.to_string();
    let mut add_12 = false;
    let mut is_night = false;

    for marker in &night_markers {
        if cleaned.ends_with(marker) {
            cleaned.truncate(cleaned.len() - marker.len());
            is_night = true;
            add_12 = true;
            break;
        }
    }
    if !is_night {
        for marker in &noon_markers {
            if cleaned.ends_with(marker) {
                cleaned.truncate(cleaned.len() - marker.len());
                // "trưa" is noon — hour 12 stays 12, no +12 offset.
                break;
            }
        }
    }
    if !is_night {
        for marker in &pm_markers {
            if cleaned.ends_with(marker) {
                cleaned.truncate(cleaned.len() - marker.len());
                add_12 = true;
                break;
            }
        }
    }
    if !add_12 && !is_night {
        for marker in &am_markers {
            if cleaned.ends_with(marker) {
                cleaned.truncate(cleaned.len() - marker.len());
                break;
            }
        }
    }
    (cleaned.trim().to_string(), add_12, is_night)
}

/// Format hour:minute, applying the PM +12 offset and range-checking the result.
/// When `is_night` is true, hour 12 maps to 0 (midnight): "12 giờ đêm" → "0:00".
fn format_hour_minute(hour: i64, minute: i64, add_12: bool, is_night: bool) -> Option<String> {
    let mut h = hour;
    if add_12 && h < 12 {
        h += 12;
    }
    // "12 giờ đêm" = midnight (0:00), "12 giờ trưa" = noon (12:00).
    if is_night && h == 12 {
        h = 0;
    }
    if !(0..=23).contains(&h) || !(0..=59).contains(&minute) {
        return None;
    }
    Some(format!("{}:{:02}", h, minute))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(parse("mười bốn giờ ba mươi"), Some("14:30".to_string()));
        assert_eq!(parse("tám giờ"), Some("8:00".to_string()));
        assert_eq!(parse("chín giờ linh năm"), Some("9:05".to_string()));
    }

    #[test]
    fn test_special() {
        assert_eq!(parse("nửa đêm"), Some("0:00".to_string()));
        assert_eq!(parse("trưa"), Some("12:00".to_string()));
    }

    #[test]
    fn test_pm_modifier() {
        assert_eq!(parse("hai giờ chiều"), Some("14:00".to_string()));
        assert_eq!(parse("ba giờ tối"), Some("15:00".to_string()));
    }

    #[test]
    fn test_night_modifier() {
        // "12 giờ đêm" = midnight (0:00), "11 giờ đêm" = 23:00.
        assert_eq!(parse("mười hai giờ đêm"), Some("0:00".to_string()));
        assert_eq!(parse("mười một giờ đêm"), Some("23:00".to_string()));
        assert_eq!(parse("mười giờ đêm"), Some("22:00".to_string()));
    }

    #[test]
    fn test_invalid() {
        assert_eq!(parse("hai mươi lăm giờ"), None); // > 23
        assert_eq!(parse("hai mươi bốn giờ"), None); // 24:00 rejected
        assert_eq!(parse("hello"), None);
        assert_eq!(parse(""), None);
    }
}
