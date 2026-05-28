use std::collections::BTreeMap;

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
}
