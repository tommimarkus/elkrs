use elkrs_core::geometry::{Point, Size};
use elkrs_core::graph::{ElementId, ElkGraph, ElkNode, ElkPort};
use elkrs_core::options::{
    Algorithm, CoreOption, Direction, EdgeRouting, HierarchyHandling, PortAlignment,
    PortConstraints, PortSide, PropertyValue,
};
use elkrs_json::{from_str, to_string_pretty};
use serde_json::Value;

#[test]
fn imports_java_algorithm_layout_option() {
    let graph = from_str(
        r#"{
          "id": "root",
          "layoutOptions": { "org.eclipse.elk.algorithm": "layered" }
        }"#,
    )
    .unwrap();

    assert_eq!(graph.properties.algorithm(), Some(Algorithm::Layered));
}

#[test]
fn imports_short_algorithm_layout_option() {
    let graph = from_str(
        r#"{
          "id": "root",
          "layoutOptions": { "elk.algorithm": "org.eclipse.elk.layered" }
        }"#,
    )
    .unwrap();

    assert_eq!(graph.properties.algorithm(), Some(Algorithm::Layered));
}

#[test]
fn imports_other_algorithm_for_layout_validation() {
    let graph = from_str(
        r#"{
          "id": "root",
          "layoutOptions": { "org.eclipse.elk.algorithm": "org.eclipse.elk.force" }
        }"#,
    )
    .unwrap();

    assert_eq!(
        graph.properties.algorithm(),
        Some(Algorithm::Other("org.eclipse.elk.force".to_owned()))
    );
}

#[test]
fn serializes_algorithm_with_java_key() {
    let mut graph = ElkGraph::new("root");
    graph.properties.set_algorithm(Algorithm::Layered);

    let json = serialized_value(&graph);
    assert_eq!(
        json["layoutOptions"]["org.eclipse.elk.algorithm"],
        Value::String("layered".to_owned())
    );
    assert_eq!(
        json["layoutOptions"].get("elk.algorithm"),
        None,
        "short algorithm key should not be emitted"
    );
}

#[test]
fn serializes_other_algorithm_with_java_key() {
    let mut graph = ElkGraph::new("root");
    graph
        .properties
        .set_algorithm(Algorithm::Other("org.eclipse.elk.force".to_owned()));

    let json = serialized_value(&graph);
    assert_eq!(
        json["layoutOptions"]["org.eclipse.elk.algorithm"],
        Value::String("org.eclipse.elk.force".to_owned())
    );
}

#[test]
fn imports_debug_mode_layout_option() {
    let graph = from_str(
        r#"{
          "id": "root",
          "layoutOptions": { "org.eclipse.elk.debugMode": true }
        }"#,
    )
    .unwrap();

    assert_eq!(
        graph.properties.get(CoreOption::DebugMode),
        Some(&PropertyValue::Bool(true))
    );
}

#[test]
fn serializes_debug_mode_with_java_key() {
    let mut graph = ElkGraph::new("root");
    graph.properties.set_debug_mode(true);

    let json = serialized_value(&graph);
    assert_eq!(
        json["layoutOptions"]["org.eclipse.elk.debugMode"],
        Value::Bool(true)
    );
}

#[test]
fn imports_node_debug_mode_layout_option() {
    let graph = from_str(
        r#"{
          "id": "root",
          "children": [
            {
              "id": "parent",
              "layoutOptions": { "org.eclipse.elk.debugMode": true }
            }
          ]
        }"#,
    )
    .unwrap();

    assert_eq!(
        graph.nodes[&ElementId::from("parent")]
            .properties
            .get(CoreOption::DebugMode),
        Some(&PropertyValue::Bool(true))
    );
}

#[test]
fn serializes_node_debug_mode_with_java_key() {
    let mut node = ElkNode::new("node");
    node.properties.set_debug_mode(true);
    let mut graph = ElkGraph::new("root");
    graph.add_node(node);

    let json = serialized_value(&graph);
    assert_eq!(
        json["children"][0]["layoutOptions"]["org.eclipse.elk.debugMode"],
        Value::Bool(true)
    );
}

#[test]
fn imports_feedback_edges_layout_option() {
    let graph = from_str(
        r#"{
          "id": "root",
          "layoutOptions": { "org.eclipse.elk.layered.feedbackEdges": true }
        }"#,
    )
    .unwrap();

    assert_eq!(
        graph.properties.get(CoreOption::FeedbackEdges),
        Some(&PropertyValue::Bool(true))
    );
}

#[test]
fn serializes_feedback_edges_with_java_key() {
    let mut graph = ElkGraph::new("root");
    graph.properties.set_feedback_edges(true);

    let json = serialized_value(&graph);
    assert_eq!(
        json["layoutOptions"]["org.eclipse.elk.layered.feedbackEdges"],
        Value::Bool(true)
    );
}

#[test]
fn imports_node_feedback_edges_layout_option() {
    let graph = from_str(
        r#"{
          "id": "root",
          "children": [
            {
              "id": "parent",
              "layoutOptions": { "org.eclipse.elk.layered.feedbackEdges": true }
            }
          ]
        }"#,
    )
    .unwrap();

    assert_eq!(
        graph.nodes[&ElementId::from("parent")]
            .properties
            .get(CoreOption::FeedbackEdges),
        Some(&PropertyValue::Bool(true))
    );
}

#[test]
fn serializes_node_feedback_edges_with_java_key() {
    let mut node = ElkNode::new("node");
    node.properties.set_feedback_edges(true);
    let mut graph = ElkGraph::new("root");
    graph.add_node(node);

    let json = serialized_value(&graph);
    assert_eq!(
        json["children"][0]["layoutOptions"]["org.eclipse.elk.layered.feedbackEdges"],
        Value::Bool(true)
    );
}

#[test]
fn imports_parent_boolean_layout_options() {
    let graph = from_str(
        r#"{
          "id": "root",
          "layoutOptions": {
            "org.eclipse.elk.interactiveLayout": true,
            "org.eclipse.elk.layered.compaction.connectedComponents": true,
            "org.eclipse.elk.layered.considerModelOrder.portModelOrder": true,
            "org.eclipse.elk.layered.crossingMinimization.forceNodeModelOrder": true,
            "org.eclipse.elk.layered.crossingMinimization.semiInteractive": true,
            "org.eclipse.elk.layered.generatePositionAndLayerIds": true,
            "org.eclipse.elk.layered.highDegreeNodes.treatment": true,
            "org.eclipse.elk.layered.mergeEdges": true,
            "org.eclipse.elk.layered.mergeHierarchyEdges": true,
            "org.eclipse.elk.layered.nodePlacement.favorStraightEdges": true,
            "org.eclipse.elk.layered.unnecessaryBendpoints": true,
            "org.eclipse.elk.layered.wrapping.multiEdge.improveCuts": true,
            "org.eclipse.elk.layered.wrapping.multiEdge.improveWrappedEdges": true,
            "org.eclipse.elk.nodeSize.fixedGraphSize": true,
            "org.eclipse.elk.partitioning.activate": true,
            "org.eclipse.elk.separateConnectedComponents": true,
            "org.eclipse.elk.topdownLayout": true
          }
        }"#,
    )
    .unwrap();

    for (key, option) in parent_boolean_options() {
        assert_eq!(
            graph.properties.get(option),
            Some(&PropertyValue::Bool(true)),
            "{key}"
        );
    }
}

