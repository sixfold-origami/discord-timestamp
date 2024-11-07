use std::error::Error;

use chrono::{FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, ParseError, TimeZone};
use clap::{builder::ArgPredicate, Parser, ValueEnum};
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use prettytable::{format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR, Table};

const STYLE_HELP: [[&str; 5]; 8] = [
    [
        "default",
        "",
        "<t:1543392060>",
        "November 28, 2018 9:01 AM",
        "28 November 2018 09:01",
    ],
    ["short-time", "t", "<t:1543392060:t>", "9:01 AM", "09:01"],
    [
        "long-time",
        "T",
        "<t:1543392060:T>",
        "9:01:00 AM",
        "09:01:00",
    ],
    [
        "short-date",
        "d",
        "<t:1543392060:d>",
        "11/28/2018",
        "28/11/2018",
    ],
    [
        "long-date",
        "D",
        "<t:1543392060:D>",
        "November 28, 2018",
        "28 November 2018",
    ],
    [
        "short-date-time",
        "f",
        "<t:1543392060:f>",
        "November 28, 2018 9:01 AM",
        "28 November 2018 09:01",
    ],
    [
        "long-date-time",
        "F",
        "<t:1543392060:F>",
        "Wednesday, November 28, 2018 9:01 AM",
        "Wednesday, 28 November 2018 09:01",
    ],
    [
        "relative-time",
        "R",
        "<t:1543392060:R>",
        "3 years ago",
        "3 years ago",
    ],
];

/// A CLI utility to generate discord-formatted timestamps
#[derive(Parser, Debug)]
#[command(version)]
struct Cli {
    /// Date/time string in the local timezone to convert to a discord timestamp
    #[arg(
        index = 1,
        default_value_if("help_style", ArgPredicate::IsPresent, ""),
        conflicts_with = "help_style"
    )]
    input: String,

    /// Format style of the output. (Use --help-style for style options.)
    #[arg(index = 2, default_value = "default", value_parser = Style::parse, env = "DT_STYLE")]
    style: Style,

    /// Copy the result to the clipboard when complete
    #[arg(short = 'c', long)]
    copy_to_clipboard: bool,

    /// Format string for parsing datetimes
    #[arg(
        short = 'f',
        long,
        default_value = "%Y-%m-%d %H:%M:%S",
        env = "DT_DATETIME_FORMAT"
    )]
    datetime_format: String,

    /// Format string for parsing lone dates (assumes midnight)
    #[arg(short = 'd', long, default_value = "%Y-%m-%d", env = "DT_DATE_FORMAT")]
    date_format: String,

    /// Format string for parsing lone times (assumes today)
    #[arg(short = 't', long, default_value = "%H:%M:%S", env = "DT_TIME_FORMAT")]
    time_format: String,

    /// Shows options (and abbreviations) for the style argument
    #[arg(long)]
    help_style: bool,
}

impl Cli {
    fn get_naive_datetime(&self) -> Result<NaiveDateTime, ParseError> {
        // Try to parse a full datetime
        let datetime = NaiveDateTime::parse_from_str(&self.input, &self.datetime_format);
        if datetime.is_ok() {
            return datetime;
        }

        // Try to parse just a date
        if let Ok(date) = NaiveDate::parse_from_str(&self.input, &self.date_format) {
            return Ok(date.and_hms_opt(0, 0, 0).unwrap());
        }

        // Try to parse just a time
        NaiveTime::parse_from_str(&self.input, &self.time_format).map(|time| {
            let today = Local::now().date_naive();
            NaiveDateTime::new(today, time)
        })
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Style {
    #[default]
    Default,
    ShortTime,
    LongTime,
    ShortDate,
    LongDate,
    ShortDateTime,
    LongDateTime,
    RelativeTime,
}

impl Style {
    fn get_formatted(&self, unix: i64) -> String {
        match self {
            Style::Default => format!("<t:{}>", unix),
            Style::ShortTime => format!("<t:{}:{}>", unix, self.code()),
            Style::LongTime => format!("<t:{}:{}>", unix, self.code()),
            Style::ShortDate => format!("<t:{}:{}>", unix, self.code()),
            Style::LongDate => format!("<t:{}:{}>", unix, self.code()),
            Style::ShortDateTime => format!("<t:{}:{}>", unix, self.code()),
            Style::LongDateTime => format!("<t:{}:{}>", unix, self.code()),
            Style::RelativeTime => format!("<t:{}:{}>", unix, self.code()),
        }
    }

    /// Character code associated with this style
    fn code(&self) -> &str {
        match self {
            Style::Default => "",
            Style::ShortTime => "t",
            Style::LongTime => "T",
            Style::ShortDate => "d",
            Style::LongDate => "D",
            Style::ShortDateTime => "f",
            Style::LongDateTime => "F",
            Style::RelativeTime => "R",
        }
    }

    /// Clap [`value_parser`] to get a [`Self`] from either the kebab-case name, or the character code
    fn parse(s: &str) -> Result<Self, String> {
        match s {
            "default" => Ok(Style::Default),
            "t" => Ok(Style::ShortTime),
            "short-time" => Ok(Style::ShortTime),
            "T" => Ok(Style::LongTime),
            "long-time" => Ok(Style::LongTime),
            "d" => Ok(Style::ShortDate),
            "short-date" => Ok(Style::ShortDate),
            "D" => Ok(Style::LongDate),
            "long-date" => Ok(Style::LongDate),
            "f" => Ok(Style::ShortDateTime),
            "short-date-time" => Ok(Style::ShortDateTime),
            "F" => Ok(Style::LongDateTime),
            "long-date-time" => Ok(Style::LongDateTime),
            "R" => Ok(Style::RelativeTime),
            "relative-time" => Ok(Style::RelativeTime),
            _ => Err("Expected one of: default, short-time, t, long-time, T, short-date, d, long-date, D, short-date-time, f, long-date-time, F, relative-time, R".into()),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    // Special style help command
    if args.help_style {
        let mut table = Table::from(STYLE_HELP);
        table.set_titles(
            [
                "Style",
                "Alias",
                "Discord Format",
                "Output (12-hour clock)",
                "Output (24-hour clock)",
            ]
            .into(),
        );

        table.set_format(*FORMAT_NO_BORDER_LINE_SEPARATOR);
        table.printstd();
    }

    // Parse date
    let datetime = args.get_naive_datetime()?;
    let local = Local::from_offset(&FixedOffset::east_opt(0).unwrap());
    let datetime = local.from_local_datetime(&datetime).unwrap();
    println!("Formatting: {:?}", datetime);

    // Get timestamp and formatted string
    let unix = datetime.timestamp_millis() / 1000;
    let formatted = args.style.get_formatted(unix);

    // Output
    if args.copy_to_clipboard {
        let mut ctx = ClipboardContext::new().unwrap();
        ctx.set_contents(formatted.clone()).unwrap();
        println!("{} copied to clipboard!", formatted);
    } else {
        println!("{}", formatted);
    }

    Ok(())
}
