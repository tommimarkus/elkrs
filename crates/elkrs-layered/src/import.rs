use elkrs_core::graph::{ElementId, ElementRef, ElkGraph, ElkNode};
use elkrs_core::layout::LayoutError;

use crate::internal::{LEdge, LGraph, LNode};

pub(crate) fn import_graph(graph: &ElkGraph) -> Result<LGraph, LayoutError> {
    let mut nodes = Vec::new();
    for node in graph.nodes.values() {
        import_node(node, None, &mut nodes);
    }

    let mut edges = Vec::new();
    for edge in graph.edges.values() {
        let source = endpoint_node(graph, &edge.source)?;
        let target = endpoint_node(graph, &edge.target)?;
        if !contains_node(&nodes, &source) {
            return Err(LayoutError::MissingEndpoint(source.as_str().to_string()));
        }
        if !contains_node(&nodes, &target) {
            return Err(LayoutError::MissingEndpoint(target.as_str().to_string()));
        }
        edges.push(LEdge {
            id: edge.id.clone(),
            source,
            target,
            reversed: false,
            points: Vec::new(),
        });
    }

    Ok(LGraph { nodes, edges })
}

fn import_node(node: &ElkNode, parent: Option<ElementId>, nodes: &mut Vec<LNode>) {
    nodes.push(LNode {
        id: node.id.clone(),
        size: node.size,
        position: node.position,
        layer: 0,
        parent: parent.clone(),
    });
    for child in node.children.values() {
        import_node(child, Some(node.id.clone()), nodes);
    }
}

fn endpoint_node(graph: &ElkGraph, endpoint: &ElementRef) -> Result<ElementId, LayoutError> {
    match endpoint {
        ElementRef::Node(node) => Ok(node.clone()),
        ElementRef::Port { node, port } => {
            let owner = find_node(graph, node).ok_or_else(|| {
                LayoutError::MissingEndpoint(format!("{}:{}", node.as_str(), port.as_str()))
            })?;
            if owner.ports.contains_key(port) {
                Ok(node.clone())
            } else {
                Err(LayoutError::MissingEndpoint(format!(
                    "{}:{}",
                    node.as_str(),
                    port.as_str()
                )))
            }
        }
    }
}

fn find_node<'a>(graph: &'a ElkGraph, id: &ElementId) -> Option<&'a ElkNode> {
    for node in graph.nodes.values() {
        if let Some(found) = find_node_in_subtree(node, id) {
            return Some(found);
        }
    }
    None
}

fn find_node_in_subtree<'a>(node: &'a ElkNode, id: &ElementId) -> Option<&'a ElkNode> {
    if node.id == *id {
        return Some(node);
    }
    for child in node.children.values() {
        if let Some(found) = find_node_in_subtree(child, id) {
            return Some(found);
        }
    }
    None
}

fn contains_node(nodes: &[LNode], id: &ElementId) -> bool {
    nodes.iter().any(|node| node.id == *id)
}

#[cfg(test)]
mod tests {
    use elkrs_core::geometry::Size;
    use elkrs_core::graph::{ElkEdge, ElkPort};
    use elkrs_core::options::PortSide;

    use super::*;

    #[test]
    fn import_flattens_child_nodes_with_parent_reference() {
        let mut parent = ElkNode::new("group");
        parent.size = Size::new(200.0, 100.0);
        let mut child = ElkNode::new("child");
        child.size = Size::new(40.0, 20.0);
        parent.add_child(child);
        let mut graph = ElkGraph::new("root");
        graph.add_node(parent);

        let layered = import_graph(&graph).unwrap();

        let child = layered
            .nodes
            .iter()
            .find(|node| node.id.as_str() == "child")
            .unwrap();
        assert_eq!(child.parent.as_ref().map(ElementId::as_str), Some("group"));
    }

    #[test]
    fn import_rejects_missing_edge_endpoint() {
        let mut graph = ElkGraph::new("root");
        graph.add_node(ElkNode::new("a"));
        graph.add_edge(ElkEdge::new(
            "e",
            ElementRef::Node(ElementId::from("a")),
            ElementRef::Node(ElementId::from("missing")),
        ));

        assert!(matches!(
            import_graph(&graph),
            Err(LayoutError::MissingEndpoint(endpoint)) if endpoint == "missing"
        ));
    }

    #[test]
    fn import_maps_port_endpoints_to_owner_nodes() {
        let mut source = ElkNode::new("source");
        let mut source_port = ElkPort::new("out");
        source_port.side = Some(PortSide::East);
        source.add_port(source_port);
        let mut target = ElkNode::new("target");
        let mut target_port = ElkPort::new("in");
        target_port.side = Some(PortSide::West);
        target.add_port(target_port);

        let mut graph = ElkGraph::new("root");
        graph.add_node(source);
        graph.add_node(target);
        graph.add_edge(ElkEdge::new(
            "edge",
            ElementRef::Port {
                node: ElementId::from("source"),
                port: ElementId::from("out"),
            },
            ElementRef::Port {
                node: ElementId::from("target"),
                port: ElementId::from("in"),
            },
        ));

        let layered = import_graph(&graph).unwrap();

        assert_eq!(layered.edges[0].source.as_str(), "source");
        assert_eq!(layered.edges[0].target.as_str(), "target");
    }
}
