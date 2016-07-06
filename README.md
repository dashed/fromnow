fromnow
=======

> Generate dates and times relative from now.

## Usage

```
fromnow v0.1.0 (semver.org 2.0)
Alberto Leal <mailforalberto@gmail.com> (github.com/dashed)
Generate dates and times relative from now.

USAGE:
    fromnow [OPTIONS] <duration>...

OPTIONS:
    -f, --format <format>
        Format specifier for the combined date and time output.
        Example output: July 5, 2016 9:36 PM
        Default is: %B %-d, %Y %-l:%M %p
        
        See: https://lifthrasiir.github.io/rust-chrono/chrono/format/strftime/index.html
        
    -h, --help               Prints help information
    -s, --subtract           Subtract time spans/durations to generate date/time in the past.
                             By default, time spans/durations are added.
    -V, --version            Prints version information

ARGS:
    <duration>...
        A duration/span of time. (Example: 3 hours)
        A duration is an integer number followed by time span units. (Example: 42 hours)
        There may be whitespace between the integer and the time span unit.
        Whitespace between <duration> items is optional (Example: 3hours30mins).
        There may be an ' and ' between duration items (Example: 42 days and 42 mins).
        
        Arguments may compose into a valid duration/time span; removing the need to enclose durations in quotes.
                
        Time span units are case-insensitive.
        Valid time span units and their abbreviations:
        weeks/week/wks/wk/w
        days/day/dys/dy/d
        hours/hour/hrs/hr/h
        minutes/minute/mins/min/m
        seconds/second/secs/sec/s
        milliseconds/millisecond/msecs/msec/ms

```

## Examples

```
fromnow 6 hours
fromnow 6 hours 42mins
fromnow 6 hours 42mins 6 week
fromnow 6 hours 42mins 6 week and 9000 secs
fromnow 6 hours 42mins 6 week and 9000 secs -f "%B %-d, %Y %-l:%M %p"
```

## Format Specifiers

Taken from: https://lifthrasiir.github.io/rust-chrono/chrono/format/strftime/index.html

Spec. | Example       | Description
----- | ------------- | -----------
      |               | **DATE SPECIFIERS:**
`%Y`  | `2001`        | The full proleptic Gregorian year, zero-padded to 4 digits. [1]
`%C`  | `20`          | The proleptic Gregorian year divided by 100, zero-padded to 2 digits. [2]
`%y`  | `01`          | The proleptic Gregorian year modulo 100, zero-padded to 2 digits. [2]
      |               |
`%m`  | `07`          | Month number (01--12), zero-padded to 2 digits.
`%b`  | `Jul`         | Abbreviated month name. Always 3 letters.
`%B`  | `July`        | Full month name. Also accepts corresponding abbreviation in parsing.
`%h`  | `Jul`         | Same to `%b`.
      |               |
`%d`  | `08`          | Day number (01--31), zero-padded to 2 digits.
`%e`  | ` 8`          | Same to `%d` but space-padded. Same to `%_d`.
      |               |
`%a`  | `Sun`         | Abbreviated weekday name. Always 3 letters.
`%A`  | `Sunday`      | Full weekday name. Also accepts corresponding abbreviation in parsing.
`%w`  | `0`           | Sunday = 0, Monday = 1, ..., Saturday = 6.
`%u`  | `7`           | Monday = 1, Tuesday = 2, ..., Sunday = 7. (ISO 8601)
      |               |
`%U`  | `28`          | Week number starting with Sunday (00--53), zero-padded to 2 digits. [3]
`%W`  | `27`          | Same to `%U`, but week 1 starts with the first Monday in that year instead.
      |               |
`%G`  | `2001`        | Same to `%Y` but uses the year number in ISO 8601 week date. [4]
`%g`  | `01`          | Same to `%y` but uses the year number in ISO 8601 week date. [4]
`%V`  | `27`          | Same to `%U` but uses the week number in ISO 8601 week date (01--53). [4]
      |               |
