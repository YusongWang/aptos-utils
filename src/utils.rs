use chrono::{FixedOffset, NaiveDateTime};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn parse_timestamp_to_string(timestamp: i64) -> String {
    NaiveDateTime::from_timestamp(timestamp + 8 * 3600, 0).to_string()
}

pub fn get_current_unix() -> u64 {
    let unix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("get_current_unix_err");
    unix.as_secs()
}

pub fn parse_u64(s: &str) -> u64 {
    s.parse().unwrap()
}
