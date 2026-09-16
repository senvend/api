# senvend_api_macros

Procedural macros for the [`senvend_api`](https://crates.io/crates/senvend_api)
crate, part of the public APIs to interact with the SENVEND ecosystem.

Provides `#[derive(ProtoUuid4)]`, which implements conversions between a
protobuf UUID message and [`uuid::Uuid`](https://crates.io/crates/uuid) in both
directions. Conversion fails with
[`senvend_api_utils::UuidVersionError`](https://crates.io/crates/senvend_api_utils)
when the UUID is not version 4 (random).

The derive expands to code referencing `senvend_api_utils`, so that crate must
be a dependency wherever the macro is used:

```toml
[dependencies]
senvend_api_macros = "0.0.1"
senvend_api_utils = "0.0.1"
uuid = "1"
```

`senvend_api` already applies this derive to its generated `api.v1.Uuid4` type,
so you normally get the conversions without depending on this crate directly.
It is published separately because the derive is attached during code
generation, which means anyone generating their own client from the
[public proto definitions](https://github.com/senvend/api) needs it too.

## License

Licensed under either of [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
or [MIT license](https://opensource.org/license/mit) at your option.
