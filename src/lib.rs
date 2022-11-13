//! An independent FILETIME parsing / conversion crate
//!
//! The need for this came up when attempting to parse raw FILETIME structures
//! from binary files.
//!
//! ## Quickstart
//!
//! ```
//! use filetime_type::FileTime;
//! use chrono::{DateTime, Utc};
//!
//! // Parsing from i64
//! let ft_i64 = FileTime::from_i64(128930364000001000);
//! println!("Since FILETIME-Epoch: secs: {} leap-nanosecs: {}",
//!     ft_i64.seconds(),
//!     ft_i64.nanoseconds());
//!
//! // Parsing from raw bytes
//! let raw_filetime = [0xCE, 0xEB, 0x7D, 0x1A, 0x61, 0x59, 0xCE, 0x01];
//! let ft = FileTime::from_i64(i64::from_le_bytes(raw_filetime));
//! let ft2: i64 = ft.filetime();
//!
//! // Parsing from DateTime<Utc>
//! let dt: DateTime<Utc> = Utc::now();
//! let ft_dt = FileTime::from_datetime(dt);
//! let dt2: DateTime<Utc> = ft_dt.to_datetime();
//! ```
use chrono::{prelude::*, Duration};
use std::fmt;

/// FILETIME type
///
/// Used by Microsoft software to describe file creation/access timestamps
/// In contrary to unix, the FILETIME-Epoch is: 1601-01-01-00:00:00.000Z
///
/// Allows conversion between:
/// - Raw i64 value
/// - DateTime UTC
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct FileTime {
    secs: i64,
    nsecs: i64,
}

impl FileTime {
    /// January 1, 1970 as MS file time
    /// aka. 100 of nanoseconds since 1601-01-01-00:00:00.000Z
    const EPOCH_AS_FILETIME: i64 = 116444736000000000;
    const HUNDREDS_OF_NANOSECONDS: i64 = 10000000;

    /// Construct new FileTime by providing seconds and nanoseconds since 1601-01-01-00:00:00.000Z
    pub fn new(secs: i64, nsecs: i64) -> Self {
        Self { secs, nsecs }
    }

    /// Seconds since FILETIME-Epoch
    pub fn seconds(&self) -> i64 {
        self.secs
    }

    /// Leap Nanoseconds since FILETIME-Epoch
    pub fn nanoseconds(&self) -> i64 {
        self.nsecs
    }

    /// Return FILETIME as i64
    /// Can be used to parse raw filetimes from bytes by doing
    /// ```
    /// use chrono::{DateTime, TimeZone, Utc, Duration};
    /// use filetime_type::FileTime;
    ///
    /// let dt = Utc.with_ymd_and_hms(2013, 05, 25, 16, 01, 23)
    ///     .unwrap()
    ///     .checked_add_signed(Duration::nanoseconds(1482830))
    ///     .unwrap();
    /// let val: i64 = i64::from_le_bytes([0xCE, 0xEB, 0x7D, 0x1A, 0x61, 0x59, 0xCE, 0x01]);
    /// let ft = FileTime::from_i64(val);
    /// assert_eq!(ft.to_datetime(), dt);
    /// ```
    pub fn filetime(&self) -> i64 {
        (self.secs * Self::HUNDREDS_OF_NANOSECONDS) + self.nsecs
    }

    /// Return FILETIME epoch as DateTime<Utc>
    /// -> 1601-01-01-00:00:00.000Z
    pub fn filetime_epoch() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(1601, 1, 1, 0, 0, 0).unwrap()
    }

    /// Example
    /// ```
    /// use chrono::{DateTime, TimeZone, Utc, Duration};
    /// use filetime_type::FileTime;
    ///
    /// let dt = Utc.with_ymd_and_hms(1601, 1, 1, 0, 0, 0).unwrap();
    /// let ft = FileTime::from_i64(0);
    /// assert_eq!(dt, ft.to_datetime());
    /// assert_eq!(ft.filetime(), 0)
    /// ```
    pub fn from_i64(filetime: i64) -> Self {
        let secs: i64 = filetime / Self::HUNDREDS_OF_NANOSECONDS;
        let nsecs: i64 = filetime % Self::HUNDREDS_OF_NANOSECONDS;

        Self { secs, nsecs }
    }

    /// Example
    /// ```
    /// use chrono::{TimeZone, Utc, Duration};
    /// use filetime_type::FileTime;
    ///
    /// let dt = Utc.with_ymd_and_hms(2009, 7, 25, 23, 0, 0).unwrap().checked_add_signed(Duration::nanoseconds(1000)).unwrap();
    /// assert_eq!(FileTime::from_datetime(dt), FileTime::from_i64(128930364000001000));
    /// ```
    pub fn from_datetime(dt: DateTime<Utc>) -> Self {
        let nsecs = Self::EPOCH_AS_FILETIME
            + (dt.timestamp() * Self::HUNDREDS_OF_NANOSECONDS)
            + dt.timestamp_subsec_nanos() as i64;
        Self::from_i64(nsecs)
    }

    /// Example
    /// ```
    /// use chrono::{TimeZone, Utc, Duration};
    /// use filetime_type::FileTime;
    ///
    /// let dt = Utc.with_ymd_and_hms(2009, 7, 25, 23, 0, 0).unwrap().checked_add_signed(Duration::nanoseconds(1000)).unwrap();
    /// assert_eq!(FileTime::from_i64(128930364000001000).to_datetime(), dt);
    /// ```
    pub fn to_datetime(&self) -> DateTime<Utc> {
        Self::filetime_epoch() + Duration::seconds(self.secs) + Duration::nanoseconds(self.nsecs)
    }
}

impl fmt::Display for FileTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("DateTime {}", self.to_datetime()))
    }
}

impl From<i64> for FileTime {
    fn from(val: i64) -> Self {
        Self::from_i64(val)
    }
}

impl From<DateTime<Utc>> for FileTime {
    fn from(dt: DateTime<Utc>) -> Self {
        Self::from_datetime(dt)
    }
}

impl From<FileTime> for i64 {
    fn from(ft: FileTime) -> Self {
        ft.filetime()
    }
}

impl From<FileTime> for DateTime<Utc> {
    fn from(ft: FileTime) -> Self {
        ft.to_datetime()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_datetime() {
        let dt = Utc
            .with_ymd_and_hms(2009, 7, 25, 23, 0, 0)
            .unwrap()
            .checked_add_signed(Duration::nanoseconds(1000))
            .unwrap();
        assert_eq!(
            FileTime::from_datetime(dt),
            FileTime::from_i64(128930364000001000)
        );
    }

    #[test]
    fn filetime_epoch() {
        let ft_epoch = FileTime::from_i64(0);
        assert_eq!(
            Utc.with_ymd_and_hms(1601, 1, 1, 0, 0, 0).unwrap(),
            ft_epoch.to_datetime()
        );
        assert_eq!(ft_epoch.to_datetime(), FileTime::filetime_epoch());
    }
}
