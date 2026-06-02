#!/usr/bin/env python3
"""Generate ELK Layered parity inventory rows from pinned Java ELK metadata."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any


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
        "Parsed and validated; comment layout semantics remain open",
    ),
    "org.eclipse.elk.spacing.commentNode": (
        "parsed",
        "`cargo test -p elkrs-json --test json_partitions --locked additional_spacing`; `cargo test -p elkrs-json --test json_errors --locked negative_additional_spacing_returns_invalid_error`; `cargo test -p elkrs-layered --test basic_layout --locked layered_layout_rejects_negative_additional_spacing`",
        "Parsed and validated; comment layout semantics remain open",
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
        "Parsed and validated; edge label placement semantics remain open",
    ),
    "org.eclipse.elk.spacing.edgeEdge": (
        "java-parity",
        '`cargo test -p elkrs-layered --test basic_layout --locked layered_layout_applies_edge_edge_spacing_to_parallel_routes`; `cargo test -p elkrs-json --test json_partitions --locked spacing`; `cargo test -p elkrs-json --test json_errors --locked spacing`; `cargo test -p elkrs-layered --test parity_matrix --locked edge_edge_spacing_rows_have_java_fixture_evidence`; plus `ELKRS_JAVA_ELK_COMMAND="$PWD/tools/java-elk-json-runner/bin/java-elk-json" cargo test -p elkrs-layered --test java_parity --locked -- --ignored`',
        "Complete for Java-backed parallel edge fixture",
    ),
    "org.eclipse.elk.spacing.labelLabel": (
        "parsed",
        "`cargo test -p elkrs-json --test json_partitions --locked label_and_port_spacing`; `cargo test -p elkrs-json --test json_errors --locked label_and_port_spacing`; `cargo test -p elkrs-layered --test basic_layout --locked layered_layout_rejects_negative_label_and_port_spacing`",
        "Parsed and validated; label placement semantics remain open",
    ),
    "org.eclipse.elk.spacing.labelNode": (
        "parsed",
        "`cargo test -p elkrs-json --test json_partitions --locked label_and_port_spacing`; `cargo test -p elkrs-json --test json_errors --locked label_and_port_spacing`; `cargo test -p elkrs-layered --test basic_layout --locked layered_layout_rejects_negative_label_and_port_spacing`",
        "Parsed and validated; label placement semantics remain open",
    ),
    "org.eclipse.elk.spacing.labelPortHorizontal": (
        "parsed",
        "`cargo test -p elkrs-json --test json_partitions --locked label_and_port_spacing`; `cargo test -p elkrs-json --test json_errors --locked label_and_port_spacing`; `cargo test -p elkrs-layered --test basic_layout --locked layered_layout_rejects_negative_label_and_port_spacing`",
        "Parsed and validated; label-port placement semantics remain open",
    ),
    "org.eclipse.elk.spacing.labelPortVertical": (
        "parsed",
        "`cargo test -p elkrs-json --test json_partitions --locked label_and_port_spacing`; `cargo test -p elkrs-json --test json_errors --locked label_and_port_spacing`; `cargo test -p elkrs-layered --test basic_layout --locked layered_layout_rejects_negative_label_and_port_spacing`",
        "Parsed and validated; label-port placement semantics remain open",
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
        "Orthogonal routing is Java-backed; POLYLINE and SPLINES are parsed and diagnosed until route geometry parity is implemented",
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
    "org.eclipse.elk.layered.considerModelOrder.components": (
        "diagnostic",
        "`cargo test -p elkrs-core --locked model_order`; `cargo test -p elkrs-layered --test basic_layout --locked model_order`; `cargo test -p elkrs-json --test json_partitions --locked model_order`; `cargo test -p elkrs-json --test json_errors --locked model_order`",
        "Component model order is parsed and diagnosed; component ordering semantics are not implemented yet",
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
    "org.eclipse.elk.layered.generatePositionAndLayerIds": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked parent_boolean`; `cargo test -p elkrs-json --test json_partitions --locked parent_boolean`; `cargo test -p elkrs-json --test json_errors --locked parent_boolean`",
        "Position and layer ID generation is parsed and diagnosed; generated IDs are not emitted yet",
    ),
    "org.eclipse.elk.layered.highDegreeNodes.treatment": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked parent_boolean`; `cargo test -p elkrs-json --test json_partitions --locked parent_boolean`; `cargo test -p elkrs-json --test json_errors --locked parent_boolean`",
        "High degree node treatment is parsed and diagnosed; high-degree layer assignment behavior is not implemented yet",
    ),
    "org.eclipse.elk.layered.layerUnzipping.minimizeEdgeLength": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked node_boolean`; `cargo test -p elkrs-json --test json_partitions --locked node_boolean`; `cargo test -p elkrs-json --test json_errors --locked node_boolean`",
        "Layer unzipping minimize-edge-length is parsed and diagnosed; layer unzipping semantics are not implemented yet",
    ),
    "org.eclipse.elk.layered.layerUnzipping.resetOnLongEdges": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked node_boolean`; `cargo test -p elkrs-json --test json_partitions --locked node_boolean`; `cargo test -p elkrs-json --test json_errors --locked node_boolean`",
        "Layer unzipping reset-on-long-edges is parsed and diagnosed; layer unzipping semantics are not implemented yet",
    ),
    "org.eclipse.elk.layered.mergeEdges": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked parent_boolean`; `cargo test -p elkrs-json --test json_partitions --locked parent_boolean`; `cargo test -p elkrs-json --test json_errors --locked parent_boolean`",
        "Edge merging is parsed and diagnosed; merged routing and implicit ports are not implemented yet",
    ),
    "org.eclipse.elk.layered.mergeHierarchyEdges": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked parent_boolean`; `cargo test -p elkrs-json --test json_partitions --locked parent_boolean`; `cargo test -p elkrs-json --test json_errors --locked parent_boolean`",
        "Hierarchy-crossing edge merging is parsed and diagnosed; merged routing semantics are not implemented yet",
    ),
    "org.eclipse.elk.layered.nodePlacement.favorStraightEdges": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked parent_boolean`; `cargo test -p elkrs-json --test json_partitions --locked parent_boolean`; `cargo test -p elkrs-json --test json_errors --locked parent_boolean`",
        "Favoring straight edges is parsed and diagnosed; node placement balancing behavior is not implemented yet",
    ),
    "org.eclipse.elk.layered.unnecessaryBendpoints": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked parent_boolean`; `cargo test -p elkrs-json --test json_partitions --locked parent_boolean`; `cargo test -p elkrs-json --test json_errors --locked parent_boolean`",
        "Unnecessary bendpoint generation is parsed and diagnosed; extra bendpoints are not emitted yet",
    ),
    "org.eclipse.elk.hierarchyHandling": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked hierarchy_handling`; `cargo test -p elkrs-json --test json_partitions --locked hierarchy`; `cargo test -p elkrs-json --test json_errors --locked hierarchy`",
        "Parity: compound, hierarchy, and non-plugin cluster behavior",
    ),
    "org.eclipse.elk.hypernode": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked node_boolean`; `cargo test -p elkrs-json --test json_partitions --locked node_boolean`; `cargo test -p elkrs-json --test json_errors --locked node_boolean`",
        "Hypernode is parsed and diagnosed; hypernode layout semantics are not implemented yet",
    ),
    "org.eclipse.elk.insideSelfLoops.activate": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked node_boolean`; `cargo test -p elkrs-json --test json_partitions --locked node_boolean`; `cargo test -p elkrs-json --test json_errors --locked node_boolean`",
        "Inside self-loop activation is parsed and diagnosed; inside self-loop routing semantics are not implemented yet",
    ),
    "org.eclipse.elk.insideSelfLoops.yo": (
        "unsupported",
        "No typed edge option model yet for the inside self-loop edge flag",
        "Parity: edge routing variants, junctions, and merging",
    ),
    "org.eclipse.elk.portAlignment.default": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked port_alignment`; `cargo test -p elkrs-json --test json_partitions --locked port_alignment`; `cargo test -p elkrs-json --test json_errors --locked port_alignment`",
        "Port alignment is parsed and diagnosed; port distribution semantics are not implemented yet",
    ),
    "org.eclipse.elk.portAlignment.east": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked port_alignment`; `cargo test -p elkrs-json --test json_partitions --locked port_alignment`; `cargo test -p elkrs-json --test json_errors --locked port_alignment`",
        "East port alignment is parsed and diagnosed; side-specific port distribution semantics are not implemented yet",
    ),
    "org.eclipse.elk.portAlignment.north": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked port_alignment`; `cargo test -p elkrs-json --test json_partitions --locked port_alignment`; `cargo test -p elkrs-json --test json_errors --locked port_alignment`",
        "North port alignment is parsed and diagnosed; side-specific port distribution semantics are not implemented yet",
    ),
    "org.eclipse.elk.portAlignment.south": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked port_alignment`; `cargo test -p elkrs-json --test json_partitions --locked port_alignment`; `cargo test -p elkrs-json --test json_errors --locked port_alignment`",
        "South port alignment is parsed and diagnosed; side-specific port distribution semantics are not implemented yet",
    ),
    "org.eclipse.elk.portAlignment.west": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked port_alignment`; `cargo test -p elkrs-json --test json_partitions --locked port_alignment`; `cargo test -p elkrs-json --test json_errors --locked port_alignment`",
        "West port alignment is parsed and diagnosed; side-specific port distribution semantics are not implemented yet",
    ),
    "org.eclipse.elk.portConstraints": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked port_constraints`; `cargo test -p elkrs-json --test json_partitions --locked port_constraints`; `cargo test -p elkrs-json --test json_errors --locked port_constraints`",
        "Port constraints are parsed and diagnosed; fixed port-order and position semantics are not implemented yet",
    ),
    "org.eclipse.elk.port.side": (
        "java-parity",
        '`cargo test -p elkrs-json --test json_partitions --locked port_side`, `cargo test -p elkrs-json --test json_errors --locked port_side`, `cargo test -p elkrs-layered --test quality --locked port_heavy_fixture_preserves_port_anchor_fidelity`, plus `ELKRS_JAVA_ELK_COMMAND="$PWD/tools/java-elk-json-runner/bin/java-elk-json" cargo test -p elkrs-layered --test java_parity --locked -- --ignored`',
        "Complete for explicit port side anchors; broader port constraints remain open",
    ),
    "org.eclipse.elk.portLabels.nextToPortIfPossible": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked node_boolean`; `cargo test -p elkrs-json --test json_partitions --locked node_boolean`; `cargo test -p elkrs-json --test json_errors --locked node_boolean`",
        "Port-label next-to-port behavior is parsed and diagnosed; port-label placement semantics are not implemented yet",
    ),
    "org.eclipse.elk.portLabels.treatAsGroup": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked node_boolean`; `cargo test -p elkrs-json --test json_partitions --locked node_boolean`; `cargo test -p elkrs-json --test json_errors --locked node_boolean`",
        "Port-label group treatment is parsed and diagnosed; port-label placement semantics are not implemented yet",
    ),
    "org.eclipse.elk.nodeSize.options": (
        "unsupported",
        "No public option model yet",
        "Parity: labels and node sizing model",
    ),
    "org.eclipse.elk.nodeSize.constraints": (
        "unsupported",
        "No public option model yet",
        "Parity: labels and node sizing model",
    ),
    "org.eclipse.elk.nodeSize.minimum": (
        "unsupported",
        "No public option model yet",
        "Parity: labels and node sizing model",
    ),
    "org.eclipse.elk.nodeSize.fixedGraphSize": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked parent_boolean`; `cargo test -p elkrs-json --test json_partitions --locked parent_boolean`; `cargo test -p elkrs-json --test json_errors --locked parent_boolean`",
        "Fixed graph size is parsed and diagnosed; node-size semantics are not implemented yet",
    ),
    "org.eclipse.elk.noLayout": (
        "semantic",
        "`cargo test -p elkrs-layered --test basic_layout --locked layered_layout_preserves_no_layout_node_position`; `cargo test -p elkrs-json --test json_partitions --locked node_boolean`; `cargo test -p elkrs-json --test json_errors --locked node_boolean`",
        "Node no-layout preserves input node coordinates; edge, label, and port target support awaits element property storage",
    ),
    "org.eclipse.elk.separateConnectedComponents": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked parent_boolean`; `cargo test -p elkrs-json --test json_partitions --locked parent_boolean`; `cargo test -p elkrs-json --test json_errors --locked parent_boolean`",
        "Separate connected components is parsed and diagnosed; component splitting semantics are not implemented yet",
    ),
    "org.eclipse.elk.partitioning.activate": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked parent_boolean`; `cargo test -p elkrs-json --test json_partitions --locked parent_boolean`; `cargo test -p elkrs-json --test json_errors --locked parent_boolean`",
        "Layout partitioning is parsed and diagnosed; partition-aware layer assignment is not implemented yet",
    ),
    "org.eclipse.elk.topdownLayout": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked parent_boolean`; `cargo test -p elkrs-json --test json_partitions --locked parent_boolean`; `cargo test -p elkrs-json --test json_errors --locked parent_boolean`",
        "Topdown layout is parsed and diagnosed; topdown hierarchy semantics are not implemented yet",
    ),
}


FEATURE_OVERRIDES = {
    "CLUSTERS": (
        "unsupported",
        "Generated from Java ELK v0.11.0 supported feature metadata; no Rust proof mapped yet",
        "Parity: compound, hierarchy, and non-plugin cluster behavior",
    ),
    "COMPOUND": (
        "java-parity",
        '`cargo test -p elkrs-layered --test quality --locked nested_group_fixture_has_contained_children`; `cargo test -p elkrs-layered --test parity_matrix --locked graph_feature_metadata_rows_have_java_fixture_evidence`; plus `ELKRS_JAVA_ELK_COMMAND="$PWD/tools/java-elk-json-runner/bin/java-elk-json" cargo test -p elkrs-layered --test java_parity --locked -- --ignored`',
        "Complete for Java-backed nested containment semantics",
    ),
    "EDGE_LABELS": (
        "parsed",
        "`cargo test -p elkrs-json --test json_roundtrip --locked round_trips_node_and_edge_label_text`; layout does not place edge labels yet",
        "Parity: labels and node sizing model",
    ),
    "HIERARCHY": (
        "semantic",
        "`cargo test -p elkrs-layered --test consumer_acceptance --locked consumer_compound_ports_fixture_meets_current_acceptance_metrics`",
        "Parity: compound, hierarchy, and non-plugin cluster behavior",
    ),
    "INSIDE_SELF_LOOPS": (
        "unsupported",
        "Inside self-loop activation is parsed and diagnosed; inside self-loop routing and edge flag semantics remain unsupported",
        "Parity: edge routing variants, junctions, and merging",
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
