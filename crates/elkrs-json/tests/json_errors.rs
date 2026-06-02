use elkrs_json::{from_str, JsonError};

#[test]
fn malformed_json_returns_json_error() {
    let error = from_str("{").unwrap_err();

    assert!(matches!(error, JsonError::Json(_)));
}

#[test]
fn unknown_edge_endpoint_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "children": [{ "id": "source" }],
          "edges": [{ "id": "edge", "sources": ["source"], "targets": ["missing"] }]
        }"#,
        "unknown endpoint id: missing",
    );
}

#[test]
fn ambiguous_port_endpoint_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "children": [
            { "id": "left", "ports": [{ "id": "p" }] },
            { "id": "right", "ports": [{ "id": "p" }] },
            { "id": "target" }
          ],
          "edges": [{ "id": "edge", "sources": ["p"], "targets": ["target"] }]
        }"#,
        "ambiguous port endpoint id: p",
    );
}

#[test]
fn edge_with_multiple_sources_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "children": [{ "id": "a" }, { "id": "b" }, { "id": "target" }],
          "edges": [{ "id": "edge", "sources": ["a", "b"], "targets": ["target"] }]
        }"#,
        "edge sources must contain exactly one endpoint",
    );
}

#[test]
fn edge_with_multiple_targets_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "children": [{ "id": "source" }, { "id": "a" }, { "id": "b" }],
          "edges": [{ "id": "edge", "sources": ["source"], "targets": ["a", "b"] }]
        }"#,
        "edge targets must contain exactly one endpoint",
    );
}

#[test]
fn unsupported_direction_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "layoutOptions": { "org.eclipse.elk.direction": "SIDEWAYS" }
        }"#,
        "unsupported org.eclipse.elk.direction value: SIDEWAYS",
    );
}

#[test]
fn non_string_direction_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "layoutOptions": { "org.eclipse.elk.direction": 7 }
        }"#,
        "org.eclipse.elk.direction must be a string",
    );
}

#[test]
fn non_string_algorithm_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "layoutOptions": { "org.eclipse.elk.algorithm": 7 }
        }"#,
        "org.eclipse.elk.algorithm must be a string",
    );
}

#[test]
fn non_bool_debug_mode_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "layoutOptions": { "org.eclipse.elk.debugMode": "true" }
        }"#,
        "org.eclipse.elk.debugMode must be a boolean",
    );
}

#[test]
fn non_bool_node_debug_mode_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "children": [
            {
              "id": "node",
              "layoutOptions": { "org.eclipse.elk.debugMode": "true" }
            }
          ]
        }"#,
        "org.eclipse.elk.debugMode must be a boolean",
    );
}

#[test]
fn non_bool_feedback_edges_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "layoutOptions": { "org.eclipse.elk.layered.feedbackEdges": "true" }
        }"#,
        "org.eclipse.elk.layered.feedbackEdges must be a boolean",
    );
}

#[test]
fn non_bool_node_feedback_edges_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "children": [
            {
              "id": "node",
              "layoutOptions": { "org.eclipse.elk.layered.feedbackEdges": "true" }
            }
          ]
        }"#,
        "org.eclipse.elk.layered.feedbackEdges must be a boolean",
    );
}

#[test]
fn non_bool_parent_boolean_layout_options_return_invalid_errors() {
    for key in parent_boolean_option_keys() {
        let json = format!(
            r#"{{
              "id": "root",
              "layoutOptions": {{ "{key}": "true" }}
            }}"#
        );
        assert_invalid_contains(&json, &format!("{key} must be a boolean"));
    }
}

#[test]
fn non_bool_node_parent_boolean_layout_options_return_invalid_errors() {
    for key in parent_boolean_option_keys() {
        let json = format!(
            r#"{{
              "id": "root",
              "children": [
                {{
                  "id": "node",
                  "layoutOptions": {{ "{key}": "true" }}
                }}
              ]
            }}"#
        );
        assert_invalid_contains(&json, &format!("{key} must be a boolean"));
    }
}

#[test]
fn non_bool_node_boolean_layout_options_return_invalid_errors() {
    for key in node_boolean_option_keys() {
        let json = format!(
            r#"{{
              "id": "root",
              "children": [
                {{
                  "id": "node",
                  "layoutOptions": {{ "{key}": "true" }}
                }}
              ]
            }}"#
        );
        assert_invalid_contains(&json, &format!("{key} must be a boolean"));
    }
}