#[test]
fn imports_disabled_parent_boolean_layout_options() {
    let graph = from_str(
        r#"{
          "id": "root",
          "layoutOptions": {
            "org.eclipse.elk.interactiveLayout": false,
            "org.eclipse.elk.layered.compaction.connectedComponents": false,
            "org.eclipse.elk.layered.considerModelOrder.portModelOrder": false,
            "org.eclipse.elk.layered.crossingMinimization.forceNodeModelOrder": false,
            "org.eclipse.elk.layered.crossingMinimization.semiInteractive": false,
            "org.eclipse.elk.layered.generatePositionAndLayerIds": false,
            "org.eclipse.elk.layered.highDegreeNodes.treatment": false,
            "org.eclipse.elk.layered.mergeEdges": false,
            "org.eclipse.elk.layered.mergeHierarchyEdges": false,
            "org.eclipse.elk.layered.nodePlacement.favorStraightEdges": false,
            "org.eclipse.elk.layered.unnecessaryBendpoints": false,
            "org.eclipse.elk.layered.wrapping.multiEdge.improveCuts": false,
            "org.eclipse.elk.layered.wrapping.multiEdge.improveWrappedEdges": false,
            "org.eclipse.elk.nodeSize.fixedGraphSize": false,
            "org.eclipse.elk.partitioning.activate": false,
            "org.eclipse.elk.separateConnectedComponents": false,
            "org.eclipse.elk.topdownLayout": false
          }
        }"#,
    )
    .unwrap();

    for (key, option) in parent_boolean_options() {
        assert_eq!(
            graph.properties.get(option),
            Some(&PropertyValue::Bool(false)),
            "{key}"
        );
    }
}

#[test]
fn serializes_parent_boolean_layout_options() {
    let mut graph = ElkGraph::new("root");
    set_parent_boolean_options(&mut graph, true);

    let json = serialized_value(&graph);
    for (key, _) in parent_boolean_options() {
        assert_eq!(json["layoutOptions"][key], Value::Bool(true), "{key}");
    }
}

#[test]
fn serializes_disabled_parent_boolean_layout_options() {
    let mut graph = ElkGraph::new("root");
    set_parent_boolean_options(&mut graph, false);

    let json = serialized_value(&graph);
    for (key, _) in parent_boolean_options() {
        assert_eq!(json["layoutOptions"][key], Value::Bool(false), "{key}");
    }
}

#[test]
fn imports_node_parent_boolean_layout_options() {
    let graph = from_str(
        r#"{
          "id": "root",
          "children": [
            {
              "id": "parent",
              "layoutOptions": {
                "org.eclipse.elk.interactiveLayout": true,
                "org.eclipse.elk.layered.compaction.connectedComponents": true,
                "org.eclipse.elk.layered.considerModelOrder.portModelOrder": true,
                "org.eclipse.elk.layered.crossingMinimization.forceNodeModelOrder": true,
                "org.eclipse.elk.layered.crossingMinimization.semiInteractive": true,
                "org.eclipse.elk.layered.generatePositionAndLayerIds": true,
                "org.eclipse.elk.layered.highDegreeNodes.treatment": true,
                "org.eclipse.elk.layered.mergeEdges": true,
                "org.eclipse.elk.layered.mergeHierarchyEdges": true,
                "org.eclipse.elk.layered.nodePlacement.favorStraightEdges": true,
                "org.eclipse.elk.layered.unnecessaryBendpoints": true,
                "org.eclipse.elk.layered.wrapping.multiEdge.improveCuts": true,
                "org.eclipse.elk.layered.wrapping.multiEdge.improveWrappedEdges": true,
                "org.eclipse.elk.nodeSize.fixedGraphSize": true,
                "org.eclipse.elk.partitioning.activate": true,
                "org.eclipse.elk.separateConnectedComponents": true,
                "org.eclipse.elk.topdownLayout": true
              }
            }
          ]
        }"#,
    )
    .unwrap();

    let properties = &graph.nodes[&ElementId::from("parent")].properties;
    for (key, option) in parent_boolean_options() {
        assert_eq!(
            properties.get(option),
            Some(&PropertyValue::Bool(true)),
            "{key}"
        );
    }
}

#[test]
fn imports_disabled_node_parent_boolean_layout_options() {
    let graph = from_str(
        r#"{
          "id": "root",
          "children": [
            {
              "id": "parent",
              "layoutOptions": {
                "org.eclipse.elk.interactiveLayout": false,
                "org.eclipse.elk.layered.compaction.connectedComponents": false,
                "org.eclipse.elk.layered.considerModelOrder.portModelOrder": false,
                "org.eclipse.elk.layered.crossingMinimization.forceNodeModelOrder": false,
                "org.eclipse.elk.layered.crossingMinimization.semiInteractive": false,
                "org.eclipse.elk.layered.generatePositionAndLayerIds": false,
                "org.eclipse.elk.layered.highDegreeNodes.treatment": false,
                "org.eclipse.elk.layered.mergeEdges": false,
                "org.eclipse.elk.layered.mergeHierarchyEdges": false,
                "org.eclipse.elk.layered.nodePlacement.favorStraightEdges": false,
                "org.eclipse.elk.layered.unnecessaryBendpoints": false,
                "org.eclipse.elk.layered.wrapping.multiEdge.improveCuts": false,
                "org.eclipse.elk.layered.wrapping.multiEdge.improveWrappedEdges": false,
                "org.eclipse.elk.nodeSize.fixedGraphSize": false,
                "org.eclipse.elk.partitioning.activate": false,
                "org.eclipse.elk.separateConnectedComponents": false,
                "org.eclipse.elk.topdownLayout": false
              }
            }
          ]
        }"#,
    )
    .unwrap();

    let properties = &graph.nodes[&ElementId::from("parent")].properties;
    for (key, option) in parent_boolean_options() {
        assert_eq!(
            properties.get(option),
            Some(&PropertyValue::Bool(false)),
            "{key}"
        );
    }
}

#[test]
fn serializes_node_parent_boolean_layout_options() {
    let mut node = ElkNode::new("node");
    set_parent_boolean_node_options(&mut node, true);
    let mut graph = ElkGraph::new("root");
    graph.add_node(node);

    let json = serialized_value(&graph);
    for (key, _) in parent_boolean_options() {
        assert_eq!(
            json["children"][0]["layoutOptions"][key],
            Value::Bool(true),
            "{key}"
        );
    }
}

#[test]
fn serializes_disabled_node_parent_boolean_layout_options() {
    let mut node = ElkNode::new("node");
    set_parent_boolean_node_options(&mut node, false);
    let mut graph = ElkGraph::new("root");
    graph.add_node(node);

    let json = serialized_value(&graph);
    for (key, _) in parent_boolean_options() {
        assert_eq!(
            json["children"][0]["layoutOptions"][key],
            Value::Bool(false),
            "{key}"
        );
    }
}

