use chrono::{TimeZone, Utc};
fn unix_timestamp_to_rfc2822(s: &str) -> String {
    let re = Regex::new(r"\b(\d{10})\b").unwrap();
    re.replace_all(s, |caps: &regex::Captures| {
        let ts = caps[1].parse::<i64>().unwrap_or(0);
        let dt = Utc.timestamp_opt(ts, 0).single();
        match dt {
            Some(date) => date.to_rfc2822(),
            None => caps[0].to_string(),
        }
    }).into()
}
use regex::Regex;

fn replace_whole_word(haystack: &str, needle: &str, replacement: &str) -> String {
    let re = Regex::new(&format!("(?i)\\b{}\\b", regex::escape(needle))).unwrap();
    re.replace_all(haystack, replacement).into()
}

fn replace_beginning(haystack: &str, needle: &str, replacement: &str) -> String {
    let re = Regex::new(&format!("(?i)^{}\\b", regex::escape(needle))).unwrap();
    re.replace_all(haystack, replacement).into()
}

fn tonight_to_today(s: &str) -> String {
    replace_whole_word(s, "tonight", "today")
}

fn remove_on(s: &str) -> String {
    replace_whole_word(s, "on", "")
}

fn remove_at(s: &str) -> String {
    replace_whole_word(s, "at", "")
}

fn remove_and(s: &str) -> String {
    replace_whole_word(s, "and", "")
}

fn remove_in(s: &str) -> String {
    replace_beginning(s, "in", "")
}

fn remove_from_now(s: &str) -> String {
    replace_whole_word(s, "from now", "")
}

fn support_time_as_prefix(s: &str) -> String {
    let re = Regex::new(r"(?i)^(\d{1,2}(:\d{2})?\s*(am|pm))\s+(.*)").unwrap();
    re.replace_all(s, "$4 $1").into()
}

fn replace_simple_times_with_complex(s: &str) -> String {
    let re = Regex::new(r"(?i)(\d{1,2})\s*(am|pm)").unwrap();
    re.replace_all(s, "$1:00$2").into()
}

const MUTATIONS: &[fn(&str) -> String] = &[
    unix_timestamp_to_rfc2822,
    tonight_to_today,
    remove_on,
    remove_at,
    remove_and,
    remove_in,
    remove_from_now,
    support_time_as_prefix,
    replace_simple_times_with_complex
];

pub fn apply_mutations(input: &str) -> String {
    let mut s = input.to_string();
    for mutation in MUTATIONS {
        s = mutation(&s);
    }
    s.trim().to_string()
}
