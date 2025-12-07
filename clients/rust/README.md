# SENVEND Api Rust Client

## Example Usage

See the [example](./example) crate for a complete example of how to use the generated Rust client.

## buf.build plugins

We are currently using local buf.build plugins only for 2 main reasons:

- buf.build's plugin registry has a pretty hard rate limit
- neoeinstein-prost plugin's are outdated, there hasn't been a release for a while

That's why we install a git version of the plugins locally.\
See [install-protoc-plugins.sh](./install-protoc-plugins.sh) for more details.

## build manually

Ensure buf.build cli is installed. See [installation instructions](https://buf.build/docs/cli/installation/).

```bash
./gen.sh
```