#[test]
fn unsupported_edge_routing_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "layoutOptions": { "org.eclipse.elk.edgeRouting": "BEZIER" }
        }"#,
        "unsupported org.eclipse.elk.edgeRouting value: BEZIER",
    );
}

#[test]
fn non_string_edge_routing_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "layoutOptions": { "org.eclipse.elk.edgeRouting": 7 }
        }"#,
        "org.eclipse.elk.edgeRouting must be a string",
    );
}

#[test]
fn unsupported_node_edge_routing_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "children": [
            {
              "id": "node",
              "layoutOptions": { "org.eclipse.elk.edgeRouting": "BEZIER" }
            }
          ]
        }"#,
        "unsupported org.eclipse.elk.edgeRouting value: BEZIER",
    );
}

#[test]
fn non_string_node_edge_routing_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "children": [
            {
              "id": "node",
              "layoutOptions": { "org.eclipse.elk.edgeRouting": 7 }
            }
          ]
        }"#,
        "org.eclipse.elk.edgeRouting must be a string",
    );
}

#[test]
fn unsupported_hierarchy_handling_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "layoutOptions": { "org.eclipse.elk.hierarchyHandling": "FLATTEN" }
        }"#,
        "unsupported org.eclipse.elk.hierarchyHandling value: FLATTEN",
    );
}

#[test]
fn non_string_hierarchy_handling_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "layoutOptions": { "org.eclipse.elk.hierarchyHandling": 7 }
        }"#,
        "org.eclipse.elk.hierarchyHandling must be a string",
    );
}

#[test]
fn unsupported_node_hierarchy_handling_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "children": [
            {
              "id": "node",
              "layoutOptions": { "org.eclipse.elk.hierarchyHandling": "FLATTEN" }
            }
          ]
        }"#,
        "unsupported org.eclipse.elk.hierarchyHandling value: FLATTEN",
    );
}

#[test]
fn non_string_node_hierarchy_handling_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "children": [
            {
              "id": "node",
              "layoutOptions": { "org.eclipse.elk.hierarchyHandling": 7 }
            }
          ]
        }"#,
        "org.eclipse.elk.hierarchyHandling must be a string",
    );
}

#[test]
fn unsupported_model_order_components_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "layoutOptions": {
            "org.eclipse.elk.layered.considerModelOrder.components": "SORTED"
          }
        }"#,
        "unsupported org.eclipse.elk.layered.considerModelOrder.components value: SORTED",
    );
}

#[test]
fn non_string_model_order_strategy_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "layoutOptions": {
            "org.eclipse.elk.layered.considerModelOrder.strategy": 7
          }
        }"#,
        "org.eclipse.elk.layered.considerModelOrder.strategy must be a string",
    );
}

#[test]
fn unsupported_node_model_order_strategy_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "children": [
            {
              "id": "node",
              "layoutOptions": {
                "org.eclipse.elk.layered.considerModelOrder.strategy": "SORTED"
              }
            }
          ]
        }"#,
        "unsupported org.eclipse.elk.layered.considerModelOrder.strategy value: SORTED",
    );
}

#[test]
fn non_string_node_model_order_components_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "children": [
            {
              "id": "node",
              "layoutOptions": {
                "org.eclipse.elk.layered.considerModelOrder.components": 7
              }
            }
          ]
        }"#,
        "org.eclipse.elk.layered.considerModelOrder.components must be a string",
    );
}

#[test]
fn negative_greedy_switch_activation_threshold_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "layoutOptions": {
            "org.eclipse.elk.layered.crossingMinimization.greedySwitch.activationThreshold": -1
          }
        }"#,
        "org.eclipse.elk.layered.crossingMinimization.greedySwitch.activationThreshold must be non-negative",
    );
}

#[test]
fn non_integer_greedy_switch_activation_threshold_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "layoutOptions": {
            "org.eclipse.elk.layered.crossingMinimization.greedySwitch.activationThreshold": 1.5
          }
        }"#,
        "org.eclipse.elk.layered.crossingMinimization.greedySwitch.activationThreshold must be an integer",
    );
}

#[test]
fn unsupported_greedy_switch_type_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "layoutOptions": {
            "org.eclipse.elk.layered.crossingMinimization.greedySwitch.type": "AUTO"
          }
        }"#,
        "unsupported org.eclipse.elk.layered.crossingMinimization.greedySwitch.type value: AUTO",
    );
}

#[test]
fn crossing_minimization_controls_non_number_sweepiness_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "layoutOptions": {
            "org.eclipse.elk.layered.crossingMinimization.hierarchicalSweepiness": false
          }
        }"#,
        "org.eclipse.elk.layered.crossingMinimization.hierarchicalSweepiness must be a number",
    );
}

