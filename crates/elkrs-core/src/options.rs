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
    CommentBox,
    ConnectedComponentsCompaction,
    ConsiderPortOrder,
    DebugMode,
    Direction,
    EdgeRouting,
    FavorStraightEdges,
    FeedbackEdges,
    FixedGraphSize,
    ForceNodeModelOrder,
    GeneratePositionAndLayerIds,
    HighDegreeNodeTreatment,
    HierarchyHandling,
    Hypernode,
    InsideSelfLoops,
    InteractiveLayout,
    LayerUnzippingMinimizeEdgeLength,
    LayoutPartitioning,
    MergeEdges,
    NoModelOrder,
    PortLabelsNextToPortIfPossible,
    SemiInteractiveCrossingMinimization,
    SpacingNodeNode,
    SpacingLayerNodeNode,
    SpacingEdgeNode,
    SpacingEdgeEdge,
    TopdownLayout,
    UnnecessaryBendpoints,
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

    pub fn set_comment_box(&mut self, enabled: bool) -> Option<PropertyValue> {
        self.set_bool_option(CoreOption::CommentBox, enabled)
    }

    pub fn set_connected_components_compaction(&mut self, enabled: bool) -> Option<PropertyValue> {
        self.set_bool_option(CoreOption::ConnectedComponentsCompaction, enabled)
    }

    pub fn set_consider_port_order(&mut self, enabled: bool) -> Option<PropertyValue> {
        self.set_bool_option(CoreOption::ConsiderPortOrder, enabled)
    }

    pub fn set_debug_mode(&mut self, enabled: bool) -> Option<PropertyValue> {
        self.set_bool_option(CoreOption::DebugMode, enabled)
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

    pub fn set_feedback_edges(&mut self, enabled: bool) -> Option<PropertyValue> {
        self.set_bool_option(CoreOption::FeedbackEdges, enabled)
    }

    pub fn set_favor_straight_edges(&mut self, enabled: bool) -> Option<PropertyValue> {
        self.set_bool_option(CoreOption::FavorStraightEdges, enabled)
    }

    pub fn set_fixed_graph_size(&mut self, enabled: bool) -> Option<PropertyValue> {
        self.set_bool_option(CoreOption::FixedGraphSize, enabled)
    }

    pub fn set_force_node_model_order(&mut self, enabled: bool) -> Option<PropertyValue> {
        self.set_bool_option(CoreOption::ForceNodeModelOrder, enabled)
    }

    pub fn set_generate_position_and_layer_ids(&mut self, enabled: bool) -> Option<PropertyValue> {
        self.set_bool_option(CoreOption::GeneratePositionAndLayerIds, enabled)
    }

    pub fn set_high_degree_node_treatment(&mut self, enabled: bool) -> Option<PropertyValue> {
        self.set_bool_option(CoreOption::HighDegreeNodeTreatment, enabled)
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

    pub fn set_hypernode(&mut self, enabled: bool) -> Option<PropertyValue> {
        self.set_bool_option(CoreOption::Hypernode, enabled)
    }

    pub fn set_inside_self_loops(&mut self, enabled: bool) -> Option<PropertyValue> {
        self.set_bool_option(CoreOption::InsideSelfLoops, enabled)
    }

    pub fn set_interactive_layout(&mut self, enabled: bool) -> Option<PropertyValue> {
        self.set_bool_option(CoreOption::InteractiveLayout, enabled)
    }

    pub fn set_layer_unzipping_minimize_edge_length(
        &mut self,
        enabled: bool,
    ) -> Option<PropertyValue> {
        self.set_bool_option(CoreOption::LayerUnzippingMinimizeEdgeLength, enabled)
    }

    pub fn set_layout_partitioning(&mut self, enabled: bool) -> Option<PropertyValue> {
        self.set_bool_option(CoreOption::LayoutPartitioning, enabled)
    }

    pub fn set_merge_edges(&mut self, enabled: bool) -> Option<PropertyValue> {
        self.set_bool_option(CoreOption::MergeEdges, enabled)
    }

    pub fn set_no_model_order(&mut self, enabled: bool) -> Option<PropertyValue> {
        self.set_bool_option(CoreOption::NoModelOrder, enabled)
    }

    pub fn set_port_labels_next_to_port_if_possible(
        &mut self,
        enabled: bool,
    ) -> Option<PropertyValue> {
        self.set_bool_option(CoreOption::PortLabelsNextToPortIfPossible, enabled)
    }

    pub fn set_semi_interactive_crossing_minimization(
        &mut self,
        enabled: bool,
    ) -> Option<PropertyValue> {
        self.set_bool_option(CoreOption::SemiInteractiveCrossingMinimization, enabled)
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

    pub fn set_topdown_layout(&mut self, enabled: bool) -> Option<PropertyValue> {
        self.set_bool_option(CoreOption::TopdownLayout, enabled)
    }

    pub fn set_unnecessary_bendpoints(&mut self, enabled: bool) -> Option<PropertyValue> {
        self.set_bool_option(CoreOption::UnnecessaryBendpoints, enabled)
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

    pub fn connected_components_compaction(&self) -> bool {
        self.bool_option(
            CoreOption::ConnectedComponentsCompaction,
            "connected components compaction",
        )
    }

    pub fn comment_box(&self) -> bool {
        self.bool_option(CoreOption::CommentBox, "comment box")
    }

    pub fn consider_port_order(&self) -> bool {
        self.bool_option(CoreOption::ConsiderPortOrder, "consider port order")
    }

    pub fn debug_mode(&self) -> bool {
        self.bool_option(CoreOption::DebugMode, "debug mode")
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

    pub fn feedback_edges(&self) -> bool {
        self.bool_option(CoreOption::FeedbackEdges, "feedback edges")
    }

    pub fn favor_straight_edges(&self) -> bool {
        self.bool_option(CoreOption::FavorStraightEdges, "favor straight edges")
    }

    pub fn fixed_graph_size(&self) -> bool {
        self.bool_option(CoreOption::FixedGraphSize, "fixed graph size")
    }

    pub fn force_node_model_order(&self) -> bool {
        self.bool_option(CoreOption::ForceNodeModelOrder, "force node model order")
    }

    pub fn generate_position_and_layer_ids(&self) -> bool {
        self.bool_option(
            CoreOption::GeneratePositionAndLayerIds,
            "generate position and layer ids",
        )
    }

    pub fn high_degree_node_treatment(&self) -> bool {
        self.bool_option(
            CoreOption::HighDegreeNodeTreatment,
            "high degree node treatment",
        )
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

    pub fn hypernode(&self) -> bool {
        self.bool_option(CoreOption::Hypernode, "hypernode")
    }

    pub fn inside_self_loops(&self) -> bool {
        self.bool_option(CoreOption::InsideSelfLoops, "inside self-loops")
    }

    pub fn interactive_layout(&self) -> bool {
        self.bool_option(CoreOption::InteractiveLayout, "interactive layout")
    }

    pub fn layer_unzipping_minimize_edge_length(&self) -> bool {
        self.bool_option(
            CoreOption::LayerUnzippingMinimizeEdgeLength,
            "layer unzipping minimize edge length",
        )
    }

    pub fn layout_partitioning(&self) -> bool {
        self.bool_option(CoreOption::LayoutPartitioning, "layout partitioning")
    }

    pub fn merge_edges(&self) -> bool {
        self.bool_option(CoreOption::MergeEdges, "merge edges")
    }

    pub fn no_model_order(&self) -> bool {
        self.bool_option(CoreOption::NoModelOrder, "no model order")
    }

    pub fn port_labels_next_to_port_if_possible(&self) -> bool {
        self.bool_option(
            CoreOption::PortLabelsNextToPortIfPossible,
            "port labels next to port if possible",
        )
    }

    pub fn semi_interactive_crossing_minimization(&self) -> bool {
        self.bool_option(
            CoreOption::SemiInteractiveCrossingMinimization,
            "semi-interactive crossing minimization",
        )
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

    pub fn topdown_layout(&self) -> bool {
        self.bool_option(CoreOption::TopdownLayout, "topdown layout")
    }

    pub fn unnecessary_bendpoints(&self) -> bool {
        self.bool_option(CoreOption::UnnecessaryBendpoints, "unnecessary bendpoints")
    }

    fn set_bool_option(&mut self, option: CoreOption, enabled: bool) -> Option<PropertyValue> {
        self.values.insert(option, PropertyValue::Bool(enabled))
    }

    fn bool_option(&self, option: CoreOption, name: &str) -> bool {
        match self.get(option) {
            Some(PropertyValue::Bool(enabled)) => *enabled,
            Some(value) => unreachable!("{name} option stored incompatible value: {value:?}"),
            _ => false,
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
    fn feedback_edges_defaults_to_false_and_can_be_set() {
        let mut properties = Properties::default();

        assert!(!properties.feedback_edges());
        properties.set_feedback_edges(true);

        assert!(properties.feedback_edges());
    }

    #[test]
    fn parent_boolean_options_default_to_false_and_can_be_set() {
        let mut properties = Properties::default();

        assert!(!properties.generate_position_and_layer_ids());
        assert!(!properties.connected_components_compaction());
        assert!(!properties.consider_port_order());
        assert!(!properties.favor_straight_edges());
        assert!(!properties.fixed_graph_size());
        assert!(!properties.force_node_model_order());
        assert!(!properties.high_degree_node_treatment());
        assert!(!properties.interactive_layout());
        assert!(!properties.layout_partitioning());
        assert!(!properties.merge_edges());
        assert!(!properties.semi_interactive_crossing_minimization());
        assert!(!properties.topdown_layout());
        assert!(!properties.unnecessary_bendpoints());

        properties.set_generate_position_and_layer_ids(true);
        properties.set_connected_components_compaction(true);
        properties.set_consider_port_order(true);
        properties.set_favor_straight_edges(true);
        properties.set_fixed_graph_size(true);
        properties.set_force_node_model_order(true);
        properties.set_high_degree_node_treatment(true);
        properties.set_interactive_layout(true);
        properties.set_layout_partitioning(true);
        properties.set_merge_edges(true);
        properties.set_semi_interactive_crossing_minimization(true);
        properties.set_topdown_layout(true);
        properties.set_unnecessary_bendpoints(true);

        assert!(properties.generate_position_and_layer_ids());
        assert!(properties.connected_components_compaction());
        assert!(properties.consider_port_order());
        assert!(properties.favor_straight_edges());
        assert!(properties.fixed_graph_size());
        assert!(properties.force_node_model_order());
        assert!(properties.high_degree_node_treatment());
        assert!(properties.interactive_layout());
        assert!(properties.layout_partitioning());
        assert!(properties.merge_edges());
        assert!(properties.semi_interactive_crossing_minimization());
        assert!(properties.topdown_layout());
        assert!(properties.unnecessary_bendpoints());
    }

    #[test]
    fn node_boolean_options_default_to_false_and_can_be_set() {
        let mut properties = Properties::default();

        assert!(!properties.comment_box());
        assert!(!properties.hypernode());
        assert!(!properties.inside_self_loops());
        assert!(!properties.no_model_order());
        assert!(!properties.layer_unzipping_minimize_edge_length());
        assert!(!properties.port_labels_next_to_port_if_possible());

        properties.set_comment_box(true);
        properties.set_hypernode(true);
        properties.set_inside_self_loops(true);
        properties.set_no_model_order(true);
        properties.set_layer_unzipping_minimize_edge_length(true);
        properties.set_port_labels_next_to_port_if_possible(true);

        assert!(properties.comment_box());
        assert!(properties.hypernode());
        assert!(properties.inside_self_loops());
        assert!(properties.no_model_order());
        assert!(properties.layer_unzipping_minimize_edge_length());
        assert!(properties.port_labels_next_to_port_if_possible());
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
