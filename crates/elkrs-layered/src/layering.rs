use std::collections::BTreeMap;

use elkrs_core::layout::LayoutError;

use crate::internal::LGraph;
use crate::pipeline::{LayeredContext, LayeredProcessor};

pub(crate) struct LayerAssignment;

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
                if edge.kind.is_self_loop() {
                    continue;
                }
                let source_layer = *layers.get(&edge.source.node).unwrap_or(&0);
                let target_layer = *layers.get(&edge.target.node).unwrap_or(&0);
                if target_layer <= source_layer {
                    layers.insert(edge.target.node.clone(), source_layer + 1);
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