#[test]
fn crossing_minimization_controls_unsupported_strategy_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "layoutOptions": {
            "org.eclipse.elk.layered.crossingMinimization.strategy": "SIFT"
          }
        }"#,
        "unsupported org.eclipse.elk.layered.crossingMinimization.strategy value: SIFT",
    );
}

#[test]
fn crossing_minimization_controls_non_string_node_in_layer_predecessor_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "children": [
            {
              "id": "node",
              "layoutOptions": {
                "org.eclipse.elk.layered.crossingMinimization.inLayerPredOf": 1
              }
            }
          ]
        }"#,
        "org.eclipse.elk.layered.crossingMinimization.inLayerPredOf must be a string",
    );
}

#[test]
fn crossing_minimization_controls_non_integer_node_position_id_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "children": [
            {
              "id": "node",
              "layoutOptions": {
                "org.eclipse.elk.layered.crossingMinimization.positionId": true
              }
            }
          ]
        }"#,
        "org.eclipse.elk.layered.crossingMinimization.positionId must be an integer",
    );
}

#[test]
fn non_string_node_greedy_switch_hierarchical_type_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "children": [
            {
              "id": "node",
              "layoutOptions": {
                "org.eclipse.elk.layered.crossingMinimization.greedySwitchHierarchical.type": 7
              }
            }
          ]
        }"#,
        "org.eclipse.elk.layered.crossingMinimization.greedySwitchHierarchical.type must be a string",
    );
}

#[test]
fn model_order_group_non_number_crossing_counter_node_influence_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "layoutOptions": {
            "org.eclipse.elk.layered.considerModelOrder.crossingCounterNodeInfluence": true
          }
        }"#,
        "org.eclipse.elk.layered.considerModelOrder.crossingCounterNodeInfluence must be a number",
    );
}

#[test]
fn model_order_group_non_integer_cycle_breaking_preferred_source_id_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "layoutOptions": {
            "org.eclipse.elk.layered.considerModelOrder.groupModelOrder.cbPreferredSourceId": 1.5
          }
        }"#,
        "org.eclipse.elk.layered.considerModelOrder.groupModelOrder.cbPreferredSourceId must be an integer",
    );
}

#[test]
fn model_order_group_unsupported_cycle_breaking_group_order_strategy_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "layoutOptions": {
            "org.eclipse.elk.layered.considerModelOrder.groupModelOrder.cbGroupOrderStrategy": "AUTO"
          }
        }"#,
        "unsupported org.eclipse.elk.layered.considerModelOrder.groupModelOrder.cbGroupOrderStrategy value: AUTO",
    );
}

#[test]
fn model_order_enforced_group_orders_non_array_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "layoutOptions": {
            "org.eclipse.elk.layered.considerModelOrder.groupModelOrder.cmEnforcedGroupOrders": 7
          }
        }"#,
        "org.eclipse.elk.layered.considerModelOrder.groupModelOrder.cmEnforcedGroupOrders must be an integer array",
    );
}

#[test]
fn model_order_enforced_group_orders_non_integer_member_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "children": [
            {
              "id": "node",
              "layoutOptions": {
                "org.eclipse.elk.layered.considerModelOrder.groupModelOrder.cmEnforcedGroupOrders": [1, true]
              }
            }
          ]
        }"#,
        "org.eclipse.elk.layered.considerModelOrder.groupModelOrder.cmEnforcedGroupOrders must contain only integers",
    );
}

#[test]
fn model_order_group_non_string_node_long_edge_strategy_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "children": [
            {
              "id": "node",
              "layoutOptions": {
                "org.eclipse.elk.layered.considerModelOrder.longEdgeStrategy": 7
              }
            }
          ]
        }"#,
        "org.eclipse.elk.layered.considerModelOrder.longEdgeStrategy must be a string",
    );
}

#[test]
fn model_order_group_id_non_integer_node_component_group_id_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "children": [
            {
              "id": "node",
              "layoutOptions": {
                "org.eclipse.elk.layered.considerModelOrder.groupModelOrder.componentGroupId": 1.5
              }
            }
          ]
        }"#,
        "org.eclipse.elk.layered.considerModelOrder.groupModelOrder.componentGroupId must be an integer",
    );
}

