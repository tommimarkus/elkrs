use std::collections::BTreeMap;

use elkrs_core::geometry::{Point, Size};
use elkrs_core::graph::{ElementId, ElkEdgeSection, ElkGraph};
use elkrs_core::layout::{LayoutError, LayoutReport};
use elkrs_core::options::Direction;

use crate::import::import_graph;
use crate::internal::{LEdge, LGraph};
use crate::pipeline::{LayeredContext, LayeredPipeline, LayeredProcessor};

const LAYER_SPACING: f64 = 120.0;
const NODE_SPACING: f64 = 80.0;

pub struct LayeredLayout;

pub trait LayoutAlgorithm {
    fn layout(&self, graph: &mut ElkGraph) -> Result<LayoutReport, LayoutError>;
}

impl LayoutAlgorithm for LayeredLayout {
    fn layout(&self, graph: &mut ElkGraph) -> Result<LayoutReport, LayoutError> {
        let direction = graph.properties.direction();
        let mut layered = import_graph(graph)?;
        let pipeline = LayeredPipeline::new(vec![
            Box::new(CycleBreaking),
            Box::new(LayerAssignment),
            Box::new(CrossingMinimization),
            Box::new(NodePlacement { direction }),
            Box::new(EdgeRouting { direction }),
        ]);
        let context = pipeline.run(&mut layered)?;
        write_back(graph, &layered);
        Ok(LayoutReport {
            diagnostics: context.diagnostics,
        })
    }
}

struct CycleBreaking;

impl LayeredProcessor for CycleBreaking {
    fn name(&self) -> &'static str {
        "cycle-breaking"
    }

    fn run(&self, graph: &mut LGraph, _context: &mut LayeredContext) -> Result<(), LayoutError> {
        let order = graph
            .nodes
            .iter()
            .enumerate()
            .map(|(index, node)| (node.id.clone(), index))
            .collect::<BTreeMap<_, _>>();
        for edge in &mut graph.edges {
            let source = order.get(&edge.source).copied().unwrap_or(0);
            let target = order.get(&edge.target).copied().unwrap_or(0);
            if source > target {
                std::mem::swap(&mut edge.source, &mut edge.target);
                edge.reversed = !edge.reversed;
            }
        }
        Ok(())
    }
}

struct LayerAssignment;

impl LayeredProcessor for LayerAssignment {
    fn name(&self) -> &'static str {
        "layer-assignment"
    }

    fn run(&self, graph: &mut LGraph, _context: &mut LayeredContext) -> Result<(), LayoutError> {
        let mut layers = graph
            .nodes
            .iter()
            .map(|node| (node.id.clone(), 0usize))
            .collect::<BTreeMap<_, _>>();
        for _ in 0..graph.nodes.len() {
            let mut changed = false;
            for edge in &graph.edges {
                let source_layer = *layers.get(&edge.source).unwrap_or(&0);
                let target_layer = *layers.get(&edge.target).unwrap_or(&0);
                if target_layer <= source_layer {
                    layers.insert(edge.target.clone(), source_layer + 1);
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        for node in &mut graph.nodes {
            node.layer = *layers.get(&node.id).unwrap_or(&0);
        }
        Ok(())
    }
}

struct CrossingMinimization;

impl LayeredProcessor for CrossingMinimization {
    fn name(&self) -> &'static str {
        "crossing-minimization"
    }

    fn run(&self, _graph: &mut LGraph, _context: &mut LayeredContext) -> Result<(), LayoutError> {
        Ok(())
    }
}

struct NodePlacement {
    direction: Direction,
}

impl LayeredProcessor for NodePlacement {
    fn name(&self) -> &'static str {
        "node-placement"
    }

    fn run(&self, graph: &mut LGraph, _context: &mut LayeredContext) -> Result<(), LayoutError> {
        let mut per_layer_index = BTreeMap::<usize, usize>::new();
        for node in &mut graph.nodes {
            let index = per_layer_index.entry(node.layer).or_insert(0);
            let major = node.layer as f64 * LAYER_SPACING;
            let hierarchy_offset = if node.parent.is_some() {
                NODE_SPACING / 4.0
            } else {
                0.0
            };
            let minor = *index as f64 * NODE_SPACING + hierarchy_offset;
            node.position = match self.direction {
                Direction::Right => Point::new(major, minor),
                Direction::Left => Point::new(-major, minor),
                Direction::Down => Point::new(minor, major),
                Direction::Up => Point::new(minor, -major),
            };
            *index += 1;
        }
        Ok(())
    }
}

