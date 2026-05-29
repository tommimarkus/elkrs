use std::cmp::Ordering;
use std::collections::BTreeMap;

use elkrs_core::graph::ElementId;
use elkrs_core::layout::LayoutError;

use crate::internal::{LGraph, LNode};
use crate::pipeline::{LayeredContext, LayeredProcessor};

pub(crate) struct CrossingMinimization;

impl LayeredProcessor for CrossingMinimization {
    fn name(&self) -> &'static str {
        "crossing-minimization"
    }

    fn run(&self, graph: &mut LGraph, _context: &mut LayeredContext) -> Result<(), LayoutError> {
        let max_layer = graph.nodes.iter().map(|node| node.layer).max().unwrap_or(0);
        for layer in 1..=max_layer {
            reorder_layer_by_previous_barycenter(graph, layer);
        }
        Ok(())
    }
}

fn reorder_layer_by_previous_barycenter(graph: &mut LGraph, layer: usize) {
    let previous_order = order_for_layer(graph, layer - 1);
    if previous_order.is_empty() {
        return;
    }

    let mut layer_nodes = graph
        .nodes
        .iter()
        .enumerate()
        .filter(|(_, node)| node.layer == layer)
        .map(|(index, node)| {
            (
                index,
                node.clone(),
                barycenter(graph, &node.id, &previous_order),
            )
        })
        .collect::<Vec<_>>();

    layer_nodes.sort_by(compare_barycenter);
    let mut sorted_nodes = layer_nodes.into_iter().map(|(_, node, _)| node);
    for node in &mut graph.nodes {
        if node.layer == layer {
            *node = sorted_nodes
                .next()
                .expect("layer node count changed during crossing minimization");
        }
    }
}

fn order_for_layer(graph: &LGraph, layer: usize) -> BTreeMap<ElementId, usize> {
    graph
        .nodes
        .iter()
        .filter(|node| node.layer == layer)
        .enumerate()
        .map(|(index, node)| (node.id.clone(), index))
        .collect()
}

fn barycenter(
    graph: &LGraph,
    node_id: &ElementId,
    previous_order: &BTreeMap<ElementId, usize>,
) -> Option<f64> {
    let mut total = 0.0;
    let mut count = 0usize;
    for edge in &graph.edges {
        if edge.kind.is_self_loop() {
            continue;
        }
        if edge.target.node == *node_id {
            if let Some(order) = previous_order.get(&edge.source.node) {
                total += *order as f64;
                count += 1;
            }
        }
    }
    (count > 0).then_some(total / count as f64)
}

fn compare_barycenter(
    left: &(usize, LNode, Option<f64>),
    right: &(usize, LNode, Option<f64>),
) -> Ordering {
    match (left.2, right.2) {
        (Some(left_score), Some(right_score)) => left_score
            .partial_cmp(&right_score)
            .unwrap_or(Ordering::Equal)
            .then_with(|| left.0.cmp(&right.0)),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => left.0.cmp(&right.0),
    }
}
