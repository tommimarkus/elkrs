mod support;

use elkrs_core::diagnostic::Severity;
use elkrs_core::geometry::{Point, Size};
use elkrs_core::graph::{ElementId, ElementRef, ElkEdge, ElkGraph, ElkNode, ElkPort};
use elkrs_core::layout::LayoutError;
use elkrs_core::options::{
    Algorithm, Direction, EdgeRouting, HierarchyHandling, PortSide, Properties, PropertyValue,
};
use elkrs_layered::{LayeredLayout, LayoutAlgorithm};

use support::quality::major_axis_edge_node_clearance;

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
fn layered_layout_respects_left_direction() {
    let mut graph = ElkGraph::new("root");
    graph.properties.set_direction(Direction::Left);
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
    let section = &graph.edges[&ElementId::from("edge")].sections[0];

    assert!(target.position.x < source.position.x);
    assert_eq!(
        section.points[0],
        Point::new(
            source.position.x,
            source.position.y + source.size.height / 2.0
        )
    );
    assert_eq!(
        *section.points.last().unwrap(),
        Point::new(
            target.position.x + target.size.width,
            target.position.y + target.size.height / 2.0
        )
    );
}

#[test]
fn layered_layout_respects_up_direction() {
    let mut graph = ElkGraph::new("root");
    graph.properties.set_direction(Direction::Up);
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
    let section = &graph.edges[&ElementId::from("edge")].sections[0];

    assert!(target.position.y < source.position.y);
    assert_eq!(
        section.points[0],
        Point::new(
            source.position.x + source.size.width / 2.0,
            source.position.y
        )
    );
    assert_eq!(
        *section.points.last().unwrap(),
        Point::new(
            target.position.x + target.size.width / 2.0,
            target.position.y + target.size.height
        )
    );
}

#[test]
fn layered_layout_is_stable_across_node_insertion_order() {
    let mut forward = chain_with_node_order(["a", "b", "c"]);
    let mut reverse = chain_with_node_order(["c", "b", "a"]);

    LayeredLayout.layout(&mut forward).unwrap();
    LayeredLayout.layout(&mut reverse).unwrap();

    for id in ["a", "b", "c"] {
        assert_eq!(
            forward.nodes[&ElementId::from(id)].position,
            reverse.nodes[&ElementId::from(id)].position
        );
    }
}

#[test]
fn layered_layout_routes_multi_node_cycle_in_original_edge_directions() {
    let mut graph = ElkGraph::new("root");
    graph.add_node(node("a", 60.0, 30.0));
    graph.add_node(node("b", 60.0, 30.0));
    graph.add_node(node("c", 60.0, 30.0));
    graph.add_edge(ElkEdge::new(
        "ab",
        ElementRef::Node(ElementId::from("a")),
        ElementRef::Node(ElementId::from("b")),
    ));
    graph.add_edge(ElkEdge::new(
        "bc",
        ElementRef::Node(ElementId::from("b")),
        ElementRef::Node(ElementId::from("c")),
    ));
    graph.add_edge(ElkEdge::new(
        "ca",
        ElementRef::Node(ElementId::from("c")),
        ElementRef::Node(ElementId::from("a")),
    ));

    LayeredLayout.layout(&mut graph).unwrap();

    for (edge_id, source_id, target_id) in [("ab", "a", "b"), ("bc", "b", "c"), ("ca", "c", "a")] {
        let section = &graph.edges[&ElementId::from(edge_id)].sections[0];
        assert_point_on_node(section.points[0], &graph.nodes[&ElementId::from(source_id)]);
        assert_point_on_node(
            *section.points.last().unwrap(),
            &graph.nodes[&ElementId::from(target_id)],
        );
    }
}

