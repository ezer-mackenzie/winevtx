# API reference

The public API is available from `winevtx`.

## Live queries

- `query_events(channel="System", query="*", limit=None, reverse=True, format="record")`
  returns a list of events from a Windows Event Log channel.
- `iter_events(channel="System", query="*", reverse=True, format="record")`
  returns an event iterator.
- `LiveEventLog(channel="System", query="*", reverse=True)` provides the methods
  `read(limit=None, format="record")` and `iter(format="record")`.

These operations require Windows. `format` accepts `"record"`, `"dict"`, `"xml"`,
and `"json"`.

## Reading EVTX files

- `read_evtx(path, limit=None, format="record")` returns a list of records.
- `iter_evtx(path, format="record")` returns an iterator.
- `EvtxFile(path)` provides `read(limit=None, format="record")` and
  `iter(format="record")`, and can be used as a context manager.

The default format returns `EventRecord` instances.

## `EventRecord`

The object exposes the attributes `xml`, `event_id`, `record_id`, `channel`,
`provider`, `level`, `time_created`, `computer`, `system`, and `event_data`.
It also provides:

- `to_dict()` and `to_json()` to convert the event.
- `get(key, default=None)` and indexing (`event["System"]`) to access its data.

## XML utilities

- `xml_to_dict(xml)` converts event XML into a Python dictionary.
- `xml_to_json(xml)` converts event XML into a JSON string.
