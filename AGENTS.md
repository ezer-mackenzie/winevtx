# Coding agent instructions

- Keep changes focused and compatible with Python 3.9–3.14.
- Native Rust code is in `src/`; the Python package and its stubs are in
  `python/winevtx/`. Keep their public interfaces synchronized.
- Live Windows Event Log queries require Windows. Do not make the offline EVTX
  parser assume that it runs on Windows.
- Do not use real EVTX records or include sensitive data in tests or examples.
- Validate Rust changes with `cargo fmt --check`, Clippy, and `cargo test --locked`.
- Validate Python changes with
  `uv run --locked python -m unittest discover -s tests -v`.
- For documentation changes, run
  `uv run --locked --group docs mkdocs build --strict`.
- For a release, keep `Cargo.toml`, `Cargo.lock`, the changelog, tag, and
  distribution metadata consistent; use the validators in `tools/`.