#[test]
fn self_loop_does_not_change_connected_node_layers() {
    let mut baseline = ElkGraph::new("root");
    baseline.add_node(node("source", 60.0, 30.0));
    baseline.add_node(node("target", 60.0, 30.0));
    baseline.add_edge(ElkEdge::new(
        "edge",
        ElementRef::Node(ElementId::from("source")),
        ElementRef::Node(ElementId::from("target")),
    ));

    let mut with_self_loop = baseline.clone();
    with_self_loop.add_edge(ElkEdge::new(
        "self",
        ElementRef::Node(ElementId::from("source")),
        ElementRef::Node(ElementId::from("source")),
    ));

    LayeredLayout.layout(&mut baseline).unwrap();
    LayeredLayout.layout(&mut with_self_loop).unwrap();

    for id in ["source", "target"] {
        assert_eq!(
            baseline.nodes[&ElementId::from(id)].position,
            with_self_loop.nodes[&ElementId::from(id)].position
        );
    }
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

    let group = &graph.nodes[&ElementId::from("group")];
    let child = &group.children[&ElementId::from("child")];
    let section = &graph.edges[&ElementId::from("edge")].sections[0];

    assert!(section.points.len() >= 2);
    assert_eq!(
        section.points[0],
        Point::new(group.position.x, group.position.y + group.size.height / 2.0)
    );
    assert_eq!(
        *section.points.last().unwrap(),
        Point::new(
            child.position.x + child.size.width,
            child.position.y + child.size.height / 2.0
        )
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
    source.add_port(port_with_geometry(
        "out",
        PortSide::East,
        Point::new(60.0, 15.0),
        Size::new(0.0, 0.0),
    ));
    let mut target = node("target", 60.0, 30.0);
    target.add_port(port_with_geometry(
        "in",
        PortSide::West,
        Point::new(0.0, 15.0),
        Size::new(0.0, 0.0),
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

    assert!(section.points.len() >= 2);
    assert_eq!(
        section.points[0],
        Point::new(source.position.x + 60.0, source.position.y + 15.0)
    );
    assert_eq!(
        *section.points.last().unwrap(),
        Point::new(target.position.x, target.position.y + 15.0)
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
fn layered_layout_routes_from_north_and_south_port_anchor_geometry() {
    let mut source = node("source", 100.0, 40.0);
    source.add_port(port_with_geometry(
        "out",
        PortSide::South,
        Point::new(45.0, 30.0),
        Size::new(10.0, 10.0),
    ));
    let mut target = node("target", 100.0, 40.0);
    target.add_port(port_with_geometry(
        "in",
        PortSide::North,
        Point::new(45.0, 0.0),
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

    assert!(section.points.len() >= 2);
    assert_eq!(
        section.points[0],
        Point::new(source.position.x + 50.0, source.position.y + 40.0)
    );
    assert_eq!(
        *section.points.last().unwrap(),
        Point::new(target.position.x + 50.0, target.position.y)
    );
}

#[test]
fn layered_layout_routes_node_self_loop_outside_node() {
    let mut graph = ElkGraph::new("root");
    graph.add_node(node("a", 80.0, 40.0));
    graph.add_edge(ElkEdge::new(
        "aa",
        ElementRef::Node(ElementId::from("a")),
        ElementRef::Node(ElementId::from("a")),
    ));

    LayeredLayout.layout(&mut graph).unwrap();

    let node = &graph.nodes[&ElementId::from("a")];
    let section = &graph.edges[&ElementId::from("aa")].sections[0];

    assert!(section.points.len() >= 4);
    assert!(section
        .points
        .iter()
        .any(|point| point.x > node.position.x + node.size.width));
    assert_eq!(
        section.points[0],
        Point::new(
            node.position.x + node.size.width,
            node.position.y + node.size.height * 0.35
        )
    );
    assert_eq!(
        *section.points.last().unwrap(),
        Point::new(
            node.position.x + node.size.width,
            node.position.y + node.size.height * 0.65
        )
    );
}

#[test]
fn layered_layout_routes_port_self_loop_from_port_anchors() {
    let mut node = node("a", 100.0, 60.0);
    node.add_port(port_with_geometry(
        "out",
        PortSide::East,
        Point::new(90.0, 10.0),
        Size::new(10.0, 10.0),
    ));
    node.add_port(port_with_geometry(
        "in",
        PortSide::East,
        Point::new(90.0, 40.0),
        Size::new(10.0, 10.0),
    ));

    let mut graph = ElkGraph::new("root");
    graph.add_node(node);
    graph.add_edge(ElkEdge::new(
        "aa",
        ElementRef::Port {
            node: ElementId::from("a"),
            port: ElementId::from("out"),
        },
        ElementRef::Port {
            node: ElementId::from("a"),
            port: ElementId::from("in"),
        },
    ));

    LayeredLayout.layout(&mut graph).unwrap();

    let node = &graph.nodes[&ElementId::from("a")];
    let section = &graph.edges[&ElementId::from("aa")].sections[0];

    assert_eq!(
        section.points[0],
        Point::new(node.position.x + 100.0, node.position.y + 15.0)
    );
    assert_eq!(
        *section.points.last().unwrap(),
        Point::new(node.position.x + 100.0, node.position.y + 45.0)
    );
    assert!(section
        .points
        .iter()
        .any(|point| point.x > node.position.x + node.size.width));
}

#[test]
fn layered_layout_routes_mixed_side_port_self_loop_around_node() {
    let mut node = node("a", 100.0, 60.0);
    node.add_port(port_with_geometry(
        "out",
        PortSide::East,
        Point::new(90.0, 10.0),
        Size::new(10.0, 10.0),
    ));
    node.add_port(port_with_geometry(
        "in",
        PortSide::West,
        Point::new(0.0, 40.0),
        Size::new(10.0, 10.0),
    ));

    let mut graph = ElkGraph::new("root");
    graph.add_node(node);
    graph.add_edge(ElkEdge::new(
        "aa",
        ElementRef::Port {
            node: ElementId::from("a"),
            port: ElementId::from("out"),
        },
        ElementRef::Port {
            node: ElementId::from("a"),
            port: ElementId::from("in"),
        },
    ));

    LayeredLayout.layout(&mut graph).unwrap();

    let node = &graph.nodes[&ElementId::from("a")];
    let section = &graph.edges[&ElementId::from("aa")].sections[0];

    assert_eq!(
        section.points[0],
        Point::new(node.position.x + 100.0, node.position.y + 15.0)
    );
    assert_eq!(
        *section.points.last().unwrap(),
        Point::new(node.position.x, node.position.y + 45.0)
    );
    assert!(section
        .points
        .iter()
        .any(|point| point.y > node.position.y + node.size.height || point.y < node.position.y));
    assert!(section
        .points
        .iter()
        .any(|point| point.x > node.position.x + node.size.width || point.x < node.position.x));
    assert_no_axis_aligned_segment_through_node_interior(&section.points, node);
}

#[test]
fn layered_layout_routes_parallel_edges_as_distinct_sections() {
    let mut graph = ElkGraph::new("root");
    graph.add_node(node("a", 60.0, 30.0));
    graph.add_node(node("b", 60.0, 30.0));
    graph.add_edge(ElkEdge::new(
        "ab-1",
        ElementRef::Node(ElementId::from("a")),
        ElementRef::Node(ElementId::from("b")),
    ));
    graph.add_edge(ElkEdge::new(
        "ab-2",
        ElementRef::Node(ElementId::from("a")),
        ElementRef::Node(ElementId::from("b")),
    ));

    LayeredLayout.layout(&mut graph).unwrap();

    let first = &graph.edges[&ElementId::from("ab-1")].sections[0].points;
    let second = &graph.edges[&ElementId::from("ab-2")].sections[0].points;

    assert_ne!(first, second);
    assert_eq!(first[0], second[0]);
    assert_eq!(first.last(), second.last());
}

#[test]
fn layered_layout_applies_edge_edge_spacing_to_parallel_routes() {
    let mut graph = ElkGraph::new("root");
    graph.properties.set_spacing_edge_edge(36.0);
    graph.add_node(node("a", 60.0, 30.0));
    graph.add_node(node("b", 60.0, 30.0));
    graph.add_edge(ElkEdge::new(
        "ab-1",
        ElementRef::Node(ElementId::from("a")),
        ElementRef::Node(ElementId::from("b")),
    ));
    graph.add_edge(ElkEdge::new(
        "ab-2",
        ElementRef::Node(ElementId::from("a")),
        ElementRef::Node(ElementId::from("b")),
    ));

    let report = LayeredLayout.layout(&mut graph).unwrap();

    assert!(report.diagnostics.iter().all(|diagnostic| {
        !diagnostic.message.contains("edge-edge spacing")
            && !diagnostic.message.contains("edge-node spacing")
    }));

    let first = &graph.edges[&ElementId::from("ab-1")].sections[0].points;
    let second = &graph.edges[&ElementId::from("ab-2")].sections[0].points;

    assert_eq!((first[2].y - second[2].y).abs(), 36.0);
}

#[test]
fn layered_layout_applies_edge_node_spacing_to_obstacle_detours() {
    let mut graph = ElkGraph::new("root");
    graph.properties.set_spacing_edge_node(48.0);
    graph.add_node(node("a-source", 40.0, 30.0));
    graph.add_node(node("b-obstacle", 40.0, 80.0));
    graph.add_node(node("c-target", 40.0, 30.0));
    graph.add_edge(ElkEdge::new(
        "direct",
        ElementRef::Node(ElementId::from("a-source")),
        ElementRef::Node(ElementId::from("c-target")),
    ));
    graph.add_edge(ElkEdge::new(
        "source-obstacle",
        ElementRef::Node(ElementId::from("a-source")),
        ElementRef::Node(ElementId::from("b-obstacle")),
    ));
    graph.add_edge(ElkEdge::new(
        "obstacle-target",
        ElementRef::Node(ElementId::from("b-obstacle")),
        ElementRef::Node(ElementId::from("c-target")),
    ));

    let report = LayeredLayout.layout(&mut graph).unwrap();

    assert!(report.diagnostics.iter().all(|diagnostic| {
        !diagnostic.message.contains("edge-node spacing")
            && !diagnostic.message.contains("edge-edge spacing")
    }));

    let obstacle = &graph.nodes[&ElementId::from("b-obstacle")];
    let section = &graph.edges[&ElementId::from("direct")].sections[0];
    let expected_detour_y = obstacle.position.y + obstacle.size.height + 48.0;

    assert!(section
        .points
        .iter()
        .any(|point| (point.y - expected_detour_y).abs() < f64::EPSILON));
    let clearance = major_axis_edge_node_clearance(&graph, "direct", "b-obstacle").unwrap();
    assert!(
        clearance + f64::EPSILON >= 48.0,
        "direct edge should keep at least 48.0 clearance from the obstacle: {clearance}"
    );
    assert_no_axis_aligned_segment_through_node_interior(&section.points, obstacle);
}

#[test]
fn layered_layout_preserves_edge_node_spacing_for_parallel_obstacle_detours() {
    let mut graph = ElkGraph::new("root");
    graph.properties.set_spacing_edge_node(48.0);
    graph.properties.set_spacing_edge_edge(120.0);
    graph.add_node(node("a-source", 40.0, 30.0));
    graph.add_node(node("b-obstacle", 40.0, 80.0));
    graph.add_node(node("c-target", 40.0, 30.0));
    graph.add_edge(ElkEdge::new(
        "direct-1",
        ElementRef::Node(ElementId::from("a-source")),
        ElementRef::Node(ElementId::from("c-target")),
    ));
    graph.add_edge(ElkEdge::new(
        "direct-2",
        ElementRef::Node(ElementId::from("a-source")),
        ElementRef::Node(ElementId::from("c-target")),
    ));
    graph.add_edge(ElkEdge::new(
        "source-obstacle",
        ElementRef::Node(ElementId::from("a-source")),
        ElementRef::Node(ElementId::from("b-obstacle")),
    ));
    graph.add_edge(ElkEdge::new(
        "obstacle-target",
        ElementRef::Node(ElementId::from("b-obstacle")),
        ElementRef::Node(ElementId::from("c-target")),
    ));

    LayeredLayout.layout(&mut graph).unwrap();

    let obstacle = &graph.nodes[&ElementId::from("b-obstacle")];
    for edge_id in ["direct-1", "direct-2"] {
        let section = &graph.edges[&ElementId::from(edge_id)].sections[0];
        let clearance = major_axis_edge_node_clearance(&graph, edge_id, "b-obstacle").unwrap();
        assert!(
            clearance + f64::EPSILON >= 48.0,
            "{edge_id} should keep at least 48.0 clearance from the obstacle: {clearance}"
        );
        assert_no_axis_aligned_segment_through_node_interior(&section.points, obstacle);
    }
}

#[test]
fn layered_layout_rejects_negative_edge_node_spacing() {
    let mut graph = ElkGraph::new("root");
    graph.properties.set_spacing_edge_node(-1.0);
    graph.add_node(node("source", 60.0, 30.0));

    let error = LayeredLayout.layout(&mut graph).unwrap_err();

    assert!(matches!(
        error,
        LayoutError::InvalidOption(message)
            if message.contains("edge-node spacing")
    ));
}

#[test]
fn layered_layout_rejects_negative_edge_edge_spacing() {
    let mut graph = ElkGraph::new("root");
    graph.properties.set_spacing_edge_edge(-1.0);
    graph.add_node(node("source", 60.0, 30.0));

    let error = LayeredLayout.layout(&mut graph).unwrap_err();

    assert!(matches!(
        error,
        LayoutError::InvalidOption(message)
            if message.contains("edge-edge spacing")
    ));
}

#[test]
fn layered_layout_rejects_negative_layered_edge_spacing() {
    let mut graph = ElkGraph::new("root");
    graph.properties.set_spacing_edge_node_between_layers(-1.0);
    graph.add_node(node("source", 60.0, 30.0));

    let error = LayeredLayout.layout(&mut graph).unwrap_err();

    assert!(matches!(
        error,
        LayoutError::InvalidOption(message)
            if message.contains("edge-node between-layers spacing")
    ));
}

#[test]
fn layered_layout_rejects_negative_layered_edge_edge_spacing() {
    let mut graph = ElkGraph::new("root");
    graph.properties.set_spacing_edge_edge_between_layers(-1.0);
    graph.add_node(node("source", 60.0, 30.0));

    let error = LayeredLayout.layout(&mut graph).unwrap_err();

    assert!(matches!(
        error,
        LayoutError::InvalidOption(message)
            if message.contains("edge-edge between-layers spacing")
    ));
}

#[test]
fn layered_layout_rejects_negative_additional_spacing() {
    assert_negative_additional_spacing_rejected("spacing base value", |properties| {
        properties.set_spacing_base_value(-1.0);
    });
    assert_negative_additional_spacing_rejected("additional edge wrapping spacing", |properties| {
        properties.set_wrapping_additional_edge_spacing(-1.0);
    });
    assert_negative_additional_spacing_rejected("comment-comment spacing", |properties| {
        properties.set_spacing_comment_comment(-1.0);
    });
    assert_negative_additional_spacing_rejected("comment-node spacing", |properties| {
        properties.set_spacing_comment_node(-1.0);
    });
    assert_negative_additional_spacing_rejected("component-component spacing", |properties| {
        properties.set_spacing_component_component(-1.0);
    });
    assert_negative_additional_spacing_rejected("node self-loop spacing", |properties| {
        properties.set_spacing_node_self_loop(-1.0);
    });
}

#[test]
fn layered_layout_rejects_negative_label_and_port_spacing() {
    assert_negative_additional_spacing_rejected("edge-label spacing", |properties| {
        properties.set_spacing_edge_label(-1.0);
    });
    assert_negative_additional_spacing_rejected("label-label spacing", |properties| {
        properties.set_spacing_label_label(-1.0);
    });
    assert_negative_additional_spacing_rejected("label-node spacing", |properties| {
        properties.set_spacing_label_node(-1.0);
    });
    assert_negative_additional_spacing_rejected("horizontal label-port spacing", |properties| {
        properties.set_spacing_label_port_horizontal(-1.0);
    });
    assert_negative_additional_spacing_rejected("vertical label-port spacing", |properties| {
        properties.set_spacing_label_port_vertical(-1.0);
    });
    assert_negative_additional_spacing_rejected("port-port spacing", |properties| {
        properties.set_spacing_port_port(-1.0);
    });
}

#[test]
fn layered_layout_rejects_negative_node_port_port_spacing() {
    let mut graph = ElkGraph::new("root");
    let mut source = node("source", 60.0, 30.0);
    source.properties.set_spacing_port_port(-1.0);
    graph.add_node(source);

    let error = LayeredLayout.layout(&mut graph).unwrap_err();

    assert!(matches!(
        error,
        LayoutError::InvalidOption(message)
            if message.contains("port-port spacing")
                && message.contains("source")
    ));
}

fn assert_negative_additional_spacing_rejected(name: &str, set_spacing: fn(&mut Properties)) {
    let mut graph = ElkGraph::new("root");
    set_spacing(&mut graph.properties);
    graph.add_node(node("source", 60.0, 30.0));

    let error = LayeredLayout.layout(&mut graph).unwrap_err();

    assert!(
        matches!(error, LayoutError::InvalidOption(message) if message.contains(name)),
        "{name} should be rejected with its option name"
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
fn layered_layout_reports_node_unsupported_hierarchy_handling() {
    let mut graph = ElkGraph::new("root");
    let mut child = node("child", 60.0, 30.0);
    child
        .properties
        .set_hierarchy_handling(HierarchyHandling::SeparateChildren);
    graph.add_node(child);

    let report = LayeredLayout.layout(&mut graph).unwrap();

    assert!(report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "ELKRS_LAYERED_UNSUPPORTED_OPTION"
            && diagnostic.severity == Severity::Warning
            && diagnostic.message.contains("hierarchy handling")
            && diagnostic.message.contains("child")
    }));
}

#[test]
fn layered_layout_reports_unsupported_non_orthogonal_edge_routing() {
    let mut graph = ElkGraph::new("root");
    graph.properties.set_edge_routing(EdgeRouting::Polyline);
    graph.add_node(node("source", 60.0, 30.0));

    let report = LayeredLayout.layout(&mut graph).unwrap();

    assert!(report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "ELKRS_LAYERED_UNSUPPORTED_OPTION"
            && diagnostic.severity == Severity::Warning
            && diagnostic.message.contains("edge routing")
            && diagnostic.message.contains("Polyline")
    }));
}

#[test]
fn layered_layout_reports_node_unsupported_non_orthogonal_edge_routing() {
    let mut graph = ElkGraph::new("root");
    let mut child = node("child", 60.0, 30.0);
    child.properties.set_edge_routing(EdgeRouting::Splines);
    graph.add_node(child);

    let report = LayeredLayout.layout(&mut graph).unwrap();

    assert!(report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "ELKRS_LAYERED_UNSUPPORTED_OPTION"
            && diagnostic.severity == Severity::Warning
            && diagnostic.message.contains("edge routing")
            && diagnostic.message.contains("Splines")
            && diagnostic.message.contains("child")
    }));
}

#[test]
fn layered_layout_reports_unsupported_debug_mode() {
    let mut graph = ElkGraph::new("root");
    graph.properties.set_debug_mode(true);
    graph.add_node(node("source", 60.0, 30.0));

    let report = LayeredLayout.layout(&mut graph).unwrap();

    assert!(report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "ELKRS_LAYERED_UNSUPPORTED_OPTION"
            && diagnostic.severity == Severity::Warning
            && diagnostic.message.contains("debug mode")
    }));
}

#[test]
fn layered_layout_reports_node_unsupported_debug_mode() {
    let mut graph = ElkGraph::new("root");
    let mut child = node("child", 60.0, 30.0);
    child.properties.set_debug_mode(true);
    graph.add_node(child);

    let report = LayeredLayout.layout(&mut graph).unwrap();

    assert!(report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "ELKRS_LAYERED_UNSUPPORTED_OPTION"
            && diagnostic.severity == Severity::Warning
            && diagnostic.message.contains("debug mode")
            && diagnostic.message.contains("child")
    }));
}

