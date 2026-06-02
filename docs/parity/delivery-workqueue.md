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
- Batch C: #41 and #43 can be explored in parallel now that #39 settled port
  semantics and #38 settled label/node-size boundaries. Integrate #41 before
  #43 when route behavior depends on layer assignment.
- Batch D: #42 and #44 can be investigated in parallel after #41 boundaries are
  known. Integrate #42 before #44 unless node placement work is limited to a
  documented exclusion decision.
- #45 is a rolling cross-cutting contract check. It may be updated by every
  delivery issue, but it closes last.

## Current Queue

| Order | Issue | Delivery target | Why this order |
| --- | --- | --- | --- |
| 1 | #41 `Delivery: layer assignment semantics v1` | Deliver layer strategies, constraints, IDs, and partitions. | Core layered behavior; higher risk, but now isolated from pure option parsing. |
| 2 | #43 `Delivery: routing variants, self-loops, junctions, and merging v1` | Deliver or exclude route variants, inside self-loops, junctions, and merging. | Builds on existing routing and port evidence; high product visibility. |
| 3 | #42 `Delivery: crossing minimization constraints v1` | Deliver model-order, group-order, greedy-switch, and crossing controls. | Important quality behavior, but easiest after layering and routing boundaries settle. |
| 4 | #44 `Delivery: node placement, compaction, and wrapping v1` | Deliver or exclude placement, compaction, margins, padding, and wrapping. | Broadest and riskiest slice; defer until upstream boundaries are clearer. |
| 5 | #45 `Delivery: all in-scope JSON option round trip v1` | Close the full JSON contract for all in-scope 1.0.0 behavior. | Cross-cutting closeout; should follow capability decisions rather than lead them. |

## Completed Deliveries

| Issue | Local commit | Result |
| --- | --- | --- |
| #37 `Delivery: promote existing semantic rows to Java parity` | `85daf24` | Promoted `LAYERED-GRAPH-009` and component-spacing `LAYERED-P4-003` to `java-parity`; split connected-component compaction to `LAYERED-P4-004`; documented `LAYERED-JSON-001` Java JSON-oracle follow-up under #45 and `LAYERED-META-OPTION-106` non-node target follow-ups under later delivery queues. |
| #40 `Delivery: hierarchy and topdown compatibility decisions v1` | `0f0d819` | Recorded 1.0.0 compatibility exclusions for clusters, true `SEPARATE_CHILDREN` multi-run hierarchy semantics, hypernode layout semantics, and recursive topdown sizing/layout semantics; kept current parse, serialization, and diagnostics explicit. |
| #39 `Delivery: port constraints and ordering v1` | `30cbba7` | Preserved Java-backed explicit port anchors and port-port spacing; recorded 1.0.0 compatibility exclusions for fixed port-order, fixed-position, alignment distribution, offset-aware placement, non-flow side switching, port sorting strategy, port anchor offsets, and surrounding port-space object semantics. |
| #38 `Delivery: labels and node sizing v1` | `c539621` | Promoted node labels as Java-backed node-size inputs, node-size constraints, and node-size minimum to `java-parity`; added typed node-label placement input for Java-compatible sizing fixtures; recorded 1.0.0 compatibility exclusions for edge-label placement, full node-label placement/padding, port-label placement, comment spacing, label spacing, advanced node-size flags, fixed graph-size semantics, and port/port-label size constraints. |

## Closeout Standard

For each delivery issue:

- Matrix rows move to `java-parity` or documented exclusion.
- Rust tests cover the behavior or exclusion boundary.
- JSON import/export behavior is explicit when representable.
- Java oracle evidence exists for each `java-parity` claim.
- `docs/parity/elk-layered-v0.11.0.md` and inventory overrides agree.
- The issue receives a final progress marker with verification commands.
