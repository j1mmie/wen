# wen
CLI natural language date parser in rust, built on timelib, with some quality of life improvements.

### Options:
- `--unix` to output just unix timestamp
- `--utc` to use the UTC timezone
- `--tz <iana timezone>` to specify an IANA timezone (e.g. "America/New_York")

### Examples:
Any of the following can be combined with --unix, --utc, or --tz:

`wen now`
`wen 2:15pm`
`wen tomorrow at noon`
`wen tonight at midnight`
`wen in 24 hours`
`wen yesterday at 3pm`
`wen 2:32am next thursday`
`wen last tuesday at 1:30pm`
`wen 5 hours and 20 minutes from now`
