# elkrs Multi-Edge And Self-Loop Graph Model Design

## Purpose

Resolve GitHub issue `#18` for the graph-model portion of ELK Layered parity:

- `LAYERED-GRAPH-002`: multi-edges between the same endpoint pair.
- `LAYERED-GRAPH-003`: self-loops.
- `LAYERED-GRAPH-004`: inside self-loops.

This design makes multi-edges and ordinary self-loops explicit in the Rust model, import validation, routing, JSON behavior, tests, and parity matrix. It keeps inside self-loop option semantics as a documented residual row because the current public option model does not yet carry `org.eclipse.elk.insideSelfLoops.*` edge and node options.

## Scope

In scope:

- Allow multiple distinct `ElkEdge` values to share the same source and target endpoints.
- Allow same-node edges through layered import, including port-to-port self-loops on the same node.
- Keep invalid endpoint cases as errors: missing nodes, missing ports, multiple JSON sources, and multiple JSON targets remain invalid.
- Route self-loops deterministically with orthogonal sections that stay outside the node rectangle.
- Offset parallel multi-edge routes so edges with the same endpoint pair do not collapse onto identical paths.
- Preserve Java parity gating: ignored Java oracle tests remain opt-in through `ELKRS_JAVA_ELK_COMMAND`.
- Update parity matrix rows and docs to reflect the exact support boundary.

Out of scope:

- Full ELK inside self-loop option semantics.
- Spline or straight self-loop routing.
- Merge-edges semantics.
- General edge-label placement.
- New public graph representation for multi-source or multi-target hyperedges.

## Current State

The public graph model already represents a single directed edge as:

```rust
pub struct ElkEdge {
    pub id: ElementId,
    pub source: ElementRef,
    pub target: ElementRef,
    pub labels: Vec<ElkLabel>,
    pub sections: Vec<ElkEdgeSection>,
}
```

Multi-edges are naturally representable by distinct edge IDs with identical `source` and `target` values. The missing work is proving that layout and JSON keep them distinct.

Layered import currently rejects every same-node edge before routing:

```rust
if source.node == target.node {
    return Err(LayoutError::InvalidHierarchy(format!(
        "self-loop edge: {}",
        edge.id.as_str()
    )));
}
```

The layer assignment phase currently treats every edge as a layer dependency. If self-loops are simply imported as normal edges, `LayerAssignment` can repeatedly promote the same node. Self-loops therefore need an internal classification so acyclic layering and crossing minimization ignore them while routing still sees them.

## Recommended Approach

Use the existing public model and add narrow internal metadata to layered edges:

```rust
pub(crate) enum LEdgeKind {
    Normal,
    SelfLoop,
}
```

`import_graph` classifies an edge as `SelfLoop` when the resolved source and target owner nodes are the same. It still validates that referenced nodes and ports exist. Normal edges continue through all phases as today.

Layering, cycle breaking, crossing minimization, and obstacle filtering treat self-loops as non-layering edges. Routing routes them after node placement using a self-loop-specific orthogonal path.

This approach is recommended because it keeps public APIs stable, avoids inventing hyperedge abstractions, and isolates loop-specific behavior to the private layered model and routing phase.

## Routing Design

### Normal Edges

Normal edge routing stays structurally compatible with the current implementation:

- Use node anchors for node endpoints.
- Use real port anchors for port endpoints.
- Produce orthogonal point lists.
- Detour around unrelated node rectangles when the route crosses an obstacle.

For parallel normal edges, group edges by `(source endpoint, target endpoint)` after cycle breaking. Each edge gets a stable sibling index based on `ElementId` order. The route is then offset perpendicular to the primary direction:

- Horizontal layout: offset intermediate horizontal and vertical segments in `y`.
- Vertical layout: offset intermediate horizontal and vertical segments in `x`.

Endpoint points remain anchored to nodes or ports. Only bend points move. This keeps route validity while making distinct edges visible.

### Self-Loops

