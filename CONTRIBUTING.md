# Contributing to winevtx

Thank you for considering a contribution. For larger changes, open an issue
first to agree on scope and avoid duplicated work.

## Requirements

- Python 3.9–3.14.
- Stable Rust and Maturin to build the native extension.
- Windows to develop or test live Event Log queries.

Install the locked dependencies from the repository root:

```powershell
uv sync --locked
uv run maturin develop
```

## Tests and quality checks

Run the following checks before opening a pull request:

```powershell
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --locked
uv run --locked python -m unittest discover -s tests -v
uv run --locked --group docs mkdocs build --strict
```

Live Event Log query tests require Windows and access to the requested channel.
Do not add real EVTX files, personal information, or event records to tests or
the repository.

## Pull requests

- Explain the motivation and the behavior before and after the change.
- Add tests for bug fixes and behavior changes.
- Update `CHANGELOG.md` for user-visible changes.
- Keep the public API, type stubs, and documentation in sync.
- Do not include binaries, compiled wheels, credentials, or sensitive data.

## Releases and publishing

Versions follow SemVer and are recorded in `CHANGELOG.md`. Before publishing,
validate the version and tag with `tools/validate_release.py`, run the tests,
build the strict documentation, and validate the wheel and sdist. After merging
changes into `main`, the `vX.Y.Z` tag starts release validation. Publishing the
GitHub Release allows the workflow to build distributions and publish them to
PyPI using trusted publishing.

To report a vulnerability, follow the [security policy](SECURITY.md). Do not
publish exploitable details in a public issue.