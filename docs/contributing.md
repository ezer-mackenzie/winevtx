# Contributing

Contributions, bug reports, and documentation improvements are welcome. For
significant changes, open a discussion or issue first to agree on the scope.

## Set up the development environment

Python 3.13 and Rust 1.88 or later are required. From the repository root:

```powershell
uv sync --locked
```

Build and install the extension locally:

```powershell
uv run maturin develop
```

## Validate changes

Run the relevant checks before opening a pull request:

```powershell
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --locked
uv run --locked python -m unittest discover -s tests -v
uv run --locked --group docs mkdocs build --strict
```

Live Event Log queries can only be tested on Windows. Tests that require
special permissions or local `.evtx` files must document that requirement and
must not depend on personal data or secrets.

## Pull requests

- Keep each change focused and explain the problem it solves.
- Add or update tests for behavior changes.
- Update `CHANGELOG.md` when a change affects users.
- Update the documentation and Python stubs when the public API changes.
- Do not include wheels, compiled binaries, event records, or sensitive data.

## Prepare a release

1. Update the package version in `Cargo.toml` and `Cargo.lock`.
2. Add a dated entry for that version to `CHANGELOG.md`.
3. Run `python tools/validate_release.py` with `RELEASE_TAG` set to `vX.Y.Z`,
   then run the tests, strict documentation build, and distribution validators.
4. Create and push the `vX.Y.Z` tag after merging changes into `main`.
5. Publish a GitHub Release for the tag. The release workflow validates and
   builds the distributions before publishing them to PyPI through trusted
   publishing.

Do not upload compiled artifacts manually or publish from an unvalidated
branch.