#[test]
fn layered_layout_reports_unsupported_feedback_edges() {
    let mut graph = ElkGraph::new("root");
    graph.properties.set_feedback_edges(true);
    graph.add_node(node("source", 60.0, 30.0));

    let report = LayeredLayout.layout(&mut graph).unwrap();

    assert!(report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "ELKRS_LAYERED_UNSUPPORTED_OPTION"
            && diagnostic.severity == Severity::Warning
            && diagnostic.message.contains("feedback edges")
    }));
}

#[test]
fn layered_layout_accepts_disabled_feedback_edges_without_diagnostic() {
    let mut graph = ElkGraph::new("root");
    graph.properties.set_feedback_edges(false);
    graph.add_node(node("source", 60.0, 30.0));

    let report = LayeredLayout.layout(&mut graph).unwrap();

    assert!(report.diagnostics.iter().all(|diagnostic| {
        diagnostic.code != "ELKRS_LAYERED_UNSUPPORTED_OPTION"
            || !diagnostic.message.contains("feedback edges")
    }));
}

#[test]
fn layered_layout_reports_node_unsupported_feedback_edges() {
    let mut graph = ElkGraph::new("root");
    let mut child = node("child", 60.0, 30.0);
    child.properties.set_feedback_edges(true);
    graph.add_node(child);

    let report = LayeredLayout.layout(&mut graph).unwrap();

    assert!(report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "ELKRS_LAYERED_UNSUPPORTED_OPTION"
            && diagnostic.severity == Severity::Warning
            && diagnostic.message.contains("feedback edges")
            && diagnostic.message.contains("child")
    }));
}

