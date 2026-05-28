use std::collections::BTreeMap;

use elkrs_core::geometry::{Point, Rect, Size};
use elkrs_core::graph::ElementId;
use elkrs_core::layout::LayoutError;
use elkrs_core::options::{Direction, PortSide, DEFAULT_EDGE_NODE_SPACING};

use crate::internal::{LEdge, LEndpoint, LGraph, LNode, LPort};
use crate::pipeline::{LayeredContext, LayeredProcessor};

pub(crate) struct EdgeRouting {
    direction: Direction,
}

impl EdgeRouting {
    pub(crate) fn new(direction: Direction) -> Self {
        Self { direction }
    }
}

impl LayeredProcessor for EdgeRouting {
    fn name(&self) -> &'static str {
        "edge-routing"
    }

    fn run(&self, graph: &mut LGraph, _context: &mut LayeredContext) -> Result<(), LayoutError> {
        let nodes = graph
            .nodes
            .iter()
            .map(|node| (node.id.clone(), node.clone()))
            .collect::<BTreeMap<_, _>>();
        for edge in &mut graph.edges {
            route_edge(edge, &nodes, self.direction)?;
        }
        Ok(())
    }
}

fn route_edge(
    edge: &mut LEdge,
    nodes: &BTreeMap<ElementId, LNode>,
    direction: Direction,
) -> Result<(), LayoutError> {
    let start = endpoint_anchor(&edge.source, nodes, direction, true)?;
    let end = endpoint_anchor(&edge.target, nodes, direction, false)?;

    let mut points = orthogonal_route(start, end, direction);
    if let Some(obstacle) = first_intersecting_obstacle(&points, edge, nodes) {
        points = detour_route(start, end, obstacle, direction);
    }
    edge.points = points;
    Ok(())
}

fn orthogonal_route(start: Point, end: Point, direction: Direction) -> Vec<Point> {
    if direction.is_horizontal() {
        let x = (start.x + end.x) / 2.0;
        vec![start, Point::new(x, start.y), Point::new(x, end.y), end]
    } else {
        let y = (start.y + end.y) / 2.0;
        vec![start, Point::new(start.x, y), Point::new(end.x, y), end]
    }
}

fn first_intersecting_obstacle(
    points: &[Point],
    edge: &LEdge,
    nodes: &BTreeMap<ElementId, LNode>,
) -> Option<Rect> {
    nodes
        .values()
        .filter(|node| node.id != edge.source.node && node.id != edge.target.node)
        .map(node_rect)
        .find(|rect| {
            points
                .windows(2)
                .any(|segment| segment_intersects_rect_interior(segment[0], segment[1], *rect))
        })
}

fn detour_route(start: Point, end: Point, obstacle: Rect, direction: Direction) -> Vec<Point> {
    if direction.is_horizontal() {
        let x = if end.x >= start.x {
            obstacle.right() + DEFAULT_EDGE_NODE_SPACING
        } else {
            obstacle.left() - DEFAULT_EDGE_NODE_SPACING
        };
        vec![start, Point::new(x, start.y), Point::new(x, end.y), end]
    } else {
        let y = if end.y >= start.y {
            obstacle.bottom() + DEFAULT_EDGE_NODE_SPACING
        } else {
            obstacle.top() - DEFAULT_EDGE_NODE_SPACING
        };
        vec![start, Point::new(start.x, y), Point::new(end.x, y), end]
    }
}

fn segment_intersects_rect_interior(start: Point, end: Point, rect: Rect) -> bool {
    if (start.y - end.y).abs() < f64::EPSILON {
        start.y > rect.top()
            && start.y < rect.bottom()
            && start.x.min(end.x) < rect.right()
            && start.x.max(end.x) > rect.left()
    } else if (start.x - end.x).abs() < f64::EPSILON {
        start.x > rect.left()
            && start.x < rect.right()
            && start.y.min(end.y) < rect.bottom()
            && start.y.max(end.y) > rect.top()
    } else {
        false
    }
}

fn node_rect(node: &LNode) -> Rect {
    Rect::new(node.position, node.size)
}

fn endpoint_anchor(
    endpoint: &LEndpoint,
    nodes: &BTreeMap<ElementId, LNode>,
    direction: Direction,
    source: bool,
) -> Result<Point, LayoutError> {
    let node = nodes
        .get(&endpoint.node)
        .ok_or_else(|| LayoutError::MissingEndpoint(endpoint.node.as_str().to_string()))?;
    match &endpoint.port {
        Some(port_id) => {
            let port = node.ports.get(port_id).ok_or_else(|| {
                LayoutError::MissingEndpoint(format!(
                    "{}:{}",
                    endpoint.node.as_str(),
                    port_id.as_str()
                ))
            })?;
            debug_assert_eq!(&port.id, port_id);
            Ok(port_anchor(node, port))
        }
        None => Ok(node_anchor(node.position, node.size, direction, source)),
    }
}

fn node_anchor(position: Point, size: Size, direction: Direction, source: bool) -> Point {
    match (direction, source) {
        (Direction::Right, true) | (Direction::Left, false) => {
            Point::new(position.x + size.width, position.y + size.height / 2.0)
        }
        (Direction::Right, false) | (Direction::Left, true) => {
            Point::new(position.x, position.y + size.height / 2.0)
        }
        (Direction::Down, true) | (Direction::Up, false) => {
            Point::new(position.x + size.width / 2.0, position.y + size.height)
        }
        (Direction::Down, false) | (Direction::Up, true) => {
            Point::new(position.x + size.width / 2.0, position.y)
        }
    }
}

fn port_anchor(node: &LNode, port: &LPort) -> Point {
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

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use elkrs_core::geometry::{Rect, Size};

    use super::*;

    #[test]
    fn routing_detours_around_unrelated_node_rectangles() {
        let mut graph = LGraph {
            nodes: vec![
                node("source", Point::new(0.0, 0.0), Size::new(40.0, 40.0)),
                node("obstacle", Point::new(100.0, 40.0), Size::new(40.0, 80.0)),
                node("target", Point::new(200.0, 120.0), Size::new(40.0, 40.0)),
            ],
            edges: vec![LEdge {
                id: ElementId::from("edge"),
                source: LEndpoint {
                    node: ElementId::from("source"),
                    port: None,
                },
                target: LEndpoint {
                    node: ElementId::from("target"),
                    port: None,
                },
                reversed: false,
                points: Vec::new(),
            }],
        };

        EdgeRouting::new(Direction::Right)
            .run(&mut graph, &mut LayeredContext::new())
            .unwrap();

        let obstacle = Rect::new(Point::new(100.0, 40.0), Size::new(40.0, 80.0));
        assert!(!graph.edges[0]
            .points
            .windows(2)
            .any(|segment| segment_intersects_rect_interior(segment[0], segment[1], obstacle)));
    }

    fn node(id: &str, position: Point, size: Size) -> LNode {
        LNode {
            id: ElementId::from(id),
            size,
            position,
            layer: 0,
            parent: None,
            ports: BTreeMap::new(),
        }
    }

    fn segment_intersects_rect_interior(start: Point, end: Point, rect: Rect) -> bool {
        if (start.y - end.y).abs() < f64::EPSILON {
            start.y > rect.top()
                && start.y < rect.bottom()
                && start.x.min(end.x) < rect.right()
                && start.x.max(end.x) > rect.left()
        } else if (start.x - end.x).abs() < f64::EPSILON {
            start.x > rect.left()
                && start.x < rect.right()
                && start.y.min(end.y) < rect.bottom()
                && start.y.max(end.y) > rect.top()
        } else {
            false
        }
    }
}
