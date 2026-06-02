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

- Batch A: #37 and #40 can run in parallel.
  - #37 is evidence promotion for existing semantic behavior.
  - #40 is compatibility decision work for hierarchy, clusters, and topdown
    behavior.
- Batch B: after #40 decisions are recorded, #39 and #38 can run in parallel in
  separate worktrees. Expect both to touch JSON/core surfaces; integrate in
  queue order and rerun the full gate after each merge.
- Batch C: #41 and #43 can be explored in parallel after #39 settles port
  semantics, but integrate #41 before #43 when route behavior depends on layer
  assignment.
- Batch D: #42 and #44 can be investigated in parallel after #41 boundaries are
  known. Integrate #42 before #44 unless node placement work is limited to a
  documented exclusion decision.
- #45 is a rolling cross-cutting contract check. It may be updated by every
  delivery issue, but it closes last.

## Current Queue

| Order | Issue | Delivery target | Why this order |
| --- | --- | --- | --- |
| 1 | #37 `Delivery: promote existing semantic rows to Java parity` | Promote existing `semantic` rows to `java-parity`. | Fastest closable evidence, and it turns existing work into product-visible progress. |
| 2 | #40 `Delivery: hierarchy and topdown compatibility decisions v1` | Decide or implement hierarchy, cluster, and topdown compatibility. | Removes ambiguous scope before deeper compound, placement, and release claims. |
| 3 | #39 `Delivery: port constraints and ordering v1` | Deliver port constraints, ordering, alignment, and offsets. | Ports are already partially Java-backed and affect routing, labels, and JSON. |
| 4 | #38 `Delivery: labels and node sizing v1` | Deliver label and node-size behavior or exclusions. | Unlocks several parsed/unsupported graph, label, spacing, and size rows. |
| 5 | #41 `Delivery: layer assignment semantics v1` | Deliver layer strategies, constraints, IDs, and partitions. | Core layered behavior; higher risk, but now isolated from pure option parsing. |
| 6 | #43 `Delivery: routing variants, self-loops, junctions, and merging v1` | Deliver or exclude route variants, inside self-loops, junctions, and merging. | Builds on existing routing and port evidence; high product visibility. |
| 7 | #42 `Delivery: crossing minimization constraints v1` | Deliver model-order, group-order, greedy-switch, and crossing controls. | Important quality behavior, but easiest after layering and routing boundaries settle. |
| 8 | #44 `Delivery: node placement, compaction, and wrapping v1` | Deliver or exclude placement, compaction, margins, padding, and wrapping. | Broadest and riskiest slice; defer until upstream boundaries are clearer. |
| 9 | #45 `Delivery: all in-scope JSON option round trip v1` | Close the full JSON contract for all in-scope 1.0.0 behavior. | Cross-cutting closeout; should follow capability decisions rather than lead them. |

## Closeout Standard

For each delivery issue:

- Matrix rows move to `java-parity` or documented exclusion.
- Rust tests cover the behavior or exclusion boundary.
- JSON import/export behavior is explicit when representable.
- Java oracle evidence exists for each `java-parity` claim.
- `docs/parity/elk-layered-v0.11.0.md` and inventory overrides agree.
- The issue receives a final progress marker with verification commands.
