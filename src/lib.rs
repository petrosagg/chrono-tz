//! # Chrono-TZ 0.1.1
//!
//! `Chrono-TZ` is a library that provides implementors of the
//! [`TimeZone`][timezone] trait for [`rust-chrono`][chrono]. The
//! impls are generated by a build script using the [`IANA database`][iana].
//!
//! [chrono]: https://github.com/lifthrasiir/rust-chrono
//! [timezone]: ../chrono/offset/trait.TimeZone.html
//! [iana]: http://www.iana.org/time-zones
//!
//! ## Usage
//!
//! Put this in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! chrono = "0.2"
//! chrono-tz = "0.1"
//! ```
//!
//! Then you will need to write (in your crate root):
//!
//! ```
//! extern crate chrono;
//! extern crate chrono_tz;
//! ```
//!
//! ## Examples
//!
//! Create a time in one timezone and convert it to UTC
//!
//! ```
//! # extern crate chrono;
//! # extern crate chrono_tz;
//! use chrono::{TimeZone, UTC};
//! use chrono_tz::US::Pacific;
//!
//! # fn main() {
//! let pacific_time = Pacific.ymd(1990, 5, 6).and_hms(12, 30, 45);
//! let utc_time = pacific_time.with_timezone(&UTC);
//! assert_eq!(utc_time, UTC.ymd(1990, 5, 6).and_hms(19, 30, 45));
//! # }
//! ```
//!
//! London and New York change their clocks on different days in March
//! so only have a 4-hour difference on certain days.
//!
//! ```
//! # extern crate chrono;
//! # extern crate chrono_tz;
//! use chrono::TimeZone;
//! use chrono_tz::Europe::London;
//! use chrono_tz::America::New_York;
//!
//! # fn main() {
//! let london_time = London.ymd(2016, 3, 18).and_hms(3, 0, 0);
//! let ny_time = london_time.with_timezone(&New_York);
//! assert_eq!(ny_time, New_York.ymd(2016, 3, 17).and_hms(23, 0, 0));
//! # }
//! ```
//!
//! Adding 24 hours across a daylight savings change causes a change
//! in local time
//!
//! ```
//! # extern crate chrono;
//! # extern crate chrono_tz;
//! use chrono::{TimeZone, Duration};
//! use chrono_tz::Europe::London;
//!
//! # fn main() {
//! let dt = London.ymd(2016, 10, 29).and_hms(12, 0, 0);
//! let later = dt + Duration::hours(24);
//! assert_eq!(later, London.ymd(2016, 10, 30).and_hms(11, 0, 0));
//! # }
//! ```
//!
//! And of course you can always convert a local time to a unix timestamp
//!
//! ```
//! # extern crate chrono;
//! # extern crate chrono_tz;
//! use chrono::TimeZone;
//! use chrono_tz::Asia::Kolkata;
//! 
//! # fn main() {
//! let dt = Kolkata.ymd(2000, 1, 1).and_hms(0, 0, 0);
//! let timestamp = dt.timestamp();
//! assert_eq!(timestamp, 946665000);
//! # }
//! ```
//!
//! Pretty-printing a string will use the correct abbreviation for the timezone
//!
//! ```
//! # extern crate chrono;
//! # extern crate chrono_tz;
//! use chrono::TimeZone;
//! use chrono_tz::Europe::London;
//! 
//! # fn main() {
//! let dt = London.ymd(2016, 5, 10).and_hms(12, 0, 0);
//! assert_eq!(dt.to_string(), "2016-05-10 12:00:00 BST");
//! assert_eq!(dt.to_rfc3339(), "2016-05-10T12:00:00+01:00");
//! # }
//! ```

extern crate chrono;

mod timezone_impl;
mod timezones;
mod directory;

pub use directory::*;

#[cfg(test)]
mod tests {
    use super::Europe::London;
    use super::Europe::Berlin;
    use chrono::{TimeZone, Duration};

    #[test]
    fn london_to_berlin() {
        let dt = London.ymd(2016, 10, 8).and_hms(17, 0, 0);
        let converted = dt.with_timezone(&Berlin);
        let expected = Berlin.ymd(2016, 10, 8).and_hms(18, 0, 0);
        assert_eq!(converted, expected);
    }

    #[test]
    fn london_dst() {
        let dt = London.ymd(2016, 3, 10).and_hms(5, 0, 0);
        let later = dt + Duration::days(180);
        let expected = London.ymd(2016, 9, 6).and_hms(6, 0, 0);
        assert_eq!(later, expected);
    }

    #[test]
    #[should_panic]
    fn nonexistent_time() {
        let _ = London.ymd(2016, 3, 27).and_hms(1, 30, 0);
    }

    #[test]
    fn time_exists() {
        let _ = London.ymd(2016, 3, 27).and_hms(1, 0, 0);
    }

    #[test]
    fn time_exists_2() {
        let _ = London.ymd(2016, 3, 27).and_hms(2, 0, 0);
    }

    #[test]
    #[should_panic]
    fn ambiguous_time() {
        let _ = London.ymd(2016, 10, 30).and_hms(1, 0, 0);
    }

    #[test]
    #[should_panic]
    fn ambiguous_time_2() {
        let _ = London.ymd(2016, 10, 30).and_hms(2, 0, 0);
    }
}
