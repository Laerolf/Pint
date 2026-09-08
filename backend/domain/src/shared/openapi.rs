use chrono::NaiveDate;

use crate::shared::Source;

/// Returns an example ID.
pub fn example_id() -> i32 {
    666666
}

/// Returns an example source.
pub fn example_source() -> String {
    Source::Square.to_string()
}

/// Returns an example date string.
pub fn example_date_string() -> String {
    NaiveDate::parse_from_str("1948-12-03", "%Y-%m-%d")
        .expect("Expected '1948-12-03' to be a valid date string.")
        .and_hms_opt(0, 0, 0)
        .expect("Expected midnight is always a valid time.")
        .and_utc()
        .to_rfc3339()
}
