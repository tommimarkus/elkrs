use elkrs_core::geometry::{Point, Size};
use elkrs_core::graph::ElementId;

#[derive(Debug, Clone)]
pub(crate) struct LGraph {
    pub(crate) nodes: Vec<LNode>,
    pub(crate) edges: Vec<LEdge>,
}

#[derive(Debug, Clone)]
pub(crate) struct LNode {
    pub(crate) id: ElementId,
    pub(crate) size: Size,
    pub(crate) position: Point,
    pub(crate) layer: usize,
    pub(crate) parent: Option<ElementId>,
}

#[derive(Debug, Clone)]
pub(crate) struct LEdge {
    pub(crate) id: ElementId,
    pub(crate) source: ElementId,
    pub(crate) target: ElementId,
    pub(crate) reversed: bool,
    pub(crate) points: Vec<Point>,
}