#[test]
fn layered_layout_reports_unsupported_parent_boolean_options() {
    for (name, setter) in parent_boolean_option_cases() {
        let mut graph = ElkGraph::new("root");
        setter(&mut graph.properties, true);
        graph.add_node(node("source", 60.0, 30.0));

        let report = LayeredLayout.layout(&mut graph).unwrap();

        assert!(
            report.diagnostics.iter().any(|diagnostic| {
                diagnostic.code == "ELKRS_LAYERED_UNSUPPORTED_OPTION"
                    && diagnostic.severity == Severity::Warning
                    && diagnostic.message.contains(name)
            }),
            "{name}"
        );
    }
}

#[test]
fn layered_layout_accepts_disabled_parent_boolean_options_without_diagnostic() {
    for (name, setter) in parent_boolean_option_cases() {
        let mut graph = ElkGraph::new("root");
        setter(&mut graph.properties, false);
        graph.add_node(node("source", 60.0, 30.0));

        let report = LayeredLayout.layout(&mut graph).unwrap();

        assert!(
            report.diagnostics.iter().all(|diagnostic| {
                diagnostic.code != "ELKRS_LAYERED_UNSUPPORTED_OPTION"
                    || !diagnostic.message.contains(name)
            }),
            "{name}"
        );
    }
}

