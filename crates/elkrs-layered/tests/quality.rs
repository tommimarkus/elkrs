mod support;

use elkrs_core::geometry::{Point, Size};
use elkrs_core::graph::{ElementId, ElementRef, ElkEdge, ElkGraph};
use elkrs_core::options::PortSide;
use elkrs_layered::{LayeredLayout, LayoutAlgorithm};

use support::fixtures::{
    chain, cross_group_edge, diamond, edge, fan_in, fan_out, nested_group, node, port, port_heavy,
    two_layer_crossing,
};
use support::quality::{
    containment_violation_count, crossing_count, edge_through_node_count, node_overlap_count,
    port_anchor_mismatch_count, route_segment_count,
};

#[test]
fn simple_chain_has_no_node_overlap_and_routed_edges() {
    let mut graph = chain();

    LayeredLayout.layout(&mut graph).unwrap();

    assert_eq!(node_overlap_count(&graph), 0);
    assert!(graph.edges.values().all(|edge| edge.sections.len() == 1));
    assert!(graph
        .edges
        .values()
        .all(|edge| edge.sections[0].points.len() >= 2));
}

#[test]
fn backward_edge_section_follows_original_edge_direction() {
    let mut graph = ElkGraph::new("root");
    graph.add_node(node("a", 40.0, 30.0));
    graph.add_node(node("b", 40.0, 30.0));
    graph.add_edge(edge("ba", "b", "a"));

    LayeredLayout.layout(&mut graph).unwrap();

    let section = &graph.edges[&ElementId::from("ba")].sections[0];
    let source = &graph.nodes[&ElementId::from("b")];
    let target = &graph.nodes[&ElementId::from("a")];
    assert_point_on_node(section.points[0], source);
    assert_point_on_node(*section.points.last().unwrap(), target);
}

#[test]
fn backward_port_edge_section_follows_original_port_direction() {
    let mut a = node("a", 100.0, 40.0);
    a.add_port(port(
        "in",
        PortSide::West,
        Point::new(0.0, 25.0),
        Size::new(10.0, 10.0),
    ));
    let mut b = node("b", 100.0, 40.0);
    b.add_port(port(
        "out",
        PortSide::East,
        Point::new(90.0, 5.0),
        Size::new(10.0, 10.0),
    ));

    let mut graph = ElkGraph::new("root");
    graph.add_node(a);
    graph.add_node(b);
    graph.add_edge(ElkEdge::new(
        "ba",
        ElementRef::Port {
            node: ElementId::from("b"),
            port: ElementId::from("out"),
        },
        ElementRef::Port {
            node: ElementId::from("a"),
            port: ElementId::from("in"),
        },
    ));

    LayeredLayout.layout(&mut graph).unwrap();

    let a = &graph.nodes[&ElementId::from("a")];
    let b = &graph.nodes[&ElementId::from("b")];
    let section = &graph.edges[&ElementId::from("ba")].sections[0];

    assert_eq!(
        section.points[0],
        Point::new(b.position.x + 100.0, b.position.y + 10.0)
    );
    assert_eq!(
        *section.points.last().unwrap(),
        Point::new(a.position.x, a.position.y + 30.0)
    );
}

#[test]
fn same_layer_large_nodes_do_not_overlap() {
    let mut graph = ElkGraph::new("root");
    graph.add_node(node("a", 80.0, 200.0));
    graph.add_node(node("b", 80.0, 200.0));

    LayeredLayout.layout(&mut graph).unwrap();

    assert_eq!(node_overlap_count(&graph), 0);
}

#[test]
fn adjacent_layer_large_nodes_do_not_overlap() {
    let mut graph = ElkGraph::new("root");
    graph.add_node(node("a", 240.0, 40.0));
    graph.add_node(node("b", 240.0, 40.0));
    graph.add_edge(edge("ab", "a", "b"));

    LayeredLayout.layout(&mut graph).unwrap();

    assert_eq!(node_overlap_count(&graph), 0);
}

#[test]
fn custom_node_spacing_separates_same_layer_nodes() {
    let mut graph = ElkGraph::new("root");
    graph.properties.set_spacing_node_node(200.0);
    graph.add_node(node("a", 40.0, 30.0));
    graph.add_node(node("b", 40.0, 30.0));

    LayeredLayout.layout(&mut graph).unwrap();

    let a = &graph.nodes[&ElementId::from("a")];
    let b = &graph.nodes[&ElementId::from("b")];
    assert!(b.position.y >= a.position.y + a.size.height + 200.0);
}

#[test]
fn custom_layer_spacing_separates_connected_layers() {
    let mut graph = ElkGraph::new("root");
    graph.properties.set_spacing_layer_node_node(300.0);
    graph.add_node(node("a", 40.0, 30.0));
    graph.add_node(node("b", 40.0, 30.0));
    graph.add_edge(edge("ab", "a", "b"));

    LayeredLayout.layout(&mut graph).unwrap();

    let a = &graph.nodes[&ElementId::from("a")];
    let b = &graph.nodes[&ElementId::from("b")];
    assert!(b.position.x >= a.position.x + a.size.width + 300.0);
}

#[test]
fn diamond_fixture_has_structural_metrics() {
    assert_structural_metrics(diamond());
}

#[test]
fn fan_in_fixture_has_structural_metrics() {
    assert_structural_metrics(fan_in());
}

#[test]
fn fan_out_fixture_has_structural_metrics() {
    assert_structural_metrics(fan_out());
}

#[test]
fn cross_group_edge_fixture_has_structural_metrics() {
    assert_structural_metrics(cross_group_edge());
}

#[test]
fn chain_metrics_report_no_crossings_or_route_through_nodes() {
    let mut graph = chain();

    LayeredLayout.layout(&mut graph).unwrap();

    assert_eq!(crossing_count(&graph), 0);
    assert_eq!(edge_through_node_count(&graph), 0);
}

#[test]
fn crossing_minimization_reorders_two_layer_targets() {
    let mut graph = two_layer_crossing();

    LayeredLayout.layout(&mut graph).unwrap();

    assert!(
        graph.nodes[&ElementId::from("c")].position.y
            < graph.nodes[&ElementId::from("d")].position.y
    );
    assert_eq!(crossing_count(&graph), 0);
}

#[test]
fn nested_group_fixture_has_contained_children() {
    let mut graph = nested_group();

    LayeredLayout.layout(&mut graph).unwrap();

    assert_eq!(containment_violation_count(&graph), 0);
}

#[test]
fn port_heavy_fixture_preserves_port_anchor_fidelity() {
    let mut graph = port_heavy();

    LayeredLayout.layout(&mut graph).unwrap();

    assert_eq!(port_anchor_mismatch_count(&graph), 0);
    assert!(route_segment_count(&graph) >= graph.edges.len());
}

fn assert_structural_metrics(mut graph: ElkGraph) {
    LayeredLayout.layout(&mut graph).unwrap();

    assert_eq!(node_overlap_count(&graph), 0);
    assert!(route_segment_count(&graph) >= graph.edges.len());
}

fn assert_point_on_node(point: Point, node: &elkrs_core::graph::ElkNode) {
    assert!(point.x >= node.position.x);
    assert!(point.x <= node.position.x + node.size.width);
    assert!(point.y >= node.position.y);
    assert!(point.y <= node.position.y + node.size.height);
}
