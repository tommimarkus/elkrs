use elkrs_core::geometry::{Point, Size};
use elkrs_core::graph::{ElementId, ElkGraph, ElkNode, ElkPort};
use elkrs_core::options::{Algorithm, CoreOption, Direction, EdgeRouting, PortSide, PropertyValue};
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
fn serializes_north_and_south_port_sides() {
    let mut node = ElkNode::new("node");
    node.add_port(port(
        "north",
        PortSide::North,
        Point::new(20.0, 0.0),
        Size::new(10.0, 10.0),
    ));
    node.add_port(port(
        "south",
        PortSide::South,
        Point::new(20.0, 40.0),
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
        port_value(node, "south")["layoutOptions"]["org.eclipse.elk.port.side"],
        Value::String("SOUTH".to_owned())
    );
}

fn serialized_value(graph: &ElkGraph) -> Value {
    serde_json::from_str(&to_string_pretty(graph).unwrap()).unwrap()
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
