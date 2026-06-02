#!/usr/bin/env python3
"""Generate ELK Layered parity inventory rows from pinned Java ELK metadata."""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path
from typing import Any

sys.pycache_prefix = str(Path(__file__).resolve().parents[2] / ".cache" / "python")

START = "<!-- elkrs-generated-layered-metadata:start -->"
END = "<!-- elkrs-generated-layered-metadata:end -->"
RELEASE_RULE = "\n## Release Rule\n"
METADATA_ARTIFACT = "docs/parity/generated/elk-layered-v0.11.0-metadata.json"


STATUS_OVERRIDES = {
    "org.eclipse.elk.algorithm": (
        "java-parity",
        '`cargo test -p elkrs-layered --test basic_layout --locked layered_layout_rejects_non_layered_algorithm_option`, `cargo test -p elkrs-json --test json_partitions --locked imports_java_algorithm_layout_option`, plus `ELKRS_JAVA_ELK_COMMAND="$PWD/tools/java-elk-json-runner/bin/java-elk-json" cargo test -p elkrs-layered --test java_parity --locked -- --ignored`',
        "Complete for layered selection and unsupported-algorithm diagnostics",
    ),
    "org.eclipse.elk.alignment": (
        "diagnostic",
        "`cargo test -p elkrs-core --locked alignment_and_aspect`; `cargo test -p elkrs-layered --test basic_layout --locked alignment_and_aspect`; `cargo test -p elkrs-json --test json_partitions --locked alignment_and_aspect`; `cargo test -p elkrs-json --test json_errors --locked alignment_and_aspect`",
        "Alignment is parsed and diagnosed; node placement alignment semantics are not implemented yet",
    ),
    "org.eclipse.elk.aspectRatio": (
        "diagnostic",
        "`cargo test -p elkrs-core --locked alignment_and_aspect`; `cargo test -p elkrs-layered --test basic_layout --locked alignment_and_aspect`; `cargo test -p elkrs-json --test json_partitions --locked alignment_and_aspect`; `cargo test -p elkrs-json --test json_errors --locked alignment_and_aspect`",
        "Aspect ratio is parsed and diagnosed; component compaction and aspect-aware placement semantics are not implemented yet",
    ),
    "org.eclipse.elk.commentBox": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked node_boolean`; `cargo test -p elkrs-json --test json_partitions --locked node_boolean`; `cargo test -p elkrs-json --test json_errors --locked node_boolean`",
        "Comment box is parsed and diagnosed; label-like placement semantics are not implemented yet",
    ),
    "org.eclipse.elk.debugMode": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked debug_mode`; `cargo test -p elkrs-json --test json_partitions --locked debug_mode`; `cargo test -p elkrs-json --test json_errors --locked debug_mode`",
        "Debug mode is parsed and diagnosed; debug artifacts are not generated yet",
    ),
    "org.eclipse.elk.direction": (
        "java-parity",
        '`cargo test -p elkrs-layered --test basic_layout --locked layered_layout_respects_left_direction`, `cargo test -p elkrs-json --test json_partitions --locked direction`, `cargo test -p elkrs-json --test json_errors --locked direction`, `cargo test -p elkrs-layered --test parity_matrix --locked direction_metadata_row_has_java_fixture_evidence`, plus `ELKRS_JAVA_ELK_COMMAND="$PWD/tools/java-elk-json-runner/bin/java-elk-json" cargo test -p elkrs-layered --test java_parity --locked -- --ignored`',
        "Complete for Java-backed direction fixture",
    ),
    "org.eclipse.elk.edge.thickness": (
        "diagnostic",
        "`cargo test -p elkrs-core --locked edge_scoped_options`; `cargo test -p elkrs-layered --test basic_layout --locked edge_scoped`; `cargo test -p elkrs-json --test json_partitions --locked edge_scoped_options`; `cargo test -p elkrs-json --test json_errors --locked edge_scoped_options`",
        "1.0.0 compatibility exclusion: thickness-aware edge routing and spacing are out of scope; the option is parsed, serialized, and diagnosed only",
    ),
    "org.eclipse.elk.edgeLabels.inline": (
        "unsupported",
        "Generated from Java ELK v0.11.0 option metadata; no Rust proof mapped yet",
        "1.0.0 compatibility exclusion: edge-label inline placement is out of scope; edge label text and geometry remain JSON round-trip only",
    ),
    "org.eclipse.elk.edgeLabels.placement": (
        "unsupported",
        "Generated from Java ELK v0.11.0 option metadata; no Rust proof mapped yet",
        "1.0.0 compatibility exclusion: edge-label placement is out of scope; edge label text and geometry remain JSON round-trip only",
    ),
    "org.eclipse.elk.spacing.nodeNode": (
        "java-parity",
        '`cargo test -p elkrs-layered --test quality --locked custom_node_spacing_separates_same_layer_nodes`, `cargo test -p elkrs-json --test json_partitions --locked spacing`, `cargo test -p elkrs-json --test json_errors --locked spacing`, `cargo test -p elkrs-layered --test parity_matrix --locked spacing_metadata_rows_have_java_fixture_evidence`, plus `ELKRS_JAVA_ELK_COMMAND="$PWD/tools/java-elk-json-runner/bin/java-elk-json" cargo test -p elkrs-layered --test java_parity --locked -- --ignored`',
        "Complete for Java-backed node spacing fixture",
    ),
    "org.eclipse.elk.layered.spacing.nodeNodeBetweenLayers": (
        "java-parity",
        '`cargo test -p elkrs-layered --test quality --locked custom_layer_spacing_separates_connected_layers`; `cargo test -p elkrs-layered --test parity_matrix --locked spacing_metadata_rows_have_java_fixture_evidence`; plus `ELKRS_JAVA_ELK_COMMAND="$PWD/tools/java-elk-json-runner/bin/java-elk-json" cargo test -p elkrs-layered --test java_parity --locked -- --ignored`',
        "Complete for Java-backed canonical connected adjacent-layer node spacing",
    ),
    "org.eclipse.elk.layered.spacing.baseValue": (
        "parsed",
        "`cargo test -p elkrs-json --test json_partitions --locked additional_spacing`; `cargo test -p elkrs-json --test json_errors --locked negative_additional_spacing_returns_invalid_error`; `cargo test -p elkrs-layered --test basic_layout --locked layered_layout_rejects_negative_additional_spacing`",
        "Parsed and validated; spacing base-value semantics remain open",
    ),
    "org.eclipse.elk.layered.spacing.edgeEdgeBetweenLayers": (
        "parsed",
        "`cargo test -p elkrs-json --test json_partitions --locked layered_edge_spacing`; `cargo test -p elkrs-json --test json_errors --locked negative_layered_edge_spacing_returns_invalid_error`; `cargo test -p elkrs-layered --test basic_layout --locked layered_layout_rejects_negative_layered_edge`",
        "Parsed and validated; between-layer edge-edge routing semantics remain open",
    ),
    "org.eclipse.elk.layered.spacing.edgeNodeBetweenLayers": (
        "parsed",
        "`cargo test -p elkrs-json --test json_partitions --locked layered_edge_spacing`; `cargo test -p elkrs-json --test json_errors --locked negative_layered_edge_spacing_returns_invalid_error`; `cargo test -p elkrs-layered --test basic_layout --locked layered_layout_rejects_negative_layered_edge`",
        "Parsed and validated; between-layer edge-node routing semantics remain open",
    ),
    "org.eclipse.elk.layered.wrapping.additionalEdgeSpacing": (
        "parsed",
        "`cargo test -p elkrs-json --test json_partitions --locked additional_spacing`; `cargo test -p elkrs-json --test json_errors --locked negative_additional_spacing_returns_invalid_error`; `cargo test -p elkrs-layered --test basic_layout --locked layered_layout_rejects_negative_additional_spacing`",
        "Parsed and validated; wrapping edge spacing semantics remain open",
    ),
    "org.eclipse.elk.layered.wrapping.multiEdge.improveCuts": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked parent_boolean`; `cargo test -p elkrs-json --test json_partitions --locked parent_boolean`; `cargo test -p elkrs-json --test json_errors --locked parent_boolean`",
        "Wrapped-edge cut improvement is parsed and diagnosed; graph wrapping semantics are not implemented yet",
    ),
    "org.eclipse.elk.layered.wrapping.multiEdge.improveWrappedEdges": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked parent_boolean`; `cargo test -p elkrs-json --test json_partitions --locked parent_boolean`; `cargo test -p elkrs-json --test json_errors --locked parent_boolean`",
        "Wrapped-edge improvement is parsed and diagnosed; graph wrapping semantics are not implemented yet",
    ),
    "org.eclipse.elk.spacing.commentComment": (
        "parsed",
        "`cargo test -p elkrs-json --test json_partitions --locked additional_spacing`; `cargo test -p elkrs-json --test json_errors --locked negative_additional_spacing_returns_invalid_error`; `cargo test -p elkrs-layered --test basic_layout --locked layered_layout_rejects_negative_additional_spacing`",
        "1.0.0 compatibility exclusion: comment layout spacing semantics are out of scope; the option is parsed and validated only",
    ),
    "org.eclipse.elk.spacing.commentNode": (
        "parsed",
        "`cargo test -p elkrs-json --test json_partitions --locked additional_spacing`; `cargo test -p elkrs-json --test json_errors --locked negative_additional_spacing_returns_invalid_error`; `cargo test -p elkrs-layered --test basic_layout --locked layered_layout_rejects_negative_additional_spacing`",
        "1.0.0 compatibility exclusion: comment-node spacing semantics are out of scope; the option is parsed and validated only",
    ),
    "org.eclipse.elk.spacing.componentComponent": (
        "java-parity",
        '`cargo test -p elkrs-layered --test basic_layout --locked layered_layout_applies_component_component_spacing_between_disconnected_components`; `cargo test -p elkrs-json --test json_partitions --locked additional_spacing`; `cargo test -p elkrs-json --test json_errors --locked negative_additional_spacing_returns_invalid_error`; `cargo test -p elkrs-layered --test parity_matrix --locked component_spacing_row_has_java_fixture_evidence`; plus `ELKRS_JAVA_ELK_COMMAND="$PWD/tools/java-elk-json-runner/bin/java-elk-json" cargo test -p elkrs-layered --test java_parity --locked -- --ignored`',
        "Complete for Java-backed disconnected component spacing fixture",
    ),
    "org.eclipse.elk.spacing.edgeNode": (
        "java-parity",
        '`cargo test -p elkrs-layered --test basic_layout --locked layered_layout_applies_edge_node_spacing_to_obstacle_detours`, `cargo test -p elkrs-json --test json_partitions --locked spacing`, `cargo test -p elkrs-json --test json_errors --locked spacing`, `cargo test -p elkrs-layered --test parity_matrix --locked spacing_metadata_rows_have_java_fixture_evidence`, plus `ELKRS_JAVA_ELK_COMMAND="$PWD/tools/java-elk-json-runner/bin/java-elk-json" cargo test -p elkrs-layered --test java_parity --locked -- --ignored`',
        "Complete for Java-backed obstacle-crossing semantics",
    ),
    "org.eclipse.elk.spacing.edgeLabel": (
        "parsed",
        "`cargo test -p elkrs-json --test json_partitions --locked label_and_port_spacing`; `cargo test -p elkrs-json --test json_errors --locked label_and_port_spacing`; `cargo test -p elkrs-layered --test basic_layout --locked layered_layout_rejects_negative_label_and_port_spacing`",
        "1.0.0 compatibility exclusion: edge-label placement spacing semantics are out of scope; the option is parsed and validated only",
    ),
    "org.eclipse.elk.spacing.edgeEdge": (
        "java-parity",
        '`cargo test -p elkrs-layered --test basic_layout --locked layered_layout_applies_edge_edge_spacing_to_parallel_routes`; `cargo test -p elkrs-json --test json_partitions --locked spacing`; `cargo test -p elkrs-json --test json_errors --locked spacing`; `cargo test -p elkrs-layered --test parity_matrix --locked edge_edge_spacing_rows_have_java_fixture_evidence`; plus `ELKRS_JAVA_ELK_COMMAND="$PWD/tools/java-elk-json-runner/bin/java-elk-json" cargo test -p elkrs-layered --test java_parity --locked -- --ignored`',
        "Complete for Java-backed parallel edge fixture",
    ),
    "org.eclipse.elk.spacing.individual": (
        "unsupported",
        "No public individual-spacing object model yet",
        "1.0.0 compatibility exclusion: per-element individual spacing override semantics are out of scope; supported spacing remains graph-level typed options",
    ),
    "org.eclipse.elk.spacing.labelLabel": (
        "parsed",
        "`cargo test -p elkrs-json --test json_partitions --locked label_and_port_spacing`; `cargo test -p elkrs-json --test json_errors --locked label_and_port_spacing`; `cargo test -p elkrs-layered --test basic_layout --locked layered_layout_rejects_negative_label_and_port_spacing`",
        "1.0.0 compatibility exclusion: label-label placement spacing semantics are out of scope; the option is parsed and validated only",
    ),
    "org.eclipse.elk.spacing.labelNode": (
        "parsed",
        "`cargo test -p elkrs-json --test json_partitions --locked label_and_port_spacing`; `cargo test -p elkrs-json --test json_errors --locked label_and_port_spacing`; `cargo test -p elkrs-layered --test basic_layout --locked layered_layout_rejects_negative_label_and_port_spacing`",
        "1.0.0 compatibility exclusion: label-node placement spacing semantics are out of scope; the option is parsed and validated only",
    ),
    "org.eclipse.elk.spacing.labelPortHorizontal": (
        "parsed",
        "`cargo test -p elkrs-json --test json_partitions --locked label_and_port_spacing`; `cargo test -p elkrs-json --test json_errors --locked label_and_port_spacing`; `cargo test -p elkrs-layered --test basic_layout --locked layered_layout_rejects_negative_label_and_port_spacing`",
        "1.0.0 compatibility exclusion: label-port placement spacing semantics are out of scope; the option is parsed and validated only",
    ),
    "org.eclipse.elk.spacing.labelPortVertical": (
        "parsed",
        "`cargo test -p elkrs-json --test json_partitions --locked label_and_port_spacing`; `cargo test -p elkrs-json --test json_errors --locked label_and_port_spacing`; `cargo test -p elkrs-layered --test basic_layout --locked layered_layout_rejects_negative_label_and_port_spacing`",
        "1.0.0 compatibility exclusion: label-port placement spacing semantics are out of scope; the option is parsed and validated only",
    ),
    "org.eclipse.elk.spacing.nodeSelfLoop": (
        "java-parity",
        '`cargo test -p elkrs-layered --test basic_layout --locked layered_layout_applies_node_self_loop_spacing`; `cargo test -p elkrs-json --test json_partitions --locked additional_spacing`; `cargo test -p elkrs-json --test json_errors --locked negative_additional_spacing_returns_invalid_error`; `cargo test -p elkrs-layered --test parity_matrix --locked node_self_loop_spacing_row_has_java_fixture_evidence`; plus `ELKRS_JAVA_ELK_COMMAND="$PWD/tools/java-elk-json-runner/bin/java-elk-json" cargo test -p elkrs-layered --test java_parity --locked -- --ignored`',
        "Complete for Java-backed node self-loop spacing fixture",
    ),
    "org.eclipse.elk.spacing.portPort": (
        "java-parity",
        '`cargo test -p elkrs-layered --test basic_layout --locked port_port_spacing`; `cargo test -p elkrs-json --test json_partitions --locked label_and_port_spacing`; `cargo test -p elkrs-json --test json_errors --locked label_and_port_spacing`; `cargo test -p elkrs-json --test json_errors --locked negative_node_port_spacing_returns_invalid_error`; `cargo test -p elkrs-layered --test parity_matrix --locked port_spacing_row_has_java_fixture_evidence`; plus `ELKRS_JAVA_ELK_COMMAND="$PWD/tools/java-elk-json-runner/bin/java-elk-json" cargo test -p elkrs-layered --test java_parity --locked -- --ignored`',
        "Complete for Java-backed default same-side port spacing fixture",
    ),
    "org.eclipse.elk.edgeRouting": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked edge_routing`; `cargo test -p elkrs-json --test json_partitions --locked edge_routing`; `cargo test -p elkrs-json --test json_errors --locked edge_routing`",
        "Orthogonal routing is Java-backed; 1.0.0 compatibility exclusion: POLYLINE and SPLINES route geometry is out of scope and remains parsed, serialized, and diagnosed only",
    ),
    "org.eclipse.elk.junctionPoints": (
        "unsupported",
        "No public junction-point model or serializer yet",
        "1.0.0 compatibility exclusion: junction-point output for hyperedges and merged orthogonal routes is out of scope",
    ),
    "org.eclipse.elk.interactiveLayout": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked parent_boolean`; `cargo test -p elkrs-json --test json_partitions --locked parent_boolean`; `cargo test -p elkrs-json --test json_errors --locked parent_boolean`",
        "Interactive layout is parsed and diagnosed; interactive placement constraints are not implemented yet",
    ),
    "org.eclipse.elk.layered.compaction.connectedComponents": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked parent_boolean`; `cargo test -p elkrs-json --test json_partitions --locked parent_boolean`; `cargo test -p elkrs-json --test json_errors --locked parent_boolean`",
        "Connected-components compaction is parsed and diagnosed; component compaction semantics are not implemented yet",
    ),
    "org.eclipse.elk.layered.edgeLabels.centerLabelPlacementStrategy": (
        "unsupported",
        "Generated from Java ELK v0.11.0 option metadata; no Rust proof mapped yet",
        "1.0.0 compatibility exclusion: edge center-label placement strategy is out of scope; edge label text and geometry remain JSON round-trip only",
    ),
    "org.eclipse.elk.layered.edgeLabels.sideSelection": (
        "unsupported",
        "Generated from Java ELK v0.11.0 option metadata; no Rust proof mapped yet",
        "1.0.0 compatibility exclusion: edge label side-selection semantics are out of scope; edge label text and geometry remain JSON round-trip only",
    ),
    "org.eclipse.elk.layered.edgeRouting.polyline.slopedEdgeZoneWidth": (
        "unsupported",
        "No public polyline router option model yet",
        "1.0.0 compatibility exclusion: polyline sloped-edge zone semantics are out of scope with POLYLINE route geometry",
    ),
    "org.eclipse.elk.layered.edgeRouting.selfLoopDistribution": (
        "unsupported",
        "No public self-loop distribution option model yet",
        "1.0.0 compatibility exclusion: configurable self-loop distribution semantics are out of scope; ordinary external self-loops remain Java-backed",
    ),
    "org.eclipse.elk.layered.edgeRouting.selfLoopOrdering": (
        "unsupported",
        "No public self-loop ordering option model yet",
        "1.0.0 compatibility exclusion: configurable self-loop ordering semantics are out of scope; deterministic external self-loop routing remains supported",
    ),
    "org.eclipse.elk.layered.edgeRouting.splines.mode": (
        "unsupported",
        "No public spline routing option model yet",
        "1.0.0 compatibility exclusion: spline routing modes are out of scope with SPLINES route geometry",
    ),
    "org.eclipse.elk.layered.edgeRouting.splines.sloppy.layerSpacingFactor": (
        "unsupported",
        "No public spline routing option model yet",
        "1.0.0 compatibility exclusion: sloppy-spline layer-spacing semantics are out of scope with SPLINES route geometry",
    ),
    "org.eclipse.elk.layered.considerModelOrder.components": (
        "diagnostic",
        "`cargo test -p elkrs-core --locked model_order`; `cargo test -p elkrs-layered --test basic_layout --locked model_order`; `cargo test -p elkrs-json --test json_partitions --locked model_order`; `cargo test -p elkrs-json --test json_errors --locked model_order`",
        "Component model order is parsed and diagnosed; component ordering semantics are not implemented yet",
    ),
    "org.eclipse.elk.layered.considerModelOrder.crossingCounterNodeInfluence": (
        "diagnostic",
        "`cargo test -p elkrs-core --locked model_order_group`; `cargo test -p elkrs-layered --test basic_layout --locked model_order_group`; `cargo test -p elkrs-json --test json_partitions --locked model_order_group`; `cargo test -p elkrs-json --test json_errors --locked model_order_group`",
        "Crossing-counter node-order influence is parsed and diagnosed; model-order crossing semantics are not implemented yet",
    ),
    "org.eclipse.elk.layered.considerModelOrder.crossingCounterPortInfluence": (
        "diagnostic",
        "`cargo test -p elkrs-core --locked model_order_group`; `cargo test -p elkrs-layered --test basic_layout --locked model_order_group`; `cargo test -p elkrs-json --test json_partitions --locked model_order_group`; `cargo test -p elkrs-json --test json_errors --locked model_order_group`",
        "Crossing-counter port-order influence is parsed and diagnosed; port-order crossing semantics are not implemented yet",
    ),
    "org.eclipse.elk.layered.considerModelOrder.groupModelOrder.cbGroupOrderStrategy": (
        "diagnostic",
        "`cargo test -p elkrs-core --locked model_order_group`; `cargo test -p elkrs-layered --test basic_layout --locked model_order_group`; `cargo test -p elkrs-json --test json_partitions --locked model_order_group`; `cargo test -p elkrs-json --test json_errors --locked model_order_group`",
        "Cycle-breaking group order strategy is parsed and diagnosed; group model-order semantics are not implemented yet",
    ),
    "org.eclipse.elk.layered.considerModelOrder.groupModelOrder.cbPreferredSourceId": (
        "diagnostic",
        "`cargo test -p elkrs-core --locked model_order_group`; `cargo test -p elkrs-layered --test basic_layout --locked model_order_group`; `cargo test -p elkrs-json --test json_partitions --locked model_order_group`; `cargo test -p elkrs-json --test json_errors --locked model_order_group`",
        "Cycle-breaking preferred source ID is parsed and diagnosed; group model-order semantics are not implemented yet",
    ),
    "org.eclipse.elk.layered.considerModelOrder.groupModelOrder.cbPreferredTargetId": (
        "diagnostic",
        "`cargo test -p elkrs-core --locked model_order_group`; `cargo test -p elkrs-layered --test basic_layout --locked model_order_group`; `cargo test -p elkrs-json --test json_partitions --locked model_order_group`; `cargo test -p elkrs-json --test json_errors --locked model_order_group`",
        "Cycle-breaking preferred target ID is parsed and diagnosed; group model-order semantics are not implemented yet",
    ),
    "org.eclipse.elk.layered.considerModelOrder.groupModelOrder.cmEnforcedGroupOrders": (
        "diagnostic",
        "`cargo test -p elkrs-core --locked model_order_enforced_group_orders`; `cargo test -p elkrs-layered --test basic_layout --locked model_order_enforced_group_orders`; `cargo test -p elkrs-json --test json_partitions --locked model_order_enforced_group_orders`; `cargo test -p elkrs-json --test json_errors --locked model_order_enforced_group_orders`",
        "Crossing-minimization enforced group orders are parsed and diagnosed; group model-order semantics are not implemented yet",
    ),
    "org.eclipse.elk.layered.considerModelOrder.groupModelOrder.cmGroupOrderStrategy": (
        "diagnostic",
        "`cargo test -p elkrs-core --locked model_order_group`; `cargo test -p elkrs-layered --test basic_layout --locked model_order_group`; `cargo test -p elkrs-json --test json_partitions --locked model_order_group`; `cargo test -p elkrs-json --test json_errors --locked model_order_group`",
        "Crossing-minimization group order strategy is parsed and diagnosed; group model-order semantics are not implemented yet",
    ),
    "org.eclipse.elk.layered.considerModelOrder.groupModelOrder.componentGroupId": (
        "diagnostic",
        "`cargo test -p elkrs-core --locked model_order_group_id`; `cargo test -p elkrs-layered --test basic_layout --locked model_order_group_ids`; `cargo test -p elkrs-json --test json_partitions --locked model_order_group_ids`; `cargo test -p elkrs-json --test json_errors --locked model_order_group_id`",
        "Node-scope component group ID is parsed and diagnosed; edge and port target storage remains open",
    ),
    "org.eclipse.elk.layered.considerModelOrder.groupModelOrder.crossingMinimizationId": (
        "diagnostic",
        "`cargo test -p elkrs-core --locked model_order_group_id`; `cargo test -p elkrs-layered --test basic_layout --locked model_order_group_ids`; `cargo test -p elkrs-json --test json_partitions --locked model_order_group_ids`; `cargo test -p elkrs-json --test json_errors --locked model_order_group_id`",
        "Node-scope crossing minimization ID is parsed and diagnosed; edge and port target storage remains open",
    ),
    "org.eclipse.elk.layered.considerModelOrder.groupModelOrder.cycleBreakingId": (
        "diagnostic",
        "`cargo test -p elkrs-core --locked model_order_group_id`; `cargo test -p elkrs-layered --test basic_layout --locked model_order_group_ids`; `cargo test -p elkrs-json --test json_partitions --locked model_order_group_ids`; `cargo test -p elkrs-json --test json_errors --locked model_order_group_id`",
        "Cycle-breaking group ID is parsed and diagnosed at node scope; group model-order semantics are not implemented yet",
    ),
    "org.eclipse.elk.layered.considerModelOrder.longEdgeStrategy": (
        "diagnostic",
        "`cargo test -p elkrs-core --locked model_order_group`; `cargo test -p elkrs-layered --test basic_layout --locked model_order_group`; `cargo test -p elkrs-json --test json_partitions --locked model_order_group`; `cargo test -p elkrs-json --test json_errors --locked model_order_group`",
        "Long-edge ordering strategy is parsed and diagnosed; long-edge model-order semantics are not implemented yet",
    ),
    "org.eclipse.elk.layered.considerModelOrder.portModelOrder": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked parent_boolean`; `cargo test -p elkrs-json --test json_partitions --locked parent_boolean`; `cargo test -p elkrs-json --test json_errors --locked parent_boolean`",
        "Port model order is parsed and diagnosed; port-order-aware crossing behavior is not implemented yet",
    ),
    "org.eclipse.elk.layered.considerModelOrder.strategy": (
        "diagnostic",
        "`cargo test -p elkrs-core --locked model_order`; `cargo test -p elkrs-layered --test basic_layout --locked model_order`; `cargo test -p elkrs-json --test json_partitions --locked model_order`; `cargo test -p elkrs-json --test json_errors --locked model_order`",
        "Model order strategy is parsed and diagnosed; model-order crossing semantics are not implemented yet",
    ),
    "org.eclipse.elk.layered.considerModelOrder.noModelOrder": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked node_boolean`; `cargo test -p elkrs-json --test json_partitions --locked node_boolean`; `cargo test -p elkrs-json --test json_errors --locked node_boolean`",
        "No model order is parsed and diagnosed; model-order crossing constraints are not implemented yet",
    ),
    "org.eclipse.elk.layered.crossingMinimization.forceNodeModelOrder": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked parent_boolean`; `cargo test -p elkrs-json --test json_partitions --locked parent_boolean`; `cargo test -p elkrs-json --test json_errors --locked parent_boolean`",
        "Forced node model order is parsed and diagnosed; crossing minimization constraints are not implemented yet",
    ),
    "org.eclipse.elk.layered.crossingMinimization.greedySwitch.activationThreshold": (
        "diagnostic",
        "`cargo test -p elkrs-core --locked greedy_switch`; `cargo test -p elkrs-layered --test basic_layout --locked greedy_switch`; `cargo test -p elkrs-json --test json_partitions --locked greedy_switch`; `cargo test -p elkrs-json --test json_errors --locked greedy_switch`",
        "Greedy switch activation threshold is parsed and diagnosed; greedy-switch crossing minimization is not implemented yet",
    ),
    "org.eclipse.elk.layered.crossingMinimization.greedySwitch.type": (
        "diagnostic",
        "`cargo test -p elkrs-core --locked greedy_switch`; `cargo test -p elkrs-layered --test basic_layout --locked greedy_switch`; `cargo test -p elkrs-json --test json_partitions --locked greedy_switch`; `cargo test -p elkrs-json --test json_errors --locked greedy_switch`",
        "Greedy switch type is parsed and diagnosed; greedy-switch crossing minimization is not implemented yet",
    ),
    "org.eclipse.elk.layered.crossingMinimization.greedySwitchHierarchical.type": (
        "diagnostic",
        "`cargo test -p elkrs-core --locked greedy_switch`; `cargo test -p elkrs-layered --test basic_layout --locked greedy_switch`; `cargo test -p elkrs-json --test json_partitions --locked greedy_switch`; `cargo test -p elkrs-json --test json_errors --locked greedy_switch`",
        "Hierarchical greedy switch type is parsed and diagnosed; hierarchical greedy-switch crossing minimization is not implemented yet",
    ),
    "org.eclipse.elk.layered.crossingMinimization.hierarchicalSweepiness": (
        "diagnostic",
        "`cargo test -p elkrs-core --locked crossing_minimization_controls`; `cargo test -p elkrs-layered --test basic_layout --locked crossing_minimization_controls`; `cargo test -p elkrs-json --test json_partitions --locked crossing_minimization_controls`; `cargo test -p elkrs-json --test json_errors --locked crossing_minimization_controls`",
        "Hierarchical sweepiness is parsed and diagnosed; hierarchical crossing minimization semantics are not implemented yet",
    ),
    "org.eclipse.elk.layered.crossingMinimization.inLayerPredOf": (
        "diagnostic",
        "`cargo test -p elkrs-core --locked crossing_minimization_controls`; `cargo test -p elkrs-layered --test basic_layout --locked crossing_minimization_controls`; `cargo test -p elkrs-json --test json_partitions --locked crossing_minimization_controls`; `cargo test -p elkrs-json --test json_errors --locked crossing_minimization_controls`",
        "In-layer predecessor constraints are parsed and diagnosed; node ordering constraints are not implemented yet",
    ),
    "org.eclipse.elk.layered.crossingMinimization.inLayerSuccOf": (
        "diagnostic",
        "`cargo test -p elkrs-core --locked crossing_minimization_controls`; `cargo test -p elkrs-layered --test basic_layout --locked crossing_minimization_controls`; `cargo test -p elkrs-json --test json_partitions --locked crossing_minimization_controls`; `cargo test -p elkrs-json --test json_errors --locked crossing_minimization_controls`",
        "In-layer successor constraints are parsed and diagnosed; node ordering constraints are not implemented yet",
    ),
    "org.eclipse.elk.layered.crossingMinimization.positionChoiceConstraint": (
        "diagnostic",
        "`cargo test -p elkrs-core --locked crossing_minimization_controls`; `cargo test -p elkrs-layered --test basic_layout --locked crossing_minimization_controls`; `cargo test -p elkrs-json --test json_partitions --locked crossing_minimization_controls`; `cargo test -p elkrs-json --test json_errors --locked crossing_minimization_controls`",
        "Position choice constraints are parsed and diagnosed; position-constrained crossing minimization is not implemented yet",
    ),
    "org.eclipse.elk.layered.crossingMinimization.positionId": (
        "diagnostic",
        "`cargo test -p elkrs-core --locked crossing_minimization_controls`; `cargo test -p elkrs-layered --test basic_layout --locked crossing_minimization_controls`; `cargo test -p elkrs-json --test json_partitions --locked crossing_minimization_controls`; `cargo test -p elkrs-json --test json_errors --locked crossing_minimization_controls`",
        "Position IDs are parsed and diagnosed; position-constrained crossing minimization is not implemented yet",
    ),
    "org.eclipse.elk.layered.crossingMinimization.strategy": (
        "diagnostic",
        "`cargo test -p elkrs-core --locked crossing_minimization_controls`; `cargo test -p elkrs-layered --test basic_layout --locked crossing_minimization_controls`; `cargo test -p elkrs-json --test json_partitions --locked crossing_minimization_controls`; `cargo test -p elkrs-json --test json_errors --locked crossing_minimization_controls`",
        "Crossing minimization strategy is parsed and diagnosed; alternate crossing minimization strategies are not implemented yet",
    ),
    "org.eclipse.elk.layered.crossingMinimization.semiInteractive": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked parent_boolean`; `cargo test -p elkrs-json --test json_partitions --locked parent_boolean`; `cargo test -p elkrs-json --test json_errors --locked parent_boolean`",
        "Semi-interactive crossing minimization is parsed and diagnosed; position-derived ordering is not implemented yet",
    ),
    "org.eclipse.elk.layered.feedbackEdges": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked feedback_edges`; `cargo test -p elkrs-json --test json_partitions --locked feedback_edges`; `cargo test -p elkrs-json --test json_errors --locked feedback_edges`",
        "Feedback-edge highlighting is parsed and diagnosed; cycle-breaking strategy semantics remain open",
    ),
    "org.eclipse.elk.layered.directionCongruency": (
        "unsupported",
        "No public direction-congruency option model yet",
        "1.0.0 compatibility exclusion: direction-congruency layer-assignment heuristics are out of scope; explicit layout direction remains Java-backed",
    ),
    "org.eclipse.elk.layered.generatePositionAndLayerIds": (
        "java-parity",
        '`cargo test -p elkrs-layered --test basic_layout --locked layered_layout_generates_position_and_layer_ids`; `cargo test -p elkrs-json --test json_partitions --locked imports_java_boolean_layout_option_strings`; `cargo test -p elkrs-layered --test parity_matrix --locked layer_assignment_rows_have_java_fixture_evidence`; plus `ELKRS_JAVA_ELK_COMMAND="$PWD/tools/java-elk-json-runner/bin/java-elk-json" cargo test -p elkrs-layered --test java_parity --locked -- --ignored`',
        "Complete for Java-backed generated layer IDs and crossing-minimization position IDs on laid-out nodes",
    ),
    "org.eclipse.elk.layered.highDegreeNodes.treatment": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked parent_boolean`; `cargo test -p elkrs-json --test json_partitions --locked parent_boolean`; `cargo test -p elkrs-json --test json_errors --locked parent_boolean`",
        "1.0.0 compatibility exclusion: high-degree layer-assignment treatment is out of scope; the option is parsed, serialized, and diagnosed only",
    ),
    "org.eclipse.elk.layered.highDegreeNodes.threshold": (
        "diagnostic",
        "`cargo test -p elkrs-core --locked high_degree_node_numeric`; `cargo test -p elkrs-layered --test basic_layout --locked high_degree_node_numeric`; `cargo test -p elkrs-json --test json_partitions --locked high_degree_node_numeric`; `cargo test -p elkrs-json --test json_errors --locked high_degree_numeric`",
        "1.0.0 compatibility exclusion: high-degree layer-assignment threshold behavior is out of scope; the option is parsed, serialized, and diagnosed only",
    ),
    "org.eclipse.elk.layered.highDegreeNodes.treeHeight": (
        "diagnostic",
        "`cargo test -p elkrs-core --locked high_degree_node_numeric`; `cargo test -p elkrs-layered --test basic_layout --locked high_degree_node_numeric`; `cargo test -p elkrs-json --test json_partitions --locked high_degree_node_numeric`; `cargo test -p elkrs-json --test json_errors --locked high_degree_numeric`",
        "1.0.0 compatibility exclusion: high-degree tree layer-assignment behavior is out of scope; the option is parsed, serialized, and diagnosed only",
    ),
    "org.eclipse.elk.layered.interactiveReferencePoint": (
        "diagnostic",
        "`cargo test -p elkrs-core --locked interactive_reference_point`; `cargo test -p elkrs-layered --test basic_layout --locked interactive_reference_point`; `cargo test -p elkrs-json --test json_partitions --locked interactive_reference_point`; `cargo test -p elkrs-json --test json_errors --locked interactive_reference_point`",
        "Interactive reference points are parsed and diagnosed; interactive placement reference semantics are not implemented yet",
    ),
    "org.eclipse.elk.layered.layering.coffmanGraham.layerBound": (
        "diagnostic",
        "`cargo test -p elkrs-core --locked layer_assignment_options`; `cargo test -p elkrs-layered --test basic_layout --locked layer_assignment`; `cargo test -p elkrs-json --test json_partitions --locked layer_assignment`; `cargo test -p elkrs-json --test json_errors --locked layer_assignment`",
        "1.0.0 compatibility exclusion: Coffman-Graham layer assignment is out of scope; the bound is parsed, serialized, and diagnosed only",
    ),
    "org.eclipse.elk.layered.layering.layerChoiceConstraint": (
        "diagnostic",
        "`cargo test -p elkrs-core --locked layer_assignment_options`; `cargo test -p elkrs-layered --test basic_layout --locked layer_assignment`; `cargo test -p elkrs-json --test json_partitions --locked layer_assignment`; `cargo test -p elkrs-json --test json_errors --locked layer_assignment`",
        "1.0.0 compatibility exclusion: layer-choice constraint assignment is out of scope; the option is parsed, serialized, and diagnosed only",
    ),
    "org.eclipse.elk.layered.layering.layerConstraint": (
        "diagnostic",
        "`cargo test -p elkrs-core --locked layer_assignment_options`; `cargo test -p elkrs-layered --test basic_layout --locked layer_assignment`; `cargo test -p elkrs-json --test json_partitions --locked layer_assignment`; `cargo test -p elkrs-json --test json_errors --locked layer_assignment`",
        "1.0.0 compatibility exclusion: layer-constraint assignment semantics are out of scope; the option is parsed, serialized, and diagnosed only",
    ),
    "org.eclipse.elk.layered.layering.layerId": (
        "java-parity",
        '`cargo test -p elkrs-layered --test basic_layout --locked layered_layout_generates_position_and_layer_ids`; `cargo test -p elkrs-json --test json_partitions --locked layer_assignment`; `cargo test -p elkrs-layered --test parity_matrix --locked layer_assignment_rows_have_java_fixture_evidence`; plus `ELKRS_JAVA_ELK_COMMAND="$PWD/tools/java-elk-json-runner/bin/java-elk-json" cargo test -p elkrs-layered --test java_parity --locked -- --ignored`',
        "Complete for Java-backed generated layer IDs when `generatePositionAndLayerIds` is enabled; ID-driven input layer assignment remains a 1.0.0 compatibility exclusion",
    ),
    "org.eclipse.elk.layered.layering.strategy": (
        "java-parity",
        '`cargo test -p elkrs-layered --test basic_layout --locked layered_layout_accepts_network_simplex_strategy_without_diagnostic`; `cargo test -p elkrs-json --test json_partitions --locked layer_assignment`; `cargo test -p elkrs-layered --test parity_matrix --locked layer_assignment_rows_have_java_fixture_evidence`; plus `ELKRS_JAVA_ELK_COMMAND="$PWD/tools/java-elk-json-runner/bin/java-elk-json" cargo test -p elkrs-layered --test java_parity --locked -- --ignored`',
        "Complete for Java-backed `NETWORK_SIMPLEX` default strategy; alternate layer-assignment algorithms are 1.0.0 compatibility exclusions",
    ),
    "org.eclipse.elk.layered.layering.minWidth.upperBoundOnWidth": (
        "diagnostic",
        "`cargo test -p elkrs-core --locked layer_assignment_options`; `cargo test -p elkrs-layered --test basic_layout --locked layer_assignment`; `cargo test -p elkrs-json --test json_partitions --locked min_width`; `cargo test -p elkrs-json --test json_errors --locked layer_assignment`",
        "1.0.0 compatibility exclusion: MinWidth layer assignment is out of scope; the option is parsed, serialized, and diagnosed only",
    ),
    "org.eclipse.elk.layered.layering.minWidth.upperLayerEstimationScalingFactor": (
        "diagnostic",
        "`cargo test -p elkrs-core --locked layer_assignment_options`; `cargo test -p elkrs-layered --test basic_layout --locked layer_assignment`; `cargo test -p elkrs-json --test json_partitions --locked min_width`; `cargo test -p elkrs-json --test json_errors --locked layer_assignment`",
        "1.0.0 compatibility exclusion: MinWidth layer assignment is out of scope; the option is parsed, serialized, and diagnosed only",
    ),
    "org.eclipse.elk.layered.layering.nodePromotion.maxIterations": (
        "diagnostic",
        "`cargo test -p elkrs-core --locked layer_assignment_options`; `cargo test -p elkrs-layered --test basic_layout --locked layer_assignment`; `cargo test -p elkrs-json --test json_partitions --locked min_width`; `cargo test -p elkrs-json --test json_errors --locked layer_assignment`",
        "1.0.0 compatibility exclusion: node promotion layer refinement is out of scope; the option is parsed, serialized, and diagnosed only",
    ),
    "org.eclipse.elk.layered.layering.nodePromotion.strategy": (
        "diagnostic",
        "`cargo test -p elkrs-core --locked layer_assignment_options`; `cargo test -p elkrs-layered --test basic_layout --locked layer_assignment`; `cargo test -p elkrs-json --test json_partitions --locked min_width`; `cargo test -p elkrs-json --test json_errors --locked layer_assignment`",
        "1.0.0 compatibility exclusion: node promotion layer refinement is out of scope; the option is parsed, serialized, and diagnosed only",
    ),
    "org.eclipse.elk.layered.layerUnzipping.layerSplit": (
        "diagnostic",
        "`cargo test -p elkrs-core --locked layer_unzipping_layer_split`; `cargo test -p elkrs-layered --test basic_layout --locked layer_unzipping_layer_split`; `cargo test -p elkrs-json --test json_partitions --locked layer_unzipping_layer_split`; `cargo test -p elkrs-json --test json_errors --locked layer_unzipping_layer_split`",
        "1.0.0 compatibility exclusion: layer unzipping semantics are out of scope; the option is parsed, serialized, and diagnosed only",
    ),
    "org.eclipse.elk.layered.layerUnzipping.minimizeEdgeLength": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked node_boolean`; `cargo test -p elkrs-json --test json_partitions --locked node_boolean`; `cargo test -p elkrs-json --test json_errors --locked node_boolean`",
        "1.0.0 compatibility exclusion: layer unzipping minimize-edge-length semantics are out of scope; the option is parsed, serialized, and diagnosed only",
    ),
    "org.eclipse.elk.layered.layerUnzipping.resetOnLongEdges": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked node_boolean`; `cargo test -p elkrs-json --test json_partitions --locked node_boolean`; `cargo test -p elkrs-json --test json_errors --locked node_boolean`",
        "1.0.0 compatibility exclusion: layer unzipping reset-on-long-edges semantics are out of scope; the option is parsed, serialized, and diagnosed only",
    ),
    "org.eclipse.elk.layered.layerUnzipping.strategy": (
        "diagnostic",
        "`cargo test -p elkrs-core --locked layer_unzipping_strategy`; `cargo test -p elkrs-layered --test basic_layout --locked layer_unzipping_strategy`; `cargo test -p elkrs-json --test json_partitions --locked layer_unzipping_strategy`; `cargo test -p elkrs-json --test json_errors --locked layer_unzipping_strategy`",
        "1.0.0 compatibility exclusion: layer unzipping strategies are out of scope; the option is parsed, serialized, and diagnosed only",
    ),
    "org.eclipse.elk.layered.mergeEdges": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked parent_boolean`; `cargo test -p elkrs-json --test json_partitions --locked parent_boolean`; `cargo test -p elkrs-json --test json_errors --locked parent_boolean`",
        "1.0.0 compatibility exclusion: merged routing and implicit merge-port semantics are out of scope; the option is parsed, serialized, and diagnosed only",
    ),
    "org.eclipse.elk.layered.mergeHierarchyEdges": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked parent_boolean`; `cargo test -p elkrs-json --test json_partitions --locked parent_boolean`; `cargo test -p elkrs-json --test json_errors --locked parent_boolean`",
        "1.0.0 compatibility exclusion: hierarchy-crossing edge merge semantics are out of scope; the option is parsed, serialized, and diagnosed only",
    ),
    "org.eclipse.elk.layered.nodePlacement.favorStraightEdges": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked parent_boolean`; `cargo test -p elkrs-json --test json_partitions --locked parent_boolean`; `cargo test -p elkrs-json --test json_errors --locked parent_boolean`",
        "Favoring straight edges is parsed and diagnosed; node placement balancing behavior is not implemented yet",
    ),
    "org.eclipse.elk.layered.priority.direction": (
        "unsupported",
        "No public edge-routing priority option model yet",
        "1.0.0 compatibility exclusion: edge direction priority weighting is out of scope; current routing remains deterministic and option-independent",
    ),
    "org.eclipse.elk.layered.priority.shortness": (
        "unsupported",
        "No public edge-routing priority option model yet",
        "1.0.0 compatibility exclusion: edge shortness priority weighting is out of scope; current routing remains deterministic and option-independent",
    ),
    "org.eclipse.elk.layered.priority.straightness": (
        "unsupported",
        "No public edge-routing priority option model yet",
        "1.0.0 compatibility exclusion: edge straightness priority weighting is out of scope; current routing remains deterministic and option-independent",
    ),
    "org.eclipse.elk.layered.unnecessaryBendpoints": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked parent_boolean`; `cargo test -p elkrs-json --test json_partitions --locked parent_boolean`; `cargo test -p elkrs-json --test json_errors --locked parent_boolean`",
        "1.0.0 compatibility exclusion: unnecessary bendpoint generation is out of scope; the option is parsed, serialized, and diagnosed only",
    ),
    "org.eclipse.elk.hierarchyHandling": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked hierarchy_handling`; `cargo test -p elkrs-json --test json_partitions --locked hierarchy`; `cargo test -p elkrs-json --test json_errors --locked hierarchy`",
        "1.0.0 compatibility exclusion: true SEPARATE_CHILDREN multi-run hierarchy semantics are out of scope; current layout keeps single-run include-like behavior with diagnostics for unsupported separate-child requests",
    ),
    "org.eclipse.elk.hypernode": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked node_boolean`; `cargo test -p elkrs-json --test json_partitions --locked node_boolean`; `cargo test -p elkrs-json --test json_errors --locked node_boolean`",
        "1.0.0 compatibility exclusion: hypernode layout semantics are out of scope; the option is parsed, serialized, and diagnosed only",
    ),
    "org.eclipse.elk.insideSelfLoops.activate": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked node_boolean`; `cargo test -p elkrs-json --test json_partitions --locked node_boolean`; `cargo test -p elkrs-json --test json_errors --locked node_boolean`",
        "1.0.0 compatibility exclusion: inside self-loop routing semantics are out of scope; the option is parsed, serialized, and diagnosed only",
    ),
    "org.eclipse.elk.insideSelfLoops.yo": (
        "diagnostic",
        "`cargo test -p elkrs-core --locked edge_scoped_options`; `cargo test -p elkrs-layered --test basic_layout --locked edge_scoped`; `cargo test -p elkrs-json --test json_partitions --locked edge_scoped_options`; `cargo test -p elkrs-json --test json_errors --locked edge_scoped_options`",
        "1.0.0 compatibility exclusion: inside self-loop edge routing semantics are out of scope; the option is parsed, serialized, and diagnosed only",
    ),
    "org.eclipse.elk.layered.allowNonFlowPortsToSwitchSides": (
        "diagnostic",
        "`cargo test -p elkrs-core --locked port_scoped_options`; `cargo test -p elkrs-layered --test basic_layout --locked port_scoped_options`; `cargo test -p elkrs-json --test json_partitions --locked port_scoped_options`; `cargo test -p elkrs-json --test json_errors --locked port_scoped_options`",
        "1.0.0 compatibility exclusion: non-flow port side switching semantics are out of scope; the option is parsed, serialized, and diagnosed only",
    ),
    "org.eclipse.elk.layered.portSortingStrategy": (
        "unsupported",
        "No public option model yet",
        "1.0.0 compatibility exclusion: port sorting strategy semantics are out of scope until `elkrs-core` exposes a public sorting option and ordering contract",
    ),
    "org.eclipse.elk.portAlignment.default": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked port_alignment`; `cargo test -p elkrs-json --test json_partitions --locked port_alignment`; `cargo test -p elkrs-json --test json_errors --locked port_alignment`",
        "1.0.0 compatibility exclusion: default port alignment distribution semantics are out of scope; the option is parsed, serialized, and diagnosed only",
    ),
    "org.eclipse.elk.portAlignment.east": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked port_alignment`; `cargo test -p elkrs-json --test json_partitions --locked port_alignment`; `cargo test -p elkrs-json --test json_errors --locked port_alignment`",
        "1.0.0 compatibility exclusion: east-side port alignment distribution semantics are out of scope; the option is parsed, serialized, and diagnosed only",
    ),
    "org.eclipse.elk.portAlignment.north": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked port_alignment`; `cargo test -p elkrs-json --test json_partitions --locked port_alignment`; `cargo test -p elkrs-json --test json_errors --locked port_alignment`",
        "1.0.0 compatibility exclusion: north-side port alignment distribution semantics are out of scope; the option is parsed, serialized, and diagnosed only",
    ),
    "org.eclipse.elk.portAlignment.south": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked port_alignment`; `cargo test -p elkrs-json --test json_partitions --locked port_alignment`; `cargo test -p elkrs-json --test json_errors --locked port_alignment`",
        "1.0.0 compatibility exclusion: south-side port alignment distribution semantics are out of scope; the option is parsed, serialized, and diagnosed only",
    ),
    "org.eclipse.elk.portAlignment.west": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked port_alignment`; `cargo test -p elkrs-json --test json_partitions --locked port_alignment`; `cargo test -p elkrs-json --test json_errors --locked port_alignment`",
        "1.0.0 compatibility exclusion: west-side port alignment distribution semantics are out of scope; the option is parsed, serialized, and diagnosed only",
    ),
    "org.eclipse.elk.port.anchor": (
        "unsupported",
        "No public object option model yet",
        "1.0.0 compatibility exclusion: port anchor offset object semantics are out of scope; explicit port side anchors remain supported",
    ),
    "org.eclipse.elk.portConstraints": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked port_constraints`; `cargo test -p elkrs-json --test json_partitions --locked port_constraints`; `cargo test -p elkrs-json --test json_errors --locked port_constraints`",
        "1.0.0 compatibility exclusion: fixed port-order and fixed-position constraint semantics are out of scope; the option is parsed, serialized, and diagnosed only",
    ),
    "org.eclipse.elk.port.borderOffset": (
        "diagnostic",
        "`cargo test -p elkrs-core --locked port_scoped_options`; `cargo test -p elkrs-layered --test basic_layout --locked port_scoped_options`; `cargo test -p elkrs-json --test json_partitions --locked port_scoped_options`; `cargo test -p elkrs-json --test json_errors --locked port_scoped_options`",
        "1.0.0 compatibility exclusion: offset-aware port placement is out of scope; the option is parsed, serialized, and diagnosed only",
    ),
    "org.eclipse.elk.port.index": (
        "diagnostic",
        "`cargo test -p elkrs-core --locked port_scoped_options`; `cargo test -p elkrs-layered --test basic_layout --locked port_scoped_options`; `cargo test -p elkrs-json --test json_partitions --locked port_scoped_options`; `cargo test -p elkrs-json --test json_errors --locked port_scoped_options`",
        "1.0.0 compatibility exclusion: fixed port-order semantics are out of scope; the option is parsed, serialized, and diagnosed only",
    ),
    "org.eclipse.elk.port.side": (
        "java-parity",
        '`cargo test -p elkrs-json --test json_partitions --locked port_side`, `cargo test -p elkrs-json --test json_errors --locked port_side`, `cargo test -p elkrs-layered --test quality --locked port_heavy_fixture_preserves_port_anchor_fidelity`, plus `ELKRS_JAVA_ELK_COMMAND="$PWD/tools/java-elk-json-runner/bin/java-elk-json" cargo test -p elkrs-layered --test java_parity --locked -- --ignored`',
        "Complete for explicit port side anchors; broader port constraints remain open",
    ),
    "org.eclipse.elk.portLabels.nextToPortIfPossible": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked node_boolean`; `cargo test -p elkrs-json --test json_partitions --locked node_boolean`; `cargo test -p elkrs-json --test json_errors --locked node_boolean`",
        "1.0.0 compatibility exclusion: port-label placement semantics are out of scope; the option is parsed, serialized, and diagnosed only",
    ),
    "org.eclipse.elk.portLabels.placement": (
        "unsupported",
        "Generated from Java ELK v0.11.0 option metadata; no Rust proof mapped yet",
        "1.0.0 compatibility exclusion: port-label placement semantics are out of scope; no public enumset model is exposed yet",
    ),
    "org.eclipse.elk.portLabels.treatAsGroup": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked node_boolean`; `cargo test -p elkrs-json --test json_partitions --locked node_boolean`; `cargo test -p elkrs-json --test json_errors --locked node_boolean`",
        "1.0.0 compatibility exclusion: port-label group placement semantics are out of scope; the option is parsed, serialized, and diagnosed only",
    ),
    "org.eclipse.elk.spacing.portsSurrounding": (
        "unsupported",
        "No public object option model yet",
        "1.0.0 compatibility exclusion: surrounding port-space object semantics are out of scope; Java-backed port-port spacing remains supported",
    ),
    "org.eclipse.elk.nodeSize.options": (
        "unsupported",
        "No public option model yet",
        "1.0.0 compatibility exclusion: advanced node-size option flags are out of scope; explicit minimum-size and node-label sizing constraints are Java-backed",
    ),
    "org.eclipse.elk.nodeSize.constraints": (
        "java-parity",
        '`cargo test -p elkrs-core --locked node_size_options_can_be_set`; `cargo test -p elkrs-json --test json_partitions --locked node_size`; `cargo test -p elkrs-json --test json_errors --locked node_size`; `cargo test -p elkrs-layered --test basic_layout --locked node_size`; `cargo test -p elkrs-layered --test parity_matrix --locked node_label_and_size_rows_have_java_fixture_evidence`; plus `ELKRS_JAVA_ELK_COMMAND="$PWD/tools/java-elk-json-runner/bin/java-elk-json" cargo test -p elkrs-layered --test java_parity --locked -- --ignored`',
        "Complete for Java-backed `MINIMUM_SIZE` and `NODE_LABELS`; `PORTS` and `PORT_LABELS` are diagnosed as a 1.0.0 compatibility exclusion",
    ),
    "org.eclipse.elk.nodeSize.minimum": (
        "java-parity",
        '`cargo test -p elkrs-core --locked node_size_options_can_be_set`; `cargo test -p elkrs-json --test json_partitions --locked node_size`; `cargo test -p elkrs-json --test json_errors --locked node_size`; `cargo test -p elkrs-layered --test basic_layout --locked layered_layout_applies_node_size_minimum_constraint`; `cargo test -p elkrs-layered --test parity_matrix --locked node_label_and_size_rows_have_java_fixture_evidence`; plus `ELKRS_JAVA_ELK_COMMAND="$PWD/tools/java-elk-json-runner/bin/java-elk-json" cargo test -p elkrs-layered --test java_parity --locked -- --ignored`',
        "Complete for Java-backed node minimum-size growth",
    ),
    "org.eclipse.elk.nodeSize.fixedGraphSize": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked parent_boolean`; `cargo test -p elkrs-json --test json_partitions --locked parent_boolean`; `cargo test -p elkrs-json --test json_errors --locked parent_boolean`",
        "1.0.0 compatibility exclusion: fixed graph-size semantics are out of scope; the option is parsed, serialized, and diagnosed only",
    ),
    "org.eclipse.elk.nodeLabels.padding": (
        "unsupported",
        "Generated from Java ELK v0.11.0 option metadata; no Rust proof mapped yet",
        "1.0.0 compatibility exclusion: node-label padding is out of scope; node labels are Java-backed only as node-size inputs",
    ),
    "org.eclipse.elk.nodeLabels.placement": (
        "parsed",
        "`cargo test -p elkrs-core --locked node_label_placement_options_can_be_set`; `cargo test -p elkrs-json --test json_partitions --locked node_label_placement`; `cargo test -p elkrs-layered --test parity_matrix --locked node_label_placement_row_documents_compatibility_boundary`",
        "1.0.0 compatibility exclusion: node-label placement behavior is out of scope; the option is parsed and serialized only to activate Java-compatible node-label sizing fixtures",
    ),
    "org.eclipse.elk.noLayout": (
        "semantic",
        "`cargo test -p elkrs-layered --test basic_layout --locked layered_layout_preserves_no_layout_node_position`; `cargo test -p elkrs-json --test json_partitions --locked node_boolean`; `cargo test -p elkrs-json --test json_errors --locked node_boolean`",
        "Node no-layout preserves input node coordinates; edge, label, and port target semantics remain deferred to the edge, label, port, and JSON delivery queues because the full metadata row targets EDGES, LABELS, NODES, and PORTS",
    ),
    "org.eclipse.elk.separateConnectedComponents": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked parent_boolean`; `cargo test -p elkrs-json --test json_partitions --locked parent_boolean`; `cargo test -p elkrs-json --test json_errors --locked parent_boolean`",
        "Separate connected components is parsed and diagnosed; component splitting semantics are not implemented yet",
    ),
    "org.eclipse.elk.partitioning.activate": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked parent_boolean`; `cargo test -p elkrs-json --test json_partitions --locked parent_boolean`; `cargo test -p elkrs-json --test json_errors --locked parent_boolean`",
        "1.0.0 compatibility exclusion: partition-aware layer assignment is out of scope; the option is parsed, serialized, and diagnosed only",
    ),
    "org.eclipse.elk.partitioning.partition": (
        "diagnostic",
        "`cargo test -p elkrs-core --locked layer_assignment_options`; `cargo test -p elkrs-layered --test basic_layout --locked layer_assignment`; `cargo test -p elkrs-json --test json_partitions --locked layer_assignment`; `cargo test -p elkrs-json --test json_errors --locked layer_assignment`",
        "1.0.0 compatibility exclusion: partition-aware layer assignment is out of scope; partition IDs are parsed, serialized, and diagnosed only",
    ),
    "org.eclipse.elk.priority": (
        "unsupported",
        "No public priority option model yet",
        "1.0.0 compatibility exclusion: priority-driven layer assignment is out of scope until elkrs-core exposes a public priority contract",
    ),
    "org.eclipse.elk.topdownLayout": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked parent_boolean`; `cargo test -p elkrs-json --test json_partitions --locked parent_boolean`; `cargo test -p elkrs-json --test json_errors --locked parent_boolean`",
        "1.0.0 compatibility exclusion: recursive topdown hierarchy layout and scaling semantics are out of scope; the topdown layout flag is parsed, serialized, and diagnosed only",
    ),
    "org.eclipse.elk.layered.thoroughness": (
        "diagnostic",
        "`cargo test -p elkrs-core --locked crossing_minimization_controls`; `cargo test -p elkrs-layered --test basic_layout --locked crossing_minimization_controls`; `cargo test -p elkrs-json --test json_partitions --locked crossing_minimization_controls`; `cargo test -p elkrs-json --test json_errors --locked crossing_minimization_controls`",
        "Thoroughness is parsed and diagnosed; sweep iteration tuning is not implemented yet",
    ),
    "org.eclipse.elk.randomSeed": (
        "diagnostic",
        "`cargo test -p elkrs-core --locked crossing_minimization_controls`; `cargo test -p elkrs-layered --test basic_layout --locked crossing_minimization_controls`; `cargo test -p elkrs-json --test json_partitions --locked crossing_minimization_controls`; `cargo test -p elkrs-json --test json_errors --locked crossing_minimization_controls`",
        "Randomization seed is parsed and diagnosed; random-dependent crossing behavior is not implemented yet",
    ),
    "org.eclipse.elk.topdown.hierarchicalNodeAspectRatio": (
        "unsupported",
        "No public option model yet",
        "1.0.0 compatibility exclusion: topdown recursive sizing and scaling detail options are out of scope",
    ),
    "org.eclipse.elk.topdown.hierarchicalNodeWidth": (
        "unsupported",
        "No public option model yet",
        "1.0.0 compatibility exclusion: topdown recursive sizing and scaling detail options are out of scope",
    ),
    "org.eclipse.elk.topdown.nodeType": (
        "unsupported",
        "No public option model yet",
        "1.0.0 compatibility exclusion: topdown recursive sizing and scaling detail options are out of scope",
    ),
    "org.eclipse.elk.topdown.scaleFactor": (
        "unsupported",
        "No public option model yet",
        "1.0.0 compatibility exclusion: topdown recursive sizing and scaling detail options are out of scope",
    ),
}


