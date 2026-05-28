use elkrs_core::geometry::Size;
use elkrs_core::graph::{ElementId, ElementRef, ElkEdge, ElkGraph, ElkNode, ElkPort};
use elkrs_core::options::{Direction, PortSide};
use elkrs_layered::{LayeredLayout, LayoutAlgorithm};

#[test]
fn layered_layout_respects_down_direction() {
    let mut graph = ElkGraph::new("root");
    graph.properties.set_direction(Direction::Down);
    graph.add_node(node("source", 60.0, 30.0));
    graph.add_node(node("target", 60.0, 30.0));
    graph.add_edge(ElkEdge::new(
        "edge",
        ElementRef::Node(ElementId::from("source")),
        ElementRef::Node(ElementId::from("target")),
    ));

    LayeredLayout.layout(&mut graph).unwrap();

    let source = &graph.nodes[&ElementId::from("source")];
    let target = &graph.nodes[&ElementId::from("target")];
    assert!(target.position.y > source.position.y);
}

#[test]
fn layered_layout_returns_missing_endpoint_error() {
    let mut graph = ElkGraph::new("root");
    graph.add_node(node("source", 60.0, 30.0));
    graph.add_edge(ElkEdge::new(
        "edge",
        ElementRef::Node(ElementId::from("source")),
        ElementRef::Node(ElementId::from("missing")),
    ));

    let error = LayeredLayout.layout(&mut graph).unwrap_err();

    assert_eq!(error.to_string(), "missing endpoint: missing");
}

#[test]
fn layered_layout_keeps_child_node_layout_available() {
    let mut group = node("group", 200.0, 120.0);
    group.add_child(node("child", 40.0, 30.0));
    let mut graph = ElkGraph::new("root");
    graph.add_node(group);

    LayeredLayout.layout(&mut graph).unwrap();

    let child = &graph.nodes[&ElementId::from("group")].children[&ElementId::from("child")];
    assert!(child.position.y > 0.0);
}

#[test]
fn layered_layout_accepts_edges_connected_to_ports() {
    let mut source = node("source", 60.0, 30.0);
    source.add_port(port("out", PortSide::East));
    let mut target = node("target", 60.0, 30.0);
    target.add_port(port("in", PortSide::West));

    let mut graph = ElkGraph::new("root");
    graph.add_node(source);
    graph.add_node(target);
    graph.add_edge(ElkEdge::new(
        "edge",
        ElementRef::Port {
            node: ElementId::from("source"),
            port: ElementId::from("out"),
        },
        ElementRef::Port {
            node: ElementId::from("target"),
            port: ElementId::from("in"),
        },
    ));

    LayeredLayout.layout(&mut graph).unwrap();

    assert_eq!(
        graph.edges[&ElementId::from("edge")].sections[0]
            .points
            .len(),
        4
    );
}

fn node(id: &str, width: f64, height: f64) -> ElkNode {
    let mut node = ElkNode::new(id);
    node.size = Size::new(width, height);
    node
}

fn port(id: &str, side: PortSide) -> ElkPort {
    let mut port = ElkPort::new(id);
    port.side = Some(side);
    port
}
