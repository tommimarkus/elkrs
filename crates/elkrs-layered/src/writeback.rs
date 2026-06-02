use std::collections::BTreeMap;

use elkrs_core::graph::ElementId;
use elkrs_core::graph::{ElkEdgeSection, ElkGraph, ElkNode};

use crate::internal::{LGraph, LNode};

pub(crate) fn write_back(
    graph: &mut ElkGraph,
    layered: &LGraph,
    generate_position_and_layer_ids: bool,
) {
    let generated_ids = if generate_position_and_layer_ids {
        generated_node_ids(layered)
    } else {
        BTreeMap::new()
    };

    for layered_node in &layered.nodes {
        for node in graph.nodes.values_mut() {
            if write_node_layout(node, layered_node, generated_ids.get(&layered_node.id)) {
                break;
            }
        }
    }
    for layered_edge in &layered.edges {
        if let Some(edge) = graph.edges.get_mut(&layered_edge.id) {
            let points = if layered_edge.reversed {
                layered_edge.points.iter().rev().copied().collect()
            } else {
                layered_edge.points.clone()
            };
            edge.sections = vec![ElkEdgeSection { points }];
        }
    }
}

fn write_node_layout(
    node: &mut ElkNode,
    layered_node: &LNode,
    generated_ids: Option<&GeneratedNodeIds>,
) -> bool {
    if node.id == layered_node.id {
        node.position = layered_node.position;
        node.size = layered_node.size;
        if let Some(generated_ids) = generated_ids {
            node.properties.set_layer_id(generated_ids.layer_id);
            node.properties
                .set_crossing_minimization_position_id(generated_ids.position_id);
        }
        for layered_port in layered_node.ports.values() {
            if let Some(port) = node.ports.get_mut(&layered_port.id) {
                port.position = layered_port.position;
                port.size = layered_port.size;
            }
        }
        return true;
    }
    for child in node.children.values_mut() {
        if write_node_layout(child, layered_node, generated_ids) {
            return true;
        }
    }
    false
}

#[derive(Debug, Clone, Copy)]
struct GeneratedNodeIds {
    layer_id: i64,
    position_id: i64,
}

fn generated_node_ids(layered: &LGraph) -> BTreeMap<ElementId, GeneratedNodeIds> {
    let mut next_position_by_layer = BTreeMap::<usize, i64>::new();
    let mut ids = BTreeMap::new();

    for node in &layered.nodes {
        let position_id = next_position_by_layer.entry(node.layer).or_insert(0);
        ids.insert(
            node.id.clone(),
            GeneratedNodeIds {
                layer_id: node.layer as i64,
                position_id: *position_id,
            },
        );
        *position_id += 1;
    }

    ids
}