FEATURE_OVERRIDES = {
    "CLUSTERS": (
        "unsupported",
        "Generated from Java ELK v0.11.0 supported feature metadata; no Rust proof mapped yet",
        "1.0.0 compatibility exclusion: non-plugin cluster graph model is out of scope until elkrs-core exposes a public cluster representation",
    ),
    "COMPOUND": (
        "java-parity",
        '`cargo test -p elkrs-layered --test quality --locked nested_group_fixture_has_contained_children`; `cargo test -p elkrs-layered --test parity_matrix --locked graph_feature_metadata_rows_have_java_fixture_evidence`; plus `ELKRS_JAVA_ELK_COMMAND="$PWD/tools/java-elk-json-runner/bin/java-elk-json" cargo test -p elkrs-layered --test java_parity --locked -- --ignored`',
        "Complete for Java-backed nested containment semantics",
    ),
    "EDGE_LABELS": (
        "parsed",
        "`cargo test -p elkrs-json --test json_roundtrip --locked round_trips_node_and_edge_label_text`; layout does not place edge labels yet",
        "1.0.0 compatibility exclusion: edge-label placement semantics are out of scope; edge label text and geometry remain JSON round-trip only",
    ),
    "HIERARCHY": (
        "semantic",
        "`cargo test -p elkrs-layered --test consumer_acceptance --locked consumer_compound_ports_fixture_meets_current_acceptance_metrics`",
        "Parity: compound, hierarchy, and non-plugin cluster behavior",
    ),
    "INSIDE_SELF_LOOPS": (
        "unsupported",
        "Inside self-loop activation and edge flag are parsed and diagnosed; ordinary external self-loops remain Java-backed",
        "1.0.0 compatibility exclusion: inside self-loop routing semantics are out of scope",
    ),
    "MULTI_EDGES": (
        "java-parity",
        '`cargo test -p elkrs-layered --test parity_matrix --locked graph_feature_metadata_rows_have_java_fixture_evidence`; plus `ELKRS_JAVA_ELK_COMMAND="$PWD/tools/java-elk-json-runner/bin/java-elk-json" cargo test -p elkrs-layered --test java_parity --locked -- --ignored`',
        "Complete for Java-backed multi-edge fixture",
    ),
    "PORTS": (
        "java-parity",
        '`cargo test -p elkrs-layered --test quality --locked port_heavy_fixture_preserves_port_anchor_fidelity`; `cargo test -p elkrs-layered --test parity_matrix --locked graph_feature_metadata_rows_have_java_fixture_evidence`; plus `ELKRS_JAVA_ELK_COMMAND="$PWD/tools/java-elk-json-runner/bin/java-elk-json" cargo test -p elkrs-layered --test java_parity --locked -- --ignored`',
        "Complete for Java-backed explicit port endpoint anchors",
    ),
    "SELF_LOOPS": (
        "java-parity",
        '`cargo test -p elkrs-layered --test parity_matrix --locked graph_feature_metadata_rows_have_java_fixture_evidence`; plus `ELKRS_JAVA_ELK_COMMAND="$PWD/tools/java-elk-json-runner/bin/java-elk-json" cargo test -p elkrs-layered --test java_parity --locked -- --ignored`',
        "Complete for Java-backed self-loop fixture",
    ),
}


