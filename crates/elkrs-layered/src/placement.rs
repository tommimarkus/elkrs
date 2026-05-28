use std::collections::BTreeMap;

use elkrs_core::geometry::{Point, Size};
use elkrs_core::graph::ElementId;
use elkrs_core::layout::LayoutError;
use elkrs_core::options::{Direction, Properties};

use crate::internal::LGraph;
use crate::pipeline::{LayeredContext, LayeredProcessor};

#[derive(Debug, Clone, Copy)]
struct LayoutSpacing {
    node_node: f64,
    layer_node_node: f64,
}

impl LayoutSpacing {
    fn from_properties(properties: &Properties) -> Self {
        Self {
            node_node: properties.spacing_node_node(),
            layer_node_node: properties.spacing_layer_node_node(),
        }
    }
}

pub(crate) struct NodePlacement {
    direction: Direction,
    spacing: LayoutSpacing,
}

impl NodePlacement {
    pub(crate) fn from_properties(direction: Direction, properties: &Properties) -> Self {
        Self {
            direction,
            spacing: LayoutSpacing::from_properties(properties),
        }
    }
}

impl LayeredProcessor for NodePlacement {
    fn name(&self) -> &'static str {
        "node-placement"
    }

    fn run(&self, graph: &mut LGraph, _context: &mut LayeredContext) -> Result<(), LayoutError> {
        let mut nodes_by_layer = BTreeMap::<usize, Vec<usize>>::new();
        let mut layer_major_extent = BTreeMap::<usize, f64>::new();
        for (index, node) in graph.nodes.iter().enumerate() {
            nodes_by_layer.entry(node.layer).or_default().push(index);
            layer_major_extent
                .entry(node.layer)
                .and_modify(|extent| *extent = extent.max(major_extent(self.direction, node.size)))
                .or_insert_with(|| major_extent(self.direction, node.size));
        }

        let mut next_major = 0.0;
        let mut layer_major_position = BTreeMap::<usize, f64>::new();
        for (layer, extent) in &layer_major_extent {
            layer_major_position.insert(*layer, next_major);
            next_major += *extent + self.spacing.layer_node_node;
        }

        let mut positions = vec![Point::new(0.0, 0.0); graph.nodes.len()];
        for (layer, indices) in nodes_by_layer {
            let major = layer_major_position[&layer];
            let mut minor = 0.0;
            for index in indices {
                let node = &graph.nodes[index];
                let hierarchy_offset = if node.parent.is_some() {
                    self.spacing.node_node / 4.0
                } else {
                    0.0
                };
                positions[index] =
                    position_from_axes(self.direction, major, minor + hierarchy_offset, node.size);
                minor += minor_extent(self.direction, node.size) + self.spacing.node_node;
            }
        }

        for (node, position) in graph.nodes.iter_mut().zip(positions) {
            node.position = position;
        }
        place_children_inside_parents(graph, self.spacing);
        Ok(())
    }
}

fn place_children_inside_parents(graph: &mut LGraph, spacing: LayoutSpacing) {
    let mut children_by_parent = BTreeMap::<ElementId, Vec<usize>>::new();
    for (index, node) in graph.nodes.iter().enumerate() {
        if let Some(parent) = &node.parent {
            children_by_parent
                .entry(parent.clone())
                .or_default()
                .push(index);
        }
    }

    let padding = spacing.node_node / 4.0;
    for (parent_id, child_indices) in children_by_parent {
        let Some(parent_index) = graph.nodes.iter().position(|node| node.id == parent_id) else {
            continue;
        };
        let parent_position = graph.nodes[parent_index].position;
        let mut child_y = parent_position.y + padding;
        for child_index in child_indices {
            graph.nodes[child_index].position = Point::new(parent_position.x + padding, child_y);
            child_y += graph.nodes[child_index].size.height + spacing.node_node;
        }
    }
}

fn major_extent(direction: Direction, size: Size) -> f64 {
    if direction.is_horizontal() {
        size.width
    } else {
        size.height
    }
}

fn minor_extent(direction: Direction, size: Size) -> f64 {
    if direction.is_horizontal() {
        size.height
    } else {
        size.width
    }
}

fn position_from_axes(direction: Direction, major: f64, minor: f64, size: Size) -> Point {
    match direction {
        Direction::Right => Point::new(major, minor),
        Direction::Left => Point::new(-major - size.width, minor),
        Direction::Down => Point::new(minor, major),
        Direction::Up => Point::new(minor, -major - size.height),
    }
}
