use elkrs_core::diagnostic::Diagnostic;
use elkrs_core::graph::{ElkGraph, ElkNode};
use elkrs_core::layout::LayoutError;
use elkrs_core::options::{
    Algorithm, CoreOption, EdgeRouting, HierarchyHandling, Properties, PropertyValue,
};

const UNSUPPORTED_OPTION_CODE: &str = "ELKRS_LAYERED_UNSUPPORTED_OPTION";
const PARENT_UNSUPPORTED_BOOLEAN_OPTIONS: &[(CoreOption, &str)] = &[
    (
        CoreOption::ConnectedComponentsCompaction,
        "connected components compaction",
    ),
    (CoreOption::ConsiderPortOrder, "consider port order"),
    (CoreOption::DebugMode, "debug mode"),
    (CoreOption::FeedbackEdges, "feedback edges"),
    (CoreOption::FavorStraightEdges, "favor straight edges"),
    (CoreOption::FixedGraphSize, "fixed graph size"),
    (CoreOption::ForceNodeModelOrder, "force node model order"),
    (
        CoreOption::GeneratePositionAndLayerIds,
        "generate position and layer IDs",
    ),
    (
        CoreOption::HighDegreeNodeTreatment,
        "high degree node treatment",
    ),
    (CoreOption::InteractiveLayout, "interactive layout"),
    (CoreOption::LayoutPartitioning, "layout partitioning"),
    (CoreOption::MergeEdges, "merge edges"),
    (
        CoreOption::MergeHierarchyEdges,
        "merge hierarchy-crossing edges",
    ),
    (
        CoreOption::SeparateConnectedComponents,
        "separate connected components",
    ),
    (
        CoreOption::SemiInteractiveCrossingMinimization,
        "semi-interactive crossing minimization",
    ),
    (CoreOption::TopdownLayout, "topdown layout"),
    (CoreOption::UnnecessaryBendpoints, "unnecessary bendpoints"),
    (CoreOption::WrappingImproveCuts, "improve cuts"),
    (
        CoreOption::WrappingImproveWrappedEdges,
        "improve wrapped edges",
    ),
];
const NODE_UNSUPPORTED_BOOLEAN_OPTIONS: &[(CoreOption, &str)] = &[
    (CoreOption::CommentBox, "comment box"),
    (CoreOption::Hypernode, "hypernode"),
    (CoreOption::InsideSelfLoops, "inside self-loops"),
    (CoreOption::NoModelOrder, "no model order"),
    (CoreOption::NoLayout, "no layout"),
    (
        CoreOption::LayerUnzippingMinimizeEdgeLength,
        "layer unzipping minimize edge length",
    ),
    (
        CoreOption::LayerUnzippingResetOnLongEdges,
        "layer unzipping reset on long edges",
    ),
    (
        CoreOption::PortLabelsNextToPortIfPossible,
        "port labels next to port if possible",
    ),
];

pub(crate) fn validate_options(graph: &ElkGraph) -> Result<Vec<Diagnostic>, LayoutError> {
    let mut diagnostics = validate_graph_properties(&graph.properties)?;
    for node in graph.nodes.values() {
        collect_node_hierarchy_diagnostics(node, &mut diagnostics)?;
    }
    Ok(diagnostics)
}

