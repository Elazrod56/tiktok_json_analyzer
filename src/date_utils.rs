// date_utils.rs
// date_utils.rs is in charge of :
// - providing functions that help calculate the time (in days) between two dates
// - these functions are used in lib.rs - for instance they're used when we need to calculate likes/day or videos/day...

use chrono::{NaiveDateTime, TimeZone};
use std::time::{Duration, SystemTime};

fn date_to_unix_timestamp(date_str: &str) -> Option<i64> {
    let date_time = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S").ok()?;
    let timestamp = chrono::Utc
        .timestamp_millis_opt(date_time.timestamp_millis())
        .unwrap()
        .timestamp();
    Some(timestamp)
}

pub fn days_since_date(date_str: &str) -> Option<usize> {
    let date = match SystemTime::UNIX_EPOCH
        .checked_add(Duration::from_secs(date_to_unix_timestamp(date_str)? as u64))
    {
        Some(date) => date,
        None => return None,
    };

    let duration = match SystemTime::now().duration_since(date) {
        Ok(duration) => duration,
        Err(_) => return None,
    };

    let days = duration.as_secs() / (24 * 60 * 60);
    Some(days as usize)
}
