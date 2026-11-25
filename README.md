# SENVEND Public API

This repository contains public APIs for the SENVEND ecosystem.  
We rely on Protocol Buffers (protobuf) and gRPC to define and implement our APIs.  
To make handling protobuf definitions easier, we use [buf.build](https://buf.build/).  
All protobuf definitions are stored in the [`proto/`](./proto/) directory.

The current version of the API is 1.0.0.
Changes between versions are listed in the [CHANGELOG](./CHANGELOG.md).

## Help us improve

Feel free to open issues if something is missing, unclear or not working as expected.  
You are also welcome to contribute directly by opening Pull Requests.  
If you need any help during your integration and you want to handle it confidentially, please reach out to us via [api@senvend.com](mailto:api@senvend.com).

## Local vs Cloud APIs

In the future, you can choose between two main APIs: cloud and local.
The payloads are almost identical, cloud and local APIs only provide small wrappers around the main data structures.

Currently only the local version is available.

### Local API

The local API runs on SENVEND Payment Terminals and can be accessed via your local network.  
You can find the protobuf definitions and documentation for the local API in the [`proto/local/`](./proto/local/) directory and on [buf.build](https://buf.build/senvend/api/docs/main:local.v1).
The terminal needs to be connected to the same network as your client application. You can connect it via Ethernet or Wi-Fi.  
Before using the local API with your terminal, you need to enable and configure it in the terminal settings on [my.senvend.com](https://my.senvend.com).  
By default the API listens on port `11111` without authentication and encryption. 
But it can be configured to use TLS (self-signed via [senvendCA.crt](./senvendCA.crt)) and an authentication token.  
Verify the root CA before trusting it, its SHA-256 fingerprint is
`C0:18:70:DC:71:94:35:1A:51:BD:84:75:FF:FE:80:91:31:4A:98:E4:CE:DD:88:43:43:DF:0E:4A:2A:E9:A2:51`.  
The IP and port where the API can be reached can be displayed on the terminal, in the Service screen, subsection "RPC Info".
The IPs of the last 5 connected clients are listed there as well, including a notice when authentication from a client IP failed.

### Cloud API (not released yet)

The cloud API will be accessible at `api.senvend.com:443` once released.  
You can find the protobuf definitions for the cloud API in the [`proto/cloud/`](./proto/cloud/) directory.

## Commandline client

This repository contains a wrapper script for the buf curl cli tool.

You can use it like this for one off commands:

```bash
./curl http://192.168.0.10:11111/local.v1.VersionService/Version --data '{}'
```

Or like this to interact with a continuous stream (messages can be pasted to your terminal):

```bash
./curl http://192.168.0.10:11111/local.v1.VersionService/Version --data @-
```

## Generated clients

We provide example clients for various programming languages to help developers get started quickly.  
Take a look at the [`clients/`](./clients/) directory for more information.  
We currently provide clients for the following programming languages:  

- Rust ([clients/rust/](./clients/rust/))
- Python ([clients/python/](./clients/python/))
- Kotlin ([clients/kotlin/](./clients/kotlin/))
- C# ([clients/csharp/](./clients/csharp/))

You can request clients for other programming languages by opening an issue.

### Example Code

Every client should contain at least one example code snippet demonstrating how to use the generated code.  
Those example snippets should use `TERMINAL_IP` and `TERMINAL_PORT` environment variables to configure the connection to the terminal.

### Regenerate clients

You can use [./gen.sh](./gen.sh) to regenerate the example clients after making changes to the protobuf definitions.

## Buf schema registry

All definitions in this repository are published to the [SENVEND schema registry on buf.build](https://buf.build/senvend/api).  
This means you can use them as dependencies in your [buf.build](https://buf.build/) projects if you want to generate your own client.

## Previews

Before a release, we can publish a preview of the upcoming version, so you can try unreleased features early.
A preview covers every client library and the protobuf definitions on buf.build.
Previews are prereleases: you only get one by naming its version, while plain installs and version ranges keep resolving to the latest release.

A preview of the upcoming version `X.Y.Z` has a number `N` and goes by the same version everywhere, spelled the way each registry expects:

| Registry  | Preview version        | Example                                                   |
| --------- | ---------------------- | --------------------------------------------------------- |
| crates.io | `X.Y.Z-dev.N`          | `senvend_api = "=1.0.0-dev.42"`                           |
| PyPI      | `X.Y.Z.devN`           | `pip install senvend-api==1.0.0.dev42`                    |
| NuGet     | `X.Y.Z-dev.N`          | `dotnet add package Senvend.Api --version 1.0.0-dev.42`   |
| Maven     | `X.Y.Z-dev.N-SNAPSHOT` | `com.senbax.senvend:senvend-api:1.0.0-dev.42-SNAPSHOT`    |
| buf.build | `vX.Y.Z-dev.N`         | `buf.build/senvend/api:v1.0.0-dev.42`                     |

Maven previews are snapshots in the Maven Central snapshot repository, which your build has to know about, limited to our artifact:

```kotlin
repositories {
    maven("https://central.sonatype.com/repository/maven-snapshots/") {
        content { includeModule("com.senbax.senvend", "senvend-api") }
    }
    mavenCentral()
}
```

PyPI only receives previews of a package once the package has a release there.

Previews are meant for testing: what a preview contains can still change before the release, and Maven snapshots are removed after 90 days.
Switch to the release once it is out.
If you would like to test an unreleased feature, reach out to us via [api@senvend.com](mailto:api@senvend.com).

## License

Licensed under either of

- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
- [MIT license](https://opensource.org/license/mit)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this repository, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
