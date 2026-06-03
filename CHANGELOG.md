# Changelog

## Unreleased

- Marked the project as discontinued. No further feature work, parity expansion,
  release preparation, or security maintenance is planned unless the project is
  explicitly revived.
- Aligned default spacing values with the pinned ELK Layered v0.11.0 option
  metadata for node-node, between-layer node-node, and edge-node spacing.
- Rejected negative node-node and between-layer node-node spacing through both
  JSON import validation and direct `elkrs-layered` layout validation.
- Corrected the `1.0.0` release disclosure: issue #46 remained open after the
  tag was published, so `1.0.0` must not be read as full upstream
  test/model-corpus parity or renewed release-readiness.
- Added `elkrs-visual-parity`, a workspace-local PNG artifact generator for
  side-by-side Java ELK oracle and `elkrs` visual comparison output.

## 1.0.0 - 2026-06-03

- Completed the then-closed delivery rows for the clean-room ELK Layered v0.11.0
  supported matrix target, excluding external plugins and documented 1.0.0
  compatibility exclusions. The upstream portable Java test/model corpus gate
  tracked by issue #46 remained open after publication.
- Promoted Java-backed evidence for directed graphs, multi-edges, ordinary self-loops, ports as endpoints, compound nodes, hierarchy-crossing edges, direction, spacing, node labels as size inputs, node size constraints, layer assignment defaults, orthogonal routing, component spacing, and adjacent-layer crossing minimization.
- Closed the supported ELK-style JSON contract for graph, node, edge, port, label, edge-section, option import/export/validation, unknown-field discard, and unrepresented known-option discard.
- Documented compatibility exclusions for out-of-scope labels, inside self-loops, clusters, alternate layerers, model-order and crossing constraints, Brandes-Koepf placement, graph wrapping, non-orthogonal route geometry, junction output, edge merging, port ordering variants, and object-only option rows.
- Kept Java parity evidence opt-in through `ELKRS_JAVA_ELK_COMMAND` and kept distribution scoped to GitHub Releases plus pinned Git dependencies.

This release does not claim full upstream test/model-corpus parity, external
plugin behavior, crates.io distribution, CLI support, Dediren adapter support,
excluded matrix rows, or pixel-perfect Java coordinates.

## 0.2.0 - 2026-06-02

- Expanded canonical ELK JSON option import/export for the current clean-room Layered subset.
- Added diagnostics for additional recognized but unsupported parent, node, routing, hierarchy, spacing, wrapping, and port-label options.
- Added parity inventory coverage from pinned Java ELK v0.11.0 metadata and kept the matrix status vocabulary aligned with the current Rust proof surface.
- Added Java oracle fixtures for selected deterministic layout behavior while keeping the oracle harness opt-in.
- Tightened validation for selected option values and spacing values.
- Preserved node coordinates for nodes marked with `org.eclipse.elk.noLayout`.
- Applied `org.eclipse.elk.spacing.nodeSelfLoop` to node self-loop route clearance.
- Applied `org.eclipse.elk.spacing.portPort` to default-geometry same-side ports.

This release is still a clean-room subset. It does not claim full ELK Layered v0.11.0 parity, full option semantics, crates.io distribution, CLI support, Dediren adapter support, or pixel-perfect Java coordinates.

## 0.1.0 - 2026-05-28

- Added `elkrs-core` graph, geometry, option, diagnostic, and layout result types.
- Added `elkrs-layered` with an initial deterministic Layered pipeline.
- Added direction, spacing, algorithm, routing, hierarchy, port, compound, and edge-section support for the first narrow subset.
- Added `elkrs-json` for a narrow ELK-style JSON import/export subset.
- Added structural quality metrics, consumer-shaped acceptance fixtures, and an opt-in Java ELK black-box comparison harness.

This release is an initial clean-room subset. It does not claim full ELK algorithm coverage or pixel-perfect Java coordinate parity.
