# Security policy

## Supported versions

Security fixes are prioritized for the latest stable release. Include the exact
`winevtx` version affected and whether the issue also affects another `0.1.x`
release.

## Reporting a vulnerability

Do not publish vulnerabilities, proof-of-concept exploits, or sensitive event
data in a public issue. Use
[GitHub's private vulnerability reporting](https://github.com/ezer-mackenzie/winevtx/security/advisories/new)
if it is enabled for the repository. If you cannot access that form, contact
the maintainer privately through GitHub before disclosing details.

Include the affected version, operating system, Python version, observed
impact, and minimal reproduction steps. Do not attach EVTX files containing
personal or confidential data. No response time is guaranteed; the maintainer
will acknowledge the report when possible.

## Event data

Windows event records may contain identifying information, file paths, user
names, and infrastructure details. Treat EVTX files and `winevtx` output with
the same care as the original records. Share only synthetic or redacted
samples.