#[test]
fn imports_node_boolean_layout_options() {
    let graph = from_str(
        r#"{
          "id": "root",
          "children": [
            {
              "id": "node",
              "layoutOptions": {
                "org.eclipse.elk.commentBox": true,
                "org.eclipse.elk.hypernode": true,
                "org.eclipse.elk.insideSelfLoops.activate": true,
                "org.eclipse.elk.layered.considerModelOrder.noModelOrder": true,
                "org.eclipse.elk.layered.layerUnzipping.resetOnLongEdges": true,
                "org.eclipse.elk.layered.layerUnzipping.minimizeEdgeLength": true,
                "org.eclipse.elk.noLayout": true,
                "org.eclipse.elk.portLabels.nextToPortIfPossible": true,
                "org.eclipse.elk.portLabels.treatAsGroup": true
              }
            }
          ]
        }"#,
    )
    .unwrap();

    let properties = &graph.nodes[&ElementId::from("node")].properties;
    for (key, option) in node_boolean_options() {
        assert_eq!(
            properties.get(option),
            Some(&PropertyValue::Bool(true)),
            "{key}"
        );
    }
}

#[test]
fn imports_disabled_node_boolean_layout_options() {
    let graph = from_str(
        r#"{
          "id": "root",
          "children": [
            {
              "id": "node",
              "layoutOptions": {
                "org.eclipse.elk.commentBox": false,
                "org.eclipse.elk.hypernode": false,
                "org.eclipse.elk.insideSelfLoops.activate": false,
                "org.eclipse.elk.layered.considerModelOrder.noModelOrder": false,
                "org.eclipse.elk.layered.layerUnzipping.resetOnLongEdges": false,
                "org.eclipse.elk.layered.layerUnzipping.minimizeEdgeLength": false,
                "org.eclipse.elk.noLayout": false,
                "org.eclipse.elk.portLabels.nextToPortIfPossible": false,
                "org.eclipse.elk.portLabels.treatAsGroup": false
              }
            }
          ]
        }"#,
    )
    .unwrap();

    let properties = &graph.nodes[&ElementId::from("node")].properties;
    for (key, option) in node_boolean_options() {
        assert_eq!(
            properties.get(option),
            Some(&PropertyValue::Bool(false)),
            "{key}"
        );
    }
}

#[test]
fn serializes_node_boolean_layout_options() {
    let mut node = ElkNode::new("node");
    set_node_boolean_options(&mut node, true);
    let mut graph = ElkGraph::new("root");
    graph.add_node(node);

    let json = serialized_value(&graph);
    for (key, _) in node_boolean_options() {
        assert_eq!(
            json["children"][0]["layoutOptions"][key],
            Value::Bool(true),
            "{key}"
        );
    }
}

#[test]
fn serializes_disabled_node_boolean_layout_options() {
    let mut node = ElkNode::new("node");
    set_node_boolean_options(&mut node, false);
    let mut graph = ElkGraph::new("root");
    graph.add_node(node);

    let json = serialized_value(&graph);
    for (key, _) in node_boolean_options() {
        assert_eq!(
            json["children"][0]["layoutOptions"][key],
            Value::Bool(false),
            "{key}"
        );
    }
}

#[test]
fn imports_left_direction_layout_option() {
    let graph = from_str(
        r#"{
          "id": "root",
          "layoutOptions": { "org.eclipse.elk.direction": "LEFT" }
        }"#,
    )
    .unwrap();

    assert_eq!(graph.properties.direction(), Direction::Left);
}

#[test]
fn imports_up_direction_layout_option() {
    let graph = from_str(
        r#"{
          "id": "root",
          "layoutOptions": { "org.eclipse.elk.direction": "UP" }
        }"#,
    )
    .unwrap();

    assert_eq!(graph.properties.direction(), Direction::Up);
}

#[test]
fn imports_short_direction_layout_option() {
    let graph = from_str(
        r#"{
          "id": "root",
          "layoutOptions": { "elk.direction": "LEFT" }
        }"#,
    )
    .unwrap();

    assert_eq!(graph.properties.direction(), Direction::Left);
}

#[test]
fn imports_java_spacing_layout_options() {
    let graph = from_str(
        r#"{
          "id": "root",
          "layoutOptions": {
            "org.eclipse.elk.spacing.nodeNode": 42,
            "org.eclipse.elk.spacing.edgeNode": 21,
            "org.eclipse.elk.spacing.edgeEdge": 9
          }
        }"#,
    )
    .unwrap();

    assert_eq!(
        graph.properties.get(CoreOption::SpacingNodeNode),
        Some(&PropertyValue::Number(42.0))
    );
    assert_eq!(
        graph.properties.get(CoreOption::SpacingEdgeNode),
        Some(&PropertyValue::Number(21.0))
    );
    assert_eq!(
        graph.properties.get(CoreOption::SpacingEdgeEdge),
        Some(&PropertyValue::Number(9.0))
    );
}

#[test]
fn imports_java_layered_edge_spacing_layout_options() {
    let graph = from_str(
        r#"{
          "id": "root",
          "layoutOptions": {
            "org.eclipse.elk.layered.spacing.edgeEdgeBetweenLayers": 33,
            "org.eclipse.elk.layered.spacing.edgeNodeBetweenLayers": 44
          }
        }"#,
    )
    .unwrap();

    assert_eq!(
        graph
            .properties
            .get(CoreOption::SpacingEdgeEdgeBetweenLayers),
        Some(&PropertyValue::Number(33.0))
    );
    assert_eq!(
        graph
            .properties
            .get(CoreOption::SpacingEdgeNodeBetweenLayers),
        Some(&PropertyValue::Number(44.0))
    );
}

#[test]
fn imports_java_additional_spacing_layout_options() {
    let graph = from_str(
        r#"{
          "id": "root",
          "layoutOptions": {
            "org.eclipse.elk.layered.spacing.baseValue": 11,
            "org.eclipse.elk.layered.wrapping.additionalEdgeSpacing": 12,
            "org.eclipse.elk.spacing.commentComment": 13,
            "org.eclipse.elk.spacing.commentNode": 14,
            "org.eclipse.elk.spacing.componentComponent": 15,
            "org.eclipse.elk.spacing.nodeSelfLoop": 16
          }
        }"#,
    )
    .unwrap();

    assert_eq!(
        graph.properties.get(CoreOption::SpacingBaseValue),
        Some(&PropertyValue::Number(11.0))
    );
    assert_eq!(
        graph
            .properties
            .get(CoreOption::WrappingAdditionalEdgeSpacing),
        Some(&PropertyValue::Number(12.0))
    );
    assert_eq!(
        graph.properties.get(CoreOption::SpacingCommentComment),
        Some(&PropertyValue::Number(13.0))
    );
    assert_eq!(
        graph.properties.get(CoreOption::SpacingCommentNode),
        Some(&PropertyValue::Number(14.0))
    );
    assert_eq!(
        graph.properties.get(CoreOption::SpacingComponentComponent),
        Some(&PropertyValue::Number(15.0))
    );
    assert_eq!(
        graph.properties.get(CoreOption::SpacingNodeSelfLoop),
        Some(&PropertyValue::Number(16.0))
    );
}

