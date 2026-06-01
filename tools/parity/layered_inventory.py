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
    "org.eclipse.elk.direction": (
        "java-parity",
        '`cargo test -p elkrs-layered --test basic_layout --locked layered_layout_respects_left_direction` plus `ELKRS_JAVA_ELK_COMMAND="$PWD/tools/java-elk-json-runner/bin/java-elk-json" cargo test -p elkrs-layered --test java_parity --locked -- --ignored`',
        "Complete",
    ),
    "org.eclipse.elk.spacing.nodeNode": (
        "java-parity",
        '`cargo test -p elkrs-layered --test quality --locked custom_node_spacing_separates_same_layer_nodes` plus `ELKRS_JAVA_ELK_COMMAND="$PWD/tools/java-elk-json-runner/bin/java-elk-json" cargo test -p elkrs-layered --test java_parity --locked -- --ignored`',
        "Complete",
    ),
    "org.eclipse.elk.layered.spacing.nodeNodeBetweenLayers": (
        "java-parity",
        '`cargo test -p elkrs-layered --test quality --locked custom_layer_spacing_separates_connected_layers` plus `ELKRS_JAVA_ELK_COMMAND="$PWD/tools/java-elk-json-runner/bin/java-elk-json" cargo test -p elkrs-layered --test java_parity --locked -- --ignored`',
        "Complete for canonical connected adjacent-layer node spacing",
    ),
    "org.eclipse.elk.spacing.edgeNode": (
        "java-parity",
        '`cargo test -p elkrs-layered --test basic_layout --locked layered_layout_applies_edge_node_spacing_to_obstacle_detours` plus `ELKRS_JAVA_ELK_COMMAND="$PWD/tools/java-elk-json-runner/bin/java-elk-json" cargo test -p elkrs-layered --test java_parity --locked -- --ignored`',
        "Complete",
    ),
    "org.eclipse.elk.spacing.edgeEdge": (
        "semantic",
        "`cargo test -p elkrs-layered --test basic_layout --locked layered_layout_applies_edge_edge_spacing_to_parallel_routes`",
        "Parity: Java oracle fixture suite expansion",
    ),
    "org.eclipse.elk.edgeRouting": (
        "java-parity",
        '`cargo test -p elkrs-json --test json_partitions --locked edge_routing`, `cargo test -p elkrs-json --test json_errors --locked edge_routing`, plus `ELKRS_JAVA_ELK_COMMAND="$PWD/tools/java-elk-json-runner/bin/java-elk-json" cargo test -p elkrs-layered --test java_parity --locked -- --ignored`',
        "Complete for explicit orthogonal edge routing selection",
    ),
    "org.eclipse.elk.hierarchyHandling": (
        "diagnostic",
        "`cargo test -p elkrs-layered --test basic_layout --locked layered_layout_reports_unsupported_hierarchy_handling`",
        "Parity: compound, hierarchy, and non-plugin cluster behavior",
    ),
    "org.eclipse.elk.insideSelfLoops.activate": (
        "unsupported",
        "No typed inside self-loop option model yet",
        "Parity: edge routing variants, junctions, and merging",
    ),
    "org.eclipse.elk.insideSelfLoops.yo": (
        "unsupported",
        "No typed inside self-loop option model yet",
        "Parity: edge routing variants, junctions, and merging",
    ),
    "org.eclipse.elk.portConstraints": (
        "unsupported",
        "No public option model yet",
        "Parity: port constraints and ordering",
    ),
    "org.eclipse.elk.port.side": (
        "unsupported",
        "No public option model yet",
        "Parity: port constraints and ordering",
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
}


FEATURE_OVERRIDES = {
    "CLUSTERS": (
        "unsupported",
        "Generated from Java ELK v0.11.0 supported feature metadata; no Rust proof mapped yet",
        "Parity: compound, hierarchy, and non-plugin cluster behavior",
    ),
    "COMPOUND": (
        "java-parity",
        '`cargo test -p elkrs-layered --test quality --locked nested_group_fixture_has_contained_children` plus `ELKRS_JAVA_ELK_COMMAND="$PWD/tools/java-elk-json-runner/bin/java-elk-json" cargo test -p elkrs-layered --test java_parity --locked -- --ignored`',
        "Complete for nested containment semantics",
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
        "Inside self-loop option semantics are not represented in typed options yet",
        "Parity: edge routing variants, junctions, and merging",
    ),
    "MULTI_EDGES": (
        "java-parity",
        '`ELKRS_JAVA_ELK_COMMAND="$PWD/tools/java-elk-json-runner/bin/java-elk-json" cargo test -p elkrs-layered --test java_parity --locked -- --ignored`',
        "Complete",
    ),
    "PORTS": (
        "java-parity",
        '`cargo test -p elkrs-layered --test quality --locked port_heavy_fixture_preserves_port_anchor_fidelity` plus `ELKRS_JAVA_ELK_COMMAND="$PWD/tools/java-elk-json-runner/bin/java-elk-json" cargo test -p elkrs-layered --test java_parity --locked -- --ignored`',
        "Complete for explicit port endpoint anchors",
    ),
    "SELF_LOOPS": (
        "java-parity",
        '`ELKRS_JAVA_ELK_COMMAND="$PWD/tools/java-elk-json-runner/bin/java-elk-json" cargo test -p elkrs-layered --test java_parity --locked -- --ignored`',
        "Complete",
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
