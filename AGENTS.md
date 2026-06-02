# Repository Guidelines

## Project Structure & Module Organization

`elkrs` is a Rust workspace targeting clean-room layered graph-layout behavior. Workspace crates live under `crates/`:

- `crates/elkrs-core`: graph, geometry, options, diagnostics, and layout types.
- `crates/elkrs-json`: narrow layout JSON import/export support.
- `crates/elkrs-layered`: layered layout implementation and layout-quality tests.

Integration tests live in each crate's `tests/` directory. Shared layered helpers are in `crates/elkrs-layered/tests/support/`. Parity tracking is in `docs/parity/elk-layered-v0.11.0.md`. The optional Java oracle runner lives in `tools/java-elk-json-runner/`.

Before starting parity implementation, read `docs/parity/delivery-workqueue.md`.
It is the durable execution-order source and is mirrored in GitHub issue #30.

## Initialized Policies

This repository initializes `souroldgeezer-policy` for agent workflow:

- `git-workflow-policy`: feature branches or git worktrees by default, clean worktree, no direct `main`.
- `release-policy`: SemVer workspace versions, annotated `v<version>` git tags, GitHub Releases, pinned Git dependencies, no crates.io publication yet.

Local-only direct `main` work is allowed only when explicitly authorized for the current task. It does not authorize pushing, tagging, force-pushing, branch deletion, GitHub Releases, or publication. Check `git status --short --branch` before edits, staging, commits, tags, or provider operations; preserve unrelated changes and stage explicit paths only.

## Build, Test, and Development Commands

Tool caches should stay under workspace `.cache/` whenever the tool supports a
project config or cache path flag. `.cargo/config.toml` keeps Cargo build
artifacts under `.cache/cargo-target`, and `.cargo/audit.toml` keeps the
RustSec advisory DB under `.cache/cargo-audit`; install Cargo tools with
`--root .cache/cargo-install` and add `.cache/cargo-install/bin` to `PATH`.
Run the default verification gate before sending changes:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo doc --workspace --locked --no-deps
```

Use `cargo audit` when changing dependencies or release files. Run Java parity only when SDKMAN is available:

```bash
export PATH="$PWD/.cache/cargo-install/bin:$PATH"
cargo audit
tools/java-elk-json-runner/bin/build
ELKRS_JAVA_ELK_COMMAND="$PWD/tools/java-elk-json-runner/bin/java-elk-json" cargo test -p elkrs-layered --test java_parity --locked -- --ignored
```

## Coding Style & Naming Conventions

Use Rust 2021 and standard `rustfmt` formatting. Keep modules focused and named by layout phase or domain concept, for example `routing.rs`, `placement.rs`, and `options.rs`. Prefer typed options and diagnostics over stringly typed behavior. Keep public APIs library-first.

## Testing Guidelines

Add tests near the behavior being changed. Use integration tests for public crate behavior and unit tests for local invariants. Name test functions by observable behavior, such as `layered_layout_respects_up_direction`. Keep the Java parity test ignored by default; it must remain gated by `ELKRS_JAVA_ELK_COMMAND`.

## Versioning & Git Tags

Use SemVer across the workspace and keep `elkrs-core`, `elkrs-json`, and `elkrs-layered` on the same version unless there is a documented reason not to. During `0.x`, public API breakage bumps the minor version, for example `0.1.0` to `0.2.0`; compatible fixes bump patch. The `1.0.0` target requires all in-scope ELK Layered v0.11.0 parity rows to be `java-parity` or covered by documented compatibility exclusions.

The workspace version source is the three crate `Cargo.toml` files. Release-prep surfaces include `Cargo.lock`, `README.md`, `RELEASE.md`, `CHANGELOG.md`, and `docs/releases/<version>.md`. For routine SemVer calculation, prefer the bundled `release-policy` `version-bump` helper in dry-run mode before applying manifest edits.

Release tags must match crate versions exactly and must be annotated: `git tag -a v0.1.0 -m "elkrs v0.1.0"` then `git push origin v0.1.0`. Version tags run the Release workflow. Do not create or mutate tags, GitHub Releases, or publication state without explicit release authority and the full release gate.

## Commit & Pull Request Guidelines

Use short imperative commit subjects. Existing history uses both plain subjects and scoped prefixes, for example `ci: add github release distribution`, `test: tighten parent child endpoint assertion`, and `Add ELK layered parity oracle runner`.

PRs should include a concise summary, verification commands run, and any skipped optional checks with reasons. Link issues when applicable.

## Clean-Room & Security Notes

Follow `CLEANROOM.md`: do not copy upstream implementation source, comments, or line-by-line Java processor structure. Use public docs, option metadata, hand-authored fixtures, and black-box Java output only. Report vulnerabilities through GitHub Security Advisories.
