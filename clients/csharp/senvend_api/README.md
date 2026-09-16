# Senvend.Api

Public APIs to interact with the SENVEND ecosystem.

This package contains the generated C# client for the SENVEND API, built on
[Google.Protobuf](https://www.nuget.org/packages/Google.Protobuf) and
[Grpc.Net.Client](https://www.nuget.org/packages/Grpc.Net.Client). It targets
`netstandard2.0` and `net8.0`.

## Installation

```bash
dotnet add package Senvend.Api
```

## Usage

The API runs on SENVEND Payment Terminals and is reachable over your local
network. Enable and configure it in the terminal settings on
[my.senvend.com](https://my.senvend.com); by default it listens on port
`11111`.

Messages live in `com.senbax.senvend.proto.Api.V1`, the service clients in
`com.senbax.senvend.proto.Local.V1`:

```csharp
using com.senbax.senvend.proto.Api.V1;
using com.senbax.senvend.proto.Local.V1;
using Grpc.Net.Client;

using var channel = GrpcChannel.ForAddress("http://192.168.0.10:11111");
var client = new VersionService.VersionServiceClient(channel);
Console.WriteLine(client.Version(new VersionRequest()));
```

`PayService`, `AgeVerificationService` and `VendService` are available on the
same channel. A runnable payment example is included in the
[repository](https://github.com/senvend/api/tree/main/clients/csharp/example).

## Documentation

- [Protobuf definitions and API documentation](https://github.com/senvend/api)
- [SENVEND schema registry on buf.build](https://buf.build/senvend/api)

If you need help during your integration and want to handle it confidentially,
reach out via [api@senvend.com](mailto:api@senvend.com).

## License

Licensed under either of
[Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) or
[MIT license](https://opensource.org/license/mit) at your option.
