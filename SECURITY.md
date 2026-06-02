# Security Policy

## Supported Versions

Until `1.0.0` is published, security fixes are provided for the `main` branch.
After the first `1.0.x` release is published, security fixes are provided for
the latest published `1.0.x` release line and the `main` branch.

## Reporting a Vulnerability

Report vulnerabilities privately through GitHub Security Advisories for `tommimarkus/elkrs`.

Do not file public issues for exploitable vulnerabilities until a fix is available.

Expected response:

- Initial acknowledgement within 7 calendar days.
- Status update or remediation plan within 30 calendar days.
- Critical reachable supply-chain or parser vulnerabilities prioritized ahead of feature work.

## Security Targets

The `1.0.x` release target is:

- SCVS Level 1 for dependency, provenance, and release-evidence practices that apply to a Rust library crate.
- SLSA Build Level 1 evidence for GitHub-built release artifacts.

These targets do not claim external plugin behavior, runtime deployment controls, or production service monitoring.