#[test]
fn imports_java_label_and_port_spacing_layout_options() {
    let graph = from_str(
        r#"{
          "id": "root",
          "layoutOptions": {
            "org.eclipse.elk.spacing.edgeLabel": 21,
            "org.eclipse.elk.spacing.labelLabel": 22,
            "org.eclipse.elk.spacing.labelNode": 23,
            "org.eclipse.elk.spacing.labelPortHorizontal": 24,
            "org.eclipse.elk.spacing.labelPortVertical": 25,
            "org.eclipse.elk.spacing.portPort": 26
          },
          "children": [
            {
              "id": "node",
              "layoutOptions": { "org.eclipse.elk.spacing.portPort": 27 }
            }
          ]
        }"#,
    )
    .unwrap();

    assert_eq!(
        graph.properties.get(CoreOption::SpacingEdgeLabel),
        Some(&PropertyValue::Number(21.0))
    );
    assert_eq!(
        graph.properties.get(CoreOption::SpacingLabelLabel),
        Some(&PropertyValue::Number(22.0))
    );
    assert_eq!(
        graph.properties.get(CoreOption::SpacingLabelNode),
        Some(&PropertyValue::Number(23.0))
    );
    assert_eq!(
        graph.properties.get(CoreOption::SpacingLabelPortHorizontal),
        Some(&PropertyValue::Number(24.0))
    );
    assert_eq!(
        graph.properties.get(CoreOption::SpacingLabelPortVertical),
        Some(&PropertyValue::Number(25.0))
    );
    assert_eq!(
        graph.properties.get(CoreOption::SpacingPortPort),
        Some(&PropertyValue::Number(26.0))
    );
    assert_eq!(
        graph.nodes[&ElementId::from("node")]
            .properties
            .get(CoreOption::SpacingPortPort),
        Some(&PropertyValue::Number(27.0))
    );
}

#[test]
fn imports_short_spacing_layout_options() {
    let graph = from_str(
        r#"{
          "id": "root",
          "layoutOptions": {
            "elk.spacing.nodeNode": 42,
            "elk.spacing.edgeNode": 21,
            "elk.spacing.edgeEdge": 9
          }
        }"#,
    )
    .unwrap();

    assert_eq!(graph.properties.spacing_node_node(), 42.0);
    assert_eq!(graph.properties.spacing_edge_node(), 21.0);
    assert_eq!(graph.properties.spacing_edge_edge(), 9.0);
}

#[test]
fn serializes_left_and_up_direction_options() {
    let mut left = ElkGraph::new("left");
    left.properties.set_direction(Direction::Left);
    let mut up = ElkGraph::new("up");
    up.properties.set_direction(Direction::Up);

    let left_json = serialized_value(&left);
    assert_eq!(
        left_json["layoutOptions"]["org.eclipse.elk.direction"],
        Value::String("LEFT".to_owned()),
    );
    assert_eq!(
        left_json["layoutOptions"].get("elk.direction"),
        None,
        "short direction key should not be emitted"
    );
    let up_json = serialized_value(&up);
    assert_eq!(
        up_json["layoutOptions"]["org.eclipse.elk.direction"],
        Value::String("UP".to_owned()),
    );
    assert_eq!(
        up_json["layoutOptions"].get("elk.direction"),
        None,
        "short direction key should not be emitted"
    );
}

#[test]
fn serializes_spacing_with_java_keys() {
    let mut graph = ElkGraph::new("root");
    graph.properties.set_spacing_node_node(42.0);
    graph.properties.set_spacing_edge_node(21.0);
    graph.properties.set_spacing_edge_edge(9.0);

    let json = serialized_value(&graph);
    assert_eq!(
        json["layoutOptions"]["org.eclipse.elk.spacing.nodeNode"],
        Value::from(42.0)
    );
    assert_eq!(
        json["layoutOptions"]["org.eclipse.elk.spacing.edgeNode"],
        Value::from(21.0)
    );
    assert_eq!(
        json["layoutOptions"]["org.eclipse.elk.spacing.edgeEdge"],
        Value::from(9.0)
    );
    assert_eq!(
        json["layoutOptions"].get("elk.spacing.nodeNode"),
        None,
        "short node spacing key should not be emitted"
    );
    assert_eq!(
        json["layoutOptions"].get("elk.spacing.edgeNode"),
        None,
        "short edge-node spacing key should not be emitted"
    );
    assert_eq!(
        json["layoutOptions"].get("elk.spacing.edgeEdge"),
        None,
        "short edge-edge spacing key should not be emitted"
    );
}

#[test]
fn serializes_layered_edge_spacing_with_java_keys() {
    let mut graph = ElkGraph::new("root");
    graph.properties.set_spacing_edge_edge_between_layers(33.0);
    graph.properties.set_spacing_edge_node_between_layers(44.0);

    let json = serialized_value(&graph);
    assert_eq!(
        json["layoutOptions"]["org.eclipse.elk.layered.spacing.edgeEdgeBetweenLayers"],
        Value::from(33.0)
    );
    assert_eq!(
        json["layoutOptions"]["org.eclipse.elk.layered.spacing.edgeNodeBetweenLayers"],
        Value::from(44.0)
    );
}

#[test]
fn serializes_additional_spacing_with_java_keys() {
    let mut graph = ElkGraph::new("root");
    graph.properties.set_spacing_base_value(11.0);
    graph.properties.set_wrapping_additional_edge_spacing(12.0);
    graph.properties.set_spacing_comment_comment(13.0);
    graph.properties.set_spacing_comment_node(14.0);
    graph.properties.set_spacing_component_component(15.0);
    graph.properties.set_spacing_node_self_loop(16.0);

    let json = serialized_value(&graph);
    assert_eq!(
        json["layoutOptions"]["org.eclipse.elk.layered.spacing.baseValue"],
        Value::from(11.0)
    );
    assert_eq!(
        json["layoutOptions"]["org.eclipse.elk.layered.wrapping.additionalEdgeSpacing"],
        Value::from(12.0)
    );
    assert_eq!(
        json["layoutOptions"]["org.eclipse.elk.spacing.commentComment"],
        Value::from(13.0)
    );
    assert_eq!(
        json["layoutOptions"]["org.eclipse.elk.spacing.commentNode"],
        Value::from(14.0)
    );
    assert_eq!(
        json["layoutOptions"]["org.eclipse.elk.spacing.componentComponent"],
        Value::from(15.0)
    );
    assert_eq!(
        json["layoutOptions"]["org.eclipse.elk.spacing.nodeSelfLoop"],
        Value::from(16.0)
    );
}