`%j`  | `189`         | Day of the year (001--366), zero-padded to 3 digits.
      |               |
`%D`  | `07/08/01`    | Month-day-year format. Same to `%m/%d/%y`.
`%x`  | `07/08/01`    | Same to `%D`.
`%F`  | `2001-07-08`  | Year-month-day format (ISO 8601). Same to `%Y-%m-%d`.
`%v`  | ` 8-Jul-2001` | Day-month-year format. Same to `%e-%b-%Y`.
      |               |
      |               | **TIME SPECIFIERS:**
`%H`  | `00`          | Hour number (00--23), zero-padded to 2 digits.
`%k`  | ` 0`          | Same to `%H` but space-padded. Same to `%_H`.
`%I`  | `12`          | Hour number in 12-hour clocks (01--12), zero-padded to 2 digits.
`%l`  | `12`          | Same to `%I` but space-padded. Same to `%_I`.
      |               |
`%P`  | `am`          | `am` or `pm` in 12-hour clocks.
`%p`  | `AM`          | `AM` or `PM` in 12-hour clocks.
      |               |
`%M`  | `34`          | Minute number (00--59), zero-padded to 2 digits.
`%S`  | `60`          | Second number (00--60), zero-padded to 2 digits. [5]
`%f`  | `026490000`   | The fractional seconds (in nanoseconds) since last whole second. [8]
`%.f` | `.026490`     | Similar to `.%f` but left-aligned. [8]
`%.3f`| `.026`        | Similar to `.%f` but left-aligned but fixed to a length of 3. [8]
`%.6f`| `.026490`     | Similar to `.%f` but left-aligned but fixed to a length of 6. [8]
`%.9f`| `.026490000`  | Similar to `.%f` but left-aligned but fixed to a length of 9. [8]
      |               |
`%R`  | `00:34`       | Hour-minute format. Same to `%H:%M`.
`%T`  | `00:34:60`    | Hour-minute-second format. Same to `%H:%M:%S`.
`%X`  | `00:34:60`    | Same to `%T`.
`%r`  | `12:34:60 AM` | Hour-minute-second format in 12-hour clocks. Same to `%I:%M:%S %p`.
      |               |
      |               | **TIME ZONE SPECIFIERS:**
`%Z`  | `ACST`        | *Formatting only:* Local time zone name.
`%z`  | `+0930`       | Offset from the local time to UTC (with UTC being `+0000`).
`%:z` | `+09:30`      | Same to `%z` but with a colon.
      |               |
      |               | **DATE & TIME SPECIFIERS:**
`%c`  | `Sun Jul  8 00:34:60 2001` | `ctime` date & time format. Same to `%a %b %e %T %Y` sans `\n`.
`%+`  | `2001-07-08T00:34:60.026490+09:30` | ISO 8601 / RFC 3339 date & time format. [6]
      |               |
`%s`  | `994518299`   | UNIX timestamp, the number of seconds since 1970-01-01 00:00 UTC. [7]
      |               |
      |               | **SPECIAL SPECIFIERS:**
`%t`  |               | Literal tab (`\t`).
`%n`  |               | Literal newline (`\n`).
`%%`  |               | Literal percent sign.


## Install

Direct downloads are available through the [releases page](https://github.com/dashed/fromnow/releases).

If you're on OSX, you may install using [Homebrew](http://brew.sh/):

```
brew install https://raw.githubusercontent.com/dashed/fromnow/master/fromnow.rb
```

## Why?

I'm aware there exists a `date` command that does essentially the same thing. 
However, there are different flavours of the `date` command in OSX and linux. Example:

- http://stackoverflow.com/q/9804966/412627
- http://stackoverflow.com/q/498358/412627

I created `fromnow` for my own usecases rather than making a bash script to 'monkeypatch' the `date` command.


License
=======

MIT.
