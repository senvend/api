# SENVEND Public API

This repository contains public APIs for the SENVEND ecosystem.  
We rely on Protocol Buffers (protobuf) and gRPC to define and implement our APIs.  
To make handling protobuf definitions easier, we use [buf.build](https://buf.build/).  
All protobuf definitions are stored in the [`proto/`](./proto/) directory.

## Help us improve

Feel free to open issues if something is missing, unclear or not working as expected.  
You are also welcome to contribute directly by opening Pull Requests.  
If you need any help during your integration and you want to handle it confidentially, please reach out to us via [api@senvend.com](mailto:api@senvend.com).

## Local vs Cloud APIs

At the moment, you can choose between two main APIs: cloud and local.  
The payloads are almost identical, cloud and local APIs only provide small wrappers around the main datastructures.

### Local API

The local API runs on SENVEND Payment Terminals and can be accessed via your local network.  
You can find the protobuf definitions for the local API in the [`proto/local/`](./proto/local/) directory.  
The terminal needs to be connected to the same network as your client application. You can connect it via Ethernet or Wi-Fi.  
Before using the local API with your terminal, you need to enable and configure it in the terminal settings on [my.senvend.com](https://my.senvend.com).  
By default the API listens on port `11111` without authentication and encryption.

### Cloud API (not released yet)

The cloud API can be accessed at `api.senvend.com:443`.  
You can find the protobuf definitions for the cloud API in the [`proto/cloud/`](./proto/cloud/) directory.

## Generated clients

We provide example clients for various programming languages to help developers get started quickly.  
Take a look at the [`clients/`](./clients/) directory for more information.  
We currently provide clients for the following programming languages:  

- Rust ([clients/rust/](./clients/rust/))
- Python ([clients/python/](./clients/python/))
  You can request clients for other programming languages by opening an issue.

### Example Code

Every client should contain at least one example code snippet demonstrating how to use the generated code.  
Those example snippets should use `TERMINAL_IP` and `TERMINAL_PORT` environment variables to configure the connection to the terminal.

### Regenerate clients

You can use [./gen.sh](./gen.sh) to regenerate the example clients after making changes to the protobuf definitions.

## Buf schema registry

All definitions in this repository are published to the [SENVEND schema registry on buf.build](https://buf.build/senvend/api).  
This means you can use them as dependencies in your [buf.build](https://buf.build/) projects for advanced use cases.
