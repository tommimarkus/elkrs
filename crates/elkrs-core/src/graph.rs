use indexmap::IndexMap;

use crate::geometry::{Point, Size};
use crate::options::{PortSide, Properties};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ElementId(String);

impl ElementId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for ElementId {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ElkGraph {
    pub id: ElementId,
    pub properties: Properties,
    pub nodes: IndexMap<ElementId, ElkNode>,
    pub edges: IndexMap<ElementId, ElkEdge>,
}

impl ElkGraph {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            properties: Properties::default(),
            nodes: IndexMap::new(),
            edges: IndexMap::new(),
        }
    }

    pub fn add_node(&mut self, node: ElkNode) -> Option<ElkNode> {
        self.nodes.insert(node.id.clone(), node)
    }

    pub fn add_edge(&mut self, edge: ElkEdge) -> Option<ElkEdge> {
        self.edges.insert(edge.id.clone(), edge)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ElkNode {
    pub id: ElementId,
    pub properties: Properties,
    pub position: Point,
    pub size: Size,
    pub labels: Vec<ElkLabel>,
    pub ports: IndexMap<ElementId, ElkPort>,
    pub children: IndexMap<ElementId, ElkNode>,
}

impl ElkNode {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            properties: Properties::default(),
            position: Point::new(0.0, 0.0),
            size: Size::new(0.0, 0.0),
            labels: Vec::new(),
            ports: IndexMap::new(),
            children: IndexMap::new(),
        }
    }

    pub fn add_port(&mut self, port: ElkPort) -> Option<ElkPort> {
        self.ports.insert(port.id.clone(), port)
    }

    pub fn add_child(&mut self, child: ElkNode) -> Option<ElkNode> {
        self.children.insert(child.id.clone(), child)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ElkPort {
    pub id: ElementId,
    pub properties: Properties,
    pub side: Option<PortSide>,
    pub position: Point,
    pub size: Size,
}

impl ElkPort {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            properties: Properties::default(),
            side: None,
            position: Point::new(0.0, 0.0),
            size: Size::new(0.0, 0.0),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ElkEdge {
    pub id: ElementId,
    pub source: ElementRef,
    pub target: ElementRef,
    pub labels: Vec<ElkLabel>,
    pub sections: Vec<ElkEdgeSection>,
}

impl ElkEdge {
    pub fn new(id: impl Into<ElementId>, source: ElementRef, target: ElementRef) -> Self {
        Self {
            id: id.into(),
            source,
            target,
            labels: Vec::new(),
            sections: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElementRef {
    Node(ElementId),
    Port { node: ElementId, port: ElementId },
}

#[derive(Debug, Clone, PartialEq)]
pub struct ElkLabel {
    pub text: String,
    pub position: Point,
    pub size: Size,
}

impl ElkLabel {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            position: Point::new(0.0, 0.0),
            size: Size::new(0.0, 0.0),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ElkEdgeSection {
    pub points: Vec<Point>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graph_preserves_node_insertion_order() {
        let mut graph = ElkGraph::new("root");
        graph.add_node(ElkNode::new("first"));
        graph.add_node(ElkNode::new("second"));

        let ids: Vec<_> = graph.nodes.keys().map(ElementId::as_str).collect();

        assert_eq!(ids, vec!["first", "second"]);
    }

    #[test]
    fn node_tracks_ports_and_children() {
        let mut node = ElkNode::new("parent");
        node.add_port(ElkPort::new("port-a"));
        node.add_child(ElkNode::new("child-a"));

        assert!(node.ports.contains_key(&ElementId::from("port-a")));
        assert!(node.children.contains_key(&ElementId::from("child-a")));
    }

    #[test]
    fn port_scoped_options_are_stored_on_ports() {
        let mut port = ElkPort::new("port-a");
        port.properties.set_port_index(3);

        assert_eq!(port.properties.port_index(), Some(3));
    }

    #[test]
    fn element_id_new_preserves_text() {
        let id = ElementId::new("node-a");

        assert_eq!(id.as_str(), "node-a");
    }

    #[test]
    fn label_new_preserves_text_and_defaults_geometry() {
        let label = ElkLabel::new("caption");

        assert_eq!(label.text, "caption");
        assert_eq!(label.position, Point::new(0.0, 0.0));
        assert_eq!(label.size, Size::new(0.0, 0.0));
    }
}