OPTION_NEXT_PLAN_GROUPS = {
    "Parity: node placement and component compaction": (
        "org.eclipse.elk.alignment",
        "org.eclipse.elk.aspectRatio",
        "org.eclipse.elk.contentAlignment",
        "org.eclipse.elk.interactiveLayout",
        "org.eclipse.elk.layered.interactiveReferencePoint",
        "org.eclipse.elk.layered.wrapping.correctionFactor",
        "org.eclipse.elk.layered.wrapping.cutting.cuts",
        "org.eclipse.elk.layered.wrapping.cutting.msd.freedom",
        "org.eclipse.elk.layered.wrapping.cutting.strategy",
        "org.eclipse.elk.layered.wrapping.multiEdge.distancePenalty",
        "org.eclipse.elk.layered.wrapping.multiEdge.improveCuts",
        "org.eclipse.elk.layered.wrapping.multiEdge.improveWrappedEdges",
        "org.eclipse.elk.layered.wrapping.strategy",
        "org.eclipse.elk.layered.wrapping.validify.forbiddenIndices",
        "org.eclipse.elk.layered.wrapping.validify.strategy",
        "org.eclipse.elk.margins",
        "org.eclipse.elk.padding",
        "org.eclipse.elk.position",
        "org.eclipse.elk.separateConnectedComponents",
    ),
    "Parity: labels and node sizing model": (
        "org.eclipse.elk.commentBox",
    ),
    "Parity: compound, hierarchy, and non-plugin cluster behavior": (
        "org.eclipse.elk.hypernode",
        "org.eclipse.elk.topdown.hierarchicalNodeAspectRatio",
        "org.eclipse.elk.topdown.hierarchicalNodeWidth",
        "org.eclipse.elk.topdown.nodeType",
        "org.eclipse.elk.topdown.scaleFactor",
        "org.eclipse.elk.topdownLayout",
    ),
    "Parity: layer assignment strategies and constraints": (
        "org.eclipse.elk.layered.directionCongruency",
        "org.eclipse.elk.layered.generatePositionAndLayerIds",
        "org.eclipse.elk.layered.highDegreeNodes.threshold",
        "org.eclipse.elk.layered.highDegreeNodes.treatment",
        "org.eclipse.elk.layered.highDegreeNodes.treeHeight",
        "org.eclipse.elk.layered.layerUnzipping.layerSplit",
        "org.eclipse.elk.layered.layerUnzipping.minimizeEdgeLength",
        "org.eclipse.elk.layered.layerUnzipping.resetOnLongEdges",
        "org.eclipse.elk.layered.layerUnzipping.strategy",
        "org.eclipse.elk.partitioning.activate",
        "org.eclipse.elk.partitioning.partition",
        "org.eclipse.elk.priority",
    ),
    "Parity: cycle breaking strategies": (
        "org.eclipse.elk.layered.feedbackEdges",
    ),
    "Parity: edge routing variants, junctions, and merging": (
        "org.eclipse.elk.layered.priority.direction",
        "org.eclipse.elk.layered.priority.shortness",
        "org.eclipse.elk.layered.priority.straightness",
        "org.eclipse.elk.layered.unnecessaryBendpoints",
    ),
    "Parity: crossing minimization constraints": (
        "org.eclipse.elk.layered.thoroughness",
        "org.eclipse.elk.randomSeed",
    ),
    "Parity: complete ELK JSON option and graph round trip": (
        "org.eclipse.elk.debugMode",
        "org.eclipse.elk.noLayout",
    ),
}


