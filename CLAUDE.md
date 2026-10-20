# Claude Code instructions

Follow the shared guidance in [AGENTS.md](AGENTS.md). In particular:

- Preserve Python 3.9–3.14 compatibility and PEP 561 typing support.
- Keep the Rust bindings, Python API, and `_winevtx.pyi` synchronized.
- Do not include real Windows Event Log data or sensitive EVTX files.
- Run the relevant tests and validation checks before finishing.
- Update `CHANGELOG.md` and the documentation when user-visible behavior
  changes.