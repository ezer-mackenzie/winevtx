"""Live Windows Event Log querying and streaming."""

from __future__ import annotations

from typing import Any, Dict, List, Union

from ._winevtx import (  # type: ignore[import-not-found]
    EventRecord,
    LiveEventIterator,
    LiveEventLog,
    iter_events as _iter_events,
    query_events as _query_events,
)


def query_events(
    channel: str = "System",
    query: str = "*",
    limit: int | None = None,
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
