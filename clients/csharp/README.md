# SENVEND API C# Client Example

This is a simple example of how to use the SENVEND API C# .NET Client to interact with the SENVEND Terminal.

The solution contains two projects:

- [`senvend_api/`](./senvend_api/) — the generated client library, targeting `netstandard2.0` and `net8.0`.
- [`example/`](./example/) — the runnable example, targeting `net10.0`.

## Build Manually

Building the solution needs the **.NET SDK 10** — the example targets `net10.0`, and the `.slnx` solution format needs SDK 9.0.200 or newer. The library alone builds with any SDK that can target `net8.0`: `dotnet build senvend_api/Senvend.Api.csproj`.

`gen.sh` downloads the pinned `buf` and `protoc` versions and regenerates the client into `./senvend_api`.

The `grpc_csharp_plugin` is used as a buf *local* plugin, because buf.build's plugin registry has a pretty hard rate limit. It is downloaded from the `Grpc.Tools` nuget package by [download-grpc-csharp-plugin.sh](../../scripts/download-grpc-csharp-plugin.sh).

```bash
./gen.sh
dotnet build
```

## Example Usage

```bash
export TERMINAL_IP=192.168.0.10
dotnet run --project example
```