#[test]
fn serializes_label_and_port_spacing_with_java_keys() {
    let mut graph = ElkGraph::new("root");
    graph.properties.set_spacing_edge_label(21.0);
    graph.properties.set_spacing_label_label(22.0);
    graph.properties.set_spacing_label_node(23.0);
    graph.properties.set_spacing_label_port_horizontal(24.0);
    graph.properties.set_spacing_label_port_vertical(25.0);
    graph.properties.set_spacing_port_port(26.0);

    let mut node = ElkNode::new("node");
    node.properties.set_spacing_port_port(27.0);
    graph.add_node(node);

    let json = serialized_value(&graph);
    assert_eq!(
        json["layoutOptions"]["org.eclipse.elk.spacing.edgeLabel"],
        Value::from(21.0)
    );
    assert_eq!(
        json["layoutOptions"]["org.eclipse.elk.spacing.labelLabel"],
        Value::from(22.0)
    );
    assert_eq!(
        json["layoutOptions"]["org.eclipse.elk.spacing.labelNode"],
        Value::from(23.0)
    );
    assert_eq!(
        json["layoutOptions"]["org.eclipse.elk.spacing.labelPortHorizontal"],
        Value::from(24.0)
    );
    assert_eq!(
        json["layoutOptions"]["org.eclipse.elk.spacing.labelPortVertical"],
        Value::from(25.0)
    );
    assert_eq!(
        json["layoutOptions"]["org.eclipse.elk.spacing.portPort"],
        Value::from(26.0)
    );
    assert_eq!(
        json["children"][0]["layoutOptions"]["org.eclipse.elk.spacing.portPort"],
        Value::from(27.0)
    );
}

#[test]
fn imports_java_edge_routing_layout_option() {
    let graph = from_str(
        r#"{
          "id": "root",
          "layoutOptions": { "org.eclipse.elk.edgeRouting": "ORTHOGONAL" }
        }"#,
    )
    .unwrap();

    assert_eq!(
        graph.properties.get(CoreOption::EdgeRouting),
        Some(&PropertyValue::EdgeRouting(EdgeRouting::Orthogonal))
    );
}

#[test]
fn imports_non_orthogonal_edge_routing_layout_options_for_validation() {
    let polyline = from_str(
        r#"{
          "id": "root",
          "layoutOptions": { "org.eclipse.elk.edgeRouting": "POLYLINE" }
        }"#,
    )
    .unwrap();
    let splines = from_str(
        r#"{
          "id": "root",
          "layoutOptions": { "org.eclipse.elk.edgeRouting": "SPLINES" }
        }"#,
    )
    .unwrap();

    assert_eq!(
        polyline.properties.get(CoreOption::EdgeRouting),
        Some(&PropertyValue::EdgeRouting(EdgeRouting::Polyline))
    );
    assert_eq!(
        splines.properties.get(CoreOption::EdgeRouting),
        Some(&PropertyValue::EdgeRouting(EdgeRouting::Splines))
    );
}

#[test]
fn imports_undefined_edge_routing_as_unset() {
    let graph = from_str(
        r#"{
          "id": "root",
          "layoutOptions": { "org.eclipse.elk.edgeRouting": "UNDEFINED" }
        }"#,
    )
    .unwrap();

    assert_eq!(graph.properties.get(CoreOption::EdgeRouting), None);
}

#[test]
fn serializes_edge_routing_with_java_key() {
    let mut graph = ElkGraph::new("root");
    graph.properties.set_edge_routing(EdgeRouting::Orthogonal);

    let json = serialized_value(&graph);
    assert_eq!(
        json["layoutOptions"]["org.eclipse.elk.edgeRouting"],
        Value::String("ORTHOGONAL".to_owned())
    );
    assert_eq!(
        json["layoutOptions"].get("elk.edgeRouting"),
        None,
        "short edge routing key should not be emitted"
    );
}

#[test]
fn serializes_non_orthogonal_edge_routing_with_java_key() {
    let mut polyline = ElkGraph::new("polyline");
    polyline.properties.set_edge_routing(EdgeRouting::Polyline);
    let mut splines = ElkGraph::new("splines");
    splines.properties.set_edge_routing(EdgeRouting::Splines);

    let polyline_json = serialized_value(&polyline);
    assert_eq!(
        polyline_json["layoutOptions"]["org.eclipse.elk.edgeRouting"],
        Value::String("POLYLINE".to_owned())
    );
    let splines_json = serialized_value(&splines);
    assert_eq!(
        splines_json["layoutOptions"]["org.eclipse.elk.edgeRouting"],
        Value::String("SPLINES".to_owned())
    );
}

#[test]
fn imports_node_edge_routing_layout_option() {
    let graph = from_str(
        r#"{
          "id": "root",
          "children": [
            {
              "id": "parent",
              "layoutOptions": { "org.eclipse.elk.edgeRouting": "POLYLINE" }
            },
            {
              "id": "child",
              "layoutOptions": { "org.eclipse.elk.edgeRouting": "SPLINES" }
            }
          ]
        }"#,
    )
    .unwrap();

    assert_eq!(
        graph.nodes[&ElementId::from("parent")]
            .properties
            .get(CoreOption::EdgeRouting),
        Some(&PropertyValue::EdgeRouting(EdgeRouting::Polyline))
    );
    assert_eq!(
        graph.nodes[&ElementId::from("child")]
            .properties
            .get(CoreOption::EdgeRouting),
        Some(&PropertyValue::EdgeRouting(EdgeRouting::Splines))
    );
}

#[test]
fn imports_node_undefined_edge_routing_as_unset() {
    let graph = from_str(
        r#"{
          "id": "root",
          "children": [
            {
              "id": "node",
              "layoutOptions": { "org.eclipse.elk.edgeRouting": "UNDEFINED" }
            }
          ]
        }"#,
    )
    .unwrap();

    assert_eq!(
        graph.nodes[&ElementId::from("node")]
            .properties
            .get(CoreOption::EdgeRouting),
        None
    );
}

#[test]
fn serializes_node_edge_routing_with_java_key() {
    let mut node = ElkNode::new("node");
    node.properties.set_edge_routing(EdgeRouting::Splines);
    let mut graph = ElkGraph::new("root");
    graph.add_node(node);

    let json = serialized_value(&graph);
    assert_eq!(
        json["children"][0]["layoutOptions"]["org.eclipse.elk.edgeRouting"],
        Value::String("SPLINES".to_owned())
    );
}

#[test]
fn imports_java_hierarchy_handling_layout_option() {
    let include_children = from_str(
        r#"{
          "id": "include",
          "layoutOptions": { "org.eclipse.elk.hierarchyHandling": "INCLUDE_CHILDREN" }
        }"#,
    )
    .unwrap();
    let separate_children = from_str(
        r#"{
          "id": "separate",
          "layoutOptions": { "org.eclipse.elk.hierarchyHandling": "SEPARATE_CHILDREN" }
        }"#,
    )
    .unwrap();

    assert_eq!(
        include_children
            .properties
            .get(CoreOption::HierarchyHandling),
        Some(&PropertyValue::HierarchyHandling(
            HierarchyHandling::IncludeChildren
        ))
    );
    assert_eq!(
        separate_children
            .properties
            .get(CoreOption::HierarchyHandling),
        Some(&PropertyValue::HierarchyHandling(
            HierarchyHandling::SeparateChildren
        ))
    );
}

#[test]
fn imports_inherit_hierarchy_handling_as_unset() {
    let graph = from_str(
        r#"{
          "id": "root",
          "layoutOptions": { "org.eclipse.elk.hierarchyHandling": "INHERIT" }
        }"#,
    )
    .unwrap();

    assert_eq!(graph.properties.get(CoreOption::HierarchyHandling), None);
}

