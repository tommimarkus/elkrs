use std::collections::BTreeMap;

use elkrs_core::graph::{ElementId, ElementRef, ElkGraph, ElkNode};
use elkrs_core::layout::LayoutError;
use elkrs_core::options::{CoreOption, PropertyValue};

use crate::internal::{LEdge, LEdgeKind, LEndpoint, LGraph, LNode, LPort};

pub(crate) fn import_graph(graph: &ElkGraph) -> Result<LGraph, LayoutError> {
    let mut nodes = Vec::new();
    let mut node_ids = BTreeMap::<ElementId, ()>::new();
    for node in graph.nodes.values() {
        import_node(node, None, &mut nodes, &mut node_ids)?;
    }

    let mut edges = Vec::new();
    for edge in graph.edges.values() {
        let source = endpoint_node(graph, &edge.source)?;
        let target = endpoint_node(graph, &edge.target)?;
        if !contains_node(&nodes, &source.node) {
            return Err(LayoutError::MissingEndpoint(
                source.node.as_str().to_string(),
            ));
        }
        if !contains_node(&nodes, &target.node) {
            return Err(LayoutError::MissingEndpoint(
                target.node.as_str().to_string(),
            ));
        }
        let kind = if source.node == target.node {
            LEdgeKind::SelfLoop
        } else {
            LEdgeKind::Normal
        };
        edges.push(LEdge {
            id: edge.id.clone(),
            source,
            target,
            kind,
            reversed: false,
            points: Vec::new(),
        });
    }

    Ok(LGraph { nodes, edges })
}

fn import_node(
    node: &ElkNode,
    parent: Option<ElementId>,
    nodes: &mut Vec<LNode>,
    node_ids: &mut BTreeMap<ElementId, ()>,
) -> Result<(), LayoutError> {
    if node_ids.insert(node.id.clone(), ()).is_some() {
        return Err(LayoutError::InvalidHierarchy(format!(
            "duplicate node id: {}",
            node.id.as_str()
        )));
    }

    let ports = node
        .ports
        .values()
        .map(|port| {
            (
                port.id.clone(),
                LPort {
                    id: port.id.clone(),
                    side: port.side,
                    position: port.position,
                    size: port.size,
                },
            )
        })
        .collect::<BTreeMap<_, _>>();

    nodes.push(LNode {
        id: node.id.clone(),
        size: node.size,
        position: node.position,
        layer: 0,
        no_layout: node.properties.no_layout(),
        port_port_spacing: node_port_port_spacing(node),
        parent: parent.clone(),
        ports,
    });
    for child in node.children.values() {
        import_node(child, Some(node.id.clone()), nodes, node_ids)?;
    }
    Ok(())
}

fn node_port_port_spacing(node: &ElkNode) -> Option<f64> {
    match node.properties.get(CoreOption::SpacingPortPort) {
        Some(PropertyValue::Number(spacing)) => Some(*spacing),
        Some(value) => unreachable!("port-port spacing stored incompatible value: {value:?}"),
        _ => None,
    }
}

