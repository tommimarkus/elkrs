use std::collections::BTreeMap;

use elkrs_core::geometry::{Point, Size};
use elkrs_core::graph::ElementId;
use elkrs_core::layout::LayoutError;
use elkrs_core::options::{Direction, PortSide};

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

    edge.points = if direction.is_horizontal() {
        let x = (start.x + end.x) / 2.0;
        vec![start, Point::new(x, start.y), Point::new(x, end.y), end]
    } else {
        let y = (start.y + end.y) / 2.0;
        vec![start, Point::new(start.x, y), Point::new(end.x, y), end]
    };
    Ok(())
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
