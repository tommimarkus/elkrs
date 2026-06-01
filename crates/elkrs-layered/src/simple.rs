use elkrs_core::graph::ElkGraph;
use elkrs_core::layout::{LayoutError, LayoutReport};

use crate::crossing::CrossingMinimization;
use crate::cycle::CycleBreaking;
use crate::import::import_graph;
use crate::layering::LayerAssignment;
use crate::pipeline::LayeredPipeline;
use crate::placement::NodePlacement;
use crate::routing::EdgeRouting;
use crate::validation::validate_options;
use crate::writeback::write_back;

pub struct LayeredLayout;

pub trait LayoutAlgorithm {
    fn layout(&self, graph: &mut ElkGraph) -> Result<LayoutReport, LayoutError>;
}

impl LayoutAlgorithm for LayeredLayout {
    fn layout(&self, graph: &mut ElkGraph) -> Result<LayoutReport, LayoutError> {
        let mut diagnostics = validate_options(&graph.properties)?;
        let direction = graph.properties.direction();
        let node_placement = NodePlacement::from_properties(direction, &graph.properties);
        let mut layered = import_graph(graph)?;
        let pipeline = LayeredPipeline::new(vec![
            Box::new(CycleBreaking),
            Box::new(LayerAssignment),
            Box::new(CrossingMinimization),
            Box::new(node_placement),
            Box::new(EdgeRouting::from_properties(direction, &graph.properties)),
        ]);
        let context = pipeline.run(&mut layered)?;
        write_back(graph, &layered);
        diagnostics.extend(context.diagnostics);
        Ok(LayoutReport { diagnostics })
    }
}

#[cfg(test)]
mod tests {
    use elkrs_core::geometry::Size;
    use elkrs_core::graph::{ElementId, ElementRef, ElkEdge, ElkNode};

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
