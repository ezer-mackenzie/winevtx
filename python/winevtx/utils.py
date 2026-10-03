"""Utility functions for Windows Event XML parsing and conversion."""

from __future__ import annotations

from typing import Any, Dict

from winevtx._winevtx import (  # type: ignore[import-not-found]
    xml_to_dict as _xml_to_dict,
    xml_to_json as _xml_to_json,
)


def xml_to_dict(xml: str) -> Dict[str, Any]:
    """Parse a raw Windows Event XML string into a native Python dictionary.

    Args:
        xml: Raw XML string of a Windows Event.

    Returns:
        Parsed dictionary structure.
    """
    return _xml_to_dict(xml)


def xml_to_json(xml: str) -> str:
    """Convert a raw Windows Event XML string into a JSON string.

    Args:
        xml: Raw XML string of a Windows Event.

    Returns:
        Serialized JSON string.
    """
    return _xml_to_json(xml)