#[test]
fn model_order_group_id_non_integer_node_crossing_minimization_id_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "children": [
            {
              "id": "node",
              "layoutOptions": {
                "org.eclipse.elk.layered.considerModelOrder.groupModelOrder.crossingMinimizationId": true
              }
            }
          ]
        }"#,
        "org.eclipse.elk.layered.considerModelOrder.groupModelOrder.crossingMinimizationId must be an integer",
    );
}

#[test]
fn unsupported_node_port_constraints_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "children": [
            {
              "id": "node",
              "layoutOptions": { "org.eclipse.elk.portConstraints": "FLOATING" }
            }
          ]
        }"#,
        "unsupported org.eclipse.elk.portConstraints value: FLOATING",
    );
}

#[test]
fn non_string_node_port_constraints_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "children": [
            {
              "id": "node",
              "layoutOptions": { "org.eclipse.elk.portConstraints": 7 }
            }
          ]
        }"#,
        "org.eclipse.elk.portConstraints must be a string",
    );
}

#[test]
fn unsupported_node_port_alignment_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "children": [
            {
              "id": "node",
              "layoutOptions": { "org.eclipse.elk.portAlignment.default": "FLOATING" }
            }
          ]
        }"#,
        "unsupported org.eclipse.elk.portAlignment.default value: FLOATING",
    );
}

#[test]
fn non_string_node_port_alignment_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "children": [
            {
              "id": "node",
              "layoutOptions": { "org.eclipse.elk.portAlignment.east": 7 }
            }
          ]
        }"#,
        "org.eclipse.elk.portAlignment.east must be a string",
    );
}

#[test]
fn non_number_node_spacing_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "layoutOptions": { "org.eclipse.elk.spacing.nodeNode": "wide" }
        }"#,
        "org.eclipse.elk.spacing.nodeNode must be a number",
    );
}

#[test]
fn non_number_layer_spacing_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "layoutOptions": { "elk.spacing.layerNodeNode": false }
        }"#,
        "elk.spacing.layerNodeNode must be a number",
    );
}

#[test]
fn non_finite_string_spacing_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "layoutOptions": { "org.eclipse.elk.layered.spacing.nodeNodeBetweenLayers": "NaN" }
        }"#,
        "org.eclipse.elk.layered.spacing.nodeNodeBetweenLayers must be a number",
    );
}

#[test]
fn non_number_edge_spacing_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "layoutOptions": { "org.eclipse.elk.spacing.edgeNode": "wide" }
        }"#,
        "org.eclipse.elk.spacing.edgeNode must be a number",
    );
}

#[test]
fn negative_edge_spacing_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "layoutOptions": { "org.eclipse.elk.spacing.edgeEdge": -1 }
        }"#,
        "org.eclipse.elk.spacing.edgeEdge must be non-negative",
    );
}

#[test]
fn negative_layered_edge_spacing_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "layoutOptions": { "org.eclipse.elk.layered.spacing.edgeNodeBetweenLayers": -1 }
        }"#,
        "org.eclipse.elk.layered.spacing.edgeNodeBetweenLayers must be non-negative",
    );
}

#[test]
fn negative_additional_spacing_returns_invalid_error() {
    for key in [
        "org.eclipse.elk.layered.spacing.baseValue",
        "org.eclipse.elk.layered.wrapping.additionalEdgeSpacing",
        "org.eclipse.elk.spacing.commentComment",
        "org.eclipse.elk.spacing.commentNode",
        "org.eclipse.elk.spacing.componentComponent",
        "org.eclipse.elk.spacing.nodeSelfLoop",
    ] {
        assert_invalid_contains(
            &format!(
                r#"{{
                  "id": "root",
                  "layoutOptions": {{ "{key}": -1 }}
                }}"#
            ),
            &format!("{key} must be non-negative"),
        );
    }
}

#[test]
fn negative_label_and_port_spacing_returns_invalid_error() {
    for key in [
        "org.eclipse.elk.spacing.edgeLabel",
        "org.eclipse.elk.spacing.labelLabel",
        "org.eclipse.elk.spacing.labelNode",
        "org.eclipse.elk.spacing.labelPortHorizontal",
        "org.eclipse.elk.spacing.labelPortVertical",
        "org.eclipse.elk.spacing.portPort",
    ] {
        assert_invalid_contains(
            &format!(
                r#"{{
                  "id": "root",
                  "layoutOptions": {{ "{key}": -1 }}
                }}"#
            ),
            &format!("{key} must be non-negative"),
        );
    }
}