struct EdgeRouting {
    direction: Direction,
}

impl LayeredProcessor for EdgeRouting {
    fn name(&self) -> &'static str {
        "edge-routing"
    }

    fn run(&self, graph: &mut LGraph, _context: &mut LayeredContext) -> Result<(), LayoutError> {
        let positions = graph
            .nodes
            .iter()
            .map(|node| (node.id.clone(), (node.position, node.size)))
            .collect::<BTreeMap<_, _>>();
        for edge in &mut graph.edges {
            route_edge(edge, &positions, self.direction)?;
        }
        Ok(())
    }
}

fn route_edge(
    edge: &mut LEdge,
    positions: &BTreeMap<ElementId, (Point, Size)>,
    direction: Direction,
) -> Result<(), LayoutError> {
    let (source_pos, source_size) = positions
        .get(&edge.source)
        .copied()
        .ok_or_else(|| LayoutError::MissingEndpoint(edge.source.as_str().to_string()))?;
    let (target_pos, target_size) = positions
        .get(&edge.target)
        .copied()
        .ok_or_else(|| LayoutError::MissingEndpoint(edge.target.as_str().to_string()))?;

    let start = match direction {
        Direction::Right => Point::new(
            source_pos.x + source_size.width,
            source_pos.y + source_size.height / 2.0,
        ),
        Direction::Left => Point::new(source_pos.x, source_pos.y + source_size.height / 2.0),
        Direction::Down => Point::new(
            source_pos.x + source_size.width / 2.0,
            source_pos.y + source_size.height,
        ),
        Direction::Up => Point::new(source_pos.x + source_size.width / 2.0, source_pos.y),
    };
    let end = match direction {
        Direction::Right => Point::new(target_pos.x, target_pos.y + target_size.height / 2.0),
        Direction::Left => Point::new(
            target_pos.x + target_size.width,
            target_pos.y + target_size.height / 2.0,
        ),
        Direction::Down => Point::new(target_pos.x + target_size.width / 2.0, target_pos.y),
        Direction::Up => Point::new(
            target_pos.x + target_size.width / 2.0,
            target_pos.y + target_size.height,
        ),
    };
    edge.points = if direction.is_horizontal() {
        let x = (start.x + end.x) / 2.0;
        vec![start, Point::new(x, start.y), Point::new(x, end.y), end]
    } else {
        let y = (start.y + end.y) / 2.0;
        vec![start, Point::new(start.x, y), Point::new(end.x, y), end]
    };
    Ok(())
}

fn write_back(graph: &mut ElkGraph, layered: &LGraph) {
    for layered_node in &layered.nodes {
        for node in graph.nodes.values_mut() {
            if write_node_position(node, layered_node) {
                break;
            }
        }
    }
    for layered_edge in &layered.edges {
        if let Some(edge) = graph.edges.get_mut(&layered_edge.id) {
            edge.sections = vec![ElkEdgeSection {
                points: layered_edge.points.clone(),
            }];
        }
    }
}

fn write_node_position(
    node: &mut elkrs_core::graph::ElkNode,
    layered_node: &crate::internal::LNode,
) -> bool {
    if node.id == layered_node.id {
        node.position = layered_node.position;
        return true;
    }
    for child in node.children.values_mut() {
        if write_node_position(child, layered_node) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use elkrs_core::geometry::Size;
    use elkrs_core::graph::{ElementRef, ElkEdge, ElkNode};

    use super::*;

    #[test]
    fn layout_places_target_after_source_for_right_direction() {
        let mut graph = ElkGraph::new("root");
        graph.add_node(node("a"));
        graph.add_node(node("b"));
        graph.add_edge(ElkEdge::new(
            "e",
            ElementRef::Node(ElementId::from("a")),
            ElementRef::Node(ElementId::from("b")),
        ));

        LayeredLayout.layout(&mut graph).unwrap();

        assert!(
            graph.nodes[&ElementId::from("b")].position.x
                > graph.nodes[&ElementId::from("a")].position.x
        );
        assert_eq!(
            graph.edges[&ElementId::from("e")].sections[0].points.len(),
            4
        );
    }

    fn node(id: &str) -> ElkNode {
        let mut node = ElkNode::new(id);
        node.size = Size::new(20.0, 20.0);
        node
    }
}