Self-loop routing uses the resolved source and target anchors and the owning node rectangle.

For node-to-node loops, choose the loop side from layout direction:

- `Right`: east side.
- `Left`: west side.
- `Down`: south side.
- `Up`: north side.

For port-to-port loops, honor the source port side when present. If the source port has no side, fall back to layout direction.

The route has four or five points:

- Start at source node or port anchor.
- Move outward by `DEFAULT_EDGE_NODE_SPACING`.
- Move along the outside of the node rectangle far enough to make a visible loop.
- Return to the target node or port anchor.

Self-loop sibling routes use the same stable sibling index idea and increase the outside distance by `DEFAULT_EDGE_EDGE_SPACING` so multiple loops on the same node do not overlap exactly.

## JSON Design

The JSON importer keeps the current one-source and one-target rule:

- `sources: ["a"]`, `targets: ["a"]` is valid and imports as a self-loop.
- `sources: ["out"]`, `targets: ["in"]` is valid when both ports exist on the same node.
- `sources: ["a", "b"]` remains invalid.
- `targets: ["a", "b"]` remains invalid.

The JSON exporter keeps serializing one source ID and one target ID per `ElkEdge`. Distinct multi-edges remain distinct because `ElkGraph.edges` is keyed by edge ID.

## Diagnostics

The old `self-loop edge: <id>` invalid-hierarchy diagnostic is removed for valid same-node endpoints. Diagnostics remain for:

- Missing node endpoint.
- Missing port endpoint.
- Duplicate effective node IDs across hierarchy.
- Unsupported layout options.

Inside self-loop option metadata remains unsupported unless a later option-semantics issue adds typed properties for:

- `org.eclipse.elk.insideSelfLoops.activate`
- `org.eclipse.elk.insideSelfLoops.yo`

## Testing

Rust semantic tests:

- Layered layout routes two edges between the same node pair as distinct sections.
- Layered layout routes a node self-loop outside the node rectangle.
- Layered layout routes a port-to-port self-loop on the same node from the real port anchors.
- Import accepts same-node self-loops and preserves endpoint identity.
- JSON imports and exports self-loop edges.
- JSON imports and exports parallel multi-edges.
- JSON still rejects multiple source or target endpoint arrays.

Java oracle tests:

- Add fixtures for comparable multi-edge and ordinary self-loop graphs.
- Keep them under the ignored Java parity test path.
- Compare structural validity rather than exact coordinates.

Quality metrics:

- Self-loop sections must have at least four points.
- Self-loop bend points must leave the node rectangle interior.
- Parallel multi-edge sections with the same endpoint pair must not be identical.

## Parity Matrix And Docs

Update `docs/parity/elk-layered-v0.11.0.md`:

- `LAYERED-GRAPH-002` moves from `unsupported` to `semantic` or `java-parity` depending on Java fixture completion in the implementation slice.
- `LAYERED-GRAPH-003` moves from `unsupported` to `semantic` or `java-parity` depending on Java fixture completion in the implementation slice.
- `LAYERED-GRAPH-004` stays `unsupported` with an explicit residual reason: inside self-loop option semantics are not represented in typed options yet.

The generated metadata rows for `INSIDE_SELF_LOOPS`, `org.eclipse.elk.insideSelfLoops.activate`, and `org.eclipse.elk.insideSelfLoops.yo` remain linked to issue `#31` until that residual issue is split into a narrower inside-self-loop option issue.

## Acceptance Criteria

Issue `#18` is complete when:

- Multi-edge and ordinary self-loop graph rows have Rust semantic proof.
- Java oracle fixture evidence is present for comparable cases, or the matrix records why Java evidence is deferred.
- Inside self-loop semantics are explicitly documented as unsupported and linked to residual option work.
- The default workspace gate passes:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo doc --workspace --locked --no-deps
```

- The ignored Java parity test passes when `ELKRS_JAVA_ELK_COMMAND` points at the local Java runner.
