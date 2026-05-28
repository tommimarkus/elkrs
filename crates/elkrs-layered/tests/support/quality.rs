#![allow(dead_code)]

use elkrs_core::geometry::{Point, Rect};
use elkrs_core::graph::{ElementId, ElementRef, ElkEdge, ElkGraph, ElkNode, ElkPort};
use elkrs_core::options::PortSide;

const EPSILON: f64 = 0.000_001;

struct NodeBounds<'a> {
    id: &'a ElementId,
    node: &'a ElkNode,
    ancestors: Vec<&'a ElementId>,
}

pub fn node_overlap_count(graph: &ElkGraph) -> usize {
    let nodes = node_bounds(graph);
    let mut count = 0;
    for left in 0..nodes.len() {
        for right in left + 1..nodes.len() {
            if is_ancestor(nodes[left].id, &nodes[right])
                || is_ancestor(nodes[right].id, &nodes[left])
            {
                continue;
            }
            if rect(nodes[left].node).intersects(&rect(nodes[right].node)) {
                count += 1;
            }
        }
    }
    count
}

pub fn containment_violation_count(graph: &ElkGraph) -> usize {
    graph
        .nodes
        .values()
        .map(containment_violations_in_subtree)
        .sum()
}

pub fn route_segment_count(graph: &ElkGraph) -> usize {
    graph
        .edges
        .values()
        .flat_map(|edge| edge.sections.iter())
        .map(|section| section.points.windows(2).count())
        .sum()
}

