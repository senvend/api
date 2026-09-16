# SENVEND API Rust Client

## Example Usage

See the [example](./example) crate for a complete example of how to use the generated Rust client.

## Generate your own client

Instead of using the pre-generated [senvend_api](./senvend_api) crate, you can generate your own client from the public proto definitions in [../../proto](../../proto). Our own codegen setup in [buf.gen.yaml](./buf.gen.yaml) is a good starting point.

### Derive macros

The [senvend_api_macros](./senvend_api_macros) crate provides `#[derive(ProtoUuid4)]` for converting between the protobuf `api.v1.Uuid4` message and `uuid::Uuid`. It is attached to the generated type via the `type_attribute` option in [buf.gen.yaml](./buf.gen.yaml) and requires [senvend_api_utils](./senvend_api_utils) as a dependency. See the macro's rustdoc for details and the [example](./example) crate for usage.

## buf.build plugins

We are currently using local buf.build plugins only for 2 main reasons:

- buf.build's plugin registry has a pretty hard rate limit
- neoeinstein-prost plugin's are outdated, there hasn't been a release for a while

That's why we install a git version of the plugins locally.\
See [install-protoc-plugins.sh](./install-protoc-plugins.sh) for more details.

## Build Manually

Ensure buf.build cli is installed. See [installation instructions](https://buf.build/docs/cli/installation/).

```bash
./gen.sh
```
