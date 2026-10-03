# winevtx

[![PyPI version](https://img.shields.io/pypi/v/winevtx.svg)](https://pypi.org/project/winevtx/)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](LICENSE.md)

High-performance Python library built with **Rust**, **PyO3**, and **Maturin**,
designed to:

1. **Query live operating system events** through the native Windows API
   (`wevtapi.dll` / `EvtQuery` / `EvtRender`).
2. **Parse and analyze `.evtx` files offline** with multi-threaded processing
   powered by Rayon.
3. Return structured data as `EventRecord` objects, native Python dictionaries
   (`dict`), raw XML strings, or JSON.

---

## Features

- **Native performance**: Rust engine with minimal overhead that releases the
  GIL during intensive I/O and decoding.
- **Live OS events**: Access channels such as `System`, `Security`,
  `Application`, or any registered channel, with custom XPath filters (for
  example, `*[System[(Level <= 3)]]`).
- **Offline `.evtx` files**: Forensic reading of exported Windows records
  without relying on external utilities.
- **Multiple formats**:
  - `record` (default): `EventRecord` object with direct properties
    (`event_id`, `provider`, `channel`, `time_created`, `event_data`, `system`),
    a `.to_dict()` method, and indexing support such as `event["System"]`.
  - `dict`: Native Python dictionary deserialized directly in CPython.
  - `xml`: Raw Windows XML string.
  - `json`: Formatted JSON string.
- **Type hints**: PEP 561 static typing support through `py.typed` and type
  stubs for editor completion in VS Code, PyCharm, and other Python IDEs.


---

## Local installation and build

To build and test locally with Maturin:

```powershell
# Create a virtual environment and install Maturin
python -m venv .venv
.\.venv\Scripts\pip install maturin

# Build and install in editable mode
.\.venv\Scripts\maturin develop
```

To build a wheel (`.whl`):

```powershell
maturin build --release
```

---

## Usage

### 1. Read live OS events

#### Quick query for recent events

```python
import winevtx

# Get the 10 most recent events from the System channel
events = winevtx.query_events(channel="System", limit=10)

for event in events:
    print(f"[{event.time_created}] EventID: {event.event_id} | Provider: {event.provider}")
    print(event.to_dict())
```

#### Filter with XPath queries

```python
import winevtx

# Filter error and warning events (Level <= 3)
query = "*[System[(Level <= 3)]]"
events = winevtx.query_events(channel="System", query=query, limit=5)

for event in events:
    print(f"Alert level {event.level}: {event.provider} (ID: {event.event_id})")
```

#### Streaming iterator

```python
import winevtx

# Stream events without loading them all into memory
for event in winevtx.iter_events(channel="Application", reverse=True):
    if event.event_id == 1000:
        print("Application error detected:", event.event_data)
        break
```

---

### 2. Read `.evtx` files offline

```python
import winevtx

# Read an .evtx file directly
events = winevtx.read_evtx(r"C:\Logs\Security.evtx", limit=50)

# Or use a context manager
with winevtx.EvtxFile(r"C:\Logs\System.evtx") as log:
    for event in log.iter(format="record"):
        print(event.event_id, event.computer, event.time_created)
```

---

### 3. Output formats (`record`, `dict`, `xml`, `json`)

```python
import winevtx

# 1. EventRecord object (default)
event = winevtx.query_events("System", limit=1, format="record")[0]
print(event.event_id)
print(event.xml)           # Raw XML
print(event.to_dict())     # Python dictionary
print(event.to_json())     # JSON string

# 2. Plain Python dictionary
data = winevtx.query_events("System", limit=1, format="dict")[0]
print(data["Event"]["System"]["Channel"])

# 3. Raw native XML
raw_xml = winevtx.query_events("System", limit=1, format="xml")[0]
print(raw_xml)

# 4. JSON
json_string = winevtx.query_events("System", limit=1, format="json")[0]
print(json_string)
```

---

### 4. Helper utilities

```python
import winevtx

xml_string = "<Event>...</Event>"

# Convert Windows event XML to a dictionary
event_data = winevtx.xml_to_dict(xml_string)

# Convert XML to JSON
json_string = winevtx.xml_to_json(xml_string)
```

---

## API reference

### Main functions

- `query_events(channel="System", query="*", limit=None, reverse=True, format="record") -> List`:
  Query events from a live Windows channel.
- `iter_events(channel="System", query="*", reverse=True, format="record") -> LiveEventIterator`:
  Iterate over live events.
- `read_evtx(path, limit=None, format="record") -> List`:
  Read events from an `.evtx` file on disk.
- `iter_evtx(path, format="record") -> EvtxRecordIterator`:
  Iterate over an `.evtx` file.
- `xml_to_dict(xml: str) -> dict`:
  Parse event XML into a Python dictionary.
- `xml_to_json(xml: str) -> str`:
  Parse event XML into a JSON string.

### Classes

- `winevtx.EventRecord`: Event container.
  - Attributes: `xml`, `event_id`, `record_id`, `channel`, `provider`, `level`,
    `time_created`, `computer`, `system`, `event_data`.
  - Methods: `to_dict()`, `to_json()`, `get(key, default)`.
- `winevtx.LiveEventLog(channel, query, reverse)`: Live channel handler.
- `winevtx.EvtxFile(path)`: Offline `.evtx` file handler (supports `with`).

---

## License

This project is licensed under the MIT License.
