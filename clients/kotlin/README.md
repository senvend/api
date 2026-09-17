# SENVEND API Kotlin Client

Public APIs to interact with the SENVEND ecosystem.

The `app` module contains the generated Kotlin client library, published to
Maven Central as `com.senbax.senvend:senvend-api`. The `example` module is a
runnable example of how to use it against a SENVEND Terminal.

## Installation

Gradle:

```kotlin
dependencies {
    implementation("com.senbax.senvend:senvend-api:0.0.1")
}
```

Maven:

```xml
<dependency>
  <groupId>com.senbax.senvend</groupId>
  <artifactId>senvend-api</artifactId>
  <version>0.0.1</version>
</dependency>
```

The library targets Java 21. It brings its own gRPC and protobuf dependencies,
including `grpc-netty` for the channel.

## Usage

The API runs on SENVEND Payment Terminals and is reachable over your local
network. Enable and configure it in the terminal settings on
[my.senvend.com](https://my.senvend.com); by default it listens on port
`11111`.

Messages live in `com.senbax.senvend.proto.api.v1`, the coroutine stubs in
`com.senbax.senvend.proto.local.v1`:

```kotlin
import com.senbax.senvend.proto.api.v1.versionRequest
import com.senbax.senvend.proto.local.v1.VersionServiceGrpcKt
import io.grpc.netty.NettyChannelBuilder
import kotlinx.coroutines.runBlocking

val channel = NettyChannelBuilder.forAddress("192.168.0.10", 11111).usePlaintext().build()
val client = VersionServiceGrpcKt.VersionServiceCoroutineStub(channel)
runBlocking { println(client.version(versionRequest {})) }
```

`PayServiceGrpcKt`, `AgeVerificationServiceGrpcKt` and `VendServiceGrpcKt` are
available on the same channel. A runnable payment example is in the
[repository](https://github.com/senvend/api/tree/main/clients/kotlin/example):

```bash
export TERMINAL_IP=192.168.0.10
export TERMINAL_PORT=11111
./gradlew :example:run
```

## Documentation

- [Protobuf definitions and API documentation](https://github.com/senvend/api)
- [SENVEND schema registry on buf.build](https://buf.build/senvend/api)

If you need help during your integration and want to handle it confidentially,
reach out via [api@senvend.com](mailto:api@senvend.com).

## Build Manually

`gen.sh` downloads the pinned `buf` and `protoc` versions and regenerates the client into `app/senvend_api`.

The `grpc-java` and `grpc-kotlin` protoc plugins are used as buf *local* plugins, because buf.build's plugin registry has a pretty hard rate limit. They are resolved from Maven Central into `.plugins` by the gradle `downloadGrpcPlugins` task, pinned in [app/build.gradle.kts](./app/build.gradle.kts).

```bash
./gen.sh
```

## License

Licensed under either of
[Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) or
[MIT license](https://opensource.org/license/mit) at your option.