OPTION_NEXT_PLAN_OVERRIDES = {
    option_id: next_plan
    for next_plan, option_ids in OPTION_NEXT_PLAN_GROUPS.items()
    for option_id in option_ids
}


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Generate ELK Layered metadata inventory rows into a parity matrix."
    )
    parser.add_argument("--metadata", required=True, type=Path)
    parser.add_argument("--matrix", required=True, type=Path)
    args = parser.parse_args()

    metadata = json.loads(args.metadata.read_text(encoding="utf-8"))
    matrix = args.matrix.read_text(encoding="utf-8")
    generated = render_generated_section(metadata)
    args.matrix.write_text(replace_generated_section(matrix, generated), encoding="utf-8")
    return 0


def render_generated_section(metadata: dict[str, Any]) -> str:
    algorithm = metadata["algorithm"]
    features = sorted(str(feature) for feature in algorithm["supportedFeatures"])
    options = sorted(metadata["knownOptions"], key=option_sort_key)

    lines = [
        START,
        "",
        "## Generated ELK Layered v0.11.0 Metadata Inventory",
        "",
        "This section is generated from the pinned Java ELK `0.11.0` metadata export.",
        "Do not edit rows in this section by hand; update `tools/parity/layered_inventory.py`",
        "or the metadata artifact, then regenerate the section.",
        "",
        f"- Algorithm: `{algorithm['id']}`",
        f"- Metadata artifact: `{METADATA_ARTIFACT}`",
        "",
        "### Supported Graph Features",
        "",
        "| ID | Area | ELK Layered capability | Current status | Current proof | Next plan |",
        "| --- | --- | --- | --- | --- | --- |",
    ]

    for index, feature in enumerate(features, start=1):
        status, proof, next_plan = FEATURE_OVERRIDES.get(
            feature,
            (
                "unsupported",
                "Generated from Java ELK v0.11.0 supported feature metadata; no Rust proof mapped yet",
                "Parity inventory follow-up",
            ),
        )
        lines.append(
            f"| LAYERED-META-FEATURE-{index:03d} | Graph feature metadata | `{escape_md(feature)}` | "
            f"`{status}` | {proof} | {next_plan} |"
        )

    lines.extend(
        [
            "",
            "### Known Algorithm Options",
            "",
            "| ID | Area | ELK Layered capability | Current status | Current proof | Next plan |",
            "| --- | --- | --- | --- | --- | --- |",
        ]
    )

    for index, option in enumerate(options, start=1):
        option_id = str(option["id"])
        name = str(option.get("name") or option_id)
        option_type = str(option.get("type", "UNDEFINED"))
        targets = ", ".join(str(target) for target in option.get("targets", []))
        status, proof, next_plan = STATUS_OVERRIDES.get(
            option_id,
            (
                "unsupported",
                "Generated from Java ELK v0.11.0 option metadata; no Rust proof mapped yet",
                next_plan_for_option(option_id),
            ),
        )
        capability = (
            f"{escape_md(name)} (`{escape_md(option_id)}`, {escape_md(option_type)}, "
            f"targets: {escape_md(targets)})"
        )
        lines.append(
            f"| LAYERED-META-OPTION-{index:03d} | Option metadata | {capability} | "
            f"`{status}` | {proof} | {next_plan} |"
        )

    lines.extend(["", END, ""])
    return "\n".join(lines)


