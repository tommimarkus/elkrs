use elkrs_core::diagnostic::Diagnostic;
use elkrs_core::graph::{ElkGraph, ElkNode};
use elkrs_core::layout::LayoutError;
use elkrs_core::options::{Algorithm, CoreOption, EdgeRouting, HierarchyHandling, Properties};

const UNSUPPORTED_OPTION_CODE: &str = "ELKRS_LAYERED_UNSUPPORTED_OPTION";

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
    if properties.debug_mode() {
        diagnostics.push(unsupported_debug_mode_diagnostic(None));
    }
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

fn unsupported_debug_mode_diagnostic(node_id: Option<&str>) -> Diagnostic {
    let message = if let Some(node_id) = node_id {
        format!(
            "debug mode on node {node_id} is recognized but not implemented by elkrs-layered yet"
        )
    } else {
        "debug mode is recognized but not implemented by elkrs-layered yet".to_owned()
    };
    Diagnostic::warning(UNSUPPORTED_OPTION_CODE, message)
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

fn collect_node_hierarchy_diagnostics(node: &ElkNode, diagnostics: &mut Vec<Diagnostic>) {
    if node.properties.debug_mode() {
        diagnostics.push(unsupported_debug_mode_diagnostic(Some(node.id.as_str())));
    }
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