fn validate_graph_properties(properties: &Properties) -> Result<Vec<Diagnostic>, LayoutError> {
    match properties.algorithm() {
        Some(Algorithm::Layered) | None => {}
        Some(Algorithm::Other(algorithm)) => {
            return Err(LayoutError::UnsupportedAlgorithm(algorithm));
        }
    }

    let mut diagnostics = Vec::new();
    collect_unsupported_boolean_option_diagnostics(
        properties,
        None,
        PARENT_UNSUPPORTED_BOOLEAN_OPTIONS,
        &mut diagnostics,
    );
    match properties.edge_routing() {
        EdgeRouting::Orthogonal => {}
        edge_routing => diagnostics.push(unsupported_edge_routing_diagnostic(edge_routing, None)),
    }
    if matches!(
        properties.hierarchy_handling(),
        HierarchyHandling::SeparateChildren
    ) {
        diagnostics.push(unsupported_hierarchy_handling_diagnostic(None));
    }
    validate_non_negative_spacing(properties, CoreOption::SpacingEdgeNode, "edge-node spacing")?;
    validate_non_negative_spacing(properties, CoreOption::SpacingEdgeEdge, "edge-edge spacing")?;
    validate_non_negative_spacing(
        properties,
        CoreOption::SpacingBaseValue,
        "spacing base value",
    )?;
    validate_non_negative_spacing(
        properties,
        CoreOption::WrappingAdditionalEdgeSpacing,
        "additional edge wrapping spacing",
    )?;
    validate_non_negative_spacing(
        properties,
        CoreOption::SpacingCommentComment,
        "comment-comment spacing",
    )?;
    validate_non_negative_spacing(
        properties,
        CoreOption::SpacingCommentNode,
        "comment-node spacing",
    )?;
    validate_non_negative_spacing(
        properties,
        CoreOption::SpacingComponentComponent,
        "component-component spacing",
    )?;
    validate_non_negative_spacing(
        properties,
        CoreOption::SpacingEdgeNodeBetweenLayers,
        "edge-node between-layers spacing",
    )?;
    validate_non_negative_spacing(
        properties,
        CoreOption::SpacingEdgeEdgeBetweenLayers,
        "edge-edge between-layers spacing",
    )?;
    validate_non_negative_spacing(
        properties,
        CoreOption::SpacingEdgeLabel,
        "edge-label spacing",
    )?;
    validate_non_negative_spacing(
        properties,
        CoreOption::SpacingLabelLabel,
        "label-label spacing",
    )?;
    validate_non_negative_spacing(
        properties,
        CoreOption::SpacingLabelNode,
        "label-node spacing",
    )?;
    validate_non_negative_spacing(
        properties,
        CoreOption::SpacingLabelPortHorizontal,
        "horizontal label-port spacing",
    )?;
    validate_non_negative_spacing(
        properties,
        CoreOption::SpacingLabelPortVertical,
        "vertical label-port spacing",
    )?;
    validate_non_negative_spacing(
        properties,
        CoreOption::SpacingNodeSelfLoop,
        "node self-loop spacing",
    )?;
    validate_non_negative_spacing(properties, CoreOption::SpacingPortPort, "port-port spacing")?;

    Ok(diagnostics)
}

fn unsupported_edge_routing_diagnostic(
    edge_routing: EdgeRouting,
    node_id: Option<&str>,
) -> Diagnostic {
    let message = if let Some(node_id) = node_id {
        format!(
            "edge routing {edge_routing:?} on node {node_id} is recognized but not implemented by elkrs-layered yet; using orthogonal routing"
        )
    } else {
        format!(
            "edge routing {edge_routing:?} is recognized but not implemented by elkrs-layered yet; using orthogonal routing"
        )
    };
    Diagnostic::warning(UNSUPPORTED_OPTION_CODE, message)
}

fn collect_unsupported_boolean_option_diagnostics(
    properties: &Properties,
    node_id: Option<&str>,
    options: &[(CoreOption, &str)],
    diagnostics: &mut Vec<Diagnostic>,
) {
    for (option, name) in options {
        if matches!(properties.get(*option), Some(PropertyValue::Bool(true))) {
            diagnostics.push(unsupported_boolean_option_diagnostic(name, node_id));
        }
    }
}

fn unsupported_boolean_option_diagnostic(name: &str, node_id: Option<&str>) -> Diagnostic {
    let message = if let Some(node_id) = node_id {
        format!(
            "layout option {name} on node {node_id} is recognized but not implemented by elkrs-layered yet"
        )
    } else {
        format!("layout option {name} is recognized but not implemented by elkrs-layered yet")
    };
    Diagnostic::warning(UNSUPPORTED_OPTION_CODE, message)
}

