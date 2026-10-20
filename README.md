# winevtx

[![PyPI version](https://img.shields.io/pypi/v/winevtx.svg)](https://pypi.org/project/winevtx/)
[![CI](https://github.com/ezer-mackenzie/winevtx/actions/workflows/ci.yml/badge.svg)](https://github.com/ezer-mackenzie/winevtx/actions/workflows/ci.yml)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](LICENSE.md)

Read the full documentation at [winevtx.readthedocs.io](https://winevtx.readthedocs.io/).

`winevtx` is a high-performance Python library built with **Rust**, **PyO3**,
and **Maturin**. It is designed to:

1. Query live Windows Event Log channels through the native Windows API
   (`wevtapi.dll` / `EvtQuery` / `EvtRender`).
2. Parse `.evtx` files offline with multi-threaded processing powered by Rayon.
3. Return structured data as `EventRecord` objects, native Python dictionaries,
   raw XML strings, or JSON.

## Features

- **Native performance**: Rust engine that releases the GIL during intensive
  I/O and decoding.
- **Live Windows events**: Query channels such as `System`, `Security`, and
  `Application`, with custom XPath filters (for example,
  `*[System[(Level <= 3)]]`).
- **Offline `.evtx` files**: Read exported event logs without connecting to
  Windows Event Log.
- **Multiple output formats**:
  - `record` (default): `EventRecord` objects with metadata properties
    (`event_id`, `provider`, `channel`, `time_created`, `event_data`, `system`),
    `.to_dict()`, and dict-style indexing such as `event["System"]`.
  - `dict`: Native Python dictionaries.
  - `xml`: Raw Windows Event XML strings.
  - `json`: JSON strings.
- **Type hints**: PEP 561 support through `py.typed` and stubs for the compiled
  extension.

## Requirements and installation

- Python 3.9 through 3.14.
- Windows is required for live Event Log queries. Offline `.evtx` parsing does
  not require a connection to the Windows Event Log service.
- Prebuilt wheels are provided for supported little-endian targets. The current
  upstream EVTX parser dependencies do not support `s390x` or 32-bit x86.

Install the published package from PyPI:

```bash
python -m pip install winevtx
```

## Build from source

To build and install the package locally with Maturin:

```powershell
uv sync --locked
uv run maturin develop
```

To build a wheel:

```powershell
uv run maturin build --release
```

## Usage

### Query live Windows events

```python
import winevtx

events = winevtx.query_events(channel="System", limit=10)
for event in events:
    print(f"[{event.time_created}] EventID: {event.event_id} | Provider: {event.provider}")
    print(event.to_dict())
```

Filter the results with an XPath query:

```python
events = winevtx.query_events(
    channel="System",
    query="*[System[(Level <= 3)]]",
    limit=5,
)
```

Use `iter_events` to process results incrementally:

```python
for event in winevtx.iter_events(channel="Application", reverse=True):
    if event.event_id == 1000:
        print("Application error:", event.event_data)
        break
```

### Read `.evtx` files offline

```python
import winevtx

events = winevtx.read_evtx(r"C:\Logs\Security.evtx", limit=50)

with winevtx.EvtxFile(r"C:\Logs\System.evtx") as log:
    for event in log.iter():
        print(event.event_id, event.computer, event.time_created)
```

### Choose an output format

The `format` argument accepts `"record"` (the default), `"dict"`, `"xml"`, or
`"json"`:

```python
event = winevtx.query_events("System", limit=1, format="record")[0]
print(event.event_id)
print(event.xml)
print(event.to_dict())
print(event.to_json())

data = winevtx.query_events("System", limit=1, format="dict")[0]
print(data["Event"]["System"]["Channel"])
```

### XML utilities

```python
import winevtx

xml_string = "<Event>...</Event>"
event_data = winevtx.xml_to_dict(xml_string)
json_string = winevtx.xml_to_json(xml_string)
```

## API overview

- `query_events(channel="System", query="*", limit=None, reverse=True, format="record")`
  queries a live Windows Event Log channel.
- `iter_events(channel="System", query="*", reverse=True, format="record")`
  iterates over live events.
- `read_evtx(path, limit=None, format="record")` reads records from an `.evtx`
  file.
- `iter_evtx(path, format="record")` iterates over records in an `.evtx` file.
- `xml_to_dict(xml)` parses Windows Event XML into a Python dictionary.
- `xml_to_json(xml)` converts Windows Event XML to JSON.

The package also exposes `EventRecord`, `LiveEventLog`, and `EvtxFile`. See the
[API reference](docs/api.md) for details.

## Project links

- [Documentation](https://winevtx.readthedocs.io/)
- [Changelog](CHANGELOG.md)
- [Contributing](CONTRIBUTING.md)
- [Code of conduct](CODE_OF_CONDUCT.md)
- [Security policy](SECURITY.md)
- [License](LICENSE.md)