#[test]
fn negative_node_port_spacing_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "children": [
            {
              "id": "node",
              "layoutOptions": { "org.eclipse.elk.spacing.portPort": -1 }
            }
          ]
        }"#,
        "org.eclipse.elk.spacing.portPort must be non-negative",
    );
}

#[test]
fn unsupported_port_side_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "children": [
            { "id": "source", "ports": [{ "id": "out", "side": "DIAGONAL" }] }
          ]
        }"#,
        "unsupported side value: DIAGONAL",
    );
}

#[test]
fn unsupported_port_side_layout_option_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "children": [
            {
              "id": "source",
              "ports": [
                {
                  "id": "out",
                  "layoutOptions": { "org.eclipse.elk.port.side": "DIAGONAL" }
                }
              ]
            }
          ]
        }"#,
        "unsupported org.eclipse.elk.port.side value: DIAGONAL",
    );
}

#[test]
fn non_string_port_side_layout_option_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "children": [
            {
              "id": "source",
              "ports": [
                {
                  "id": "out",
                  "layoutOptions": { "org.eclipse.elk.port.side": 7 }
                }
              ]
            }
          ]
        }"#,
        "org.eclipse.elk.port.side must be a string",
    );
}

#[test]
fn port_scoped_options_non_integer_index_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "children": [
            {
              "id": "source",
              "ports": [
                {
                  "id": "out",
                  "layoutOptions": { "org.eclipse.elk.port.index": 1.5 }
                }
              ]
            }
          ]
        }"#,
        "org.eclipse.elk.port.index must be an integer",
    );
}

#[test]
fn port_scoped_options_non_number_border_offset_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "children": [
            {
              "id": "source",
              "ports": [
                {
                  "id": "out",
                  "layoutOptions": { "org.eclipse.elk.port.borderOffset": false }
                }
              ]
            }
          ]
        }"#,
        "org.eclipse.elk.port.borderOffset must be a number",
    );
}

#[test]
fn port_scoped_options_non_boolean_allow_switch_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "children": [
            {
              "id": "source",
              "ports": [
                {
                  "id": "out",
                  "layoutOptions": { "org.eclipse.elk.layered.allowNonFlowPortsToSwitchSides": "yes" }
                }
              ]
            }
          ]
        }"#,
        "org.eclipse.elk.layered.allowNonFlowPortsToSwitchSides must be a boolean",
    );
}

fn assert_invalid_contains(input: &str, expected: &str) {
    let error = from_str(input).unwrap_err();

    assert!(
        matches!(error, JsonError::Invalid(ref message) if message.contains(expected)),
        "expected JsonError::Invalid containing {expected:?}, got {error:?}",
    );
}

fn parent_boolean_option_keys() -> [&'static str; 17] {
    [
        "org.eclipse.elk.interactiveLayout",
        "org.eclipse.elk.layered.compaction.connectedComponents",
        "org.eclipse.elk.layered.considerModelOrder.portModelOrder",
        "org.eclipse.elk.layered.crossingMinimization.forceNodeModelOrder",
        "org.eclipse.elk.layered.crossingMinimization.semiInteractive",
        "org.eclipse.elk.layered.generatePositionAndLayerIds",
        "org.eclipse.elk.layered.highDegreeNodes.treatment",
        "org.eclipse.elk.layered.mergeEdges",
        "org.eclipse.elk.layered.mergeHierarchyEdges",
        "org.eclipse.elk.layered.nodePlacement.favorStraightEdges",
        "org.eclipse.elk.layered.unnecessaryBendpoints",
        "org.eclipse.elk.layered.wrapping.multiEdge.improveCuts",
        "org.eclipse.elk.layered.wrapping.multiEdge.improveWrappedEdges",
        "org.eclipse.elk.nodeSize.fixedGraphSize",
        "org.eclipse.elk.partitioning.activate",
        "org.eclipse.elk.separateConnectedComponents",
        "org.eclipse.elk.topdownLayout",
    ]
}

fn node_boolean_option_keys() -> [&'static str; 9] {
    [
        "org.eclipse.elk.commentBox",
        "org.eclipse.elk.hypernode",
        "org.eclipse.elk.insideSelfLoops.activate",
        "org.eclipse.elk.layered.considerModelOrder.noModelOrder",
        "org.eclipse.elk.layered.layerUnzipping.minimizeEdgeLength",
        "org.eclipse.elk.layered.layerUnzipping.resetOnLongEdges",
        "org.eclipse.elk.noLayout",
        "org.eclipse.elk.portLabels.nextToPortIfPossible",
        "org.eclipse.elk.portLabels.treatAsGroup",
    ]
}
