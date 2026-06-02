use std::collections::BTreeMap;

use elkrs_core::geometry::{Point, Size};
use elkrs_core::graph::ElementId;
use elkrs_core::options::PortSide;

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
    pub(crate) no_layout: bool,
    pub(crate) parent: Option<ElementId>,
    pub(crate) ports: BTreeMap<ElementId, LPort>,
}

#[derive(Debug, Clone)]
pub(crate) struct LPort {
    pub(crate) id: ElementId,
    pub(crate) side: Option<PortSide>,
    pub(crate) position: Point,
    pub(crate) size: Size,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct LEndpoint {
    pub(crate) node: ElementId,
    pub(crate) port: Option<ElementId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LEdgeKind {
    Normal,
    SelfLoop,
}

impl LEdgeKind {
    pub(crate) fn is_self_loop(self) -> bool {
        matches!(self, Self::SelfLoop)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct LEdge {
    pub(crate) id: ElementId,
    pub(crate) source: LEndpoint,
    pub(crate) target: LEndpoint,
    pub(crate) kind: LEdgeKind,
    pub(crate) reversed: bool,
    pub(crate) points: Vec<Point>,
}
