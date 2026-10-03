# Changelog

All notable changes to this project will be documented in this file.

## [0.1.0] - 2026-10-02

### Added
- Implementation of Windows Event Log live query API using `wevtapi.dll` (`EvtQuery`, `EvtNext`, `EvtRender`).
- Offline `.evtx` file parser using pure Rust `evtx` crate with multi-threaded streaming iterators.
- PyO3 module exposing:
  - `query_events` and `iter_events` for querying live OS event channels.
  - `read_evtx` and `iter_evtx` for reading `.evtx` files.
  - `LiveEventLog` and `EvtxFile` classes with context manager support.
  - `EventRecord` wrapper class supporting direct metadata attributes (`event_id`, `provider`, `channel`, etc.), `to_dict()`, `to_json()`, and indexing.
  - `xml_to_dict` and `xml_to_json` conversion utilities.
- Output formats: `record`, `dict`, `xml`, `json`.
- Python typing stubs (`winevtx.pyi`).
- Comprehensive unit test suite in `tests/test_winevtx.py`.
