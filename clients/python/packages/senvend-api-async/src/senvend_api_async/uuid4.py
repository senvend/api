# Do not edit copies in the packages: gen.sh copies this file from clients/python/shared/.
"""Conversion helpers between the Uuid4 proto message and Python's uuid.UUID."""

import uuid as _uuid
from uuid import UUID

from .api.v1 import Uuid4

_LSB_MASK = (1 << 64) - 1


def from_uuid(uuid: UUID) -> Uuid4:
    """Create a Uuid4 message from a Python UUID (must be version 4)."""
    if uuid.version != 4:
        raise ValueError(f"Expected UUID version 4, got version {uuid.version}")
    uuid_int = int(uuid)
    return Uuid4(msb=uuid_int >> 64, lsb=uuid_int & _LSB_MASK)


def to_uuid(msg: Uuid4) -> UUID:
    """Convert a Uuid4 message to a Python UUID (must decode to version 4)."""
    decoded = UUID(int=(msg.msb << 64) | msg.lsb)
    if decoded.version != 4:
        raise ValueError(f"Expected UUID version 4, got version {decoded.version}")
    return decoded


def random_uuid4() -> Uuid4:
    """Create a Uuid4 message from a newly generated random UUID."""
    return from_uuid(_uuid.uuid4())
