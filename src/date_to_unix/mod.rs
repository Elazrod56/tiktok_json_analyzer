use chrono::{NaiveDateTime, TimeZone};

pub fn date_to_unix_timestamp(date_str: &str) -> Option<i64> {
    let date_time = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S").ok()?;
    let timestamp = chrono::Utc
        .timestamp_millis(date_time.timestamp_millis())
        .timestamp();

    Some(timestamp)
}