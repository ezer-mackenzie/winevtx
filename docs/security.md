# Security

`winevtx` processes Windows Event Log data and EVTX files. Treat records and
source files as potentially sensitive: they may contain user names, file paths,
network addresses, and other identifying information. Do not attach real event
records to public issues.

## Report a vulnerability

Do not disclose vulnerability details in a public issue or pull request. Use
[GitHub's private vulnerability reporting](https://github.com/ezer-mackenzie/winevtx/security/advisories/new)
if it is enabled for the repository. If you cannot access that form, contact
the maintainer privately through GitHub before disclosing details.

Include the affected version, operating system, Python version, observed impact,
and minimal reproduction steps. Remove personal data and secrets. No response
time is guaranteed; the report will be acknowledged when possible.

## Supported versions

Fixes are prioritized for the latest stable release. When reporting an issue,
state whether it also affects an earlier `0.1.x` release.

## Safe use

- Restrict access to EVTX files according to the sensitivity of their contents.
- Verify file provenance before parsing and keep the package updated to a
  release containing relevant fixes.
- Live queries use the permissions of the current process and requested
  channel; run with elevated privileges only when necessary.
