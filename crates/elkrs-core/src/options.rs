use std::collections::BTreeMap;

pub const DEFAULT_NODE_NODE_SPACING: f64 = 80.0;
pub const DEFAULT_LAYER_NODE_NODE_SPACING: f64 = 120.0;
pub const DEFAULT_EDGE_NODE_SPACING: f64 = 20.0;
pub const DEFAULT_EDGE_EDGE_SPACING: f64 = 10.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Right,
    Left,
    Down,
    Up,
}

impl Direction {
    pub fn is_horizontal(self) -> bool {
        matches!(self, Self::Right | Self::Left)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Algorithm {
    Layered,
    Other(String),
}

impl Algorithm {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Layered => "layered",
            Self::Other(value) => value,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeRouting {
    Orthogonal,
    Polyline,
    Splines,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortSide {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HierarchyHandling {
    SeparateChildren,
    IncludeChildren,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PropertyValue {
    Bool(bool),
    Number(f64),
    Text(String),
    Algorithm(Algorithm),
    Direction(Direction),
    EdgeRouting(EdgeRouting),
    HierarchyHandling(HierarchyHandling),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CoreOption {
    Algorithm,
    DebugMode,
    Direction,
    EdgeRouting,
    HierarchyHandling,
    SpacingNodeNode,
    SpacingLayerNodeNode,
    SpacingEdgeNode,
    SpacingEdgeEdge,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Properties {
    values: BTreeMap<CoreOption, PropertyValue>,
}

impl Properties {
    pub fn set_algorithm(&mut self, algorithm: Algorithm) -> Option<PropertyValue> {
        self.values
            .insert(CoreOption::Algorithm, PropertyValue::Algorithm(algorithm))
    }

    pub fn set_debug_mode(&mut self, enabled: bool) -> Option<PropertyValue> {
        self.values
            .insert(CoreOption::DebugMode, PropertyValue::Bool(enabled))
    }

    pub fn set_direction(&mut self, direction: Direction) -> Option<PropertyValue> {
        self.values
            .insert(CoreOption::Direction, PropertyValue::Direction(direction))
    }

    pub fn set_edge_routing(&mut self, edge_routing: EdgeRouting) -> Option<PropertyValue> {
        self.values.insert(
            CoreOption::EdgeRouting,
            PropertyValue::EdgeRouting(edge_routing),
        )
    }

    pub fn set_hierarchy_handling(
        &mut self,
        hierarchy_handling: HierarchyHandling,
    ) -> Option<PropertyValue> {
        self.values.insert(
            CoreOption::HierarchyHandling,
            PropertyValue::HierarchyHandling(hierarchy_handling),
        )
    }

    pub fn set_spacing_node_node(&mut self, spacing: f64) -> Option<PropertyValue> {
        self.values
            .insert(CoreOption::SpacingNodeNode, PropertyValue::Number(spacing))
    }

    pub fn set_spacing_layer_node_node(&mut self, spacing: f64) -> Option<PropertyValue> {
        self.values.insert(
            CoreOption::SpacingLayerNodeNode,
            PropertyValue::Number(spacing),
        )
    }

    pub fn set_spacing_edge_node(&mut self, spacing: f64) -> Option<PropertyValue> {
        self.values
            .insert(CoreOption::SpacingEdgeNode, PropertyValue::Number(spacing))
    }

    pub fn set_spacing_edge_edge(&mut self, spacing: f64) -> Option<PropertyValue> {
        self.values
            .insert(CoreOption::SpacingEdgeEdge, PropertyValue::Number(spacing))
    }

    pub fn get(&self, option: CoreOption) -> Option<&PropertyValue> {
        self.values.get(&option)
    }

    pub fn algorithm(&self) -> Option<Algorithm> {
        match self.get(CoreOption::Algorithm) {
            Some(PropertyValue::Algorithm(algorithm)) => Some(algorithm.clone()),
            Some(value) => unreachable!("algorithm option stored incompatible value: {value:?}"),
            _ => None,
        }
    }

    pub fn debug_mode(&self) -> bool {
        match self.get(CoreOption::DebugMode) {
            Some(PropertyValue::Bool(enabled)) => *enabled,
            Some(value) => unreachable!("debug mode option stored incompatible value: {value:?}"),
            _ => false,
        }
    }

    pub fn direction(&self) -> Direction {
        match self.get(CoreOption::Direction) {
            Some(PropertyValue::Direction(direction)) => *direction,
            Some(value) => unreachable!("direction option stored incompatible value: {value:?}"),
            _ => Direction::Right,
        }
    }

    pub fn edge_routing(&self) -> EdgeRouting {
        match self.get(CoreOption::EdgeRouting) {
            Some(PropertyValue::EdgeRouting(edge_routing)) => *edge_routing,
            Some(value) => unreachable!("edge routing option stored incompatible value: {value:?}"),
            _ => EdgeRouting::Orthogonal,
        }
    }

    pub fn hierarchy_handling(&self) -> HierarchyHandling {
        match self.get(CoreOption::HierarchyHandling) {
            Some(PropertyValue::HierarchyHandling(hierarchy_handling)) => *hierarchy_handling,
            Some(value) => {
                unreachable!("hierarchy handling option stored incompatible value: {value:?}")
            }
            _ => HierarchyHandling::IncludeChildren,
        }
    }

    pub fn spacing_node_node(&self) -> f64 {
        match self.get(CoreOption::SpacingNodeNode) {
            Some(PropertyValue::Number(spacing)) => *spacing,
            Some(value) => unreachable!("node-node spacing stored incompatible value: {value:?}"),
            _ => DEFAULT_NODE_NODE_SPACING,
        }
    }

    pub fn spacing_layer_node_node(&self) -> f64 {
        match self.get(CoreOption::SpacingLayerNodeNode) {
            Some(PropertyValue::Number(spacing)) => *spacing,
            Some(value) => {
                unreachable!("layer node-node spacing stored incompatible value: {value:?}")
            }
            _ => DEFAULT_LAYER_NODE_NODE_SPACING,
        }
    }

    pub fn spacing_edge_node(&self) -> f64 {
        match self.get(CoreOption::SpacingEdgeNode) {
            Some(PropertyValue::Number(spacing)) => *spacing,
            Some(value) => unreachable!("edge-node spacing stored incompatible value: {value:?}"),
            _ => DEFAULT_EDGE_NODE_SPACING,
        }
    }

    pub fn spacing_edge_edge(&self) -> f64 {
        match self.get(CoreOption::SpacingEdgeEdge) {
            Some(PropertyValue::Number(spacing)) => *spacing,
            Some(value) => unreachable!("edge-edge spacing stored incompatible value: {value:?}"),
            _ => DEFAULT_EDGE_EDGE_SPACING,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direction_defaults_to_right() {
        let properties = Properties::default();

        assert_eq!(properties.direction(), Direction::Right);
    }

    #[test]
    fn direction_uses_override() {
        let mut properties = Properties::default();
        properties.set_direction(Direction::Down);

        assert_eq!(properties.direction(), Direction::Down);
    }

    #[test]
    fn algorithm_option_can_be_set() {
        let mut properties = Properties::default();

        assert_eq!(properties.algorithm(), None);
        properties.set_algorithm(Algorithm::Layered);

        assert_eq!(properties.algorithm(), Some(Algorithm::Layered));
    }

    #[test]
    fn debug_mode_defaults_to_false_and_can_be_set() {
        let mut properties = Properties::default();

        assert!(!properties.debug_mode());
        properties.set_debug_mode(true);

        assert!(properties.debug_mode());
    }

    #[test]
    fn edge_routing_defaults_to_orthogonal_and_can_be_set() {
        let mut properties = Properties::default();

        assert_eq!(properties.edge_routing(), EdgeRouting::Orthogonal);
        properties.set_edge_routing(EdgeRouting::Polyline);

        assert_eq!(properties.edge_routing(), EdgeRouting::Polyline);
    }

    #[test]
    fn hierarchy_handling_defaults_to_include_children_and_can_be_set() {
        let mut properties = Properties::default();

        assert_eq!(
            properties.hierarchy_handling(),
            HierarchyHandling::IncludeChildren
        );
        properties.set_hierarchy_handling(HierarchyHandling::SeparateChildren);

        assert_eq!(
            properties.hierarchy_handling(),
            HierarchyHandling::SeparateChildren
        );
    }

    #[test]
    fn spacing_defaults_match_layered_layout_defaults() {
        let properties = Properties::default();

        assert_eq!(properties.spacing_node_node(), DEFAULT_NODE_NODE_SPACING);
        assert_eq!(
            properties.spacing_layer_node_node(),
            DEFAULT_LAYER_NODE_NODE_SPACING
        );
        assert_eq!(properties.spacing_edge_node(), DEFAULT_EDGE_NODE_SPACING);
        assert_eq!(properties.spacing_edge_edge(), DEFAULT_EDGE_EDGE_SPACING);
    }

    #[test]
    fn spacing_values_can_be_overridden() {
        let mut properties = Properties::default();

        properties.set_spacing_node_node(42.0);
        properties.set_spacing_layer_node_node(300.0);
        properties.set_spacing_edge_node(12.0);
        properties.set_spacing_edge_edge(24.0);

        assert_eq!(properties.spacing_node_node(), 42.0);
        assert_eq!(properties.spacing_layer_node_node(), 300.0);
        assert_eq!(properties.spacing_edge_node(), 12.0);
        assert_eq!(properties.spacing_edge_edge(), 24.0);
    }
}
