mod zcolorize;
mod mutations;
use zcolorize::ZColorize;
use mutations::apply_mutations;

use clap::Parser;
use std::str::FromStr;
use std::process;
use chrono::TimeZone;

/// Parse natural language date/time and print in various formats
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Free-form date/time string
    #[arg()]
    input: Vec<String>,

    /// Output as unix timestamp
    #[arg(long)]
    unix: bool,

    /// Output in UTC
    #[arg(long)]
    utc: bool,

    /// Specify IANA timezone
    #[arg(long)]
    tz: Option<String>,

    /// Disable all color output
    #[arg(long, alias = "plain")]
    no_color: bool,
}

fn main() {
    let args = Args::parse();

    if args.input.is_empty() {
        eprintln!("usage: wen <free-form date/time> [--unix] [--utc] [--tz <iana_tz>] [--no-color]");
        process::exit(2);
    }

    if args.no_color {
        owo_colors::set_override(false);
    }

    let joined = args.input.join(" ");
    let input = apply_mutations(&joined);

    let tz_name = if args.utc {
        "UTC".to_string()
    } else if let Some(tz) = args.tz.clone() {
        tz
    } else {
        match iana_time_zone::get_timezone() {
            Ok(t) => t,
            Err(e) => exit_with(&format!("failed to detect system timezone: {e}")),
        }
    };

    let tz = match timelib::Timezone::parse(&tz_name) {
        Ok(t) => t,
        Err(e) => exit_with(&format!("invalid timezone '{}': {}", tz_name, e)),
    };

    let ts = match timelib::strtotime(&input, None, &tz) {
        Ok(ts) => ts,
        Err(err) => exit_with(&format!("parse error: {err}")),
    };

    if args.unix {
        println!("{ts}");
        return;
    }
    print_report(ts, &tz_name);
}

fn print_report(ts: i64, tz_name: &str) {
    let unix_secs = ts;
    // Try to use the selected timezone for output
    let dt = match chrono_tz::Tz::from_str(tz_name) {
        Ok(tz) => tz.timestamp_opt(ts, 0).single(),
        Err(_) => None,
    };

    println!("");
    println!("  {}     {}", "Unix:".z_bright_blue(), unix_secs.to_string().z_magenta());
    println!("  {} {}", "Timezone:".z_bright_blue(), tz_name.z_green());

    if let Some(dt) = dt {
        println!("  {} {}", "ISO 8601:".z_bright_blue(), dt.to_rfc3339().z_yellow());
        println!("  {} {}", "RFC 2822:".z_bright_blue(), dt.to_rfc2822().z_yellow());
    }

    println!("");
}

fn exit_with(msg: &str) -> ! {
    eprintln!("{}", msg);
    process::exit(1);
}
