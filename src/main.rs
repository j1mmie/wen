
use std::env;
use std::process;
use regex::Regex;
use chrono::{Local, TimeZone, Utc};

fn replace_whole_word(haystack: &str, needle: &str, replacement: &str) -> String {
    let re = Regex::new(&format!("(?i)\\b{}\\b", regex::escape(needle))).unwrap();
    re.replace_all(haystack, replacement).into()
}

fn replace_beginning(haystack: &str, needle: &str, replacement: &str) -> String {
    let re = Regex::new(&format!("(?i)^{}\\b", regex::escape(needle))).unwrap();
    re.replace_all(haystack, replacement).into()
}

// Support phrases like "at 11pm tonight"
fn tonight_to_today(s: &str) -> String {
    replace_whole_word(s, "tonight", "today")
}

// Support phrases like "on Monday" or "at 2pm on Monday"
fn remove_on(s: &str) -> String {
    replace_whole_word(s, "on", "")
}

// Support phrases like "at 4pm today" and "on Monday at 2pm"
fn remove_at(s: &str) -> String {
    replace_whole_word(s, "at", "")
}

// Support phrases like "in 24 hours"
fn remove_in(s: &str) -> String {
    replace_beginning(s, "in", "")
}

// Support phrases like "12 days from now"
fn remove_from_now(s: &str) -> String {
    replace_whole_word(s, "from now", "")
}

// Support phrases like "2pm Monday", "11:20am last friday", and "4 pm August 22"
fn support_time_as_prefix(s: &str) -> String {
    // If the string starts with a time, has a space, and then any other non-whitespace characters
    // swap the time and the rest of the string. For example, "2pm Monday" turns into "Monday 2pm"
    let re = Regex::new(r"(?i)^(\d{1,2}(:\d{2})?\s*(am|pm))\s+(.*)").unwrap();
    re.replace_all(s, "$4 $1").into()
}

// Support phrases like 3pm, 2am, etc
fn replace_simple_times_with_complex(s: &str) -> String {
    // Replace 3pm with 3:00pm, 2am with 2:00am
    let re = Regex::new(r"(?i)(\d{1,2})\s*(am|pm)").unwrap();
    re.replace_all(s, "$1:00$2").into()
}

// List of mutation functions as function pointers
const MUTATIONS: &[fn(&str) -> String] = &[
    tonight_to_today,
    remove_on,
    remove_at,
    remove_in,
    remove_from_now,
    support_time_as_prefix,
    replace_simple_times_with_complex
];

fn main() {
    // Join all CLI args into one input string (so spaces are allowed).
    let mut args = env::args().skip(1).collect::<Vec<_>>();
    if args.is_empty() {
        eprintln!("usage: wen <free-form date/time> [--unix]");
        process::exit(2);
    }
    let unix_mode = args.contains(&"--unix".to_string());
    args.retain(|a| a != "--unix");
    let joined = args.join(" ").trim().to_string();

    // Apply all mutation functions in order
    let mut input = joined;
    for mutation in MUTATIONS {
        input = mutation(&input);
    }
    input = input.trim().to_string();

    println!("Parsing: '{}'", input);

    let tz_name = match iana_time_zone::get_timezone() {
        Ok(t) => t,
        Err(e) => exit_with(&format!("failed to detect system timezone: {e}")),
    };

    let tz = match timelib::Timezone::parse(&tz_name) {
        Ok(t) => t,
        Err(e) => exit_with(&format!("invalid system timezone '{}': {}", tz_name, e)),
    };

    let ts = match timelib::strtotime(&input, None, &tz) {
        Ok(ts) => ts,
        Err(err) => exit_with(&format!("parse error: {err}")),
    };

    if unix_mode {
        println!("{ts}");
        return;
    }
    print_report(ts);
}

fn print_report(ts: i64) {
    let unix_secs = ts;
    let dt_utc = Utc.timestamp_opt(ts, 0).single();
    let dt_local = Local.timestamp_opt(ts, 0).single();
    println!("");
    println!("Unix Time: {}", unix_secs);
    // Table header
    println!("");
    println!("                | C-time                    | ISO 8601                  | RFC 2822");
    println!("----------------+---------------------------+---------------------------+---------------------------------");
    // Local row
    if let Some(dt_local) = dt_local {
        println!("  Local         | {:<25} | {:<25} | {:<30}",
            dt_local.format("%a %b %e %T %Y"),
            dt_local.to_rfc3339(),
            dt_local.to_rfc2822()
        );
    } else {
        println!("  Local         | {:<25} | {:<25} | {:<30}", "<invalid>", "<invalid>", "<invalid>");
    }
    // UTC row
    if let Some(dt_utc) = dt_utc {
        println!("  UTC           | {:<25} | {:<25} | {:<30}",
            dt_utc.format("%a %b %e %T %Y"),
            dt_utc.to_rfc3339(),
            dt_utc.to_rfc2822()
        );
    } else {
        println!("  UTC           | {:<25} | {:<25} | {:<30}", "<invalid>", "<invalid>", "<invalid>");
    }
}

fn exit_with(msg: &str) -> ! {
    eprintln!("{}", msg);
    process::exit(1);
}
