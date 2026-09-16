# SENVEND API Kotlin Client

The `app` module contains the SENVEND API Kotlin client library.
It is published to Maven Central as `com.senbax.senvend:senvend-api`.
The `example` module is a simple example of how to use the client library to interact with the SENVEND Terminal.

## Build Manually

`gen.sh` downloads the pinned `buf` and `protoc` versions and regenerates the client into `app/senvend_api`.

The `grpc-java` and `grpc-kotlin` protoc plugins are used as buf *local* plugins, because buf.build's plugin registry has a pretty hard rate limit. They are resolved from Maven Central into `.plugins` by the gradle `downloadGrpcPlugins` task, pinned in [app/build.gradle.kts](./app/build.gradle.kts).

```bash
./gen.sh
```

## Example Usage

```bash
export TERMINAL_IP=192.168.0.10
./gradlew :example:run
```

