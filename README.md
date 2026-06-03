# elkrs

elkrs is a Rust port of Eclipse Layout Kernel concepts and behavior.

The 1.0.0 target is library-first strict clean-room ELK Layered behavior aligned with ELK v0.11.0.

Downstream users adapt their own graph contracts.

Current scope:

- Rust graph, geometry, options, diagnostics, and layout error/report types
- `elkrs-layered` layout API with `LayeredLayout` and `LayoutAlgorithm`
- `elkrs-json` import/export API for the supported 1.0.0 ELK-style JSON contract
- Opt-in Java ELK black-box comparison harness via `ELKRS_JAVA_ELK_COMMAND`
- Consumer-shaped compound/port acceptance fixtures without downstream adapters
- Typed direction, spacing, algorithm, routing, hierarchy, sizing, placement, crossing, layer-assignment, and port options for the supported contract
- Diagnostics for recognized but unsupported `elkrs-layered` options
- Deterministic, structurally valid layout for simple directed graphs, port-aware edge endpoints, and basic compound child nodes
- Stable layer normalization across equivalent node insertion orders
- Basic barycenter-style crossing minimization for adjacent layers
- Basic compound child placement inside parent bounds
- Simple orthogonal edge detours around the first unrelated node obstacle
- Import-time validation for duplicate node IDs, missing endpoints, and self-loop edges
- Layout output writes child node coordinates as absolute graph coordinates

Security:

- Vulnerability reporting and supported versions are documented in `SECURITY.md`.
- The current `1.0.x` supply-chain target is SCVS Level 1 and SLSA Build Level 1 evidence for GitHub-built release artifacts.

Not current scope:

- Dediren adapter
- CLI-first runtime
- ELK algorithms outside Layered
- Matrix rows documented as 1.0.0 compatibility exclusions
- Pixel-perfect Java coordinate parity
- Copying Eclipse ELK implementation source

Verification:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo doc --workspace --locked --no-deps
```

Cargo build artifacts are written under `.cache/cargo-target` by repo config.

Distribution:

- Version tags (`v1.0.0`, etc.) run the release workflow.
- Current distribution is GitHub Releases and pinned Git dependencies, not
  crates.io.
- `v1.0.0` is a supported-row Git dependency release. It is not a full upstream
  ELK Layered parity claim while issue #46 remains open.
- Downstream Cargo projects can depend on a release tag, for example:

  ```toml
  elkrs-json = { git = "https://github.com/tommimarkus/elkrs", tag = "v1.0.0" }
  elkrs-layered = { git = "https://github.com/tommimarkus/elkrs", tag = "v1.0.0" }
  ```

- The release workflow attaches a source archive, source file manifests, SBOMs,
  checksums, and build provenance to the GitHub Release.

Parity tracking:

- ELK Layered parity is tracked in [docs/parity/elk-layered-v0.11.0.md](docs/parity/elk-layered-v0.11.0.md).
- Release claims are limited to matrix rows marked `semantic` or `java-parity` and rows with documented 1.0.0 compatibility exclusions.
- Full ELK Layered parity also requires issue #46: portable upstream Java
  black-box tests and relevant `elk-models` fixtures must be inventoried and
  covered by Java oracle evidence or documented exclusions.

The Java ELK parity harness is opt-in and ignored by default. Run it only when a
Java ELK JSON command is available. This repository includes a pinned SDKMAN
test runner for that command:

```bash
tools/java-elk-json-runner/bin/build
ELKRS_JAVA_ELK_COMMAND="$PWD/tools/java-elk-json-runner/bin/java-elk-json" cargo test -p elkrs-layered --test java_parity --locked -- --ignored
```

Visual parity evidence can be generated as PNG files for browser inspection:

```bash
ELKRS_JAVA_ELK_COMMAND="$PWD/tools/java-elk-json-runner/bin/java-elk-json" \
  cargo run -p elkrs-visual-parity -- --fixture node-node-spacing --out .cache/visual-parity
```

Use `--all` instead of `--fixture <name>` to render every Java-comparable
fixture. The PNGs are visual review artifacts; structural parity assertions
remain in the Java parity test.
