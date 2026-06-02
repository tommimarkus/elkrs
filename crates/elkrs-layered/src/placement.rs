use std::collections::BTreeMap;

use elkrs_core::geometry::{Point, Size};
use elkrs_core::graph::ElementId;
use elkrs_core::layout::LayoutError;
use elkrs_core::options::{Direction, PortSide, Properties};

use crate::internal::{LGraph, LPort};
use crate::pipeline::{LayeredContext, LayeredProcessor};

#[derive(Debug, Clone, Copy)]
struct LayoutSpacing {
    node_node: f64,
    layer_node_node: f64,
    component_component: f64,
    port_port: f64,
}

impl LayoutSpacing {
    fn from_properties(properties: &Properties) -> Self {
        Self {
            node_node: properties.spacing_node_node(),
            layer_node_node: properties.spacing_layer_node_node(),
            component_component: properties.spacing_component_component(),
            port_port: properties.spacing_port_port(),
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

        let component_by_node = connected_components(graph);
        let mut positions = vec![Point::new(0.0, 0.0); graph.nodes.len()];
        for (layer, indices) in nodes_by_layer {
            let major = layer_major_position[&layer];
            let mut minor = 0.0;
            let mut previous_component = None;
            for index in indices {
                if let Some(previous) = previous_component {
                    let component_spacing = if previous != component_by_node[index] {
                        self.spacing.node_node.max(self.spacing.component_component)
                    } else {
                        self.spacing.node_node
                    };
                    minor += component_spacing;
                }
                let node = &graph.nodes[index];
                let hierarchy_offset = if node.parent.is_some() {
                    self.spacing.node_node / 4.0
                } else {
                    0.0
                };
                positions[index] =
                    position_from_axes(self.direction, major, minor + hierarchy_offset, node.size);
                minor += minor_extent(self.direction, node.size);
                previous_component = Some(component_by_node[index]);
            }
        }

        for (node, position) in graph.nodes.iter_mut().zip(positions) {
            if !node.no_layout {
                node.position = position;
            }
        }
        place_children_inside_parents(graph, self.spacing);
        place_default_ports(graph, self.spacing);
        Ok(())
    }
}

fn connected_components(graph: &LGraph) -> Vec<usize> {
    let mut parents = (0..graph.nodes.len()).collect::<Vec<_>>();
    let node_index = graph
        .nodes
        .iter()
        .enumerate()
        .map(|(index, node)| (node.id.clone(), index))
        .collect::<BTreeMap<_, _>>();

    for edge in &graph.edges {
        if edge.kind.is_self_loop() {
            continue;
        }
        let Some(source) = node_index.get(&edge.source.node).copied() else {
            continue;
        };
        let Some(target) = node_index.get(&edge.target.node).copied() else {
            continue;
        };
        union(&mut parents, source, target);
    }

    (0..graph.nodes.len())
        .map(|index| find(&mut parents, index))
        .collect()
}

fn union(parents: &mut [usize], first: usize, second: usize) {
    let first_root = find(parents, first);
    let second_root = find(parents, second);
    if first_root != second_root {
        parents[second_root] = first_root;
    }
}

fn find(parents: &mut [usize], index: usize) -> usize {
    if parents[index] != index {
        parents[index] = find(parents, parents[index]);
    }
    parents[index]
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
            if graph.nodes[child_index].no_layout {
                continue;
            }
            graph.nodes[child_index].position = Point::new(parent_position.x + padding, child_y);
            child_y += graph.nodes[child_index].size.height + spacing.node_node;
        }
    }
}

fn place_default_ports(graph: &mut LGraph, spacing: LayoutSpacing) {
    for node in &mut graph.nodes {
        let port_spacing = node.port_port_spacing.unwrap_or(spacing.port_port);
        for side in [
            PortSide::North,
            PortSide::East,
            PortSide::South,
            PortSide::West,
        ] {
            let port_ids = node
                .ports
                .iter()
                .filter(|(_, port)| port.side == Some(side) && has_default_port_geometry(port))
                .map(|(id, _)| id.clone())
                .collect::<Vec<_>>();
            let count = port_ids.len();
            if count == 0 {
                continue;
            }
            for (index, port_id) in port_ids.into_iter().enumerate() {
                let offset = (index as f64 - (count - 1) as f64 / 2.0) * port_spacing;
                let port = node
                    .ports
                    .get_mut(&port_id)
                    .expect("port collected from node is missing");
                port.position = match side {
                    PortSide::North => Point::new(node.size.width / 2.0 + offset, 0.0),
                    PortSide::East => Point::new(node.size.width, node.size.height / 2.0 + offset),
                    PortSide::South => Point::new(node.size.width / 2.0 + offset, node.size.height),
                    PortSide::West => Point::new(0.0, node.size.height / 2.0 + offset),
                };
            }
        }
    }
}

fn has_default_port_geometry(port: &LPort) -> bool {
    port.position == Point::new(0.0, 0.0) && port.size == Size::new(0.0, 0.0)
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
