use std::collections::BTreeMap;

use elkrs_core::geometry::{Point, Rect, Size};
use elkrs_core::graph::ElementId;
use elkrs_core::layout::LayoutError;
use elkrs_core::options::{
    Direction, PortSide, DEFAULT_EDGE_EDGE_SPACING, DEFAULT_EDGE_NODE_SPACING,
};

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
        let siblings = sibling_routes(&graph.edges);
        for edge in &mut graph.edges {
            route_edge(
                edge,
                &nodes,
                self.direction,
                siblings
                    .get(&edge.id)
                    .copied()
                    .unwrap_or_else(SiblingRoute::single),
            )?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum EdgeRouteKey {
    Normal {
        source: LEndpoint,
        target: LEndpoint,
    },
    SelfLoop {
        node: ElementId,
    },
}

#[derive(Debug, Clone, Copy)]
struct SiblingRoute {
    index: usize,
    count: usize,
}

impl SiblingRoute {
    fn single() -> Self {
        Self { index: 0, count: 1 }
    }

    fn centered_offset(self) -> f64 {
        if self.count <= 1 {
            0.0
        } else {
            (self.index as f64 - (self.count - 1) as f64 / 2.0) * DEFAULT_EDGE_EDGE_SPACING
        }
    }

    fn loop_distance(self) -> f64 {
        DEFAULT_EDGE_NODE_SPACING + self.index as f64 * DEFAULT_EDGE_EDGE_SPACING
    }
}

fn sibling_routes(edges: &[LEdge]) -> BTreeMap<ElementId, SiblingRoute> {
    let mut groups = BTreeMap::<EdgeRouteKey, Vec<ElementId>>::new();
    for edge in edges {
        groups
            .entry(route_key(edge))
            .or_default()
            .push(edge.id.clone());
    }

    let mut routes = BTreeMap::new();
    for mut ids in groups.into_values() {
        ids.sort();
        let count = ids.len();
        for (index, id) in ids.into_iter().enumerate() {
            routes.insert(id, SiblingRoute { index, count });
        }
    }
    routes
}

fn route_key(edge: &LEdge) -> EdgeRouteKey {
    if edge.kind.is_self_loop() {
        EdgeRouteKey::SelfLoop {
            node: edge.source.node.clone(),
        }
    } else {
        EdgeRouteKey::Normal {
            source: edge.source.clone(),
            target: edge.target.clone(),
        }
    }
}

fn route_edge(
    edge: &mut LEdge,
    nodes: &BTreeMap<ElementId, LNode>,
    direction: Direction,
    sibling: SiblingRoute,
) -> Result<(), LayoutError> {
    if edge.kind.is_self_loop() {
        edge.points = self_loop_route(edge, nodes, direction, sibling)?;
        return Ok(());
    }

    let start = endpoint_anchor(&edge.source, nodes, direction, true)?;
    let end = endpoint_anchor(&edge.target, nodes, direction, false)?;
    let offset = sibling.centered_offset();

    let mut points = orthogonal_route(start, end, direction, offset);
    if let Some(obstacle) = first_intersecting_obstacle(&points, edge, nodes) {
        points = detour_route(start, end, obstacle, direction, offset);
    }
    edge.points = points;
    Ok(())
}

fn orthogonal_route(start: Point, end: Point, direction: Direction, offset: f64) -> Vec<Point> {
    if direction.is_horizontal() {
        let x = (start.x + end.x) / 2.0;
        if offset.abs() < f64::EPSILON {
            vec![start, Point::new(x, start.y), Point::new(x, end.y), end]
        } else {
            vec![
                start,
                Point::new(x, start.y),
                Point::new(x, start.y + offset),
                Point::new(x, end.y + offset),
                Point::new(x, end.y),
                end,
            ]
        }
    } else {
        let y = (start.y + end.y) / 2.0;
        if offset.abs() < f64::EPSILON {
            vec![start, Point::new(start.x, y), Point::new(end.x, y), end]
        } else {
            vec![
                start,
                Point::new(start.x, y),
                Point::new(start.x + offset, y),
                Point::new(end.x + offset, y),
                Point::new(end.x, y),
                end,
            ]
        }
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

fn detour_route(
    start: Point,
    end: Point,
    obstacle: Rect,
    direction: Direction,
    offset: f64,
) -> Vec<Point> {
    if direction.is_horizontal() {
        let x = if end.x >= start.x {
            obstacle.right() + DEFAULT_EDGE_NODE_SPACING
        } else {
            obstacle.left() - DEFAULT_EDGE_NODE_SPACING
        };
        if offset.abs() < f64::EPSILON {
            vec![start, Point::new(x, start.y), Point::new(x, end.y), end]
        } else {
            vec![
                start,
                Point::new(x, start.y),
                Point::new(x, start.y + offset),
                Point::new(x, end.y + offset),
                Point::new(x, end.y),
                end,
            ]
        }
    } else {
        let y = if end.y >= start.y {
            obstacle.bottom() + DEFAULT_EDGE_NODE_SPACING
        } else {
            obstacle.top() - DEFAULT_EDGE_NODE_SPACING
        };
        if offset.abs() < f64::EPSILON {
            vec![start, Point::new(start.x, y), Point::new(end.x, y), end]
        } else {
            vec![
                start,
                Point::new(start.x, y),
                Point::new(start.x + offset, y),
                Point::new(end.x + offset, y),
                Point::new(end.x, y),
                end,
            ]
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LoopSide {
    North,
    East,
    South,
    West,
}

fn self_loop_route(
    edge: &LEdge,
    nodes: &BTreeMap<ElementId, LNode>,
    direction: Direction,
    sibling: SiblingRoute,
) -> Result<Vec<Point>, LayoutError> {
    let node = nodes
        .get(&edge.source.node)
        .ok_or_else(|| LayoutError::MissingEndpoint(edge.source.node.as_str().to_string()))?;
    let source_side = endpoint_loop_side(&edge.source, node, direction)?;
    let target_side = endpoint_loop_side(&edge.target, node, direction)?;
    let start = self_loop_anchor(&edge.source, node, source_side, true)?;
    let end = self_loop_anchor(&edge.target, node, target_side, false)?;
    let distance = sibling.loop_distance();

    if source_side != target_side {
        return Ok(mixed_side_self_loop_route(
            start,
            source_side,
            end,
            target_side,
            expanded_loop_rect(node, distance),
        ));
    }

    Ok(match source_side {
        LoopSide::North => {
            let y = node.position.y - distance;
            vec![start, Point::new(start.x, y), Point::new(end.x, y), end]
        }
        LoopSide::East => {
            let x = node.position.x + node.size.width + distance;
            vec![start, Point::new(x, start.y), Point::new(x, end.y), end]
        }
        LoopSide::South => {
            let y = node.position.y + node.size.height + distance;
            vec![start, Point::new(start.x, y), Point::new(end.x, y), end]
        }
        LoopSide::West => {
            let x = node.position.x - distance;
            vec![start, Point::new(x, start.y), Point::new(x, end.y), end]
        }
    })
}

fn endpoint_loop_side(
    endpoint: &LEndpoint,
    node: &LNode,
    direction: Direction,
) -> Result<LoopSide, LayoutError> {
    if let Some(port_id) = &endpoint.port {
        let port = node.ports.get(port_id).ok_or_else(|| {
            LayoutError::MissingEndpoint(format!("{}:{}", endpoint.node.as_str(), port_id.as_str()))
        })?;
        if let Some(side) = port.side {
            return Ok(match side {
                PortSide::North => LoopSide::North,
                PortSide::East => LoopSide::East,
                PortSide::South => LoopSide::South,
                PortSide::West => LoopSide::West,
            });
        }
    }

    Ok(direction_loop_side(direction))
}

fn direction_loop_side(direction: Direction) -> LoopSide {
    match direction {
        Direction::Right => LoopSide::East,
        Direction::Left => LoopSide::West,
        Direction::Down => LoopSide::South,
        Direction::Up => LoopSide::North,
    }
}

fn mixed_side_self_loop_route(
    start: Point,
    source_side: LoopSide,
    end: Point,
    target_side: LoopSide,
    rect: Rect,
) -> Vec<Point> {
    let source_projection = project_to_loop_rect(start, source_side, rect);
    let target_projection = project_to_loop_rect(end, target_side, rect);
    let mut points = vec![start, source_projection];
    points.extend(shortest_perimeter_path(
        source_projection,
        source_side,
        target_projection,
        target_side,
        rect,
    ));
    points.push(end);
    dedupe_consecutive_points(points)
}

fn expanded_loop_rect(node: &LNode, distance: f64) -> Rect {
    Rect::new(
        Point::new(node.position.x - distance, node.position.y - distance),
        Size::new(
            node.size.width + distance * 2.0,
            node.size.height + distance * 2.0,
        ),
    )
}

fn project_to_loop_rect(point: Point, side: LoopSide, rect: Rect) -> Point {
    match side {
        LoopSide::North => Point::new(point.x, rect.top()),
        LoopSide::East => Point::new(rect.right(), point.y),
        LoopSide::South => Point::new(point.x, rect.bottom()),
        LoopSide::West => Point::new(rect.left(), point.y),
    }
}

fn shortest_perimeter_path(
    start: Point,
    start_side: LoopSide,
    end: Point,
    end_side: LoopSide,
    rect: Rect,
) -> Vec<Point> {
    let clockwise = perimeter_distance(start, start_side, end, end_side, rect);
    let counterclockwise = perimeter_distance(end, end_side, start, start_side, rect);

    if clockwise <= counterclockwise {
        perimeter_path(start_side, end_side, rect, true, end)
    } else {
        perimeter_path(start_side, end_side, rect, false, end)
    }
}

fn perimeter_distance(
    start: Point,
    start_side: LoopSide,
    end: Point,
    end_side: LoopSide,
    rect: Rect,
) -> f64 {
    let start_position = perimeter_position(start, start_side, rect);
    let end_position = perimeter_position(end, end_side, rect);
    let perimeter = rect.size.width * 2.0 + rect.size.height * 2.0;
    if end_position >= start_position {
        end_position - start_position
    } else {
        perimeter - start_position + end_position
    }
}

fn perimeter_position(point: Point, side: LoopSide, rect: Rect) -> f64 {
    match side {
        LoopSide::North => point.x - rect.left(),
        LoopSide::East => rect.size.width + point.y - rect.top(),
        LoopSide::South => rect.size.width + rect.size.height + rect.right() - point.x,
        LoopSide::West => rect.size.width * 2.0 + rect.size.height + rect.bottom() - point.y,
    }
}

fn perimeter_path(
    start_side: LoopSide,
    end_side: LoopSide,
    rect: Rect,
    clockwise: bool,
    end: Point,
) -> Vec<Point> {
    let mut points = Vec::new();
    let mut side = start_side;
    while side != end_side {
        points.push(perimeter_corner(side, clockwise, rect));
        side = next_perimeter_side(side, clockwise);
    }
    points.push(end);
    points
}

fn perimeter_corner(side: LoopSide, clockwise: bool, rect: Rect) -> Point {
    match (side, clockwise) {
        (LoopSide::North, true) | (LoopSide::East, false) => Point::new(rect.right(), rect.top()),
        (LoopSide::East, true) | (LoopSide::South, false) => {
            Point::new(rect.right(), rect.bottom())
        }
        (LoopSide::South, true) | (LoopSide::West, false) => Point::new(rect.left(), rect.bottom()),
        (LoopSide::West, true) | (LoopSide::North, false) => Point::new(rect.left(), rect.top()),
    }
}

fn next_perimeter_side(side: LoopSide, clockwise: bool) -> LoopSide {
    match (side, clockwise) {
        (LoopSide::North, true) | (LoopSide::South, false) => LoopSide::East,
        (LoopSide::East, true) | (LoopSide::West, false) => LoopSide::South,
        (LoopSide::South, true) | (LoopSide::North, false) => LoopSide::West,
        (LoopSide::West, true) | (LoopSide::East, false) => LoopSide::North,
    }
}

fn dedupe_consecutive_points(points: Vec<Point>) -> Vec<Point> {
    points.into_iter().fold(Vec::new(), |mut deduped, point| {
        if deduped.last() != Some(&point) {
            deduped.push(point);
        }
        deduped
    })
}

fn self_loop_anchor(
    endpoint: &LEndpoint,
    node: &LNode,
    side: LoopSide,
    source: bool,
) -> Result<Point, LayoutError> {
    if let Some(port_id) = &endpoint.port {
        let port = node.ports.get(port_id).ok_or_else(|| {
            LayoutError::MissingEndpoint(format!("{}:{}", endpoint.node.as_str(), port_id.as_str()))
        })?;
        debug_assert_eq!(&port.id, port_id);
        return Ok(port_anchor(node, port));
    }

    Ok(node_self_loop_anchor(
        node.position,
        node.size,
        side,
        source,
    ))
}

fn node_self_loop_anchor(position: Point, size: Size, side: LoopSide, source: bool) -> Point {
    let first_ratio = if source { 0.35 } else { 0.65 };
    let second_ratio = if source { 0.65 } else { 0.35 };
    match side {
        LoopSide::North => Point::new(position.x + size.width * first_ratio, position.y),
        LoopSide::East => Point::new(
            position.x + size.width,
            position.y + size.height * first_ratio,
        ),
        LoopSide::South => Point::new(
            position.x + size.width * second_ratio,
            position.y + size.height,
        ),
        LoopSide::West => Point::new(position.x, position.y + size.height * second_ratio),
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

    use elkrs_core::geometry::Size;

    use crate::internal::LEdgeKind;

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
                kind: LEdgeKind::Normal,
                reversed: false,
                points: Vec::new(),
            }],
        };

        EdgeRouting::new(Direction::Right)
            .run(&mut graph, &mut LayeredContext::new())
            .unwrap();

        assert_eq!(
            graph.edges[0].points,
            vec![
                Point::new(40.0, 20.0),
                Point::new(160.0, 20.0),
                Point::new(160.0, 140.0),
                Point::new(200.0, 140.0),
            ]
        );
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
}
