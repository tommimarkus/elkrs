use elkrs_core::graph::{ElkEdgeSection, ElkGraph, ElkNode};

use crate::internal::{LGraph, LNode};

pub(crate) fn write_back(graph: &mut ElkGraph, layered: &LGraph) {
    for layered_node in &layered.nodes {
        for node in graph.nodes.values_mut() {
            if write_node_layout(node, layered_node) {
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

fn write_node_layout(node: &mut ElkNode, layered_node: &LNode) -> bool {
    if node.id == layered_node.id {
        node.position = layered_node.position;
        for layered_port in layered_node.ports.values() {
            if let Some(port) = node.ports.get_mut(&layered_port.id) {
                port.position = layered_port.position;
                port.size = layered_port.size;
            }
        }
        return true;
    }
    for child in node.children.values_mut() {
        if write_node_layout(child, layered_node) {
            return true;
        }
    }
    false
}
