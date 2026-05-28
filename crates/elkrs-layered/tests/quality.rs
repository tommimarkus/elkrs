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
