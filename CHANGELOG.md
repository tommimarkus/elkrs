# Changelog

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
