# SENVEND API Python Client

Public APIs to interact with the SENVEND ecosystem.

The generated client is published to PyPI in two flavours, both built with
[betterproto2](https://pypi.org/project/betterproto2/):

- [`packages/senvend-api/`](./packages/senvend-api/) — the synchronous client
  ([senvend-api](https://pypi.org/project/senvend-api/)), using `grpcio`.
- [`packages/senvend-api-async/`](./packages/senvend-api-async/) — the asyncio
  client ([senvend-api-async](https://pypi.org/project/senvend-api-async/)),
  using `grpclib`.
- [`packages/example/`](./packages/example/) — a runnable example using the
  synchronous client.

Both packages expose the same messages and services, only the stubs differ.

## Installation

```bash
pip install senvend-api        # synchronous
pip install senvend-api-async  # asyncio
```

## Usage

The API runs on SENVEND Payment Terminals and is reachable over your local
network. Enable and configure it in the terminal settings on
[my.senvend.com](https://my.senvend.com); by default it listens on port
`11111`.

Messages live in `senvend_api.api.v1`, the service stubs in
`senvend_api.local.v1` (`senvend_api_async.*` for the asyncio client):

```python
import grpc
from senvend_api.api.v1 import VersionRequest
from senvend_api.local.v1 import VersionServiceStub

with grpc.insecure_channel("192.168.0.10:11111") as channel:
    print(VersionServiceStub(channel).version(VersionRequest()))
```

### UUID helpers

Both packages ship a handwritten `uuid4` module converting between the
protobuf `api.v1.Uuid4` message and Python's `uuid.UUID`
(`from_uuid`, `to_uuid`, `random_uuid4`). The source of truth is
[shared/uuid4.py](./shared/uuid4.py), `gen.sh` copies it into both packages.

## Example Usage

See [packages/example](./packages/example/) for a complete payment example.

```bash
export TERMINAL_IP=192.168.0.10
export TERMINAL_PORT=11111
uv run --package example packages/example/main.py
```

## Generate your own client

Instead of using the pre-generated packages, you can generate your own client
from the public proto definitions in [../../proto](../../proto). Our own
codegen setup in [buf.gen.yaml](./buf.gen.yaml) is a good starting point.

## Build Manually

Ensure the [uv](https://docs.astral.sh/uv/) cli is installed. `gen.sh` syncs
the dev dependencies, downloads the pinned `buf` version and regenerates both
packages.

The `protoc-gen-python_betterproto2` plugin is used as a buf *local* plugin,
because buf.build's plugin registry has a pretty hard rate limit. It is
installed into `.venv` as a dev dependency.

```bash
./gen.sh
```

## Documentation

- [Protobuf definitions and API documentation](https://github.com/senvend/api)
- [SENVEND schema registry on buf.build](https://buf.build/senvend/api)

If you need help during your integration and want to handle it confidentially,
reach out via [api@senvend.com](mailto:api@senvend.com).

## License

Licensed under either of
[Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) or
[MIT license](https://opensource.org/license/mit) at your option.