#[test]
fn serializes_hierarchy_handling_with_java_key() {
    let mut include_children = ElkGraph::new("include");
    include_children
        .properties
        .set_hierarchy_handling(HierarchyHandling::IncludeChildren);
    let mut separate_children = ElkGraph::new("separate");
    separate_children
        .properties
        .set_hierarchy_handling(HierarchyHandling::SeparateChildren);

    let include_json = serialized_value(&include_children);
    assert_eq!(
        include_json["layoutOptions"]["org.eclipse.elk.hierarchyHandling"],
        Value::String("INCLUDE_CHILDREN".to_owned())
    );
    let separate_json = serialized_value(&separate_children);
    assert_eq!(
        separate_json["layoutOptions"]["org.eclipse.elk.hierarchyHandling"],
        Value::String("SEPARATE_CHILDREN".to_owned())
    );
}

#[test]
fn imports_node_hierarchy_handling_layout_option() {
    let graph = from_str(
        r#"{
          "id": "root",
          "children": [
            {
              "id": "parent",
              "layoutOptions": { "org.eclipse.elk.hierarchyHandling": "SEPARATE_CHILDREN" }
            },
            {
              "id": "child",
              "layoutOptions": { "org.eclipse.elk.hierarchyHandling": "INCLUDE_CHILDREN" }
            }
          ]
        }"#,
    )
    .unwrap();

    assert_eq!(
        graph.nodes[&ElementId::from("parent")]
            .properties
            .get(CoreOption::HierarchyHandling),
        Some(&PropertyValue::HierarchyHandling(
            HierarchyHandling::SeparateChildren
        ))
    );
    assert_eq!(
        graph.nodes[&ElementId::from("child")]
            .properties
            .get(CoreOption::HierarchyHandling),
        Some(&PropertyValue::HierarchyHandling(
            HierarchyHandling::IncludeChildren
        ))
    );
}

#[test]
fn imports_node_inherit_hierarchy_handling_as_unset() {
    let graph = from_str(
        r#"{
          "id": "root",
          "children": [
            {
              "id": "node",
              "layoutOptions": { "org.eclipse.elk.hierarchyHandling": "INHERIT" }
            }
          ]
        }"#,
    )
    .unwrap();

    assert_eq!(
        graph.nodes[&ElementId::from("node")]
            .properties
            .get(CoreOption::HierarchyHandling),
        None
    );
}

#[test]
fn serializes_node_hierarchy_handling_with_java_key() {
    let mut node = ElkNode::new("node");
    node.properties
        .set_hierarchy_handling(HierarchyHandling::SeparateChildren);
    let mut graph = ElkGraph::new("root");
    graph.add_node(node);

    let json = serialized_value(&graph);
    assert_eq!(
        json["children"][0]["layoutOptions"]["org.eclipse.elk.hierarchyHandling"],
        Value::String("SEPARATE_CHILDREN".to_owned())
    );
}

#[test]
fn imports_node_port_constraints_layout_option() {
    let graph = from_str(
        r#"{
          "id": "root",
          "children": [
            {
              "id": "fixed",
              "layoutOptions": { "org.eclipse.elk.portConstraints": "FIXED_ORDER" }
            },
            {
              "id": "free",
              "layoutOptions": { "org.eclipse.elk.portConstraints": "FREE" }
            }
          ]
        }"#,
    )
    .unwrap();

    assert_eq!(
        graph.nodes[&ElementId::from("fixed")]
            .properties
            .get(CoreOption::PortConstraints),
        Some(&PropertyValue::PortConstraints(PortConstraints::FixedOrder))
    );
    assert_eq!(
        graph.nodes[&ElementId::from("free")]
            .properties
            .get(CoreOption::PortConstraints),
        Some(&PropertyValue::PortConstraints(PortConstraints::Free))
    );
}

#[test]
fn imports_undefined_node_port_constraints_as_unset() {
    let graph = from_str(
        r#"{
          "id": "root",
          "children": [
            {
              "id": "node",
              "layoutOptions": { "org.eclipse.elk.portConstraints": "UNDEFINED" }
            }
          ]
        }"#,
    )
    .unwrap();

    assert_eq!(
        graph.nodes[&ElementId::from("node")]
            .properties
            .get(CoreOption::PortConstraints),
        None
    );
}

#[test]
fn serializes_node_port_constraints_with_java_key() {
    let mut node = ElkNode::new("node");
    node.properties
        .set_port_constraints(PortConstraints::FixedSide);
    let mut graph = ElkGraph::new("root");
    graph.add_node(node);

    let json = serialized_value(&graph);
    assert_eq!(
        json["children"][0]["layoutOptions"]["org.eclipse.elk.portConstraints"],
        Value::String("FIXED_SIDE".to_owned())
    );
}

#[test]
fn imports_node_port_alignment_layout_options() {
    let graph = from_str(
        r#"{
          "id": "root",
          "children": [
            {
              "id": "node",
              "layoutOptions": {
                "org.eclipse.elk.portAlignment.default": "JUSTIFIED",
                "org.eclipse.elk.portAlignment.east": "BEGIN",
                "org.eclipse.elk.portAlignment.north": "CENTER",
                "org.eclipse.elk.portAlignment.south": "DISTRIBUTED",
                "org.eclipse.elk.portAlignment.west": "END"
              }
            }
          ]
        }"#,
    )
    .unwrap();
    let properties = &graph.nodes[&ElementId::from("node")].properties;

    assert_eq!(
        properties.get(CoreOption::PortAlignmentDefault),
        Some(&PropertyValue::PortAlignment(PortAlignment::Justified))
    );
    assert_eq!(
        properties.get(CoreOption::PortAlignmentEast),
        Some(&PropertyValue::PortAlignment(PortAlignment::Begin))
    );
    assert_eq!(
        properties.get(CoreOption::PortAlignmentNorth),
        Some(&PropertyValue::PortAlignment(PortAlignment::Center))
    );
    assert_eq!(
        properties.get(CoreOption::PortAlignmentSouth),
        Some(&PropertyValue::PortAlignment(PortAlignment::Distributed))
    );
    assert_eq!(
        properties.get(CoreOption::PortAlignmentWest),
        Some(&PropertyValue::PortAlignment(PortAlignment::End))
    );
}

#[test]
fn serializes_node_port_alignment_with_java_keys() {
    let mut node = ElkNode::new("node");
    node.properties
        .set_port_alignment_default(PortAlignment::Justified);
    node.properties
        .set_port_alignment_east(PortAlignment::Begin);
    node.properties
        .set_port_alignment_north(PortAlignment::Center);
    node.properties
        .set_port_alignment_south(PortAlignment::Distributed);
    node.properties.set_port_alignment_west(PortAlignment::End);
    let mut graph = ElkGraph::new("root");
    graph.add_node(node);

    let json = serialized_value(&graph);
    let options = &json["children"][0]["layoutOptions"];
    assert_eq!(
        options["org.eclipse.elk.portAlignment.default"],
        Value::String("JUSTIFIED".to_owned())
    );
    assert_eq!(
        options["org.eclipse.elk.portAlignment.east"],
        Value::String("BEGIN".to_owned())
    );
    assert_eq!(
        options["org.eclipse.elk.portAlignment.north"],
        Value::String("CENTER".to_owned())
    );
    assert_eq!(
        options["org.eclipse.elk.portAlignment.south"],
        Value::String("DISTRIBUTED".to_owned())
    );
    assert_eq!(
        options["org.eclipse.elk.portAlignment.west"],
        Value::String("END".to_owned())
    );
}

