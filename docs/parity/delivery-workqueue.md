# elkrs 1.0 Delivery Workqueue

This is the durable execution-order source for the `elkrs` `1.0.0` ELK
Layered `v0.11.0` parity program. It exists so new sessions and compacted
contexts do not infer priority from recent chat or recent commits.

GitHub issue #30 mirrors this queue for tracker-only workflows. If this file
and #30 disagree, stop and reconcile them before starting new parity work.

## Ordering Rules

- Work delivery issues in the order below unless the product owner changes this
  file and the #30 workqueue.
- Execute independent delivery issues in parallel where possible, using separate
  worktrees or branches. Merge/integrate one delivery result at a time in the
  queue order below, then rerun the required gate after each integration.
- Delivery means row promotion to `java-parity` or a documented compatibility
  exclusion. Diagnostic-only commits are support work, not delivery completion.
- Before starting a delivery issue, refresh the issue body, the latest comments,
  and `docs/parity/elk-layered-v0.11.0.md`.
- After a delivery issue closes or is split, update this file and #30 in the
  same work block.
- Keep broad epics as rollups. Do not pick work from epic titles when this
  queue has an open delivery issue for the same rows.

## Parallel Execution Lanes

Parallelism is allowed for discovery, fixture design, implementation, and
verification when the issues do not require the same decision first. The queue
order below remains the integration order.

Current safe parallel batches:

- Batch A: #37 and #40 are complete.
- Batch B: #39 and #38 are complete.
- Batch C: #41 and #43 are complete.
- Batch D: #42 and #44 are complete.
- #45 is complete. No delivery issues remain in the current queue.

## Current Queue

| Order | Issue | Delivery target | Why this order |
| --- | --- | --- | --- |
| - | - | No active delivery issue. | All planned delivery slices are complete locally. |

## Completed Deliveries

| Issue | Local commit | Result |
| --- | --- | --- |
| #37 `Delivery: promote existing semantic rows to Java parity` | `85daf24` | Promoted `LAYERED-GRAPH-009` and component-spacing `LAYERED-P4-003` to `java-parity`; split connected-component compaction to `LAYERED-P4-004`; documented `LAYERED-JSON-001` Java JSON-oracle follow-up under #45 and `LAYERED-META-OPTION-106` non-node target follow-ups under later delivery queues. |
| #40 `Delivery: hierarchy and topdown compatibility decisions v1` | `0f0d819` | Recorded 1.0.0 compatibility exclusions for clusters, true `SEPARATE_CHILDREN` multi-run hierarchy semantics, hypernode layout semantics, and recursive topdown sizing/layout semantics; kept current parse, serialization, and diagnostics explicit. |
| #39 `Delivery: port constraints and ordering v1` | `30cbba7` | Preserved Java-backed explicit port anchors and port-port spacing; recorded 1.0.0 compatibility exclusions for fixed port-order, fixed-position, alignment distribution, offset-aware placement, non-flow side switching, port sorting strategy, port anchor offsets, and surrounding port-space object semantics. |
| #38 `Delivery: labels and node sizing v1` | `c539621` | Promoted node labels as Java-backed node-size inputs, node-size constraints, and node-size minimum to `java-parity`; added typed node-label placement input for Java-compatible sizing fixtures; recorded 1.0.0 compatibility exclusions for edge-label placement, full node-label placement/padding, port-label placement, comment spacing, label spacing, advanced node-size flags, fixed graph-size semantics, and port/port-label size constraints. |
| #41 `Delivery: layer assignment semantics v1` | `48509db` | Promoted `NETWORK_SIMPLEX` default layering, generated layer IDs, and generated crossing position IDs to `java-parity`; added Java string-boolean import compatibility for oracle output; recorded 1.0.0 compatibility exclusions for alternate layerers, layer constraints, layer-choice constraints, ID-driven input assignment, high-degree layer behavior, layer unzipping, partition-aware assignment, direction congruency, and priority-driven assignment. |
| #43 `Delivery: routing variants, self-loops, junctions, and merging v1` | `bfad10b` | Preserved Java-backed orthogonal routing and ordinary external self-loop support; added JSON multi-section route round-trip coverage; recorded 1.0.0 compatibility exclusions for inside self-loops, POLYLINE/straight route geometry, SPLINES route geometry, junction output, merged routing, implicit merge-port semantics, route priorities, configurable self-loop distribution/order, unnecessary bendpoints, edge thickness, and per-element individual spacing. |
| #42 `Delivery: crossing minimization constraints v1` | `ef33def` | Preserved Java-backed adjacent-layer barycenter crossing reduction; recorded 1.0.0 compatibility exclusions for option-driven model-order, group-order, enforced-order, greedy-switch, random, thoroughness, semi-interactive, and position-constrained crossing-minimization semantics. |
| #44 `Delivery: node placement, compaction, and wrapping v1` | `8925fd3` | Preserved Java-backed basic node non-overlap and disconnected component spacing; recorded 1.0.0 compatibility exclusions for Brandes-Koepf placement strategies, connected-component compaction, post-compaction, aspect and alignment placement, margins, padding, input-position-aware placement, and graph wrapping. |
| #45 `Delivery: all in-scope JSON option round trip v1` | `d013870` | Closed the supported 1.0.0 JSON contract for graph, node, edge, port, label, edge-section, option import/export/validation, unknown-field discard, and unrepresented known-option discard; documented explicit JSON compatibility exclusions for out-of-scope label-scoped, object-only, and non-node target behavior. |

## Closeout Standard

For each delivery issue:

- Matrix rows move to `java-parity` or documented exclusion.
- Rust tests cover the behavior or exclusion boundary.
- JSON import/export behavior is explicit when representable.
- Java oracle evidence exists for each `java-parity` claim.
- `docs/parity/elk-layered-v0.11.0.md` and inventory overrides agree.
- The issue receives a final progress marker with verification commands.
