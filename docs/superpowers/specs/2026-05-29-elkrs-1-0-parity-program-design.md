# elkrs 1.0 Parity Program Design

Date: 2026-05-29

## Context

`elkrs` is a Rust workspace for a clean-room port of Eclipse Layout Kernel
behavior. The current repository already contains `elkrs-core`, `elkrs-json`,
and `elkrs-layered`, plus a parity matrix, structural quality tests, and an
opt-in Java ELK black-box comparison harness.

The product target for this program is `elkrs` `1.0.0` with full ELK `v0.11.0`
parity, excluding external plugins. Distribution remains GitHub Releases and
pinned Git dependencies for now. crates.io publication is not part of this
program unless a later product decision adds it.

## Goals

- Reach full ELK `v0.11.0` parity for in-repository, non-external-plugin
  behavior.
- Use the parity matrix as the product contract and GitHub Issues as the
  execution tracker.
- Advance every in-scope parity row to `java-parity`, or document an explicit
  exclusion with rationale.
- Preserve the strict clean-room boundary.
- Keep the public API library-first and SemVer-ready before `1.0.0`.
- Keep release claims, docs, tests, and parity evidence aligned.

## Non-Goals

- Do not publish to crates.io as part of this program.
- Do not claim parity for external plugins.
- Do not add downstream-specific adapters.
- Do not use Eclipse ELK implementation source, comments, or line-by-line Java
  processor structure.
- Do not treat fixture success alone as parity without matrix and oracle
  evidence.

## Program Model

The parity matrix starts at `docs/parity/elk-layered-v0.11.0.md` and must be
expanded or split so it covers every in-scope ELK `v0.11.0` capability that is
not provided by an external plugin. The matrix is the source of truth for
product status. GitHub Issues are the work ledger used to execute the matrix.

The operating model has five owner lanes:

- Product and parity ownership keeps the matrix complete, resolves scope and
  exclusion decisions, and blocks vague parity claims.
- API and model ownership manages public graph, options, diagnostics, JSON, and
  SemVer compatibility decisions.
- Layered algorithm ownership implements cycle breaking, layering, crossing
  minimization, node placement, edge routing, labels, sizing, and constraints.
- Oracle and quality ownership turns every claimed behavior into Rust structural
  tests and Java oracle fixtures.
- Release and evidence ownership manages GitHub release readiness, docs, audit,
  SBOM/provenance evidence, tags, and version policy.

Subagents may be used inside those lanes, but each delegated task must be
bounded to one issue, row group, or evidence question. Workers should have
explicit file/module ownership and must not revert unrelated work.

## GitHub Issue Taxonomy

Create GitHub epics for the major parity areas:

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

Each child issue maps to one parity row or a tightly coupled row group. Coupled
rows are allowed when one coherent implementation is required, for example
multi-edges with routing fixtures, or labels with sizing constraints.

Each child issue must include:

- Matrix row IDs covered.
- Current matrix status and target status.
- Public API decisions and compatibility risk.
- Rust semantic test requirements.
- Java oracle fixture requirements.
- JSON import/export expectations where applicable.
- Clean-room source and evidence notes.
- Documentation and release-note impact.
- Done condition.

## Work Sequence

### Phase 1: Parity Inventory

Expand or split the matrix so it covers all in-scope ELK `v0.11.0` behavior
excluding external plugins. Identify rows that are implemented, unsupported,
diagnostic, semantic-only, `java-parity`, or intentionally excluded.

Create the GitHub epics and child issues from this inventory. The initial issue
set should favor traceability over implementation detail; implementation plans
can refine issue bodies later.

### Phase 2: Public Contract Foundation

Stabilize the public graph model, option model, diagnostics, JSON behavior,
label and sizing representation, and coordinate semantics. This phase should
resolve public API commitments before the algorithm grows around unstable
shapes.

Expected decisions include:

- Whether public mutable fields remain part of the long-term API.
- How invalid numeric options are rejected or diagnosed.
- How unsupported JSON fields are reported or deliberately omitted.
- How absolute and relative coordinates are represented for compound graphs.
- Which diagnostic codes are stable enough for consumers.

### Phase 3: Oracle Expansion

Build a fixture-driven Java comparison suite before deep algorithm rewrites.
Every parity issue should have a clear place to add Java-backed structural
evidence.

The oracle should compare structural behavior such as layer direction,
containment, overlap, crossings, port anchors, routing validity, labels, sizing,
and option effects. Exact coordinates are acceptable only for simple, documented
cases where the coordinate contract is intentionally stable.

### Phase 4: Layered Algorithm Parity

Implement parity rows by algorithm phase:

- Cycle breaking.
- Layer assignment.
- Crossing minimization.
- Node placement.
- Edge routing.
- Labels, sizing, ports, compounds, and constraints.

Each slice should be small enough to merge independently and should update the
matrix, Rust tests, Java fixtures, docs, and issue status together.

### Phase 5: Broad Parity Closure

Close unsupported and edge-case rows, including multi-edges, self-loops, labels,
non-plugin cluster behavior, port constraints, node sizing constraints, advanced
layering/crossing/placement strategies, routing variants, junctions, edge
merging, component compaction, and option combinations.

Rows that remain out of scope must become documented exclusions, not forgotten
unsupported entries.

### Phase 6: 1.0.0 Hardening

Before `1.0.0`, run the full Rust gate, Java parity suite, docs build, release
evidence workflows, security/audit checks, API compatibility review, and final
wording review. Release through GitHub Releases and pinned Git dependencies
only unless a later decision adds crates.io.

## Completion Standards

A parity row is complete only when it has:

- Public API support, or a documented exclusion.
- Rust semantic tests for the behavior.
- ELK JSON import/export behavior where the feature is representable in ELK
  JSON.
- Opt-in Java oracle fixture evidence.
- Matrix status updated to `java-parity`, or exclusion rationale.
- Documentation that matches the actual support boundary.
- Clean-room evidence based on public docs, option metadata, hand-authored
  fixtures, or black-box Java output.

An issue is complete only when all covered rows meet those standards and the
issue links the verifying commands or workflow evidence.

The `1.0.0` program is complete only when every in-scope ELK `v0.11.0` row is
`java-parity`, and every excluded row has an explicit documented rationale.

## Error Handling And Diagnostics

Fatal errors should be used for graph or option states that prevent valid
output. Recoverable unsupported or partially supported behavior should produce
diagnostics when layout can continue.

For `1.0.0`, diagnostic semantics become part of the consumer-facing contract.
Issue work that adds or changes diagnostics must include tests and release-note
impact.

## Testing And Verification

The default local gate remains:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo doc --workspace --locked --no-deps
```

Release and parity work also requires:

```bash
cargo audit
tools/java-elk-json-runner/bin/build
ELKRS_JAVA_ELK_COMMAND="$PWD/tools/java-elk-json-runner/bin/java-elk-json" cargo test -p elkrs-layered --test java_parity --locked -- --ignored
```

Network-dependent release tooling should be separated from the fast local gate.
The Java parity suite remains opt-in outside release/evidence contexts.

## Release Policy

The `1.0.0` release is a GitHub Release and pinned Git dependency release.
crates.io publication is deferred.

Release notes must not imply:

- External plugin parity.
- Downstream adapter support.
- Pixel-perfect Java coordinate identity.
- Full support for any row that is not `java-parity` or explicitly excluded.

Version tags must match crate versions. Annotated tags are preferred for
release tags.

## Design Outcome

The approved direction is the parity-matrix program model. The next step is to
create an implementation plan that turns this spec into GitHub epics, issue
templates, and prioritized implementation slices.