fn endpoint_node(graph: &ElkGraph, endpoint: &ElementRef) -> Result<LEndpoint, LayoutError> {
    match endpoint {
        ElementRef::Node(node) => Ok(LEndpoint {
            node: node.clone(),
            port: None,
        }),
        ElementRef::Port { node, port } => {
            let owner = find_node(graph, node).ok_or_else(|| {
                LayoutError::MissingEndpoint(format!("{}:{}", node.as_str(), port.as_str()))
            })?;
            if owner.ports.contains_key(port) {
                Ok(LEndpoint {
                    node: node.clone(),
                    port: Some(port.clone()),
                })
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
    use elkrs_core::geometry::{Point, Size};
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
    fn import_rejects_duplicate_node_ids_across_hierarchy() {
        let mut group = ElkNode::new("group");
        group.add_child(ElkNode::new("duplicate"));
        let mut graph = ElkGraph::new("root");
        graph.add_node(ElkNode::new("duplicate"));
        graph.add_node(group);

        assert!(matches!(
            import_graph(&graph),
            Err(LayoutError::InvalidHierarchy(message))
                if message.contains("duplicate node id: duplicate")
        ));
    }

    #[test]
    fn import_accepts_node_self_loop_edges() {
        let mut graph = ElkGraph::new("root");
        graph.add_node(ElkNode::new("a"));
        graph.add_edge(ElkEdge::new(
            "self",
            ElementRef::Node(ElementId::from("a")),
            ElementRef::Node(ElementId::from("a")),
        ));

        let layered = import_graph(&graph).unwrap();

        assert_eq!(layered.edges[0].kind, crate::internal::LEdgeKind::SelfLoop);
    }

    #[test]
    fn import_classifies_inter_node_edges_as_normal() {
        let mut graph = ElkGraph::new("root");
        graph.add_node(ElkNode::new("source"));
        graph.add_node(ElkNode::new("target"));
        graph.add_edge(ElkEdge::new(
            "edge",
            ElementRef::Node(ElementId::from("source")),
            ElementRef::Node(ElementId::from("target")),
        ));

        let layered = import_graph(&graph).unwrap();

        assert_eq!(layered.edges[0].kind, crate::internal::LEdgeKind::Normal);
    }

    #[test]
    fn import_accepts_port_self_loop_edges() {
        let mut node = ElkNode::new("a");
        node.add_port(ElkPort::new("out"));
        node.add_port(ElkPort::new("in"));
        let mut graph = ElkGraph::new("root");
        graph.add_node(node);
        graph.add_edge(ElkEdge::new(
            "self",
            ElementRef::Port {
                node: ElementId::from("a"),
                port: ElementId::from("out"),
            },
            ElementRef::Port {
                node: ElementId::from("a"),
                port: ElementId::from("in"),
            },
        ));

        let layered = import_graph(&graph).unwrap();

        assert_eq!(layered.edges[0].kind, crate::internal::LEdgeKind::SelfLoop);
        assert_eq!(layered.edges[0].source.node.as_str(), "a");
        assert_eq!(
            layered.edges[0].source.port.as_ref().map(ElementId::as_str),
            Some("out")
        );
        assert_eq!(layered.edges[0].target.node.as_str(), "a");
        assert_eq!(
            layered.edges[0].target.port.as_ref().map(ElementId::as_str),
            Some("in")
        );
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

        assert_eq!(layered.edges[0].source.node.as_str(), "source");
        assert_eq!(layered.edges[0].target.node.as_str(), "target");
    }

    #[test]
    fn import_preserves_port_endpoint_identity() {
        let mut source = ElkNode::new("source");
        let mut source_port = ElkPort::new("out");
        source_port.side = Some(PortSide::East);
        source_port.position = Point::new(10.0, 12.0);
        source_port.size = Size::new(3.0, 4.0);
        source.add_port(source_port);
        let mut target = ElkNode::new("target");
        let mut target_port = ElkPort::new("in");
        target_port.side = Some(PortSide::West);
        target_port.position = Point::new(1.5, 2.5);
        target_port.size = Size::new(5.0, 6.0);
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

        assert_eq!(layered.edges[0].source.node.as_str(), "source");
        assert_eq!(
            layered.edges[0].source.port.as_ref().map(ElementId::as_str),
            Some("out")
        );
        assert_eq!(layered.edges[0].target.node.as_str(), "target");
        assert_eq!(
            layered.edges[0].target.port.as_ref().map(ElementId::as_str),
            Some("in")
        );

        let source = layered
            .nodes
            .iter()
            .find(|node| node.id.as_str() == "source")
            .unwrap();
        let source_port = source.ports.get(&ElementId::from("out")).unwrap();
        assert_eq!(source_port.side, Some(PortSide::East));
        assert_eq!(source_port.position, Point::new(10.0, 12.0));
        assert_eq!(source_port.size, Size::new(3.0, 4.0));

        let target = layered
            .nodes
            .iter()
            .find(|node| node.id.as_str() == "target")
            .unwrap();
        let target_port = target.ports.get(&ElementId::from("in")).unwrap();
        assert_eq!(target_port.side, Some(PortSide::West));
        assert_eq!(target_port.position, Point::new(1.5, 2.5));
        assert_eq!(target_port.size, Size::new(5.0, 6.0));
    }
}