def option_sort_key(option: dict[str, Any]) -> str:
    return str(option.get("id", ""))


def next_plan_for_option(option_id: str) -> str:
    if option_id in OPTION_NEXT_PLAN_OVERRIDES:
        return OPTION_NEXT_PLAN_OVERRIDES[option_id]

    lowered = option_id.lower()
    if ".cyclebreaking." in lowered:
        return "Parity: cycle breaking strategies"
    if ".layering." in lowered:
        return "Parity: layer assignment strategies and constraints"
    if ".crossingminimization." in lowered or ".considermodelorder." in lowered:
        return "Parity: crossing minimization constraints"
    if ".nodeplacement." in lowered or ".compaction." in lowered:
        return "Parity: node placement and component compaction"
    if (
        "edgerouting" in lowered
        or "junction" in lowered
        or "mergeedges" in lowered
        or "mergehierarchyedges" in lowered
        or ".edge." in lowered
    ):
        return "Parity: edge routing variants, junctions, and merging"
    if "label" in lowered or "nodesize" in lowered or "size" in lowered:
        return "Parity: labels and node sizing model"
    if "port" in lowered:
        return "Parity: port constraints and ordering"
    if "spacing" in lowered:
        return "Parity: edge spacing option semantics"
    return "Parity inventory follow-up"


def replace_generated_section(matrix: str, generated: str) -> str:
    has_start = START in matrix
    has_end = END in matrix
    if has_start != has_end:
        raise SystemExit("matrix has incomplete generated metadata markers")

    if has_start and has_end:
        before, rest = matrix.split(START, 1)
        _, after = rest.split(END, 1)
        return before.rstrip() + "\n\n" + generated.rstrip() + "\n\n" + after.lstrip()

    if RELEASE_RULE not in matrix:
        raise SystemExit("matrix is missing the Release Rule section")

    before, after = matrix.split(RELEASE_RULE, 1)
    return before.rstrip() + "\n\n" + generated.rstrip() + "\n" + RELEASE_RULE + after


def escape_md(value: str) -> str:
    return value.replace("|", "\\|").replace("\n", " ")


if __name__ == "__main__":
    raise SystemExit(main())
