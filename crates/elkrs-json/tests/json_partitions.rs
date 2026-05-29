use elkrs_core::geometry::{Point, Size};
use elkrs_core::graph::{ElementId, ElkGraph, ElkNode, ElkPort};
use elkrs_core::options::{Direction, PortSide};
use elkrs_json::{from_str, to_string_pretty};

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
fn round_trips_left_and_up_direction_options() {
    let mut left = ElkGraph::new("left");
    left.properties.set_direction(Direction::Left);
    let mut up = ElkGraph::new("up");
    up.properties.set_direction(Direction::Up);

    assert_eq!(
        from_str(&to_string_pretty(&left).unwrap())
            .unwrap()
            .properties
            .direction(),
        Direction::Left,
    );
    assert_eq!(
        from_str(&to_string_pretty(&up).unwrap())
            .unwrap()
            .properties
            .direction(),
        Direction::Up,
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
fn round_trips_north_and_south_port_sides() {
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

    let reparsed = from_str(&to_string_pretty(&graph).unwrap()).unwrap();
    let node = &reparsed.nodes[&ElementId::from("node")];

    assert_eq!(
        node.ports[&ElementId::from("north")].side,
        Some(PortSide::North)
    );
    assert_eq!(
        node.ports[&ElementId::from("south")].side,
        Some(PortSide::South)
    );
}

fn port(id: &str, side: PortSide, position: Point, size: Size) -> ElkPort {
    let mut port = ElkPort::new(id);
    port.side = Some(side);
    port.position = position;
    port.size = size;
    port
}