#[test]
fn layered_layout_accepts_disabled_node_parent_boolean_options_without_diagnostic() {
    for (name, setter) in parent_boolean_option_cases() {
        let mut graph = ElkGraph::new("root");
        let mut child = node("child", 60.0, 30.0);
        setter(&mut child.properties, false);
        graph.add_node(child);

        let report = LayeredLayout.layout(&mut graph).unwrap();

        assert!(
            report.diagnostics.iter().all(|diagnostic| {
                diagnostic.code != "ELKRS_LAYERED_UNSUPPORTED_OPTION"
                    || !diagnostic.message.contains(name)
                    || !diagnostic.message.contains("child")
            }),
            "{name}"
        );
    }
}

#[test]
fn layered_layout_reports_node_unsupported_parent_boolean_options() {
    for (name, setter) in parent_boolean_option_cases() {
        let mut graph = ElkGraph::new("root");
        let mut child = node("child", 60.0, 30.0);
        setter(&mut child.properties, true);
        graph.add_node(child);

        let report = LayeredLayout.layout(&mut graph).unwrap();

        assert!(
            report.diagnostics.iter().any(|diagnostic| {
                diagnostic.code == "ELKRS_LAYERED_UNSUPPORTED_OPTION"
                    && diagnostic.severity == Severity::Warning
                    && diagnostic.message.contains(name)
                    && diagnostic.message.contains("child")
            }),
            "{name}"
        );
    }
}

