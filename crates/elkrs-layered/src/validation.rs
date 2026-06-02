use elkrs_core::diagnostic::Diagnostic;
use elkrs_core::graph::{ElkGraph, ElkNode, ElkPort};
use elkrs_core::layout::LayoutError;
use elkrs_core::options::{
    Algorithm, ComponentOrderingStrategy, CoreOption, CrossingMinimizationStrategy, EdgeRouting,
    GreedySwitchType, GroupOrderingStrategy, HierarchyHandling, LayerConstraint,
    LongEdgeOrderingStrategy, ModelOrderStrategy, NodeLayeringStrategy, PortAlignment,
    PortConstraints, Properties, PropertyValue,
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
    (
        CoreOption::PortLabelsTreatAsGroup,
        "port labels treat as group",
    ),
];
const NODE_UNSUPPORTED_PORT_ALIGNMENT_OPTIONS: &[(CoreOption, &str)] = &[
    (CoreOption::PortAlignmentDefault, "port alignment default"),
    (CoreOption::PortAlignmentEast, "port alignment east"),
    (CoreOption::PortAlignmentNorth, "port alignment north"),
    (CoreOption::PortAlignmentSouth, "port alignment south"),
    (CoreOption::PortAlignmentWest, "port alignment west"),
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
    collect_unsupported_model_order_diagnostics(properties, None, &mut diagnostics);
    collect_unsupported_model_order_group_diagnostics(properties, None, &mut diagnostics);
    collect_unsupported_greedy_switch_diagnostics(properties, None, &mut diagnostics);
    collect_unsupported_crossing_minimization_control_diagnostics(
        properties,
        None,
        &mut diagnostics,
    );
    collect_unsupported_layer_assignment_diagnostics(properties, None, &mut diagnostics);
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

fn collect_unsupported_model_order_diagnostics(
    properties: &Properties,
    node_id: Option<&str>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    match properties.consider_model_order_components() {
        None | Some(ComponentOrderingStrategy::None) => {}
        Some(strategy) => diagnostics.push(unsupported_component_ordering_strategy_diagnostic(
            strategy, node_id,
        )),
    }

    match properties.consider_model_order_strategy() {
        None | Some(ModelOrderStrategy::None) => {}
        Some(strategy) => {
            diagnostics.push(unsupported_model_order_strategy_diagnostic(
                strategy, node_id,
            ));
        }
    }
}

fn unsupported_component_ordering_strategy_diagnostic(
    strategy: ComponentOrderingStrategy,
    node_id: Option<&str>,
) -> Diagnostic {
    let message = if let Some(node_id) = node_id {
        format!(
            "consider model order components {strategy:?} on node {node_id} is recognized but not implemented by elkrs-layered yet"
        )
    } else {
        format!(
            "consider model order components {strategy:?} is recognized but not implemented by elkrs-layered yet"
        )
    };
    Diagnostic::warning(UNSUPPORTED_OPTION_CODE, message)
}

fn unsupported_model_order_strategy_diagnostic(
    strategy: ModelOrderStrategy,
    node_id: Option<&str>,
) -> Diagnostic {
    let message = if let Some(node_id) = node_id {
        format!(
            "consider model order strategy {strategy:?} on node {node_id} is recognized but not implemented by elkrs-layered yet"
        )
    } else {
        format!(
            "consider model order strategy {strategy:?} is recognized but not implemented by elkrs-layered yet"
        )
    };
    Diagnostic::warning(UNSUPPORTED_OPTION_CODE, message)
}

fn collect_unsupported_model_order_group_diagnostics(
    properties: &Properties,
    node_id: Option<&str>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if let Some(influence) = properties.crossing_counter_node_influence() {
        diagnostics.push(unsupported_number_option_diagnostic(
            "crossing counter node influence",
            influence,
            node_id,
        ));
    }
    if let Some(influence) = properties.crossing_counter_port_influence() {
        diagnostics.push(unsupported_number_option_diagnostic(
            "crossing counter port influence",
            influence,
            node_id,
        ));
    }
    if let Some(strategy) = properties.cycle_breaking_group_order_strategy() {
        diagnostics.push(unsupported_group_ordering_strategy_diagnostic(
            "cycle breaking group order strategy",
            strategy,
            node_id,
        ));
    }
    if let Some(id) = properties.component_group_id() {
        diagnostics.push(unsupported_integer_option_diagnostic(
            "component group ID",
            id,
            node_id,
        ));
    }
    if let Some(id) = properties.cycle_breaking_preferred_source_id() {
        diagnostics.push(unsupported_integer_option_diagnostic(
            "cycle breaking preferred source ID",
            id,
            node_id,
        ));
    }
    if let Some(id) = properties.cycle_breaking_preferred_target_id() {
        diagnostics.push(unsupported_integer_option_diagnostic(
            "cycle breaking preferred target ID",
            id,
            node_id,
        ));
    }
    if let Some(id) = properties.crossing_minimization_id() {
        diagnostics.push(unsupported_integer_option_diagnostic(
            "crossing minimization ID",
            id,
            node_id,
        ));
    }
    if let Some(orders) = properties.crossing_minimization_enforced_group_orders() {
        diagnostics.push(unsupported_integer_list_option_diagnostic(
            "crossing minimization enforced group orders",
            orders,
            node_id,
        ));
    }
    if let Some(strategy) = properties.crossing_minimization_group_order_strategy() {
        diagnostics.push(unsupported_group_ordering_strategy_diagnostic(
            "crossing minimization group order strategy",
            strategy,
            node_id,
        ));
    }
    if let Some(id) = properties.cycle_breaking_id() {
        diagnostics.push(unsupported_integer_option_diagnostic(
            "cycle breaking ID",
            id,
            node_id,
        ));
    }
    if let Some(strategy) = properties.long_edge_ordering_strategy() {
        diagnostics.push(unsupported_long_edge_ordering_strategy_diagnostic(
            strategy, node_id,
        ));
    }
}

fn unsupported_number_option_diagnostic(
    name: &str,
    value: f64,
    node_id: Option<&str>,
) -> Diagnostic {
    let message = if let Some(node_id) = node_id {
        format!("{name} {value} on node {node_id} is recognized but not implemented by elkrs-layered yet")
    } else {
        format!("{name} {value} is recognized but not implemented by elkrs-layered yet")
    };
    Diagnostic::warning(UNSUPPORTED_OPTION_CODE, message)
}

fn unsupported_integer_option_diagnostic(
    name: &str,
    value: i64,
    node_id: Option<&str>,
) -> Diagnostic {
    let message = if let Some(node_id) = node_id {
        format!("{name} {value} on node {node_id} is recognized but not implemented by elkrs-layered yet")
    } else {
        format!("{name} {value} is recognized but not implemented by elkrs-layered yet")
    };
    Diagnostic::warning(UNSUPPORTED_OPTION_CODE, message)
}

fn unsupported_integer_list_option_diagnostic(
    name: &str,
    values: &[i64],
    node_id: Option<&str>,
) -> Diagnostic {
    let message = if let Some(node_id) = node_id {
        format!("{name} {values:?} on node {node_id} is recognized but not implemented by elkrs-layered yet")
    } else {
        format!("{name} {values:?} is recognized but not implemented by elkrs-layered yet")
    };
    Diagnostic::warning(UNSUPPORTED_OPTION_CODE, message)
}

fn unsupported_group_ordering_strategy_diagnostic(
    name: &str,
    strategy: GroupOrderingStrategy,
    node_id: Option<&str>,
) -> Diagnostic {
    let message = if let Some(node_id) = node_id {
        format!(
            "{name} {strategy:?} on node {node_id} is recognized but not implemented by elkrs-layered yet"
        )
    } else {
        format!("{name} {strategy:?} is recognized but not implemented by elkrs-layered yet")
    };
    Diagnostic::warning(UNSUPPORTED_OPTION_CODE, message)
}

fn unsupported_long_edge_ordering_strategy_diagnostic(
    strategy: LongEdgeOrderingStrategy,
    node_id: Option<&str>,
) -> Diagnostic {
    let message = if let Some(node_id) = node_id {
        format!(
            "long edge ordering strategy {strategy:?} on node {node_id} is recognized but not implemented by elkrs-layered yet"
        )
    } else {
        format!(
            "long edge ordering strategy {strategy:?} is recognized but not implemented by elkrs-layered yet"
        )
    };
    Diagnostic::warning(UNSUPPORTED_OPTION_CODE, message)
}

fn collect_unsupported_greedy_switch_diagnostics(
    properties: &Properties,
    node_id: Option<&str>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if let Some(threshold) = properties.greedy_switch_activation_threshold() {
        diagnostics.push(unsupported_greedy_switch_threshold_diagnostic(
            threshold, node_id,
        ));
    }

    match properties.greedy_switch_type() {
        None | Some(GreedySwitchType::Off) => {}
        Some(greedy_switch_type) => diagnostics.push(unsupported_greedy_switch_type_diagnostic(
            "greedy switch type",
            greedy_switch_type,
            node_id,
        )),
    }

    match properties.greedy_switch_hierarchical_type() {
        None | Some(GreedySwitchType::Off) => {}
        Some(greedy_switch_type) => diagnostics.push(unsupported_greedy_switch_type_diagnostic(
            "hierarchical greedy switch type",
            greedy_switch_type,
            node_id,
        )),
    }
}

fn unsupported_greedy_switch_threshold_diagnostic(
    threshold: i64,
    node_id: Option<&str>,
) -> Diagnostic {
    let message = if let Some(node_id) = node_id {
        format!(
            "greedy switch activation threshold {threshold} on node {node_id} is recognized but not implemented by elkrs-layered yet"
        )
    } else {
        format!(
            "greedy switch activation threshold {threshold} is recognized but not implemented by elkrs-layered yet"
        )
    };
    Diagnostic::warning(UNSUPPORTED_OPTION_CODE, message)
}

fn unsupported_greedy_switch_type_diagnostic(
    name: &str,
    greedy_switch_type: GreedySwitchType,
    node_id: Option<&str>,
) -> Diagnostic {
    let message = if let Some(node_id) = node_id {
        format!(
            "{name} {greedy_switch_type:?} on node {node_id} is recognized but not implemented by elkrs-layered yet"
        )
    } else {
        format!(
            "{name} {greedy_switch_type:?} is recognized but not implemented by elkrs-layered yet"
        )
    };
    Diagnostic::warning(UNSUPPORTED_OPTION_CODE, message)
}

fn collect_unsupported_crossing_minimization_control_diagnostics(
    properties: &Properties,
    node_id: Option<&str>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if let Some(sweepiness) = properties.crossing_minimization_hierarchical_sweepiness() {
        diagnostics.push(unsupported_number_option_diagnostic(
            "crossing minimization hierarchical sweepiness",
            sweepiness,
            node_id,
        ));
    }
    if let Some(strategy) = properties.crossing_minimization_strategy() {
        diagnostics.push(unsupported_crossing_minimization_strategy_diagnostic(
            strategy, node_id,
        ));
    }
    if let Some(id) = properties.crossing_minimization_in_layer_predecessor_of() {
        diagnostics.push(unsupported_text_option_diagnostic(
            "in-layer predecessor constraint",
            id,
            node_id,
        ));
    }
    if let Some(id) = properties.crossing_minimization_in_layer_successor_of() {
        diagnostics.push(unsupported_text_option_diagnostic(
            "in-layer successor constraint",
            id,
            node_id,
        ));
    }
    if let Some(constraint) = properties.crossing_minimization_position_choice_constraint() {
        diagnostics.push(unsupported_integer_option_diagnostic(
            "position choice constraint",
            constraint,
            node_id,
        ));
    }
    if let Some(id) = properties.crossing_minimization_position_id() {
        diagnostics.push(unsupported_integer_option_diagnostic(
            "position ID",
            id,
            node_id,
        ));
    }
    if let Some(thoroughness) = properties.thoroughness() {
        diagnostics.push(unsupported_integer_option_diagnostic(
            "thoroughness",
            thoroughness,
            node_id,
        ));
    }
    if let Some(seed) = properties.random_seed() {
        diagnostics.push(unsupported_integer_option_diagnostic(
            "random seed",
            seed,
            node_id,
        ));
    }
}

fn unsupported_crossing_minimization_strategy_diagnostic(
    strategy: CrossingMinimizationStrategy,
    node_id: Option<&str>,
) -> Diagnostic {
    let message = if let Some(node_id) = node_id {
        format!(
            "crossing minimization strategy {strategy:?} on node {node_id} is recognized but not implemented by elkrs-layered yet"
        )
    } else {
        format!(
            "crossing minimization strategy {strategy:?} is recognized but not implemented by elkrs-layered yet"
        )
    };
    Diagnostic::warning(UNSUPPORTED_OPTION_CODE, message)
}

fn unsupported_text_option_diagnostic(
    name: &str,
    value: &str,
    node_id: Option<&str>,
) -> Diagnostic {
    let message = if let Some(node_id) = node_id {
        format!(
            "{name} {value} on node {node_id} is recognized but not implemented by elkrs-layered yet"
        )
    } else {
        format!("{name} {value} is recognized but not implemented by elkrs-layered yet")
    };
    Diagnostic::warning(UNSUPPORTED_OPTION_CODE, message)
}

fn collect_unsupported_layer_assignment_diagnostics(
    properties: &Properties,
    node_id: Option<&str>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if let Some(strategy) = properties.layering_strategy() {
        diagnostics.push(unsupported_node_layering_strategy_diagnostic(
            strategy, node_id,
        ));
    }
    if let Some(bound) = properties.layer_bound() {
        diagnostics.push(unsupported_integer_option_diagnostic(
            "layer bound",
            bound,
            node_id,
        ));
    }
    if let Some(constraint) = properties.layer_choice_constraint() {
        diagnostics.push(unsupported_integer_option_diagnostic(
            "layer choice constraint",
            constraint,
            node_id,
        ));
    }
    if let Some(constraint) = properties.layer_constraint() {
        diagnostics.push(unsupported_layer_constraint_diagnostic(constraint, node_id));
    }
    if let Some(id) = properties.layer_id() {
        diagnostics.push(unsupported_integer_option_diagnostic(
            "layer ID", id, node_id,
        ));
    }
    if let Some(partition) = properties.layout_partition() {
        diagnostics.push(unsupported_integer_option_diagnostic(
            "layout partition",
            partition,
            node_id,
        ));
    }
}

fn unsupported_node_layering_strategy_diagnostic(
    strategy: NodeLayeringStrategy,
    node_id: Option<&str>,
) -> Diagnostic {
    let message = if let Some(node_id) = node_id {
        format!(
            "layering strategy {strategy:?} on node {node_id} is recognized but not implemented by elkrs-layered yet"
        )
    } else {
        format!(
            "layering strategy {strategy:?} is recognized but not implemented by elkrs-layered yet"
        )
    };
    Diagnostic::warning(UNSUPPORTED_OPTION_CODE, message)
}

fn unsupported_layer_constraint_diagnostic(
    constraint: LayerConstraint,
    node_id: Option<&str>,
) -> Diagnostic {
    let message = if let Some(node_id) = node_id {
        format!(
            "layer constraint {constraint:?} on node {node_id} is recognized but not implemented by elkrs-layered yet"
        )
    } else {
        format!(
            "layer constraint {constraint:?} is recognized but not implemented by elkrs-layered yet"
        )
    };
    Diagnostic::warning(UNSUPPORTED_OPTION_CODE, message)
}

fn collect_unsupported_port_alignment_diagnostics(
    properties: &Properties,
    node_id: Option<&str>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for (option, name) in NODE_UNSUPPORTED_PORT_ALIGNMENT_OPTIONS {
        if let Some(PropertyValue::PortAlignment(port_alignment)) = properties.get(*option) {
            diagnostics.push(unsupported_port_alignment_diagnostic(
                name,
                *port_alignment,
                node_id,
            ));
        }
    }
}

fn unsupported_port_alignment_diagnostic(
    name: &str,
    port_alignment: PortAlignment,
    node_id: Option<&str>,
) -> Diagnostic {
    let message = if let Some(node_id) = node_id {
        format!(
            "{name} {port_alignment:?} on node {node_id} is recognized but not implemented by elkrs-layered yet"
        )
    } else {
        format!("{name} {port_alignment:?} is recognized but not implemented by elkrs-layered yet")
    };
    Diagnostic::warning(UNSUPPORTED_OPTION_CODE, message)
}

fn collect_unsupported_port_scoped_option_diagnostics(
    port: &ElkPort,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if let Some(index) = port.properties.port_index() {
        diagnostics.push(unsupported_integer_port_option_diagnostic(
            "port index",
            index,
            port.id.as_str(),
        ));
    }
    if let Some(offset) = port.properties.port_border_offset() {
        diagnostics.push(unsupported_number_port_option_diagnostic(
            "port border offset",
            offset,
            port.id.as_str(),
        ));
    }
    if port.properties.allow_non_flow_ports_to_switch_sides() {
        diagnostics.push(unsupported_boolean_port_option_diagnostic(
            "allow non-flow ports to switch sides",
            port.id.as_str(),
        ));
    }
}

fn unsupported_integer_port_option_diagnostic(name: &str, value: i64, port_id: &str) -> Diagnostic {
    Diagnostic::warning(
        UNSUPPORTED_OPTION_CODE,
        format!(
            "{name} {value} on port {port_id} is recognized but not implemented by elkrs-layered yet"
        ),
    )
}

fn unsupported_number_port_option_diagnostic(name: &str, value: f64, port_id: &str) -> Diagnostic {
    Diagnostic::warning(
        UNSUPPORTED_OPTION_CODE,
        format!(
            "{name} {value} on port {port_id} is recognized but not implemented by elkrs-layered yet"
        ),
    )
}

fn unsupported_boolean_port_option_diagnostic(name: &str, port_id: &str) -> Diagnostic {
    Diagnostic::warning(
        UNSUPPORTED_OPTION_CODE,
        format!("{name} on port {port_id} is recognized but not implemented by elkrs-layered yet"),
    )
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
    collect_unsupported_port_alignment_diagnostics(
        &node.properties,
        Some(node.id.as_str()),
        diagnostics,
    );
    collect_unsupported_model_order_diagnostics(
        &node.properties,
        Some(node.id.as_str()),
        diagnostics,
    );
    collect_unsupported_model_order_group_diagnostics(
        &node.properties,
        Some(node.id.as_str()),
        diagnostics,
    );
    collect_unsupported_greedy_switch_diagnostics(
        &node.properties,
        Some(node.id.as_str()),
        diagnostics,
    );
    collect_unsupported_crossing_minimization_control_diagnostics(
        &node.properties,
        Some(node.id.as_str()),
        diagnostics,
    );
    collect_unsupported_layer_assignment_diagnostics(
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
    match node.properties.port_constraints() {
        PortConstraints::Undefined | PortConstraints::Free => {}
        port_constraints => diagnostics.push(unsupported_port_constraints_diagnostic(
            port_constraints,
            Some(node.id.as_str()),
        )),
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
    for port in node.ports.values() {
        collect_unsupported_port_scoped_option_diagnostics(port, diagnostics);
    }
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

fn unsupported_port_constraints_diagnostic(
    port_constraints: PortConstraints,
    node_id: Option<&str>,
) -> Diagnostic {
    let message = if let Some(node_id) = node_id {
        format!(
            "port constraints {port_constraints:?} on node {node_id} are recognized but not implemented by elkrs-layered yet"
        )
    } else {
        format!(
            "port constraints {port_constraints:?} are recognized but not implemented by elkrs-layered yet"
        )
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
