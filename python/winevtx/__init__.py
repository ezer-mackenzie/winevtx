"""High-performance Windows Event Log query and EVTX parser for Python built with Rust and PyO3.

This library provides native Windows Event Log querying (live) and fast offline .evtx file
parsing powered by Rust.
"""

from __future__ import annotations

from typing import Any, Dict, Iterator, List, Optional, Union

# Import low-level compiled Rust extension module
from winevtx._winevtx import (  # type: ignore[import-not-found]
    EventRecord,
    EvtxFile,
    EvtxRecordIterator,
    LiveEventIterator,
    LiveEventLog,
    iter_events as _iter_events,
    iter_evtx as _iter_evtx,
    query_events as _query_events,
    read_evtx as _read_evtx,
    xml_to_dict as _xml_to_dict,
    xml_to_json as _xml_to_json,
)

__version__ = "0.1.0"

__all__ = [
    "EventRecord",
    "EvtxFile",
    "EvtxRecordIterator",
    "LiveEventIterator",
    "LiveEventLog",
    "iter_events",
    "iter_evtx",
    "query_events",
    "read_evtx",
    "xml_to_dict",
    "xml_to_json",
]


def query_events(
    channel: str = "System",
    query: str = "*",
    limit: Optional[int] = None,
    reverse: bool = True,
    format: str = "record",
) -> List[Union[EventRecord, Dict[str, Any], str]]:
    """Query live events from a Windows Event Log channel (e.g. 'System', 'Application', 'Security').

    Args:
        channel: Name of the event log channel (default: 'System').
        query: XPath query filter string (default: '*').
        limit: Maximum number of events to return. If None, reads until exhausted or timeout.
        reverse: If True, returns newest events first (default: True).
        format: Format of returned items: 'record' (default), 'dict', 'xml', or 'json'.

    Returns:
        List containing the matching events in the specified format.
    """
    return _query_events(channel=channel, query=query, limit=limit, reverse=reverse, format=format)


def iter_events(
    channel: str = "System",
    query: str = "*",
    reverse: bool = True,
    format: str = "record",
) -> LiveEventIterator:
    """Create an iterator yielding events from a live Windows Event Log channel.

    Args:
        channel: Name of the event log channel (default: 'System').
        query: XPath query filter string (default: '*').
        reverse: If True, yields newest events first (default: True).
        format: Format of yielded items: 'record' (default), 'dict', 'xml', or 'json'.

    Returns:
        LiveEventIterator yielding matching events.
    """
    return _iter_events(channel=channel, query=query, reverse=reverse, format=format)


def read_evtx(
    path: str,
    limit: Optional[int] = None,
    format: str = "record",
) -> List[Union[EventRecord, Dict[str, Any], str]]:
    """Read parsed events from an offline Windows Event Log (.evtx) file.

    Args:
        path: Path to the .evtx file on disk.
        limit: Maximum number of events to return. If None, reads all available records.
        format: Format of returned items: 'record' (default), 'dict', 'xml', or 'json'.

    Returns:
        List of parsed records in the requested format.
    """
    return _read_evtx(path=path, limit=limit, format=format)


def iter_evtx(
    path: str,
    format: str = "record",
) -> EvtxRecordIterator:
    """Create a high-performance iterator over records in an offline .evtx file.

    Args:
        path: Path to the .evtx file on disk.
        format: Format of yielded items: 'record' (default), 'dict', 'xml', or 'json'.

    Returns:
        EvtxRecordIterator yielding parsed records.
    """
    return _iter_evtx(path=path, format=format)


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