#[test]
fn imports_java_layer_node_node_spacing_layout_option() {
    let graph = from_str(
        r#"{
          "id": "root",
          "layoutOptions": {
            "org.eclipse.elk.layered.spacing.nodeNodeBetweenLayers": 300
          }
        }"#,
    )
    .unwrap();

    assert_eq!(graph.properties.spacing_layer_node_node(), 300.0);
}

#[test]
fn imports_java_string_layer_node_node_spacing_layout_option() {
    let graph = from_str(
        r#"{
          "id": "root",
          "layoutOptions": {
            "org.eclipse.elk.layered.spacing.nodeNodeBetweenLayers": "300.0"
          }
        }"#,
    )
    .unwrap();

    assert_eq!(graph.properties.spacing_layer_node_node(), 300.0);
}

#[test]
fn serializes_layer_node_node_spacing_with_java_key() {
    let mut graph = ElkGraph::new("root");
    graph.properties.set_spacing_layer_node_node(300.0);

    let json = serialized_value(&graph);
    assert_eq!(
        json["layoutOptions"]["org.eclipse.elk.layered.spacing.nodeNodeBetweenLayers"],
        Value::from(300.0)
    );
    assert_eq!(
        json["layoutOptions"].get("elk.spacing.layerNodeNode"),
        None,
        "legacy layer spacing key should not be emitted"
    );
}

#[test]
fn imports_north_and_south_port_sides() {
    let graph = from_str(
        r#"{
          "id": "root",
          "children": [
            {
              "id": "node",
              "ports": [
                { "id": "north", "side": "NORTH" },
                { "id": "south", "side": "SOUTH" }
              ]
            }
          ]
        }"#,
    )
    .unwrap();

    let node = &graph.nodes[&ElementId::from("node")];
    assert_eq!(
        node.ports[&ElementId::from("north")].side,
        Some(PortSide::North)
    );
    assert_eq!(
        node.ports[&ElementId::from("south")].side,
        Some(PortSide::South)
    );
}

#[test]
fn imports_port_side_from_layout_options() {
    let graph = from_str(
        r#"{
          "id": "root",
          "children": [
            {
              "id": "node",
              "ports": [
                {
                  "id": "east",
                  "layoutOptions": { "org.eclipse.elk.port.side": "EAST" }
                }
              ]
            }
          ]
        }"#,
    )
    .unwrap();

    let node = &graph.nodes[&ElementId::from("node")];
    assert_eq!(
        node.ports[&ElementId::from("east")].side,
        Some(PortSide::East)
    );
}

#[test]
fn imports_all_defined_port_side_layout_options() {
    let graph = from_str(
        r#"{
          "id": "root",
          "children": [
            {
              "id": "node",
              "ports": [
                {
                  "id": "north",
                  "layoutOptions": { "org.eclipse.elk.port.side": "NORTH" }
                },
                {
                  "id": "east",
                  "layoutOptions": { "org.eclipse.elk.port.side": "EAST" }
                },
                {
                  "id": "south",
                  "layoutOptions": { "org.eclipse.elk.port.side": "SOUTH" }
                },
                {
                  "id": "west",
                  "layoutOptions": { "org.eclipse.elk.port.side": "WEST" }
                }
              ]
            }
          ]
        }"#,
    )
    .unwrap();

    let node = &graph.nodes[&ElementId::from("node")];
    assert_eq!(
        node.ports[&ElementId::from("north")].side,
        Some(PortSide::North)
    );
    assert_eq!(
        node.ports[&ElementId::from("east")].side,
        Some(PortSide::East)
    );
    assert_eq!(
        node.ports[&ElementId::from("south")].side,
        Some(PortSide::South)
    );
    assert_eq!(
        node.ports[&ElementId::from("west")].side,
        Some(PortSide::West)
    );
}

#[test]
fn imports_undefined_port_side_as_unset() {
    let graph = from_str(
        r#"{
          "id": "root",
          "children": [
            {
              "id": "node",
              "ports": [
                {
                  "id": "canonical",
                  "layoutOptions": { "org.eclipse.elk.port.side": "UNDEFINED" }
                },
                {
                  "id": "legacy",
                  "side": "UNDEFINED"
                }
              ]
            }
          ]
        }"#,
    )
    .unwrap();

    let node = &graph.nodes[&ElementId::from("node")];
    assert_eq!(node.ports[&ElementId::from("canonical")].side, None);
    assert_eq!(node.ports[&ElementId::from("legacy")].side, None);
}

#[test]
fn port_side_layout_option_takes_precedence_over_side_field() {
    let graph = from_str(
        r#"{
          "id": "root",
          "children": [
            {
              "id": "node",
              "ports": [
                {
                  "id": "port",
                  "side": "WEST",
                  "layoutOptions": { "org.eclipse.elk.port.side": "EAST" }
                }
              ]
            }
          ]
        }"#,
    )
    .unwrap();

    let node = &graph.nodes[&ElementId::from("node")];
    assert_eq!(
        node.ports[&ElementId::from("port")].side,
        Some(PortSide::East)
    );
}

#[test]
fn serializes_port_sides_with_java_key() {
    let mut node = ElkNode::new("node");
    node.add_port(port(
        "north",
        PortSide::North,
        Point::new(20.0, 0.0),
        Size::new(10.0, 10.0),
    ));
    node.add_port(port(
        "east",
        PortSide::East,
        Point::new(40.0, 20.0),
        Size::new(10.0, 10.0),
    ));
    node.add_port(port(
        "south",
        PortSide::South,
        Point::new(20.0, 40.0),
        Size::new(10.0, 10.0),
    ));
    node.add_port(port(
        "west",
        PortSide::West,
        Point::new(0.0, 20.0),
        Size::new(10.0, 10.0),
    ));
    let mut graph = ElkGraph::new("root");
    graph.add_node(node);

    let json = serialized_value(&graph);
    let node = json["children"]
        .as_array()
        .unwrap()
        .iter()
        .find(|node| node["id"] == Value::String("node".to_owned()))
        .unwrap();

    assert_eq!(
        port_value(node, "north")["layoutOptions"]["org.eclipse.elk.port.side"],
        Value::String("NORTH".to_owned())
    );
    assert_eq!(
        port_value(node, "east")["layoutOptions"]["org.eclipse.elk.port.side"],
        Value::String("EAST".to_owned())
    );
    assert_eq!(
        port_value(node, "south")["layoutOptions"]["org.eclipse.elk.port.side"],
        Value::String("SOUTH".to_owned())
    );
    assert_eq!(
        port_value(node, "west")["layoutOptions"]["org.eclipse.elk.port.side"],
        Value::String("WEST".to_owned())
    );
    assert_eq!(
        port_value(node, "north").get("side"),
        None,
        "legacy top-level port side should not be emitted"
    );
}

