use elkrs_core::diagnostic::Diagnostic;
use elkrs_core::graph::{ElkGraph, ElkNode};
use elkrs_core::layout::LayoutError;
use elkrs_core::options::{
    Algorithm, CoreOption, EdgeRouting, HierarchyHandling, Properties, PropertyValue,
};

const UNSUPPORTED_OPTION_CODE: &str = "ELKRS_LAYERED_UNSUPPORTED_OPTION";
const UNSUPPORTED_BOOLEAN_OPTIONS: &[(CoreOption, &str)] = &[
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
        CoreOption::SemiInteractiveCrossingMinimization,
        "semi-interactive crossing minimization",
    ),
    (CoreOption::TopdownLayout, "topdown layout"),
    (CoreOption::UnnecessaryBendpoints, "unnecessary bendpoints"),
];

pub(crate) fn validate_options(graph: &ElkGraph) -> Result<Vec<Diagnostic>, LayoutError> {
    let mut diagnostics = validate_graph_properties(&graph.properties)?;
    for node in graph.nodes.values() {
        collect_node_hierarchy_diagnostics(node, &mut diagnostics);
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
    collect_unsupported_boolean_option_diagnostics(properties, None, &mut diagnostics);
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
    diagnostics: &mut Vec<Diagnostic>,
) {
    for (option, name) in UNSUPPORTED_BOOLEAN_OPTIONS {
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

fn collect_node_hierarchy_diagnostics(node: &ElkNode, diagnostics: &mut Vec<Diagnostic>) {
    collect_unsupported_boolean_option_diagnostics(
        &node.properties,
        Some(node.id.as_str()),
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
    for child in node.children.values() {
        collect_node_hierarchy_diagnostics(child, diagnostics);
    }
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
        _ => None,
    }
}
