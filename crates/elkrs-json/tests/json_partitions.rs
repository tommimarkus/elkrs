use elkrs_core::geometry::{Point, Size};
use elkrs_core::graph::{ElementId, ElkGraph, ElkNode, ElkPort};
use elkrs_core::options::{Direction, PortSide};
use elkrs_json::{from_str, to_string_pretty};
use serde_json::Value;

#[test]
fn imports_left_direction_layout_option() {
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
fn imports_up_direction_layout_option() {
    let graph = from_str(
        r#"{
          "id": "root",
          "layoutOptions": { "elk.direction": "UP" }
        }"#,
    )
    .unwrap();

    assert_eq!(graph.properties.direction(), Direction::Up);
}

#[test]
fn serializes_left_and_up_direction_options() {
    let mut left = ElkGraph::new("left");
    left.properties.set_direction(Direction::Left);
    let mut up = ElkGraph::new("up");
    up.properties.set_direction(Direction::Up);

    assert_eq!(
        serialized_value(&left)["layoutOptions"]["elk.direction"],
        Value::String("LEFT".to_owned()),
    );
    assert_eq!(
        serialized_value(&up)["layoutOptions"]["elk.direction"],
        Value::String("UP".to_owned()),
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
