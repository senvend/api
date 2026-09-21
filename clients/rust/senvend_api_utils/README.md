# senvend_api_utils

Support types for the [`senvend_api`](https://crates.io/crates/senvend_api)
crate, part of the public APIs to interact with the SENVEND ecosystem.

Provides `UuidVersionError`, the error type returned by the UUID conversions
that [`senvend_api_macros`](https://crates.io/crates/senvend_api_macros)
generates. Code expanded from `#[derive(ProtoUuid4)]` refers to this crate, so
it must be a dependency wherever that derive is used.

```toml
[dependencies]
senvend_api_utils = "0.0.1"
```

## no_std

The crate is `#![no_std]`. The `std` feature is enabled by default and only
pulls in `std`; disable default features for a `no_std` build.

## License

Licensed under either of [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
or [MIT license](https://opensource.org/license/mit) at your option.
