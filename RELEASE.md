# Release Checklist

This repository initializes `release-policy`: SemVer workspace versions,
annotated `v<version>` git tags, GitHub Releases, pinned Git dependencies, and
no crates.io publication yet. Do not publish, tag, mutate release tags, create
GitHub Releases, or push release state without explicit release authority.

Before publishing a release:

1. Confirm `README.md`, `CLEANROOM.md`, `SECURITY.md`, `CHANGELOG.md`, and `docs/releases/<version>.md` match the actual supported behavior.
2. Review `docs/parity/elk-layered-v0.11.0.md`. Release notes may claim only
   the matrix rows that are currently `semantic` or `java-parity`. Do not claim
   full ELK Layered parity until every included row is `java-parity`, or the row
   is excluded by a documented compatibility decision.
3. Run the local gate:

   ```bash
   export PATH="$PWD/.cache/cargo-install/bin:$PATH"
   cargo install cargo-audit --locked --root .cache/cargo-install
   cargo audit
   cargo fmt --all --check
   cargo clippy --workspace --all-targets --locked -- -D warnings
   cargo test --workspace --locked
   cargo doc --workspace --locked --no-deps
   ```

4. For routine SemVer calculations, dry-run the bundled `release-policy`
   `version-bump` helper before editing release-prep surfaces. Keep
   `elkrs-core`, `elkrs-json`, and `elkrs-layered` on the same version unless a
   release note documents the exception.

5. Build the repo-local Java ELK runner, then run the ignored optional parity
   harness before publishing:

   ```bash
   tools/java-elk-json-runner/bin/build
   ELKRS_JAVA_ELK_COMMAND="$PWD/tools/java-elk-json-runner/bin/java-elk-json" cargo test -p elkrs-layered --test java_parity --locked -- --ignored
   ```

6. Create and push an annotated version tag matching the crate versions:

   ```bash
   git tag -a v1.0.0 -m "elkrs v1.0.0"
   git push origin v1.0.0
   ```

   The `Release` workflow verifies the workspace, runs the Java parity oracle,
   creates a source archive, generates source manifests and SBOMs, attests the
   source artifact, and creates the GitHub Release assets. The workflow uses
   `docs/releases/<version>.md` as the GitHub Release notes file and refuses to
   mutate an existing GitHub Release.

Downstream Cargo consumers should use pinned Git dependencies until this project
has a crates.io account and token:

```toml
elkrs-json = { git = "https://github.com/tommimarkus/elkrs", tag = "v1.0.0" }
elkrs-layered = { git = "https://github.com/tommimarkus/elkrs", tag = "v1.0.0" }
```

Do not publish if any release notes imply full ELK coverage, Dediren adapter support, CLI support, or Java coordinate parity.