fn collect_node_hierarchy_diagnostics(
    node: &ElkNode,
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<(), LayoutError> {
    collect_unsupported_boolean_option_diagnostics(
        &node.properties,
        Some(node.id.as_str()),
        PARENT_UNSUPPORTED_BOOLEAN_OPTIONS,
        diagnostics,
    );
    collect_unsupported_boolean_option_diagnostics(
        &node.properties,
        Some(node.id.as_str()),
        NODE_UNSUPPORTED_BOOLEAN_OPTIONS,
        diagnostics,
    );
    match node.properties.edge_routing() {
        EdgeRouting::Orthogonal => {}
        edge_routing => {
            diagnostics.push(unsupported_edge_routing_diagnostic(
                edge_routing,
                Some(node.id.as_str()),
            ));
        }
    }
    if matches!(
        node.properties.hierarchy_handling(),
        HierarchyHandling::SeparateChildren
    ) {
        diagnostics.push(unsupported_hierarchy_handling_diagnostic(Some(
            node.id.as_str(),
        )));
    }
    validate_non_negative_spacing(
        &node.properties,
        CoreOption::SpacingPortPort,
        &format!("port-port spacing on node {}", node.id.as_str()),
    )?;
    for child in node.children.values() {
        collect_node_hierarchy_diagnostics(child, diagnostics)?;
    }
    Ok(())
}

fn unsupported_hierarchy_handling_diagnostic(node_id: Option<&str>) -> Diagnostic {
    let message = if let Some(node_id) = node_id {
        format!(
            "hierarchy handling SeparateChildren on node {node_id} is recognized but not implemented by elkrs-layered yet; laying out children with the current graph"
        )
    } else {
        "hierarchy handling SeparateChildren is recognized but not implemented by elkrs-layered yet; laying out children with the current graph".to_owned()
    };
    Diagnostic::warning(UNSUPPORTED_OPTION_CODE, message)
}

fn validate_non_negative_spacing(
    properties: &Properties,
    option: CoreOption,
    name: &str,
) -> Result<(), LayoutError> {
    let Some(spacing) = spacing_value(properties, option) else {
        return Ok(());
    };
    if spacing.is_finite() && spacing >= 0.0 {
        return Ok(());
    }

    Err(LayoutError::InvalidOption(format!(
        "{name} must be finite and non-negative"
    )))
}

fn spacing_value(properties: &Properties, option: CoreOption) -> Option<f64> {
    match option {
        CoreOption::SpacingEdgeNode => Some(properties.spacing_edge_node()),
        CoreOption::SpacingEdgeEdge => Some(properties.spacing_edge_edge()),
        CoreOption::SpacingBaseValue => stored_number(properties, option, "spacing base value"),
        CoreOption::WrappingAdditionalEdgeSpacing => {
            stored_number(properties, option, "additional edge wrapping spacing")
        }
        CoreOption::SpacingCommentComment => {
            stored_number(properties, option, "comment-comment spacing")
        }
        CoreOption::SpacingCommentNode => stored_number(properties, option, "comment-node spacing"),
        CoreOption::SpacingComponentComponent => {
            stored_number(properties, option, "component-component spacing")
        }
        CoreOption::SpacingEdgeNodeBetweenLayers => match properties.get(option) {
            Some(PropertyValue::Number(spacing)) => Some(*spacing),
            Some(value) => unreachable!(
                "edge-node between-layers spacing stored incompatible value: {value:?}"
            ),
            _ => None,
        },
        CoreOption::SpacingEdgeEdgeBetweenLayers => match properties.get(option) {
            Some(PropertyValue::Number(spacing)) => Some(*spacing),
            Some(value) => unreachable!(
                "edge-edge between-layers spacing stored incompatible value: {value:?}"
            ),
            _ => None,
        },
        CoreOption::SpacingEdgeLabel => stored_number(properties, option, "edge-label spacing"),
        CoreOption::SpacingLabelLabel => stored_number(properties, option, "label-label spacing"),
        CoreOption::SpacingLabelNode => stored_number(properties, option, "label-node spacing"),
        CoreOption::SpacingLabelPortHorizontal => {
            stored_number(properties, option, "horizontal label-port spacing")
        }
        CoreOption::SpacingLabelPortVertical => {
            stored_number(properties, option, "vertical label-port spacing")
        }
        CoreOption::SpacingNodeSelfLoop => {
            stored_number(properties, option, "node self-loop spacing")
        }
        CoreOption::SpacingPortPort => stored_number(properties, option, "port-port spacing"),
        _ => None,
    }
}

fn stored_number(properties: &Properties, option: CoreOption, name: &str) -> Option<f64> {
    match properties.get(option) {
        Some(PropertyValue::Number(spacing)) => Some(*spacing),
        Some(value) => unreachable!("{name} stored incompatible value: {value:?}"),
        _ => None,
    }
}
