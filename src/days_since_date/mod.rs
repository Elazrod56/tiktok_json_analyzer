use std::time::{Duration, SystemTime};

pub fn days_since_date(date_str: &str) -> Option<u64> {
    // Convert the input string to a SystemTime object
    let date = match SystemTime::UNIX_EPOCH.checked_add(
        Duration::from_secs(date_str.parse().ok()?),
    ) {
        Some(date) => date,
        None => return None,
    };

    // Calculate the duration between the input date and the current time
    let duration = match SystemTime::now().duration_since(date) {
        Ok(duration) => duration,
        Err(_) => return None,
    };

    // Calculate the number of days in the duration and return it as an integer
    let days = duration.as_secs() / (24 * 60 * 60);
    Some(days)
}