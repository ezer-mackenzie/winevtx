"""Offline Windows Event Log (.evtx) file parsing."""

from __future__ import annotations

from typing import Any, Dict, List, Union

from ._winevtx import (  # type: ignore[import-not-found]
    EventRecord,
    EvtxFile,
    EvtxRecordIterator,
    iter_evtx as _iter_evtx,
    read_evtx as _read_evtx,
)


def read_evtx(
    path: str,
    limit: int | None = None,
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
