use elkrs_core::diagnostic::Severity;
use elkrs_core::geometry::{Point, Size};
use elkrs_core::graph::{ElementId, ElementRef, ElkEdge, ElkGraph, ElkNode, ElkPort};
use elkrs_core::layout::LayoutError;
use elkrs_core::options::{Algorithm, Direction, HierarchyHandling, PortSide};
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
fn layered_layout_writes_child_coordinates_as_absolute_output() {
    let mut group = node("group", 200.0, 120.0);
    group.add_child(node("child", 40.0, 30.0));
    let mut graph = ElkGraph::new("root");
    graph.add_node(group);

    LayeredLayout.layout(&mut graph).unwrap();

    let group = &graph.nodes[&ElementId::from("group")];
    let child = &group.children[&ElementId::from("child")];
    assert!(child.position.x >= group.position.x);
    assert!(child.position.y >= group.position.y);
}

#[test]
fn layered_layout_accepts_parent_child_edge_endpoints() {
    let mut group = node("group", 200.0, 120.0);
    group.add_child(node("child", 40.0, 30.0));
    let mut graph = ElkGraph::new("root");
    graph.add_node(group);
    graph.add_edge(ElkEdge::new(
        "edge",
        ElementRef::Node(ElementId::from("group")),
        ElementRef::Node(ElementId::from("child")),
    ));

    LayeredLayout.layout(&mut graph).unwrap();

    assert_eq!(
        graph.edges[&ElementId::from("edge")].sections[0]
            .points
            .len(),
        4
    );
}

#[test]
fn layered_layout_rejects_duplicate_nested_node_ids() {
    let mut group = node("group", 200.0, 120.0);
    group.add_child(node("duplicate", 40.0, 30.0));
    let mut graph = ElkGraph::new("root");
    graph.add_node(node("duplicate", 40.0, 30.0));
    graph.add_node(group);

    let error = LayeredLayout.layout(&mut graph).unwrap_err();

    assert!(matches!(
        error,
        LayoutError::InvalidHierarchy(message)
            if message.contains("duplicate node id: duplicate")
    ));
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

#[test]
fn layered_layout_routes_from_port_anchor_geometry() {
    let mut source = node("source", 100.0, 40.0);
    source.add_port(port_with_geometry(
        "out",
        PortSide::East,
        Point::new(90.0, 5.0),
        Size::new(10.0, 10.0),
    ));
    let mut target = node("target", 100.0, 40.0);
    target.add_port(port_with_geometry(
        "in",
        PortSide::West,
        Point::new(0.0, 25.0),
        Size::new(10.0, 10.0),
    ));

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

    let source = &graph.nodes[&ElementId::from("source")];
    let target = &graph.nodes[&ElementId::from("target")];
    let section = &graph.edges[&ElementId::from("edge")].sections[0];

    assert_eq!(
        section.points[0],
        Point::new(source.position.x + 100.0, source.position.y + 10.0)
    );
    assert_eq!(
        *section.points.last().unwrap(),
        Point::new(target.position.x, target.position.y + 30.0)
    );
}

#[test]
fn layered_layout_rejects_non_layered_algorithm_option() {
    let mut graph = ElkGraph::new("root");
    graph
        .properties
        .set_algorithm(Algorithm::Other("org.eclipse.elk.force".to_string()));
    graph.add_node(node("source", 60.0, 30.0));

    let error = LayeredLayout.layout(&mut graph).unwrap_err();

    assert!(matches!(
        error,
        LayoutError::UnsupportedAlgorithm(algorithm)
            if algorithm == "org.eclipse.elk.force"
    ));
}

#[test]
fn layered_layout_reports_unsupported_hierarchy_handling() {
    let mut graph = ElkGraph::new("root");
    graph
        .properties
        .set_hierarchy_handling(HierarchyHandling::SeparateChildren);
    graph.add_node(node("source", 60.0, 30.0));

    let report = LayeredLayout.layout(&mut graph).unwrap();

    assert!(report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "ELKRS_LAYERED_UNSUPPORTED_OPTION"
            && diagnostic.severity == Severity::Warning
            && diagnostic.message.contains("hierarchy handling")
    }));
}

#[test]
fn layered_layout_reports_unimplemented_edge_spacing_options() {
    let mut graph = ElkGraph::new("root");
    graph.properties.set_spacing_edge_node(12.0);
    graph.properties.set_spacing_edge_edge(24.0);
    graph.add_node(node("source", 60.0, 30.0));
    graph.add_node(node("target", 60.0, 30.0));
    graph.add_edge(ElkEdge::new(
        "edge",
        ElementRef::Node(ElementId::from("source")),
        ElementRef::Node(ElementId::from("target")),
    ));

    let report = LayeredLayout.layout(&mut graph).unwrap();

    assert!(report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "ELKRS_LAYERED_UNSUPPORTED_OPTION"
            && diagnostic.severity == Severity::Warning
            && diagnostic.message.contains("edge-node spacing")
    }));
    assert!(report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "ELKRS_LAYERED_UNSUPPORTED_OPTION"
            && diagnostic.severity == Severity::Warning
            && diagnostic.message.contains("edge-edge spacing")
    }));
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

fn port_with_geometry(id: &str, side: PortSide, position: Point, size: Size) -> ElkPort {
    let mut port = ElkPort::new(id);
    port.side = Some(side);
    port.position = position;
    port.size = size;
    port
}
