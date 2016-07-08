extern crate clap;
#[macro_use]
extern crate version;
#[macro_use]
extern crate chomp;
extern crate chrono;

use std::process;
use std::ascii::{AsciiExt};

use clap::{Arg, App, AppSettings};
use chrono::offset::local::Local;
use chrono::duration::Duration;
use chomp::{SimpleResult, Error};
use chomp::primitives::{InputBuffer};
use chomp::{Input, U8Result, parse_only};
use chomp::{token};
use chomp::parsers::{eof};
use chomp::combinators::{or, many1, skip_many};
use chomp::ascii::{decimal};

// source: https://github.com/rust-lang/rust/issues/24638
macro_rules! print_err {
    ($($arg:tt)*) => (
        {
            use std::io::prelude::*;
            if let Err(e) = write!(&mut ::std::io::stderr(), "{}\n", format_args!($($arg)*)) {
                panic!("Failed to write to stderr.\
                    \nOriginal error output: {}\
                    \nSecondary error writing to stderr: {}", format!($($arg)*), e);
            }
        }
    )
}

fn main() {

    let version: &str = &format!("v{} (semver.org 2.0)", version!());

    let cmd_matches = App::new("fromnow")
        .setting(AppSettings::ColorAuto)
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::GlobalVersion)
        .version(version) // semver semantics
        .about("Generate dates and times relative from now.")
        .author("Alberto Leal <mailforalberto@gmail.com> (github.com/dashed)")
        .arg(
            Arg::with_name("subtract")
            .help("Subtract time spans/durations to generate date/time in the past.{n}\
                By default, time spans/durations are added.\
            ")
            .short("s")
            .long("subtract")
            .takes_value(false)
            .required(false)
        )
        .arg(
            Arg::with_name("format")
            .help("Format specifier for the combined date and time output.{n}\
                Example output: July 5, 2016 9:36 PM{n}\
                Default is: %B %-d, %Y %-l:%M %p{n}{n}\
                See: https://lifthrasiir.github.io/rust-chrono/chrono/format/strftime/index.html{n}\
            ")
            .next_line_help(true)
            .short("f")
            .long("format")
            .use_delimiter(false)
            // TODO: turning this on messes with AppSettings::ArgRequiredElseHelp for some reason
            // .default_value(r"%B %-d, %Y %-l:%M %p")
            .required(false)
            .takes_value(true)
            .multiple(false)
            .validator(|format| {
                let format = format.trim();
                if format.len() <= 0 {
                    return Err(String::from("invalid date/time format specifier"));
                }
                return Ok(());
            })
        )
        .arg(
            Arg::with_name("duration")
            .next_line_help(true)
            .help("\
                A duration/span of time. (Example: 3 hours){n}\
                A duration is an integer number followed by time span units. (Example: 42 hours){n}\
                There may be whitespace between the integer and the time span unit.{n}\
                Whitespace between <duration> items is optional (Example: 3hours30mins).{n}\
                There may be an ' and ' between duration items (Example: 42 days and 42 mins).{n}\
                {n}\
                Arguments may compose into a valid duration/time span; removing the need to enclose durations in quotes.
                {n}\
                Time span units are case-insensitive.{n}\
                Valid time span units and their abbreviations:{n}\
                weeks/week/wks/wk/w{n}\
                days/day/dys/dy/d{n}\
                hours/hour/hrs/hr/h{n}\
                minutes/minute/mins/min/m{n}\
                seconds/second/secs/sec/s{n}\
                milliseconds/millisecond/msecs/msec/ms\
            ")
            .required(true)
            .multiple(true)
        ).get_matches();


    let format = if let Some(format) = cmd_matches.value_of("format") {
        let format = format.trim();
        format
    } else {
        "%B %-d, %Y %-l:%M %p"
    };

    let duration: Vec<&str> = cmd_matches.values_of("duration").unwrap().collect();
    let duration = duration.join(" ");
    let ref duration = duration.trim();

    let parsed_duration = match parse_only(|i| parse_times_ranges(i), duration.as_bytes()) {

        Ok(result) => {
            Duration::milliseconds(result as i64)
        },
        Err(_) => {
            print_err!("Unable to parse durations: {}", duration);
            process::exit(1);
            // panic!("{:?}", e);
        }
    };

    let new_date_time = if cmd_matches.is_present("subtract") {
        Local::now().naive_local() - parsed_duration
    } else {
        Local::now().naive_local() + parsed_duration
    };

    // NOTE: .to_string() will not panic on invalid formats.
    //       See https://github.com/lifthrasiir/rust-chrono/issues/47
    println!("{}", new_date_time.format(format).to_string());

}

/* time range parsers */

fn parse_times_ranges(i: Input<u8>) -> U8Result<u64> {
    parse!{i;
        skip_many(|i| space_or_tab(i));
        let result: u64 = multiple_time_range();
        skip_many(|i| space_or_tab(i));
        eof();
        ret result
    }
}

