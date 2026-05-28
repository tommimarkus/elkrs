use elkrs_core::geometry::{Point, Rect, Size};
use elkrs_core::graph::{ElementId, ElementRef, ElkEdge, ElkGraph, ElkNode};
use elkrs_layered::{LayeredLayout, LayoutAlgorithm};

#[test]
fn simple_chain_has_no_node_overlap_and_routed_edges() {
    let mut graph = ElkGraph::new("root");
    for id in ["a", "b", "c"] {
        graph.add_node(node(id, 40.0, 30.0));
    }
    graph.add_edge(edge("ab", "a", "b"));
    graph.add_edge(edge("bc", "b", "c"));

    LayeredLayout.layout(&mut graph).unwrap();

    assert_eq!(overlap_count(&graph), 0);
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
fn same_layer_large_nodes_do_not_overlap() {
    let mut graph = ElkGraph::new("root");
    graph.add_node(node("a", 80.0, 200.0));
    graph.add_node(node("b", 80.0, 200.0));

    LayeredLayout.layout(&mut graph).unwrap();

    assert_eq!(overlap_count(&graph), 0);
}

#[test]
fn adjacent_layer_large_nodes_do_not_overlap() {
    let mut graph = ElkGraph::new("root");
    graph.add_node(node("a", 240.0, 40.0));
    graph.add_node(node("b", 240.0, 40.0));
    graph.add_edge(edge("ab", "a", "b"));

    LayeredLayout.layout(&mut graph).unwrap();

    assert_eq!(overlap_count(&graph), 0);
}

fn node(id: &str, width: f64, height: f64) -> ElkNode {
    let mut node = ElkNode::new(id);
    node.size = Size::new(width, height);
    node
}

fn edge(id: &str, source: &str, target: &str) -> ElkEdge {
    ElkEdge::new(
        id,
        ElementRef::Node(ElementId::from(source)),
        ElementRef::Node(ElementId::from(target)),
    )
}

fn assert_point_on_node(point: Point, node: &ElkNode) {
    assert!(point.x >= node.position.x);
    assert!(point.x <= node.position.x + node.size.width);
    assert!(point.y >= node.position.y);
    assert!(point.y <= node.position.y + node.size.height);
}

fn overlap_count(graph: &ElkGraph) -> usize {
    let nodes = graph.nodes.values().collect::<Vec<_>>();
    let mut count = 0;
    for left in 0..nodes.len() {
        for right in left + 1..nodes.len() {
            let left_rect = Rect::new(nodes[left].position, nodes[left].size);
            let right_rect = Rect::new(
                Point::new(nodes[right].position.x, nodes[right].position.y),
                nodes[right].size,
            );
            if left_rect.intersects(&right_rect) {
                count += 1;
            }
        }
    }
    count
}