fn serialized_value(graph: &ElkGraph) -> Value {
    serde_json::from_str(&to_string_pretty(graph).unwrap()).unwrap()
}

fn parent_boolean_options() -> [(&'static str, CoreOption); 17] {
    [
        (
            "org.eclipse.elk.interactiveLayout",
            CoreOption::InteractiveLayout,
        ),
        (
            "org.eclipse.elk.layered.compaction.connectedComponents",
            CoreOption::ConnectedComponentsCompaction,
        ),
        (
            "org.eclipse.elk.layered.considerModelOrder.portModelOrder",
            CoreOption::ConsiderPortOrder,
        ),
        (
            "org.eclipse.elk.layered.crossingMinimization.forceNodeModelOrder",
            CoreOption::ForceNodeModelOrder,
        ),
        (
            "org.eclipse.elk.layered.crossingMinimization.semiInteractive",
            CoreOption::SemiInteractiveCrossingMinimization,
        ),
        (
            "org.eclipse.elk.layered.generatePositionAndLayerIds",
            CoreOption::GeneratePositionAndLayerIds,
        ),
        (
            "org.eclipse.elk.layered.highDegreeNodes.treatment",
            CoreOption::HighDegreeNodeTreatment,
        ),
        ("org.eclipse.elk.layered.mergeEdges", CoreOption::MergeEdges),
        (
            "org.eclipse.elk.layered.mergeHierarchyEdges",
            CoreOption::MergeHierarchyEdges,
        ),
        (
            "org.eclipse.elk.layered.nodePlacement.favorStraightEdges",
            CoreOption::FavorStraightEdges,
        ),
        (
            "org.eclipse.elk.layered.unnecessaryBendpoints",
            CoreOption::UnnecessaryBendpoints,
        ),
        (
            "org.eclipse.elk.layered.wrapping.multiEdge.improveCuts",
            CoreOption::WrappingImproveCuts,
        ),
        (
            "org.eclipse.elk.layered.wrapping.multiEdge.improveWrappedEdges",
            CoreOption::WrappingImproveWrappedEdges,
        ),
        (
            "org.eclipse.elk.nodeSize.fixedGraphSize",
            CoreOption::FixedGraphSize,
        ),
        (
            "org.eclipse.elk.partitioning.activate",
            CoreOption::LayoutPartitioning,
        ),
        (
            "org.eclipse.elk.separateConnectedComponents",
            CoreOption::SeparateConnectedComponents,
        ),
        ("org.eclipse.elk.topdownLayout", CoreOption::TopdownLayout),
    ]
}

fn node_boolean_options() -> [(&'static str, CoreOption); 9] {
    [
        ("org.eclipse.elk.commentBox", CoreOption::CommentBox),
        ("org.eclipse.elk.hypernode", CoreOption::Hypernode),
        (
            "org.eclipse.elk.insideSelfLoops.activate",
            CoreOption::InsideSelfLoops,
        ),
        (
            "org.eclipse.elk.layered.considerModelOrder.noModelOrder",
            CoreOption::NoModelOrder,
        ),
        (
            "org.eclipse.elk.layered.layerUnzipping.minimizeEdgeLength",
            CoreOption::LayerUnzippingMinimizeEdgeLength,
        ),
        (
            "org.eclipse.elk.layered.layerUnzipping.resetOnLongEdges",
            CoreOption::LayerUnzippingResetOnLongEdges,
        ),
        ("org.eclipse.elk.noLayout", CoreOption::NoLayout),
        (
            "org.eclipse.elk.portLabels.nextToPortIfPossible",
            CoreOption::PortLabelsNextToPortIfPossible,
        ),
        (
            "org.eclipse.elk.portLabels.treatAsGroup",
            CoreOption::PortLabelsTreatAsGroup,
        ),
    ]
}

fn set_parent_boolean_options(graph: &mut ElkGraph, enabled: bool) {
    graph.properties.set_interactive_layout(enabled);
    graph
        .properties
        .set_connected_components_compaction(enabled);
    graph.properties.set_consider_port_order(enabled);
    graph.properties.set_force_node_model_order(enabled);
    graph
        .properties
        .set_semi_interactive_crossing_minimization(enabled);
    graph
        .properties
        .set_generate_position_and_layer_ids(enabled);
    graph.properties.set_high_degree_node_treatment(enabled);
    graph.properties.set_merge_edges(enabled);
    graph.properties.set_merge_hierarchy_edges(enabled);
    graph.properties.set_favor_straight_edges(enabled);
    graph.properties.set_unnecessary_bendpoints(enabled);
    graph.properties.set_wrapping_improve_cuts(enabled);
    graph.properties.set_wrapping_improve_wrapped_edges(enabled);
    graph.properties.set_fixed_graph_size(enabled);
    graph.properties.set_layout_partitioning(enabled);
    graph.properties.set_separate_connected_components(enabled);
    graph.properties.set_topdown_layout(enabled);
}

fn set_parent_boolean_node_options(node: &mut ElkNode, enabled: bool) {
    node.properties.set_interactive_layout(enabled);
    node.properties.set_connected_components_compaction(enabled);
    node.properties.set_consider_port_order(enabled);
    node.properties.set_force_node_model_order(enabled);
    node.properties
        .set_semi_interactive_crossing_minimization(enabled);
    node.properties.set_generate_position_and_layer_ids(enabled);
    node.properties.set_high_degree_node_treatment(enabled);
    node.properties.set_merge_edges(enabled);
    node.properties.set_merge_hierarchy_edges(enabled);
    node.properties.set_favor_straight_edges(enabled);
    node.properties.set_unnecessary_bendpoints(enabled);
    node.properties.set_wrapping_improve_cuts(enabled);
    node.properties.set_wrapping_improve_wrapped_edges(enabled);
    node.properties.set_fixed_graph_size(enabled);
    node.properties.set_layout_partitioning(enabled);
    node.properties.set_separate_connected_components(enabled);
    node.properties.set_topdown_layout(enabled);
}

fn set_node_boolean_options(node: &mut ElkNode, enabled: bool) {
    node.properties.set_comment_box(enabled);
    node.properties.set_hypernode(enabled);
    node.properties.set_inside_self_loops(enabled);
    node.properties.set_no_model_order(enabled);
    node.properties.set_no_layout(enabled);
    node.properties
        .set_layer_unzipping_minimize_edge_length(enabled);
    node.properties
        .set_layer_unzipping_reset_on_long_edges(enabled);
    node.properties
        .set_port_labels_next_to_port_if_possible(enabled);
    node.properties.set_port_labels_treat_as_group(enabled);
}

fn port_value<'a>(node: &'a Value, id: &str) -> &'a Value {
    node["ports"]
        .as_array()
        .unwrap()
        .iter()
        .find(|port| port["id"] == Value::String(id.to_owned()))
        .unwrap()
}

fn port(id: &str, side: PortSide, position: Point, size: Size) -> ElkPort {
    let mut port = ElkPort::new(id);
    port.side = Some(side);
    port.position = position;
    port.size = size;
    port
}
