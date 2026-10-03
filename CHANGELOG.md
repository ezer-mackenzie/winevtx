# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [0.1.0] - 2026-10-03

### Added
- **Native Windows Event Log Live Reader**:
  - Live OS event querying using `wevtapi.dll` (`EvtQuery`, `EvtNext`, `EvtRender`).
  - Channel subscription and querying (`System`, `Application`, `Security`, etc.).
  - Custom XPath query filtering support (e.g. `*[System[(Level <= 3)]]`).
  - Streaming iterator `LiveEventIterator` with GIL release during I/O.
- **Offline EVTX File Parser**:
  - Multi-threaded `.evtx` file parsing powered by the Rust `evtx` crate and Rayon.
  - Streaming record iterator `EvtxRecordIterator`.
  - Context manager `EvtxFile` for safe file handling.
- **Structured Output Formats**:
  - `record` (default): Fast `EventRecord` object with metadata accessors (`event_id`, `provider`, `channel`, `time_created`, `event_data`, `system`), dictionary conversion (`to_dict()`), JSON export (`to_json()`), and dict-like indexing.
  - `dict`: Direct native Python dictionary conversion.
  - `xml`: Raw Windows Event XML strings.
  - `json`: Formatted JSON strings.
- **Modular Python Architecture**:
  - `winevtx.live`: Query functions and classes for live Windows Event Log channels.
  - `winevtx.offline`: Readers and iterators for offline `.evtx` log files.
  - `winevtx.utils`: XML parsing and transformation utilities (`xml_to_dict`, `xml_to_json`).
  - Clean facade `__init__.py` exposing the public API.
- **Typing & Language Server Integration**:
  - Full PEP 561 compliance with `py.typed` marker.
  - Comprehensive PEP 604 type annotations (`| None`).
  - Explicit stubs in `_winevtx.pyi` for the compiled Rust extension module.
  - Project configuration for Pyright and Pylance (`pyproject.toml`, `pyrightconfig.json`, `.vscode/settings.json`) resolving `_winevtx` resolution in IDEs with Rust/Python hybrid layouts.
- **Testing**:
  - 9 automated unit tests in `tests/test_winevtx.py` covering live queries, XPath filters, formats, and offline parsing.
