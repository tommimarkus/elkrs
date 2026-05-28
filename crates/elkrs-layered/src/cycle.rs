use std::collections::BTreeMap;

use elkrs_core::layout::LayoutError;

use crate::internal::LGraph;
use crate::pipeline::{LayeredContext, LayeredProcessor};

pub(crate) struct CycleBreaking;

impl LayeredProcessor for CycleBreaking {
    fn name(&self) -> &'static str {
        "cycle-breaking"
    }

    fn run(&self, graph: &mut LGraph, _context: &mut LayeredContext) -> Result<(), LayoutError> {
        let mut node_ids = graph
            .nodes
            .iter()
            .map(|node| node.id.clone())
            .collect::<Vec<_>>();
        node_ids.sort();
        let order = node_ids
            .into_iter()
            .enumerate()
            .map(|(index, id)| (id, index))
            .collect::<BTreeMap<_, _>>();
        for edge in &mut graph.edges {
            let source = order.get(&edge.source.node).copied().unwrap_or(0);
            let target = order.get(&edge.target.node).copied().unwrap_or(0);
            if source > target {
                std::mem::swap(&mut edge.source, &mut edge.target);
                edge.reversed = !edge.reversed;
            }
        }
        Ok(())
    }
}