fn multiple_time_range(i: Input<u8>) -> U8Result<u64> {

    parse!{i;

        let time: Vec<u64> = many1(
            |i| or(i,
                |i| parse!{i;
                    skip_many(|i| space_or_tab(i));
                    let range1: u64 = time_range();

                    space_or_tab();
                    skip_many(|i| space_or_tab(i));
                    string_ignore_case("and".as_bytes());
                    space_or_tab();
                    skip_many(|i| space_or_tab(i));

                    let range2: u64 = time_range();

                    ret {
                        range1 + range2
                    }
                },
                |i| parse!{i;
                    skip_many(|i| space_or_tab(i));
                    let range = time_range();
                    ret range
                }
            )
        );

        ret {
            let time = time.iter().fold(0, |mut sum, &val| {sum += val; sum});
            time
        }
    }
}

fn time_range(i: Input<u8>) -> U8Result<u64> {
    parse!{i;

        let range: u64 = decimal();

        skip_many(|i| space_or_tab(i));

        let multiplier = time_range_unit_minutes() <|>
            time_range_unit_hours() <|>
            time_range_unit_days() <|>
            time_range_unit_seconds() <|>
            time_range_unit_weeks() <|>
            time_range_unit_milliseconds();

        ret {
            range * multiplier
        }
    }
}

fn time_range_unit_milliseconds(i: Input<u8>) -> U8Result<u64> {
    parse!{i;

        string_ignore_case("milliseconds".as_bytes()) <|>
        string_ignore_case("millisecond".as_bytes()) <|>
        string_ignore_case("msecs".as_bytes()) <|>
        string_ignore_case("msec".as_bytes()) <|>
        string_ignore_case("ms".as_bytes());

        ret 1
    }
}

fn time_range_unit_seconds(i: Input<u8>) -> U8Result<u64> {
    parse!{i;

        string_ignore_case("seconds".as_bytes()) <|>
        string_ignore_case("second".as_bytes()) <|>
        string_ignore_case("secs".as_bytes()) <|>
        string_ignore_case("sec".as_bytes()) <|>
        string_ignore_case("s".as_bytes());

        // 1000 ms in 1 sec
        ret 1000
    }
}

fn time_range_unit_minutes(i: Input<u8>) -> U8Result<u64> {
    parse!{i;

        string_ignore_case("minutes".as_bytes()) <|>
        string_ignore_case("minute".as_bytes()) <|>
        string_ignore_case("mins".as_bytes()) <|>
        string_ignore_case("min".as_bytes()) <|>
        string_ignore_case("m".as_bytes());

        // 60000 ms in a minute
        ret 60000
    }
}

fn time_range_unit_hours(i: Input<u8>) -> U8Result<u64> {
    parse!{i;

        string_ignore_case("hours".as_bytes()) <|>
        string_ignore_case("hour".as_bytes()) <|>
        string_ignore_case("hrs".as_bytes()) <|>
        string_ignore_case("hr".as_bytes()) <|>
        string_ignore_case("h".as_bytes());

        // 3600000 ms in an hour
        ret 3600000
    }
}

fn time_range_unit_days(i: Input<u8>) -> U8Result<u64> {
    parse!{i;

        string_ignore_case("days".as_bytes()) <|>
        string_ignore_case("day".as_bytes()) <|>
        string_ignore_case("dys".as_bytes()) <|>
        string_ignore_case("dy".as_bytes()) <|>
        string_ignore_case("d".as_bytes());

        // 86400000 ms in a day
        ret 86400000
    }
}

fn time_range_unit_weeks(i: Input<u8>) -> U8Result<u64> {
    parse!{i;

        string_ignore_case("weeks".as_bytes()) <|>
        string_ignore_case("week".as_bytes()) <|>
        string_ignore_case("wks".as_bytes()) <|>
        string_ignore_case("wk".as_bytes()) <|>
        string_ignore_case("w".as_bytes());

        // 604800000 ms in a week
        ret 604800000
    }
}

/* utility parsers */

fn space_or_tab(input: Input<u8>) -> U8Result<()> {
    parse!{input;
        or(
            |i| token(i, b' '),
            |i| token(i, b'\t')
        );
        ret ()
    }
}

fn string_ignore_case<'a>(i: Input<'a, u8>, s: &[u8])
    -> SimpleResult<'a, u8, &'a [u8]> {
    let b = i.buffer();

    if s.len() > b.len() {
        return i.incomplete(s.len() - b.len());
    }

    let d = &b[..s.len()];

    for j in 0..s.len() {

        if !(s[j]).eq_ignore_ascii_case(&(d[j])) {
            return i.replace(&b[j..]).err(Error::expected(d[j]))
        }
    }

    i.replace(&b[s.len()..]).ret(d)
}
