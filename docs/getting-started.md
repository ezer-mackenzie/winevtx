# Getting started

## Installation

Install the published package from PyPI:

```bash
python -m pip install winevtx
```

Prebuilt wheels are provided for supported little-endian targets. The current
upstream EVTX parser dependencies do not support `s390x` or 32-bit x86.

## Query live events

Live Windows Event Log queries require Windows. The current user must have
permission to read the requested channel; some channels, such as `Security`,
may require elevated permissions.

```python
import winevtx

events = winevtx.query_events(channel="System", limit=10)
for event in events:
    print(event.time_created, event.event_id, event.provider)
```

Filter events with XPath:

```python
events = winevtx.query_events(
    channel="System",
    query="*[System[(Level <= 3)]]",
    limit=10,
)
```

Use `winevtx.iter_events(...)` to process results incrementally.

## Parse an EVTX file

Offline parsing does not need a connection to the Windows Event Log service:

```python
import winevtx

events = winevtx.read_evtx(r"C:\Logs\System.evtx", limit=50)

with winevtx.EvtxFile(r"C:\Logs\System.evtx") as log:
    for event in log.iter():
        print(event.event_id, event.computer)
```

You can also use `winevtx.iter_evtx(path)` to iterate without loading all
records into a list.

## Choose an output format

Read functions accept `format="record"` (the default), `"dict"`, `"xml"`, or
`"json"`:

```python
event = winevtx.read_evtx(r"C:\Logs\System.evtx", limit=1)[0]
print(event.to_dict())
print(event.to_json())
```
