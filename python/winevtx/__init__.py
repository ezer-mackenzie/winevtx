"""High-performance Windows Event Log query and EVTX parser for Python built with Rust and PyO3."""

from __future__ import annotations

from winevtx.live import LiveEventIterator, LiveEventLog, iter_events, query_events
from winevtx.offline import EvtxFile, EvtxRecordIterator, iter_evtx, read_evtx
from winevtx.record import EventRecord
from winevtx.utils import xml_to_dict, xml_to_json

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
