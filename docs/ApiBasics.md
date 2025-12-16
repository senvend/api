# Basic API description

## Protocol buffers

The messages transferred via the API are based on protocol buffers.
The schema describing the structure and contents of all possible messages is hosted on [buf.build/senvend/api](https://buf.build/senvend/api).

## Local (network) API: gRPC

The API itself is implemented using the google RPC framework: [gRPC](https://grpc.io)

It is divided into individual services, each providing a specific workflow for a possible SENVEND Terminal interaction.
Each service can define multiple endpoints, although most are implemented using only one specific RPC endpoint.

Definitions of these services and their endpoints can be found within the protobuf schema hosted on [buf.build](https://buf.build.senvend/api)

Each service comes with its own documentation, specifying the usage and restrictions of individual services.
These documents can be found in the same folder as this one.

### Local gRPC connection

By default the API listens on port `11111` without authentication and encryption.

Each endpoint can accommodate multiple concurrent connections at the same time.
ApiSuccess and ApiFailure messages are sent to the connection on which the command was issued,
all other messages are sent to all connected clients.

It is not strictly necessary to close a connection after a process, but strongly recommended.
The server side will never close a connection on its own:
The client either provides a deadline (a.k.a timeout) which the server enforces,
or can close the connection at his own leisure.

## Cloud API: TODO

TODO

## Examples

Individual service documentations contain examples demonstrating the message flow for that particular service.
These are all based on the following tools and principles.

### buf.build test tools

[buf.build](https://buf.build/) offers a commandline program for easy testing of service endpoints:  
[buf curl](https://buf.build/docs/reference/cli/buf/curl/)

It allows sending protobuf messages from a commandline via JSON.
All example workflows in the service documentations are given as "buf curl" JSON strings.

The following `buf curl` example command uses `--data @-` as a parameter,
which will keep the connection open and accepts JSON input via `stdin`.

Invocation example (connecting to the terminal at 192.168.0.1 using the PayService/Pay function):

```
buf curl --schema buf.build/senvend/api --protocol grpc --http2-prior-knowledge http://192.168.0.1:11111/local.v1.PayService/Pay --data @-
```

### Example format

Each service documentation contains examples of typical messages exchanged between client and server during a call.

The following format is used to distinguish between messages from client or server, as well as interactions done on the terminal itself.

> **->** Sent via `stdin` to the running buf curl command

> **\<-** Received from the server and printed to `stdout`

- *Interaction done on actual device*