#[test]
fn layered_layout_reports_node_unsupported_node_boolean_options() {
    for (name, setter) in node_boolean_option_cases() {
        let mut graph = ElkGraph::new("root");
        let mut child = node("child", 60.0, 30.0);
        setter(&mut child.properties, true);
        graph.add_node(child);

        let report = LayeredLayout.layout(&mut graph).unwrap();

        assert!(
            report.diagnostics.iter().any(|diagnostic| {
                diagnostic.code == "ELKRS_LAYERED_UNSUPPORTED_OPTION"
                    && diagnostic.severity == Severity::Warning
                    && diagnostic.message.contains(name)
                    && diagnostic.message.contains("child")
            }),
            "{name}"
        );
    }
}

#[test]
fn layered_layout_accepts_disabled_node_boolean_options_without_diagnostic() {
    for (name, setter) in node_boolean_option_cases() {
        let mut graph = ElkGraph::new("root");
        let mut child = node("child", 60.0, 30.0);
        setter(&mut child.properties, false);
        graph.add_node(child);

        let report = LayeredLayout.layout(&mut graph).unwrap();

        assert!(
            report.diagnostics.iter().all(|diagnostic| {
                diagnostic.code != "ELKRS_LAYERED_UNSUPPORTED_OPTION"
                    || !diagnostic.message.contains(name)
                    || !diagnostic.message.contains("child")
            }),
            "{name}"
        );
    }
}

