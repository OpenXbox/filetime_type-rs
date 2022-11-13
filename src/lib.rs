//! An independent FILETIME parsing / conversion crate
//!
//! The need for this came up when attempting to parse raw FILETIME structures
//! from binary files.
//!
//! Be aware: It certainly has it's quirks when receiving unexpected/invalid input!
//!
//! ## Quickstart
//!
//! ```
//! use filetime_type::FileTime;
//! use chrono::{DateTime, Utc};
//!
//! // Create FileTime from current system time
//! let ft_now = FileTime::now();
//!
//! // Parsing from i64
//! let ft_i64 = FileTime::from_i64(128930364000001000);
//! println!("Since FILETIME-Epoch: secs: {} leap-nanosecs: {}",
//!     ft_i64.seconds(),
//!     ft_i64.nanoseconds());
//!
//! // Parsing from raw bytes
//! let raw_filetime: [u8; 8] = [0xCE, 0xEB, 0x7D, 0x1A, 0x61, 0x59, 0xCE, 0x01];
//! let ft = FileTime::from(raw_filetime);
//!
//! // Into raw bytes
//! let raw: [u8; 8] = FileTime::now().into();
//!
//! // Parsing from DateTime<Utc>
//! let ft_dt = FileTime::from_datetime(Utc::now());
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

    /// Creates a new timestamp representing the current system time
    pub fn now() -> Self {
        Utc::now().into()
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
    /// ```
    /// use filetime_type::FileTime;
    ///
    /// let ft_i64 = FileTime::now().filetime();
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
    /// use filetime_type::FileTime;
    ///
    /// // 2009-07-25T23:00:00.0000Z
    /// let ft = FileTime::from_i64(128930364000001000);
    /// ```
    pub fn from_i64(filetime: i64) -> Self {
        let secs: i64 = filetime / Self::HUNDREDS_OF_NANOSECONDS;
        let nsecs: i64 = filetime % Self::HUNDREDS_OF_NANOSECONDS;

        Self { secs, nsecs }
    }

    /// Example
    /// ```
    /// use chrono::Utc;
    /// use filetime_type::FileTime;
    ///
    /// let ft = FileTime::from_datetime(Utc::now());
    /// ```
    pub fn from_datetime(dt: DateTime<Utc>) -> Self {
        let nsecs = Self::EPOCH_AS_FILETIME
            + (dt.timestamp() * Self::HUNDREDS_OF_NANOSECONDS)
            + dt.timestamp_subsec_nanos() as i64;
        Self::from_i64(nsecs)
    }

    /// Example
    /// ```
    /// use chrono::{DateTime, Utc};
    /// use filetime_type::FileTime;
    ///
    /// let ft_now: DateTime<Utc> = FileTime::now().to_datetime();
    /// ```
    pub fn to_datetime(&self) -> DateTime<Utc> {
        Self::filetime_epoch() + Duration::seconds(self.secs) + Duration::nanoseconds(self.nsecs)
    }
}

impl fmt::Display for FileTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DateTime={} secs={} nsecs={}",
            self.to_datetime(),
            self.secs,
            self.nsecs
        )
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

impl From<[u8; 8]> for FileTime {
    fn from(val: [u8; 8]) -> Self {
        Self::from_i64(i64::from_le_bytes(val))
    }
}

impl From<FileTime> for [u8; 8] {
    fn from(ft: FileTime) -> Self {
        ft.filetime().to_le_bytes()
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

    #[test]
    fn from_u8_array() {
        let bytes = [0xCE_u8, 0xEB, 0x7D, 0x1A, 0x61, 0x59, 0xCE, 0x01];
        let ft = FileTime::from(bytes);
        assert_eq!(
            ft,
            FileTime {
                secs: 13013971283,
                nsecs: 1482830
            }
        );
    }

    #[test]
    fn into_u8_array() {
        let bytes = [0xCE_u8, 0xEB, 0x7D, 0x1A, 0x61, 0x59, 0xCE, 0x01];
        let ft: [u8; 8] = FileTime {
            secs: 13013971283,
            nsecs: 1482830,
        }
        .into();
        assert_eq!(ft, bytes);
    }

    #[test]
    fn from_u8_array_min() {
        let bytes = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let ft: [u8; 8] = FileTime { secs: 0, nsecs: 0 }.into();
        assert_eq!(ft, bytes);
    }
}
