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
                             By default, time
                             spans/durations are added.
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

## Example

```
$ fromnow 6 hours
July 6, 2016 6:33 AM
```

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
