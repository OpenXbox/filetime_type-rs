# filetime_type - Platform-agnostic FILETIME conversion library

[![Crates.io](https://img.shields.io/crates/v/filetime_type.svg)](https://crates.io/crates/filetime_type)
[![Docs.rs](https://docs.rs/filetime_type/badge.svg)](https://docs.rs/filetime_type)
[![CI](https://github.com/OpenXbox/filetime_type-rs/workflows/Test/badge.svg)](https://github.com/OpenXbox/filetime_type-rs/actions)


An independent FILETIME parsing / conversion crate

The need for this came up when attempting to parse raw FILETIME structures
from binary files.

## Quickstart

```rs
use filetime_type::FileTime;
use chrono::{DateTime, Utc};

// Create FileTime from current system time
let ft_now = FileTime::now();

// Parsing from i64
let ft_i64 = FileTime::from_i64(128930364000001000);
println!("Since FILETIME-Epoch: secs: {} leap-nanosecs: {}",
    ft_i64.seconds(),
    ft_i64.nanoseconds());

// Parsing from raw bytes
let raw_filetime = [0xCE, 0xEB, 0x7D, 0x1A, 0x61, 0x59, 0xCE, 0x01];
let ft = FileTime::from_i64(i64::from_le_bytes(raw_filetime));
let ft2: i64 = ft.filetime();

// Parsing from DateTime<Utc>
let dt: DateTime<Utc> = Utc::now();
let ft_dt = FileTime::from_datetime(dt);
let dt2: DateTime<Utc> = ft_dt.to_datetime();
```

## Add to your project

Add the following line to your `Cargo.toml` file.

```toml
[dependencies]
filetime_type = "0.1"
```


Documentation: <https://docs.rs/filetime_type>

