# winevtx

`winevtx` is a Python library for querying live Windows events and parsing
`.evtx` files offline. It combines Rust, PyO3, and a typed Python API.

## Features

- Query live Windows Event Log channels with XPath filters.
- Read and iterate over `.evtx` files.
- Return records as `EventRecord` objects, dictionaries, XML, or JSON.
- Convert event XML to dictionaries and JSON.

Live queries require Windows. Offline `.evtx` parsing does not require a
connection to Windows Event Log.

## Get started

- [Installation and examples](getting-started.md)
- [API reference](api.md)
- [Contributing](contributing.md)
- [Reporting a security issue](security.md)

The complete guide is also available in the repository
[README](https://github.com/ezer-mackenzie/winevtx#readme).
