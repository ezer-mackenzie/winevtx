from typing import Any, Dict, Iterator, List, Optional, Union

class EventRecord:
    """Represents a Windows Event record with parsed metadata and lazy dictionary conversion."""

    xml: str
    event_id: Optional[int]
    record_id: Optional[int]
    channel: Optional[str]
    provider: Optional[str]
    level: Optional[int]
    time_created: Optional[str]
    computer: Optional[str]
    system: Optional[Dict[str, Any]]
    event_data: Optional[Union[Dict[str, Any], List[Any]]]

    def to_dict(self) -> Dict[str, Any]:
        """Convert the entire event into a Python dictionary."""
        ...

    def to_json(self) -> str:
        """Convert the entire event into a JSON string."""
        ...

    def get(self, key: str, default: Any = None) -> Any:
        """Get a value by key from the parsed event."""
        ...

    def __getitem__(self, key: str) -> Any:
        ...

    def __contains__(self, key: str) -> bool:
        ...

    def __str__(self) -> str:
        ...

    def __repr__(self) -> str:
        ...

class LiveEventLog:
    """Reader for live Windows Event Log channels."""

    def __init__(self, channel: str = "System", query: str = "*", reverse: bool = True) -> None:
        ...

    def read(
        self, limit: Optional[int] = None, format: str = "record"
    ) -> List[Union[EventRecord, Dict[str, Any], str]]:
        """Read a list of events from the live channel."""
        ...

    def iter(
        self, format: str = "record"
    ) -> "LiveEventIterator":
        """Get an iterator over the events."""
        ...

    def __iter__(self) -> "LiveEventIterator":
        ...

class LiveEventIterator:
    """Iterator yielding events from a live Windows Event query."""

    def __iter__(self) -> "LiveEventIterator":
        ...

    def __next__(self) -> Union[EventRecord, Dict[str, Any], str]:
        ...

class EvtxFile:
    """Parser for offline Windows Event Log (.evtx) files."""

    def __init__(self, path: str) -> None:
        ...

    def read(
        self, limit: Optional[int] = None, format: str = "record"
    ) -> List[Union[EventRecord, Dict[str, Any], str]]:
        """Read a list of records from the .evtx file."""
        ...

    def iter(
        self, format: str = "record"
    ) -> "EvtxRecordIterator":
        """Get an iterator over the records in the .evtx file."""
        ...

    def __iter__(self) -> "EvtxRecordIterator":
        ...

    def __enter__(self) -> "EvtxFile":
        ...

    def __exit__(self, exc_type: Any, exc_val: Any, exc_tb: Any) -> bool:
        ...

class EvtxRecordIterator:
    """Iterator yielding records parsed from an offline .evtx file."""

    def __iter__(self) -> "EvtxRecordIterator":
        ...

    def __next__(self) -> Union[EventRecord, Dict[str, Any], str]:
        ...

def query_events(
    channel: str = "System",
    query: str = "*",
    limit: Optional[int] = None,
    reverse: bool = True,
    format: str = "record",
) -> List[Union[EventRecord, Dict[str, Any], str]]:
    """Query live events from a Windows Event Log channel (e.g. 'System', 'Application', 'Security').

    Args:
        channel: Event channel name. Default is 'System'.
        query: XPath query string filter. Default is '*'.
        limit: Maximum number of events to fetch. Default is None (all).
        reverse: If True, fetches newest events first. Default is True.
        format: Format of returned items: 'record' (default), 'dict', 'xml', or 'json'.
    """
    ...

def iter_events(
    channel: str = "System",
    query: str = "*",
    reverse: bool = True,
    format: str = "record",
) -> LiveEventIterator:
    """Iterate live events from a Windows Event Log channel.

    Args:
        channel: Event channel name. Default is 'System'.
        query: XPath query filter. Default is '*'.
        reverse: If True, yields newest events first. Default is True.
        format: Format of yielded items: 'record' (default), 'dict', 'xml', or 'json'.
    """
    ...

def read_evtx(
    path: str,
    limit: Optional[int] = None,
    format: str = "record",
) -> List[Union[EventRecord, Dict[str, Any], str]]:
    """Read events from an offline .evtx file on disk.

    Args:
        path: Path to the .evtx file.
        limit: Maximum number of events to read. Default is None.
        format: Format of returned items: 'record' (default), 'dict', 'xml', or 'json'.
    """
    ...

def iter_evtx(
    path: str,
    format: str = "record",
) -> EvtxRecordIterator:
    """Iterate events from an offline .evtx file on disk.

    Args:
        path: Path to the .evtx file.
        format: Format of yielded items: 'record' (default), 'dict', 'xml', or 'json'.
    """
    ...

def xml_to_dict(xml: str) -> Dict[str, Any]:
    """Parse a Windows Event XML string into a native Python dictionary."""
    ...

def xml_to_json(xml: str) -> str:
    """Convert a Windows Event XML string into a JSON string."""
    ...