pub fn edge_through_node_count(graph: &ElkGraph) -> usize {
    let nodes = node_bounds(graph);
    let mut count = 0;
    for edge in graph.edges.values() {
        let source_id = endpoint_node_id(&edge.source);
        let target_id = endpoint_node_id(&edge.target);
        for section in &edge.sections {
            for segment in section.points.windows(2) {
                let start = segment[0];
                let end = segment[1];
                for node in &nodes {
                    if node.id == source_id || node.id == target_id {
                        continue;
                    }
                    if segment_intersects_rect_interior(start, end, rect(node.node)) {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

pub fn crossing_count(graph: &ElkGraph) -> usize {
    let edges = graph.edges.values().collect::<Vec<_>>();
    let mut count = 0;
    for left_index in 0..edges.len() {
        for right_edge in edges.iter().skip(left_index + 1) {
            count += edge_crossing_count(edges[left_index], right_edge);
        }
    }
    count
}

pub fn port_anchor_mismatch_count(graph: &ElkGraph) -> usize {
    graph
        .edges
        .values()
        .filter(|edge| edge_has_port_anchor_mismatch(graph, edge))
        .count()
}

fn node_bounds(graph: &ElkGraph) -> Vec<NodeBounds<'_>> {
    let mut nodes = Vec::new();
    for node in graph.nodes.values() {
        collect_node_bounds(node, Vec::new(), &mut nodes);
    }
    nodes
}

fn collect_node_bounds<'a>(
    node: &'a ElkNode,
    ancestors: Vec<&'a ElementId>,
    nodes: &mut Vec<NodeBounds<'a>>,
) {
    nodes.push(NodeBounds {
        id: &node.id,
        node,
        ancestors: ancestors.clone(),
    });
    let mut child_ancestors = ancestors;
    child_ancestors.push(&node.id);
    for child in node.children.values() {
        collect_node_bounds(child, child_ancestors.clone(), nodes);
    }
}

fn is_ancestor(id: &ElementId, node: &NodeBounds<'_>) -> bool {
    node.ancestors.contains(&id)
}

fn containment_violations_in_subtree(node: &ElkNode) -> usize {
    let parent_rect = rect(node);
    let own_violations = node
        .children
        .values()
        .filter(|child| !contains_rect(parent_rect, rect(child)))
        .count();
    own_violations
        + node
            .children
            .values()
            .map(containment_violations_in_subtree)
            .sum::<usize>()
}

fn contains_rect(outer: Rect, inner: Rect) -> bool {
    inner.left() >= outer.left()
        && inner.right() <= outer.right()
        && inner.top() >= outer.top()
        && inner.bottom() <= outer.bottom()
}

fn edge_crossing_count(left: &ElkEdge, right: &ElkEdge) -> usize {
    let mut count = 0;
    for left_section in &left.sections {
        for left_segment in left_section.points.windows(2) {
            for right_section in &right.sections {
                for right_segment in right_section.points.windows(2) {
                    if segments_cross(
                        left_segment[0],
                        left_segment[1],
                        right_segment[0],
                        right_segment[1],
                    ) {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

fn segments_cross(a: Point, b: Point, c: Point, d: Point) -> bool {
    if is_vertical(a, b) && is_horizontal(c, d) {
        strict_between(a.x, c.x, d.x) && strict_between(c.y, a.y, b.y)
    } else if is_horizontal(a, b) && is_vertical(c, d) {
        strict_between(c.x, a.x, b.x) && strict_between(a.y, c.y, d.y)
    } else {
        false
    }
}

fn segment_intersects_rect_interior(start: Point, end: Point, rect: Rect) -> bool {
    if is_horizontal(start, end) {
        strict_between(start.y, rect.top(), rect.bottom())
            && ranges_overlap_interior(start.x, end.x, rect.left(), rect.right())
    } else if is_vertical(start, end) {
        strict_between(start.x, rect.left(), rect.right())
            && ranges_overlap_interior(start.y, end.y, rect.top(), rect.bottom())
    } else {
        false
    }
}

fn ranges_overlap_interior(a: f64, b: f64, min: f64, max: f64) -> bool {
    a.min(b) < max && a.max(b) > min
}

fn strict_between(value: f64, a: f64, b: f64) -> bool {
    value > a.min(b) && value < a.max(b)
}

fn is_horizontal(start: Point, end: Point) -> bool {
    (start.y - end.y).abs() < EPSILON
}

fn is_vertical(start: Point, end: Point) -> bool {
    (start.x - end.x).abs() < EPSILON
}

fn edge_has_port_anchor_mismatch(graph: &ElkGraph, edge: &ElkEdge) -> bool {
    let Some(section) = edge.sections.first() else {
        return true;
    };
    let Some(start) = section.points.first().copied() else {
        return true;
    };
    let Some(end) = section.points.last().copied() else {
        return true;
    };

    port_endpoint_anchor(graph, &edge.source).is_some_and(|expected| !same_point(start, expected))
        || port_endpoint_anchor(graph, &edge.target)
            .is_some_and(|expected| !same_point(end, expected))
}

fn port_endpoint_anchor(graph: &ElkGraph, endpoint: &ElementRef) -> Option<Point> {
    let ElementRef::Port { node, port } = endpoint else {
        return None;
    };
    let node = find_node(graph, node)?;
    let port = node.ports.get(port)?;
    Some(port_anchor(node, port))
}

fn find_node<'a>(graph: &'a ElkGraph, id: &ElementId) -> Option<&'a ElkNode> {
    graph
        .nodes
        .values()
        .find_map(|node| find_node_in_subtree(node, id))
}

fn find_node_in_subtree<'a>(node: &'a ElkNode, id: &ElementId) -> Option<&'a ElkNode> {
    if node.id == *id {
        return Some(node);
    }
    node.children
        .values()
        .find_map(|child| find_node_in_subtree(child, id))
}

fn endpoint_node_id(endpoint: &ElementRef) -> &ElementId {
    match endpoint {
        ElementRef::Node(node) => node,
        ElementRef::Port { node, .. } => node,
    }
}

fn port_anchor(node: &ElkNode, port: &ElkPort) -> Point {
    let origin = Point::new(
        node.position.x + port.position.x,
        node.position.y + port.position.y,
    );
    match port.side {
        Some(PortSide::North) => Point::new(origin.x + port.size.width / 2.0, origin.y),
        Some(PortSide::East) => Point::new(
            origin.x + port.size.width,
            origin.y + port.size.height / 2.0,
        ),
        Some(PortSide::South) => Point::new(
            origin.x + port.size.width / 2.0,
            origin.y + port.size.height,
        ),
        Some(PortSide::West) => Point::new(origin.x, origin.y + port.size.height / 2.0),
        None => Point::new(
            origin.x + port.size.width / 2.0,
            origin.y + port.size.height / 2.0,
        ),
    }
}

fn same_point(left: Point, right: Point) -> bool {
    (left.x - right.x).abs() < EPSILON && (left.y - right.y).abs() < EPSILON
}

fn rect(node: &ElkNode) -> Rect {
    Rect::new(node.position, node.size)
}
