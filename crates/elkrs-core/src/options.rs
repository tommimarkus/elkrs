use std::collections::BTreeMap;

pub const DEFAULT_NODE_NODE_SPACING: f64 = 80.0;
pub const DEFAULT_LAYER_NODE_NODE_SPACING: f64 = 120.0;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeRouting {
    Orthogonal,
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
    Direction(Direction),
    EdgeRouting(EdgeRouting),
    HierarchyHandling(HierarchyHandling),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CoreOption {
    Algorithm,
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
    pub fn set_direction(&mut self, direction: Direction) -> Option<PropertyValue> {
        self.values
            .insert(CoreOption::Direction, PropertyValue::Direction(direction))
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

    pub fn get(&self, option: CoreOption) -> Option<&PropertyValue> {
        self.values.get(&option)
    }

    pub fn direction(&self) -> Direction {
        match self.get(CoreOption::Direction) {
            Some(PropertyValue::Direction(direction)) => *direction,
            Some(value) => unreachable!("direction option stored incompatible value: {value:?}"),
            _ => Direction::Right,
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
    fn spacing_defaults_match_layered_layout_defaults() {
        let properties = Properties::default();

        assert_eq!(properties.spacing_node_node(), DEFAULT_NODE_NODE_SPACING);
        assert_eq!(
            properties.spacing_layer_node_node(),
            DEFAULT_LAYER_NODE_NODE_SPACING
        );
    }

    #[test]
    fn spacing_values_can_be_overridden() {
        let mut properties = Properties::default();

        properties.set_spacing_node_node(42.0);
        properties.set_spacing_layer_node_node(300.0);

        assert_eq!(properties.spacing_node_node(), 42.0);
        assert_eq!(properties.spacing_layer_node_node(), 300.0);
    }
}
