# Chrono-TZ 0.1.1

`Chrono-TZ` is a library that provides implementors of the
[`TimeZone`][timezone] trait for [`rust-chrono`][chrono]. The
impls are generated by a build script using the [`IANA database`][iana].

[chrono]: https://github.com/lifthrasiir/rust-chrono
[timezone]: ../chrono/offset/trait.TimeZone.html
[iana]: http://www.iana.org/time-zones

## Usage

Put this in your `Cargo.toml`:

```toml
[dependencies]
chrono = "0.2"
chrono-tz = "0.1"
```

Then you will need to write (in your crate root):

```rust
extern crate chrono;
extern crate chrono_tz;
```

## Examples

Create a time in one timezone and convert it to UTC

```rust
use chrono::{TimeZone, UTC};
use chrono_tz::US::Pacific;

let pacific_time = Pacific.ymd(1990, 5, 6).and_hms(12, 30, 45);
let utc_time = pacific_time.with_timezone(&UTC);
assert_eq!(utc_time, UTC.ymd(1990, 5, 6).and_hms(19, 30, 45));
```

London and New York change their clocks on different days in March
so only have a 4-hour difference on certain days.

```rust
use chrono::TimeZone;
use chrono_tz::Europe::London;
use chrono_tz::America::New_York;

let london_time = London.ymd(2016, 3, 18).and_hms(3, 0, 0);
let ny_time = london_time.with_timezone(&New_York);
assert_eq!(ny_time, New_York.ymd(2016, 3, 17).and_hms(23, 0, 0));
```

Adding 24 hours across a daylight savings change causes a change
in local time

```rust
use chrono::{TimeZone, Duration};
use chrono_tz::Europe::London;

let dt = London.ymd(2016, 10, 29).and_hms(12, 0, 0);
let later = dt + Duration::hours(24);
assert_eq!(later, London.ymd(2016, 10, 30).and_hms(11, 0, 0));
```

And of course you can always convert a local time to a unix timestamp

```rust
use chrono::TimeZone;
use chrono_tz::Asia::Kolkata;

let dt = Kolkata.ymd(2000, 1, 1).and_hms(0, 0, 0);
let timestamp = dt.timestamp();
assert_eq!(timestamp, 946665000);
```

Pretty-printing a string will use the correct abbreviation for the timezone

```rust
use chrono::TimeZone;
use chrono_tz::Europe::London;

let dt = London.ymd(2016, 5, 10).and_hms(12, 0, 0);
assert_eq!(dt.to_string(), "2016-05-10 12:00:00 BST");
assert_eq!(dt.to_rfc3339(), "2016-05-10T12:00:00+01:00");
```

## Known Issues

The timezone info for Dushanbe is not parsed correctly by [`zoneinfo_parse`][zoneinfo_parse]
and so I have commented out a line from that file to work around it.

```
# Tajikistan
# From Shanks & Pottenger.
# Zone	NAME		GMTOFF	RULES	FORMAT	[UNTIL]
Zone	Asia/Dushanbe	4:35:12 -	LMT	1924 May  2
			5:00	-	+05	1930 Jun 21
			6:00 RussiaAsia +06/+07	1991 Mar 31  2:00s
#			5:00	1:00	+05/+06	1991 Sep  9  2:00s # FIXME: fails to parse
			5:00	-	+05
```

Strings cannot be parsed into appropriate datetimes currently, they can only be printed out.
Chrono handles fixed offsets only for parsing.

Currently no rustc-serialize or serde support.

[zoneinfo_parse]: https://github.com/rust-datetime/zoneinfo-parse
