# wen
Basically [when-cli](https://github.com/mitsuhiko/when) with some quality of life improvements
<img width="501" height="330" alt="Screenshot 2025-08-28 at 11 26 15 PM" src="https://github.com/user-attachments/assets/f35a5ebb-d63c-41c6-9307-649761689a5b" />

Differences from [when-cli](https://github.com/mitsuhiko/when):
 - More robust natural language parsing (e.g. "2 minutes from now", "tomorrow at 3am", "in 5 hours and 20 minutes", "5 minutes ago", "next thursday at 2:32am")
 - Unix timestamp output (`--unix`)
 - No need for quotes (`wen tomorrow at 3pm`)

### Install:
```
cargo install wen
```

### Options:
- `--unix` to output just unix timestamp
- `--utc` to use the UTC timezone
- `--tz <iana timezone>` to specify an IANA timezone (e.g. "America/New_York")

### Examples:
Any of the following can be combined with --unix, --utc, or --tz:

- `wen now`
- `wen 2:15pm`
- `wen tomorrow at noon`
- `wen tonight at midnight`
- `wen August 29th at 8pm`
- `wen July 4th 1776`
- `wen 3:03am on September 29th`
- `wen on Sunday`
- `wen in 24 hours`
- `wen 5 minutes ago`
- `wen yesterday at 3pm`
- `wen 2:32am next thursday`
- `wen last tuesday at 1:30pm`
- `wen next saturday at 11am`
- `wen 5 hours and 20 minutes from now`
- `wen 1756486501` (unix timestamp support)

