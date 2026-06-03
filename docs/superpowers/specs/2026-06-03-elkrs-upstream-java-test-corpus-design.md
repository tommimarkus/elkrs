# elkrs Upstream Java Test Corpus Design

## Goal

Require `elkrs` parity claims to pass the portable upstream ELK Layered
`v0.11.0` behavioral test corpus, not only local hand-authored fixtures.

The target is black-box compatibility. A full ELK Layered parity claim is valid
only after upstream Layered tests and relevant `elk-models` fixtures are
inventoried, classified, and either covered by Rust-vs-Java oracle evidence or
explicitly excluded with rationale.

## Compatibility Contract

`elkrs` must track portable upstream tests as issue-backed delivery work.

Portable upstream tests include:

- ELK Layered black-box tests whose observable input and output can be expressed
  through public graph/model formats.
- Relevant `elk-models` files for Layered behavior when they can be imported or
  clean-room recreated through public ELK-style JSON or typed `elkrs` graph
  builders.
- Random or generated model cases only when their seed, options, and expected
  structural properties are reproducible.

Non-portable upstream tests include:

- Java white-box processor tests that inspect internal Layered phases.
- Tests that depend on Java implementation classes, processor ordering,
  internal comments, or exact source structure.
- Expected coordinate dumps or fixtures derived from implementation internals
  instead of public graph behavior.

Non-portable tests still matter. They must be inventoried and either mapped to
clean-room Rust structural assertions where safe, or documented as non-portable
with a concrete rationale.

## Clean-Room Boundary

Allowed sources are public ELK docs, option metadata, public-format model
files, hand-authored fixtures, downstream consumer fixtures, and black-box Java
ELK output.

Do not copy Java test source, Java comments, internal processor structure,
line-by-line white-box assertions, or implementation-derived expected output
into this repository. When an upstream model file is used, preserve provenance
and import it through public formats or recreate a minimal fixture from the
observable graph contract.

## Issue Tracking

GitHub issue #46 is the active delivery issue:

- `Delivery: upstream Java test corpus compatibility gate`

Issue #30 remains the GitHub mirror of
`docs/parity/delivery-workqueue.md`. The delivery workqueue must list #46 until
the upstream corpus inventory and compatibility gate are complete.

Future parity issues must include upstream test/model impact in their body. A
row cannot be closed as `java-parity` if a portable upstream black-box case for
that behavior is known but not covered or explicitly blocked.

## Documentation Changes

The implementation plan must update:

- `docs/parity/elk-layered-v0.11.0.md` with a parity harness row for the
  upstream corpus gate.
- `docs/parity/issue-taxonomy.md` with upstream test/model inventory fields in
  the child issue standard.
- `docs/parity/delivery-workqueue.md` with #46 as the active delivery item.
- Release guidance so full parity and 1.0 readiness cannot be claimed before
  #46 is complete.

## Verification Strategy

The first implementation slice is inventory and harness design, not a broad
algorithm rewrite. Verification should include:

- A machine-readable or reviewable inventory of upstream Layered tests and
  relevant `elk-models` fixtures.
- Portable/non-portable/equivalent-by-assertion classification.
- Java oracle fixtures or structural Rust assertions for portable cases that
  can be represented in the current public API and JSON contract.
- Explicit blockers for cases that need new public model, JSON, or harness
  support.

Release verification still runs the normal Rust gate and opt-in Java oracle
suite. The upstream corpus gate adds a prerequisite: every portable upstream
case must have coverage, a blocker, or an approved compatibility exclusion.

## Done Condition

The design is satisfied when:

- #46 is closed with linked verification evidence.
- #30 and `docs/parity/delivery-workqueue.md` agree that no active upstream
  corpus delivery remains.
- The parity matrix documents the upstream corpus gate.
- Full ELK Layered parity and `1.0.0` release guidance require the gate.
- No copied Java implementation or test-source material is introduced.
