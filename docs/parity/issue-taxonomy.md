# elkrs 1.0 Parity Issue Taxonomy

This document defines how GitHub Issues track the `elkrs` `1.0.0` full ELK `v0.11.0` parity program, excluding external plugins.

The parity matrix is the product contract. GitHub Issues are the execution ledger.

## Product Boundary

- Target version: `elkrs` `1.0.0`.
- Parity target: ELK `v0.11.0`, excluding external plugins.
- Distribution target: GitHub Releases and pinned Git dependencies.
- crates.io publication: excluded from this program.
- Clean-room sources: public docs, option metadata, hand-authored fixtures, and black-box Java output only.

## Epic Titles

- `EPIC: Graph model parity`
- `EPIC: Option semantics parity`
- `EPIC: Layered phase P1 cycle breaking parity`
- `EPIC: Layered phase P2 layering parity`
- `EPIC: Layered phase P3 crossing minimization parity`
- `EPIC: Layered phase P4 node placement parity`
- `EPIC: Layered phase P5 edge routing parity`
- `EPIC: Labels, sizing, and constraints parity`
- `EPIC: ELK JSON parity`
- `EPIC: Java oracle fixture expansion`
- `EPIC: 1.0.0 release readiness`

## Child Issue Standard

Each child issue maps to one matrix row or a tightly coupled row group. Each child issue body must include:

- Matrix row IDs covered.
- Current matrix status and target status.
- Public API decisions and compatibility risk.
- Rust semantic test requirements.
- Java oracle fixture requirements.
- JSON import/export expectations where applicable.
- Clean-room source and evidence notes.
- Documentation and release-note impact.
- Done condition.

## Done Conditions

A row is complete only when it has:

- Public API support, or a documented exclusion.
- Rust semantic tests for the behavior.
- ELK JSON import/export behavior where the feature is representable in ELK JSON.
- Opt-in Java oracle fixture evidence.
- Matrix status updated to `java-parity`, or exclusion rationale.
- Documentation that matches the actual support boundary.
- Clean-room evidence based on public docs, option metadata, hand-authored fixtures, or black-box Java output.

An issue is complete only when all covered rows meet the row standard and the issue links the verifying commands or workflow evidence.

The `1.0.0` program is complete only when every in-scope ELK `v0.11.0` row is `java-parity`, and every excluded row has an explicit documented rationale.

## GitHub MCP Execution

Use the configured GitHub MCP against `tommimarkus/elkrs`.

Create epics first. Create child issues second. Link child issues to epics in the body with normal GitHub issue references. If GitHub sub-issues are enabled and the MCP returns the required issue IDs, sub-issues may also be attached after creation.

Do not apply labels unless the label already exists in the repository.

## Seed Child Issues

The first backlog slice should create these child issues:

- `Parity inventory: expand ELK v0.11.0 non-plugin matrix`
- `Parity: multi-edge and self-loop graph model`
- `Parity: labels and node sizing model`
- `Parity: compound, hierarchy, and non-plugin cluster behavior`
- `Parity: port constraints and ordering`
- `Parity: edge spacing option semantics`
- `Parity: cycle breaking strategies`
- `Parity: layer assignment strategies and constraints`
- `Parity: crossing minimization constraints`
- `Parity: node placement and component compaction`
- `Parity: edge routing variants, junctions, and merging`
- `Parity: complete ELK JSON option and graph round trip`
- `Parity: Java oracle fixture suite expansion`
- `Release: 1.0.0 GitHub release readiness`
- `#31 Parity: generated ELK Layered metadata residuals`

## Executed Inventory Issues

- `#17 Parity inventory: expand ELK v0.11.0 non-plugin matrix`
  - Adds pinned Java ELK `0.11.0` metadata export.
  - Adds generated Layered graph-feature and algorithm-option inventory rows.
  - Leaves row-group implementation to linked parity child issues, including residual issue `#31`.
