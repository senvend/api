# https://test-betterproto.readthedocs.io/en/docs/quick-start.html#async-grpc-support
# https://betterproto.github.io/python-betterproto2/tutorial/clients/

import os
import queue
from collections.abc import Generator

import grpc
from senvend_api.api.v1 import (
    PayApproved,
    PayGoodsIssued,
    PayRequest,
    PayResponse,
    PayStart,
    PaySuccess,
)
from senvend_api.local.v1 import PayServiceStub

TERMINAL_IP = "TERMINAL_IP"
TERMINAL_PORT = "TERMINAL_PORT"
DEFAULT_PORT = 11111


class Close:
    pass


Q = queue.Queue[PayRequest | Close]


def make_request_stream(q: Q) -> Generator[PayRequest]:
    """Synchronous generator that yields PayRequest from a Queue."""
    while True:
        msg = q.get()
        if isinstance(msg, Close):
            # End of client stream
            return
        yield msg


def main():
    print("Hello from SENVEND python gRPC example client!")

    host = os.getenv(TERMINAL_IP)
    if not host:
        raise ValueError(f"{TERMINAL_IP} environment variable is not set")
    try:
        port = int(os.getenv(TERMINAL_PORT, DEFAULT_PORT))
    except ValueError:
        raise ValueError(f"{TERMINAL_PORT} environment variable is not a valid integer")

    connection = f"{host}:{port}"
    print(f"Connecting to terminal at {connection}")

    q: Q = queue.Queue()

    # TODO: port uuid helpers
    # pay_uuid =
    pay_request = PayRequest(
        # id=pay_uuid,
        start=PayStart(amount=100),
    )
    q.put(pay_request)

    with grpc.insecure_channel(connection) as channel:
        stub = PayServiceStub(channel)

        closed = False

        def close():
            nonlocal closed
            closed = True
            q.put(Close())
            print("Closing client stream")

        for response in stub.pay(make_request_stream(q)):
            print("Received response:", response)
            match response:
                case PayResponse(approved=PayApproved()):
                    print("Payment approved!")
                    q.put(PayRequest(goods_issued=PayGoodsIssued()))
                case PayResponse(success=PaySuccess()):
                    print("Payment successful!")
                    close()
                case _:
                    print("Unhandled response type")

        if not closed:
            print("Stream closed by server, this should not happen!")

    print("Client finished, bye!")


if __name__ == "__main__":
    main()
