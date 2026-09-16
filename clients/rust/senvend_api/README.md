# senvend_api

Public APIs to interact with the SENVEND ecosystem.

This crate contains the generated Rust client for the SENVEND API, built with
[prost](https://crates.io/crates/prost) and [tonic](https://crates.io/crates/tonic).

## Usage

```toml
[dependencies]
senvend_api = "0.0.1"
tonic = "0.14"
tokio = { version = "1", features = ["full"] }
```

The API runs on SENVEND Payment Terminals and is reachable over your local
network. Enable and configure it in the terminal settings on
[my.senvend.com](https://my.senvend.com); by default it listens on port
`11111`.

Messages live under `senvend_api::protos::api::v1`, the service clients under
`senvend_api::protos::local::v1`:

```rust,no_run
use senvend_api::protos::api::v1::VersionRequest;
use senvend_api::protos::local::v1::version_service_client::VersionServiceClient;

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let mut client = VersionServiceClient::connect("http://192.168.0.10:11111").await?;
let response = client.version(VersionRequest::default()).await?;
println!("{:?}", response.into_inner());
# Ok(())
# }
```

`PayService`, `AgeVerificationService` and `VendService` are available the same
way. A runnable payment example is in the
[repository](https://github.com/senvend/api/tree/main/clients/rust/example).

## Features

`proto_full` is enabled by default and covers `api-v1`, `cloud-v1` and
`local-v1`. Disable default features to compile only what you use.

## Documentation

- [Protobuf definitions and API documentation](https://github.com/senvend/api)
- [SENVEND schema registry on buf.build](https://buf.build/senvend/api)

If you need help during your integration and want to handle it confidentially,
reach out via [api@senvend.com](mailto:api@senvend.com).

## License

Licensed under either of [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
or [MIT license](https://opensource.org/license/mit) at your option.