fn node(id: &str, width: f64, height: f64) -> ElkNode {
    let mut node = ElkNode::new(id);
    node.size = Size::new(width, height);
    node
}

type BoolOptionSetter = fn(&mut Properties, bool) -> Option<PropertyValue>;

fn parent_boolean_option_cases() -> [(&'static str, BoolOptionSetter); 17] {
    [
        ("interactive layout", Properties::set_interactive_layout),
        (
            "connected components compaction",
            Properties::set_connected_components_compaction,
        ),
        ("consider port order", Properties::set_consider_port_order),
        (
            "force node model order",
            Properties::set_force_node_model_order,
        ),
        (
            "semi-interactive crossing minimization",
            Properties::set_semi_interactive_crossing_minimization,
        ),
        (
            "generate position and layer IDs",
            Properties::set_generate_position_and_layer_ids,
        ),
        (
            "high degree node treatment",
            Properties::set_high_degree_node_treatment,
        ),
        ("merge edges", Properties::set_merge_edges),
        (
            "merge hierarchy-crossing edges",
            Properties::set_merge_hierarchy_edges,
        ),
        ("favor straight edges", Properties::set_favor_straight_edges),
        (
            "unnecessary bendpoints",
            Properties::set_unnecessary_bendpoints,
        ),
        ("improve cuts", Properties::set_wrapping_improve_cuts),
        (
            "improve wrapped edges",
            Properties::set_wrapping_improve_wrapped_edges,
        ),
        ("fixed graph size", Properties::set_fixed_graph_size),
        ("layout partitioning", Properties::set_layout_partitioning),
        (
            "separate connected components",
            Properties::set_separate_connected_components,
        ),
        ("topdown layout", Properties::set_topdown_layout),
    ]
}

fn node_boolean_option_cases() -> [(&'static str, BoolOptionSetter); 8] {
    [
        ("comment box", Properties::set_comment_box),
        ("hypernode", Properties::set_hypernode),
        ("inside self-loops", Properties::set_inside_self_loops),
        ("no model order", Properties::set_no_model_order),
        ("no layout", Properties::set_no_layout),
        (
            "layer unzipping minimize edge length",
            Properties::set_layer_unzipping_minimize_edge_length,
        ),
        (
            "layer unzipping reset on long edges",
            Properties::set_layer_unzipping_reset_on_long_edges,
        ),
        (
            "port labels next to port if possible",
            Properties::set_port_labels_next_to_port_if_possible,
        ),
    ]
}

fn port_with_geometry(id: &str, side: PortSide, position: Point, size: Size) -> ElkPort {
    let mut port = ElkPort::new(id);
    port.side = Some(side);
    port.position = position;
    port.size = size;
    port
}

fn assert_point_on_node(point: Point, node: &ElkNode) {
    assert!(point.x >= node.position.x);
    assert!(point.x <= node.position.x + node.size.width);
    assert!(point.y >= node.position.y);
    assert!(point.y <= node.position.y + node.size.height);
}

fn assert_no_axis_aligned_segment_through_node_interior(points: &[Point], node: &ElkNode) {
    for segment in points.windows(2) {
        let start = segment[0];
        let end = segment[1];
        let crosses_interior = if (start.y - end.y).abs() < f64::EPSILON {
            start.y > node.position.y
                && start.y < node.position.y + node.size.height
                && start.x.min(end.x) < node.position.x + node.size.width
                && start.x.max(end.x) > node.position.x
        } else if (start.x - end.x).abs() < f64::EPSILON {
            start.x > node.position.x
                && start.x < node.position.x + node.size.width
                && start.y.min(end.y) < node.position.y + node.size.height
                && start.y.max(end.y) > node.position.y
        } else {
            false
        };
        assert!(
            !crosses_interior,
            "segment {start:?} -> {end:?} crosses node interior"
        );
    }
}

fn chain_with_node_order(ids: [&str; 3]) -> ElkGraph {
    let mut graph = ElkGraph::new("root");
    for id in ids {
        graph.add_node(node(id, 60.0, 30.0));
    }
    graph.add_edge(ElkEdge::new(
        "ab",
        ElementRef::Node(ElementId::from("a")),
        ElementRef::Node(ElementId::from("b")),
    ));
    graph.add_edge(ElkEdge::new(
        "bc",
        ElementRef::Node(ElementId::from("b")),
        ElementRef::Node(ElementId::from("c")),
    ));
    graph
}
