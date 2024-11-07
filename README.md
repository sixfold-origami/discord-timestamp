# discord-timestamps

CLI utility to generate discord-formatted timestamps

```
Usage: discord-timestamps [OPTIONS] <INPUT> [STYLE]

Arguments:
  <INPUT>  Date/time string in the local timezone to convert to a discord timestamp
  [STYLE]  Format style of the output. (Use --help-style for style options.) [env: DT_STYLE=] [default: default]

Options:
  -c, --copy-to-clipboard
          Copy the result to the clipboard when complete
  -f, --datetime-format <DATETIME_FORMAT>
          Format string for parsing datetimes [env: DT_DATETIME_FORMAT=] [default: "%Y-%m-%d %H:%M:%S"]
  -d, --date-format <DATE_FORMAT>
          Format string for parsing lone dates (assumes midnight) [env: DT_DATE_FORMAT=] [default: %Y-%m-%d]
  -t, --time-format <TIME_FORMAT>
          Format string for parsing lone times (assumes today) [env: DT_TIME_FORMAT=] [default: %H:%M:%S]
      --help-style
          Shows options (and abbreviations) for the style argument
  -h, --help
          Print help
  -V, --version
          Print version
```

Provide a date/time (in your system's local timezone) and the formatting style, and the tool will return the discord-compatible format string for that datetime! You can provide either a lone date, a lone time, or a full datetime to convert. If only the date is provided, then the time of midnight is used. If only the time is provided, then the date of today is used.

## Installation

Currently, installation is available through cargo:
```sh
cargo install discord-timestamps
```

## Examples

Minimal:
```
> discord-timestamps "2024-11-07 12:43:00"
Formatting: 2024-11-07T12:43:00-05:00
<t:1730965380>
```

Using alternate styles:
```
> discord-timestamps "2024-11-07 12:43:00" short-time
Formatting: 2024-11-07T12:43:00-05:00
<t:1730965380:t>
```
```
> discord-timestamps "2024-11-07 12:43:00" t
Formatting: 2024-11-07T12:43:00-05:00
<t:1730965380:t>
```

Copying the result to the clipboard
```
> discord-timestamps -c "2024-11-07 12:43:00" t
Formatting: 2024-11-07T12:43:00-05:00
<t:1730965380:t> copied to clipboard!
```

Providing only the date or time:
```
> discord-timestamps 2024-11-07
Formatting: 2024-11-07T00:00:00-05:00
<t:1730955600>
```
```
> discord-timestamps 12:43:00
Formatting: 2024-11-07T12:43:00-05:00
<t:1731001380>
```

Using alternate date/time formats:
```
> discord-timestamps --datetime-format "%m/%d/%y %I:%M%p" "11/7/24 2:43pm"
Formatting: 2024-11-07T14:43:00-05:00
<t:1731008580>
```
```
> discord-timestamps --time-format %I:%M%p 2:43pm
Formatting: 2024-11-07T14:43:00-05:00
<t:1731008580>
```

## Date and time formats

When parsing dates and times, the system will first attempt to parse a full datetime (using the provided `datetime-format`), then attempt to parse just a date (using the provided `date-format`), then finally attempt to parse just a time (using the provided `time-format`). If all three fail, then an error is returned.

The date and time formats use a syntax based on strftime. You can find details on all the available escape sequences [here](https://docs.rs/chrono/0.4.38/chrono/format/strftime/index.html).

Additionally, these formats can be configured by the environment variables listed in the help documentation above:
- `datetime-format`: `DT_DATETIME_FORMAT`
- `date-format`: `DT_DATE_FORMAT`
- `time-format`: `DT_TIME_FORMAT`
