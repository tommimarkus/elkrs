# ELK Layered v0.11.0 Parity Matrix

This file tracks `elkrs-layered` parity against the clean-room ELK Layered v0.11.0 target.

The matrix is intentionally stricter than the README. README scope describes what users can rely on now. This matrix describes what must be closed before claiming full ELK Layered parity.

## Status Values

| Status | Meaning |
| --- | --- |
| `unsupported` | Not represented in public model, JSON, or layout behavior. |
| `parsed` | Accepted by public model or JSON but not semantically applied. |
| `diagnostic` | Recognized and reported as unsupported or partially supported. |
| `semantic` | Implemented with Rust-only structural proof. |
| `java-parity` | Semantic proof plus opt-in Java ELK structural comparison. |

## Default Proof Commands

```bash
cargo test --workspace --locked
tools/java-elk-json-runner/bin/build
ELKRS_JAVA_ELK_COMMAND="$PWD/tools/java-elk-json-runner/bin/java-elk-json" cargo test -p elkrs-layered --test java_parity --locked -- --ignored
```

The Java command must read ELK-style JSON from stdin and write ELK-style JSON to stdout.

## Matrix

| ID | Area | ELK Layered capability | Current status | Current proof | Next plan |
| --- | --- | --- | --- | --- | --- |
| LAYERED-GRAPH-001 | Graph model | Directed nodes and edges | `semantic` | `cargo test -p elkrs-layered --test quality --locked simple_chain_has_no_node_overlap_and_routed_edges` | Java parity fixture expansion |
| LAYERED-GRAPH-002 | Graph model | Multi-edges between the same endpoint pair | `unsupported` | No fixture yet | Edge routing parity plan |
| LAYERED-GRAPH-003 | Graph model | Self-loops | `unsupported` | Public model can express this, but layout rejects self-loops; no self-loop routing support yet | Edge routing parity plan |
| LAYERED-GRAPH-004 | Graph model | Inside self-loops | `unsupported` | No public option yet | Edge routing parity plan |
| LAYERED-GRAPH-005 | Graph model | Edge labels | `parsed` | `ElkEdge.labels` exists but layout does not place labels | Label and sizing parity plan |
| LAYERED-GRAPH-006 | Graph model | Node labels | `parsed` | `ElkNode.labels` exists but layout does not size or place labels | Label and sizing parity plan |
| LAYERED-GRAPH-007 | Graph model | Ports as edge endpoints | `semantic` | `cargo test -p elkrs-layered --test quality --locked port_heavy_fixture_preserves_port_anchor_fidelity` | Port constraints parity plan |
| LAYERED-GRAPH-008 | Graph model | Compound nodes | `semantic` | `cargo test -p elkrs-layered --test quality --locked nested_group_fixture_has_contained_children` | Compound parity plan |
| LAYERED-GRAPH-009 | Graph model | Hierarchy-crossing edges | `semantic` | `cargo test -p elkrs-layered --test consumer_acceptance --locked` | Compound parity plan |
| LAYERED-GRAPH-010 | Graph model | Clusters | `unsupported` | No public cluster model yet | Compound parity plan |
| LAYERED-OPT-001 | Options | `org.eclipse.elk.algorithm` layered selection | `semantic` | `cargo test -p elkrs-layered --test basic_layout --locked layered_layout_rejects_non_layered_algorithm_option` | Java parity fixture expansion |
| LAYERED-OPT-002 | Options | Direction right, left, down, up | `semantic` | `cargo test -p elkrs-layered --test basic_layout --locked layered_layout_respects_left_direction` | Java parity fixture expansion |
| LAYERED-OPT-003 | Options | Node-node spacing | `semantic` | `cargo test -p elkrs-layered --test quality --locked custom_node_spacing_separates_same_layer_nodes` | Java parity fixture expansion |
| LAYERED-OPT-004 | Options | Layer node-node spacing | `semantic` | `cargo test -p elkrs-layered --test quality --locked custom_layer_spacing_separates_connected_layers` | Java parity fixture expansion |
| LAYERED-OPT-005 | Options | Edge-node spacing | `diagnostic` | `cargo test -p elkrs-layered --test basic_layout --locked layered_layout_reports_unimplemented_edge_spacing_options` | Edge routing parity plan |
| LAYERED-OPT-006 | Options | Edge-edge spacing | `diagnostic` | `cargo test -p elkrs-layered --test basic_layout --locked layered_layout_reports_unimplemented_edge_spacing_options` | Edge routing parity plan |
| LAYERED-OPT-007 | Options | Hierarchy handling include vs separate | `diagnostic` | `cargo test -p elkrs-layered --test basic_layout --locked layered_layout_reports_unsupported_hierarchy_handling` | Compound parity plan |
| LAYERED-OPT-008 | Options | Port constraints, index, alignment, sorting | `unsupported` | No public option model yet | Port constraints parity plan |
| LAYERED-OPT-009 | Options | Node size constraints and minimum size | `unsupported` | No public option model yet | Label and sizing parity plan |
| LAYERED-P1-001 | Cycle breaking | Deterministic cycle handling | `semantic` | `cargo test -p elkrs-layered --test basic_layout --locked layered_layout_routes_multi_node_cycle_in_original_edge_directions` | Cycle and layering parity plan |
| LAYERED-P1-002 | Cycle breaking | Greedy strategy and feedback edge options | `unsupported` | No public option model yet | Cycle and layering parity plan |
| LAYERED-P2-001 | Layering | Stable layer normalization | `semantic` | `cargo test -p elkrs-layered --test basic_layout --locked layered_layout_is_stable_across_node_insertion_order` | Cycle and layering parity plan |
| LAYERED-P2-002 | Layering | Network simplex default strategy | `unsupported` | Current layerer is a narrow implementation | Cycle and layering parity plan |
| LAYERED-P2-003 | Layering | Layer constraints and layer IDs | `unsupported` | No public option model yet | Cycle and layering parity plan |
| LAYERED-P3-001 | Crossing minimization | Adjacent-layer barycenter style ordering | `semantic` | `cargo test -p elkrs-layered --test quality --locked crossing_minimization_reorders_two_layer_targets` | Crossing parity plan |
| LAYERED-P3-002 | Crossing minimization | Greedy switch and model-order constraints | `unsupported` | No public option model yet | Crossing parity plan |
| LAYERED-P4-001 | Node placement | Non-overlap for basic graphs | `semantic` | `cargo test -p elkrs-layered --test quality --locked same_layer_large_nodes_do_not_overlap` | Node placement parity plan |
| LAYERED-P4-002 | Node placement | Brandes-Koepf placement options | `unsupported` | Current placement is a narrow implementation | Node placement parity plan |
| LAYERED-P4-003 | Node placement | Component spacing and component compaction | `unsupported` | No component fixture yet | Node placement parity plan |
| LAYERED-P5-001 | Edge routing | Orthogonal routing to node endpoints | `semantic` | `cargo test -p elkrs-layered --test quality --locked chain_metrics_report_no_crossings_or_route_through_nodes` | Edge routing parity plan |
| LAYERED-P5-002 | Edge routing | Orthogonal routing to port anchors | `semantic` | `cargo test -p elkrs-layered --test quality --locked port_heavy_fixture_preserves_port_anchor_fidelity` | Port constraints parity plan |
| LAYERED-P5-003 | Edge routing | Obstacle detours around unrelated nodes | `semantic` | `cargo test -p elkrs-layered routing_detours_around_unrelated_node_rectangles --locked` | Edge routing parity plan |
| LAYERED-P5-004 | Edge routing | Straight routing | `unsupported` | No public routing variant yet | Edge routing parity plan |
| LAYERED-P5-005 | Edge routing | Spline routing | `unsupported` | No public routing variant yet | Edge routing parity plan |
| LAYERED-P5-006 | Edge routing | Junction points and edge merging | `unsupported` | No public junction model yet | Edge routing parity plan |
| LAYERED-JSON-001 | JSON | Narrow graph, port, option, and edge-section round trip | `semantic` | `cargo test -p elkrs-json --locked` | Public model and JSON parity plan |
| LAYERED-JSON-002 | JSON | All parity matrix options round trip | `unsupported` | Narrow importer intentionally ignores unknown options | Public model and JSON parity plan |
| LAYERED-ORACLE-001 | Parity harness | Opt-in Java comparison for chain fixture | `java-parity` | `tools/java-elk-json-runner/bin/build` plus `ELKRS_JAVA_ELK_COMMAND="$PWD/tools/java-elk-json-runner/bin/java-elk-json" cargo test -p elkrs-layered --test java_parity --locked -- --ignored` | This plan |
| LAYERED-ORACLE-002 | Parity harness | Fixture-driven Java comparison suite | `semantic` | `cargo test -p elkrs-layered --test parity_matrix --locked` plus ignored Java fixture runner when `ELKRS_JAVA_ELK_COMMAND` is set | Java parity fixture expansion |

## Release Rule

Do not claim full ELK Layered parity until every matrix row is `java-parity`, or the row is intentionally excluded by a documented compatibility decision.
