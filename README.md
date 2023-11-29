# filetime_type - Platform-agnostic FILETIME conversion library

[![Crates.io](https://img.shields.io/crates/v/filetime_type.svg)](https://crates.io/crates/filetime_type)
[![Docs.rs](https://docs.rs/filetime_type/badge.svg)](https://docs.rs/filetime_type)
[![CI](https://github.com/OpenXbox/filetime_type-rs/workflows/Test/badge.svg)](https://github.com/OpenXbox/filetime_type-rs/actions)


An independent FILETIME parsing / conversion crate

The need for this came up when attempting to parse raw FILETIME structures
from binary files.

## Documentation

Please see: <https://docs.rs/filetime_type>

## Add to your project

Add the following line to your `Cargo.toml` file.

```toml
[dependencies]
filetime_type = "0.1"
```

## Related projects / crates

- [nt-time](https://crates.io/crates/nt-time) - Does serve the same purpose + serde support + more tests
