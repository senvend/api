# senvend-api-async

Public APIs to interact with the SENVEND ecosystem.

This package contains the generated asyncio Python client for the SENVEND API,
built with [betterproto2](https://pypi.org/project/betterproto2/) and
[grpclib](https://pypi.org/project/grpclib/). For the synchronous client see
[senvend-api](https://pypi.org/project/senvend-api/).

## Installation

```bash
pip install senvend-api-async
```

## Usage

The API runs on SENVEND Payment Terminals and is reachable over your local
network. Enable and configure it in the terminal settings on
[my.senvend.com](https://my.senvend.com); by default it listens on port
`11111`.

Messages live in `senvend_api_async.api.v1`, the service stubs in
`senvend_api_async.local.v1`:

```python
import asyncio

from grpclib.client import Channel
from senvend_api_async.api.v1 import VersionRequest
from senvend_api_async.local.v1 import VersionServiceStub


async def main() -> None:
    async with Channel("192.168.0.10", 11111) as channel:
        print(await VersionServiceStub(channel).version(VersionRequest()))


asyncio.run(main())
```

A runnable payment example is included in the
[repository](https://github.com/senvend/api/tree/main/clients/python/packages/example).

## Documentation

- [Protobuf definitions and API documentation](https://github.com/senvend/api)
- [SENVEND schema registry on buf.build](https://buf.build/senvend/api)

If you need help during your integration and want to handle it confidentially,
reach out via [api@senvend.com](mailto:api@senvend.com).

## License

Licensed under either of
[Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) or
[MIT license](https://opensource.org/license/mit) at your option.
