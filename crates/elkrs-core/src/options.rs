use std::collections::BTreeMap;

pub const DEFAULT_NODE_NODE_SPACING: f64 = 80.0;
pub const DEFAULT_LAYER_NODE_NODE_SPACING: f64 = 120.0;
pub const DEFAULT_COMPONENT_COMPONENT_SPACING: f64 = 20.0;
pub const DEFAULT_EDGE_NODE_SPACING: f64 = 20.0;
pub const DEFAULT_EDGE_EDGE_SPACING: f64 = 10.0;
pub const DEFAULT_NODE_SELF_LOOP_SPACING: f64 = 10.0;
pub const DEFAULT_PORT_PORT_SPACING: f64 = 10.0;

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
pub enum ComponentOrderingStrategy {
    GroupModelOrder,
    InsidePortSideGroups,
    ModelOrder,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelOrderStrategy {
    NodesAndEdges,
    None,
    PreferEdges,
    PreferNodes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GreedySwitchType {
    Off,
    OneSided,
    TwoSided,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroupOrderingStrategy {
    Enforced,
    ModelOrder,
    OnlyWithinGroup,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LongEdgeOrderingStrategy {
    DummyNodeOver,
    DummyNodeUnder,
    Equal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortSide {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortConstraints {
    FixedOrder,
    FixedPos,
    FixedRatio,
    FixedSide,
    Free,
    Undefined,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortAlignment {
    Begin,
    Center,
    Distributed,
    End,
    Justified,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HierarchyHandling {
    SeparateChildren,
    IncludeChildren,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PropertyValue {
    Bool(bool),
    Integer(i64),
    Number(f64),
    Text(String),
    Algorithm(Algorithm),
    ComponentOrderingStrategy(ComponentOrderingStrategy),
    Direction(Direction),
    EdgeRouting(EdgeRouting),
    GroupOrderingStrategy(GroupOrderingStrategy),
    GreedySwitchType(GreedySwitchType),
    HierarchyHandling(HierarchyHandling),
    LongEdgeOrderingStrategy(LongEdgeOrderingStrategy),
    ModelOrderStrategy(ModelOrderStrategy),
    PortAlignment(PortAlignment),
    PortConstraints(PortConstraints),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CoreOption {
    Algorithm,
    CommentBox,
    ConsiderModelOrderComponents,
    ConsiderModelOrderStrategy,
    ConnectedComponentsCompaction,
    ConsiderPortOrder,
    CrossingCounterNodeInfluence,
    CrossingCounterPortInfluence,
    CrossingMinimizationGroupOrderStrategy,
    CycleBreakingGroupOrderStrategy,
    CycleBreakingPreferredSourceId,
    CycleBreakingPreferredTargetId,
    DebugMode,
    Direction,
    EdgeRouting,
    FavorStraightEdges,
    FeedbackEdges,
    FixedGraphSize,
    ForceNodeModelOrder,
    GeneratePositionAndLayerIds,
    GreedySwitchActivationThreshold,
    GreedySwitchHierarchicalType,
    GreedySwitchType,
    HighDegreeNodeTreatment,
    HierarchyHandling,
    Hypernode,
    InsideSelfLoops,
    InteractiveLayout,
    LayerUnzippingMinimizeEdgeLength,
    LayerUnzippingResetOnLongEdges,
    LayoutPartitioning,
    LongEdgeOrderingStrategy,
    MergeEdges,
    MergeHierarchyEdges,
    NoLayout,
    NoModelOrder,
    PortAlignmentDefault,
    PortAlignmentEast,
    PortAlignmentNorth,
    PortAlignmentSouth,
    PortAlignmentWest,
    PortConstraints,
    PortLabelsNextToPortIfPossible,
    PortLabelsTreatAsGroup,
    SeparateConnectedComponents,
    SemiInteractiveCrossingMinimization,
    SpacingNodeNode,
    SpacingLayerNodeNode,
    SpacingBaseValue,
    SpacingCommentComment,
    SpacingCommentNode,
    SpacingComponentComponent,
    SpacingEdgeNode,
    SpacingEdgeEdge,
    SpacingEdgeLabel,
    SpacingEdgeNodeBetweenLayers,
    SpacingEdgeEdgeBetweenLayers,
    SpacingLabelLabel,
    SpacingLabelNode,
    SpacingLabelPortHorizontal,
    SpacingLabelPortVertical,
    SpacingNodeSelfLoop,
    SpacingPortPort,
    TopdownLayout,
    UnnecessaryBendpoints,
    WrappingAdditionalEdgeSpacing,
    WrappingImproveCuts,
    WrappingImproveWrappedEdges,
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

    pub fn set_consider_model_order_components(
        &mut self,
        strategy: ComponentOrderingStrategy,
    ) -> Option<PropertyValue> {
        self.values.insert(
            CoreOption::ConsiderModelOrderComponents,
            PropertyValue::ComponentOrderingStrategy(strategy),
        )
    }

    pub fn set_consider_model_order_strategy(
        &mut self,
        strategy: ModelOrderStrategy,
    ) -> Option<PropertyValue> {
        self.values.insert(
            CoreOption::ConsiderModelOrderStrategy,
            PropertyValue::ModelOrderStrategy(strategy),
        )
    }

    pub fn set_consider_port_order(&mut self, enabled: bool) -> Option<PropertyValue> {
        self.set_bool_option(CoreOption::ConsiderPortOrder, enabled)
    }

    pub fn set_crossing_counter_node_influence(&mut self, influence: f64) -> Option<PropertyValue> {
        self.values.insert(
            CoreOption::CrossingCounterNodeInfluence,
            PropertyValue::Number(influence),
        )
    }

    pub fn set_crossing_counter_port_influence(&mut self, influence: f64) -> Option<PropertyValue> {
        self.values.insert(
            CoreOption::CrossingCounterPortInfluence,
            PropertyValue::Number(influence),
        )
    }

    pub fn set_cycle_breaking_group_order_strategy(
        &mut self,
        strategy: GroupOrderingStrategy,
    ) -> Option<PropertyValue> {
        self.values.insert(
            CoreOption::CycleBreakingGroupOrderStrategy,
            PropertyValue::GroupOrderingStrategy(strategy),
        )
    }

    pub fn set_cycle_breaking_preferred_source_id(&mut self, id: i64) -> Option<PropertyValue> {
        self.values.insert(
            CoreOption::CycleBreakingPreferredSourceId,
            PropertyValue::Integer(id),
        )
    }

    pub fn set_cycle_breaking_preferred_target_id(&mut self, id: i64) -> Option<PropertyValue> {
        self.values.insert(
            CoreOption::CycleBreakingPreferredTargetId,
            PropertyValue::Integer(id),
        )
    }

    pub fn set_crossing_minimization_group_order_strategy(
        &mut self,
        strategy: GroupOrderingStrategy,
    ) -> Option<PropertyValue> {
        self.values.insert(
            CoreOption::CrossingMinimizationGroupOrderStrategy,
            PropertyValue::GroupOrderingStrategy(strategy),
        )
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

    pub fn set_greedy_switch_activation_threshold(
        &mut self,
        threshold: i64,
    ) -> Option<PropertyValue> {
        self.values.insert(
            CoreOption::GreedySwitchActivationThreshold,
            PropertyValue::Integer(threshold),
        )
    }

    pub fn set_greedy_switch_type(
        &mut self,
        greedy_switch_type: GreedySwitchType,
    ) -> Option<PropertyValue> {
        self.values.insert(
            CoreOption::GreedySwitchType,
            PropertyValue::GreedySwitchType(greedy_switch_type),
        )
    }

    pub fn set_greedy_switch_hierarchical_type(
        &mut self,
        greedy_switch_type: GreedySwitchType,
    ) -> Option<PropertyValue> {
        self.values.insert(
            CoreOption::GreedySwitchHierarchicalType,
            PropertyValue::GreedySwitchType(greedy_switch_type),
        )
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

    pub fn set_layer_unzipping_reset_on_long_edges(
        &mut self,
        enabled: bool,
    ) -> Option<PropertyValue> {
        self.set_bool_option(CoreOption::LayerUnzippingResetOnLongEdges, enabled)
    }

    pub fn set_layout_partitioning(&mut self, enabled: bool) -> Option<PropertyValue> {
        self.set_bool_option(CoreOption::LayoutPartitioning, enabled)
    }

    pub fn set_long_edge_ordering_strategy(
        &mut self,
        strategy: LongEdgeOrderingStrategy,
    ) -> Option<PropertyValue> {
        self.values.insert(
            CoreOption::LongEdgeOrderingStrategy,
            PropertyValue::LongEdgeOrderingStrategy(strategy),
        )
    }

    pub fn set_merge_edges(&mut self, enabled: bool) -> Option<PropertyValue> {
        self.set_bool_option(CoreOption::MergeEdges, enabled)
    }

    pub fn set_merge_hierarchy_edges(&mut self, enabled: bool) -> Option<PropertyValue> {
        self.set_bool_option(CoreOption::MergeHierarchyEdges, enabled)
    }

    pub fn set_no_layout(&mut self, enabled: bool) -> Option<PropertyValue> {
        self.set_bool_option(CoreOption::NoLayout, enabled)
    }

    pub fn set_no_model_order(&mut self, enabled: bool) -> Option<PropertyValue> {
        self.set_bool_option(CoreOption::NoModelOrder, enabled)
    }

    pub fn set_port_constraints(
        &mut self,
        port_constraints: PortConstraints,
    ) -> Option<PropertyValue> {
        self.values.insert(
            CoreOption::PortConstraints,
            PropertyValue::PortConstraints(port_constraints),
        )
    }

    pub fn set_port_alignment_default(
        &mut self,
        port_alignment: PortAlignment,
    ) -> Option<PropertyValue> {
        self.set_port_alignment_option(CoreOption::PortAlignmentDefault, port_alignment)
    }

    pub fn set_port_alignment_east(
        &mut self,
        port_alignment: PortAlignment,
    ) -> Option<PropertyValue> {
        self.set_port_alignment_option(CoreOption::PortAlignmentEast, port_alignment)
    }

    pub fn set_port_alignment_north(
        &mut self,
        port_alignment: PortAlignment,
    ) -> Option<PropertyValue> {
        self.set_port_alignment_option(CoreOption::PortAlignmentNorth, port_alignment)
    }

    pub fn set_port_alignment_south(
        &mut self,
        port_alignment: PortAlignment,
    ) -> Option<PropertyValue> {
        self.set_port_alignment_option(CoreOption::PortAlignmentSouth, port_alignment)
    }

    pub fn set_port_alignment_west(
        &mut self,
        port_alignment: PortAlignment,
    ) -> Option<PropertyValue> {
        self.set_port_alignment_option(CoreOption::PortAlignmentWest, port_alignment)
    }

    pub fn set_port_labels_next_to_port_if_possible(
        &mut self,
        enabled: bool,
    ) -> Option<PropertyValue> {
        self.set_bool_option(CoreOption::PortLabelsNextToPortIfPossible, enabled)
    }

    pub fn set_port_labels_treat_as_group(&mut self, enabled: bool) -> Option<PropertyValue> {
        self.set_bool_option(CoreOption::PortLabelsTreatAsGroup, enabled)
    }

    pub fn set_separate_connected_components(&mut self, enabled: bool) -> Option<PropertyValue> {
        self.set_bool_option(CoreOption::SeparateConnectedComponents, enabled)
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

    pub fn set_spacing_base_value(&mut self, spacing: f64) -> Option<PropertyValue> {
        self.values
            .insert(CoreOption::SpacingBaseValue, PropertyValue::Number(spacing))
    }

    pub fn set_spacing_comment_comment(&mut self, spacing: f64) -> Option<PropertyValue> {
        self.values.insert(
            CoreOption::SpacingCommentComment,
            PropertyValue::Number(spacing),
        )
    }

    pub fn set_spacing_comment_node(&mut self, spacing: f64) -> Option<PropertyValue> {
        self.values.insert(
            CoreOption::SpacingCommentNode,
            PropertyValue::Number(spacing),
        )
    }

    pub fn set_spacing_component_component(&mut self, spacing: f64) -> Option<PropertyValue> {
        self.values.insert(
            CoreOption::SpacingComponentComponent,
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

    pub fn set_spacing_edge_label(&mut self, spacing: f64) -> Option<PropertyValue> {
        self.values
            .insert(CoreOption::SpacingEdgeLabel, PropertyValue::Number(spacing))
    }

    pub fn set_spacing_edge_node_between_layers(&mut self, spacing: f64) -> Option<PropertyValue> {
        self.values.insert(
            CoreOption::SpacingEdgeNodeBetweenLayers,
            PropertyValue::Number(spacing),
        )
    }

    pub fn set_spacing_edge_edge_between_layers(&mut self, spacing: f64) -> Option<PropertyValue> {
        self.values.insert(
            CoreOption::SpacingEdgeEdgeBetweenLayers,
            PropertyValue::Number(spacing),
        )
    }

    pub fn set_spacing_label_label(&mut self, spacing: f64) -> Option<PropertyValue> {
        self.values.insert(
            CoreOption::SpacingLabelLabel,
            PropertyValue::Number(spacing),
        )
    }

    pub fn set_spacing_label_node(&mut self, spacing: f64) -> Option<PropertyValue> {
        self.values
            .insert(CoreOption::SpacingLabelNode, PropertyValue::Number(spacing))
    }

    pub fn set_spacing_label_port_horizontal(&mut self, spacing: f64) -> Option<PropertyValue> {
        self.values.insert(
            CoreOption::SpacingLabelPortHorizontal,
            PropertyValue::Number(spacing),
        )
    }

    pub fn set_spacing_label_port_vertical(&mut self, spacing: f64) -> Option<PropertyValue> {
        self.values.insert(
            CoreOption::SpacingLabelPortVertical,
            PropertyValue::Number(spacing),
        )
    }

    pub fn set_spacing_node_self_loop(&mut self, spacing: f64) -> Option<PropertyValue> {
        self.values.insert(
            CoreOption::SpacingNodeSelfLoop,
            PropertyValue::Number(spacing),
        )
    }

    pub fn set_spacing_port_port(&mut self, spacing: f64) -> Option<PropertyValue> {
        self.values
            .insert(CoreOption::SpacingPortPort, PropertyValue::Number(spacing))
    }

    pub fn set_topdown_layout(&mut self, enabled: bool) -> Option<PropertyValue> {
        self.set_bool_option(CoreOption::TopdownLayout, enabled)
    }

    pub fn set_unnecessary_bendpoints(&mut self, enabled: bool) -> Option<PropertyValue> {
        self.set_bool_option(CoreOption::UnnecessaryBendpoints, enabled)
    }

    pub fn set_wrapping_additional_edge_spacing(&mut self, spacing: f64) -> Option<PropertyValue> {
        self.values.insert(
            CoreOption::WrappingAdditionalEdgeSpacing,
            PropertyValue::Number(spacing),
        )
    }

    pub fn set_wrapping_improve_cuts(&mut self, enabled: bool) -> Option<PropertyValue> {
        self.set_bool_option(CoreOption::WrappingImproveCuts, enabled)
    }

    pub fn set_wrapping_improve_wrapped_edges(&mut self, enabled: bool) -> Option<PropertyValue> {
        self.set_bool_option(CoreOption::WrappingImproveWrappedEdges, enabled)
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

    pub fn consider_model_order_components(&self) -> Option<ComponentOrderingStrategy> {
        match self.get(CoreOption::ConsiderModelOrderComponents) {
            Some(PropertyValue::ComponentOrderingStrategy(strategy)) => Some(*strategy),
            Some(value) => {
                unreachable!("consider model order components stored incompatible value: {value:?}")
            }
            _ => None,
        }
    }

    pub fn consider_model_order_strategy(&self) -> Option<ModelOrderStrategy> {
        match self.get(CoreOption::ConsiderModelOrderStrategy) {
            Some(PropertyValue::ModelOrderStrategy(strategy)) => Some(*strategy),
            Some(value) => {
                unreachable!("consider model order strategy stored incompatible value: {value:?}")
            }
            _ => None,
        }
    }

    pub fn consider_port_order(&self) -> bool {
        self.bool_option(CoreOption::ConsiderPortOrder, "consider port order")
    }

    pub fn crossing_counter_node_influence(&self) -> Option<f64> {
        self.number_option(
            CoreOption::CrossingCounterNodeInfluence,
            "crossing counter node influence",
        )
    }

    pub fn crossing_counter_port_influence(&self) -> Option<f64> {
        self.number_option(
            CoreOption::CrossingCounterPortInfluence,
            "crossing counter port influence",
        )
    }

    pub fn cycle_breaking_group_order_strategy(&self) -> Option<GroupOrderingStrategy> {
        self.group_ordering_strategy_option(
            CoreOption::CycleBreakingGroupOrderStrategy,
            "cycle breaking group order strategy",
        )
    }

    pub fn cycle_breaking_preferred_source_id(&self) -> Option<i64> {
        self.integer_option(
            CoreOption::CycleBreakingPreferredSourceId,
            "cycle breaking preferred source ID",
        )
    }

    pub fn cycle_breaking_preferred_target_id(&self) -> Option<i64> {
        self.integer_option(
            CoreOption::CycleBreakingPreferredTargetId,
            "cycle breaking preferred target ID",
        )
    }

    pub fn crossing_minimization_group_order_strategy(&self) -> Option<GroupOrderingStrategy> {
        self.group_ordering_strategy_option(
            CoreOption::CrossingMinimizationGroupOrderStrategy,
            "crossing minimization group order strategy",
        )
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

    pub fn greedy_switch_activation_threshold(&self) -> Option<i64> {
        match self.get(CoreOption::GreedySwitchActivationThreshold) {
            Some(PropertyValue::Integer(threshold)) => Some(*threshold),
            Some(value) => unreachable!(
                "greedy switch activation threshold stored incompatible value: {value:?}"
            ),
            _ => None,
        }
    }

    pub fn greedy_switch_type(&self) -> Option<GreedySwitchType> {
        match self.get(CoreOption::GreedySwitchType) {
            Some(PropertyValue::GreedySwitchType(greedy_switch_type)) => Some(*greedy_switch_type),
            Some(value) => unreachable!("greedy switch type stored incompatible value: {value:?}"),
            _ => None,
        }
    }

    pub fn greedy_switch_hierarchical_type(&self) -> Option<GreedySwitchType> {
        match self.get(CoreOption::GreedySwitchHierarchicalType) {
            Some(PropertyValue::GreedySwitchType(greedy_switch_type)) => Some(*greedy_switch_type),
            Some(value) => {
                unreachable!("hierarchical greedy switch type stored incompatible value: {value:?}")
            }
            _ => None,
        }
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

    pub fn layer_unzipping_reset_on_long_edges(&self) -> bool {
        self.bool_option(
            CoreOption::LayerUnzippingResetOnLongEdges,
            "layer unzipping reset on long edges",
        )
    }

    pub fn layout_partitioning(&self) -> bool {
        self.bool_option(CoreOption::LayoutPartitioning, "layout partitioning")
    }

    pub fn long_edge_ordering_strategy(&self) -> Option<LongEdgeOrderingStrategy> {
        match self.get(CoreOption::LongEdgeOrderingStrategy) {
            Some(PropertyValue::LongEdgeOrderingStrategy(strategy)) => Some(*strategy),
            Some(value) => {
                unreachable!("long edge ordering strategy stored incompatible value: {value:?}")
            }
            _ => None,
        }
    }

    pub fn merge_edges(&self) -> bool {
        self.bool_option(CoreOption::MergeEdges, "merge edges")
    }

    pub fn merge_hierarchy_edges(&self) -> bool {
        self.bool_option(
            CoreOption::MergeHierarchyEdges,
            "merge hierarchy-crossing edges",
        )
    }

    pub fn no_layout(&self) -> bool {
        self.bool_option(CoreOption::NoLayout, "no layout")
    }

    pub fn no_model_order(&self) -> bool {
        self.bool_option(CoreOption::NoModelOrder, "no model order")
    }

    pub fn port_constraints(&self) -> PortConstraints {
        match self.get(CoreOption::PortConstraints) {
            Some(PropertyValue::PortConstraints(port_constraints)) => *port_constraints,
            Some(value) => unreachable!("port constraints stored incompatible value: {value:?}"),
            _ => PortConstraints::Undefined,
        }
    }

    pub fn port_alignment_default(&self) -> Option<PortAlignment> {
        self.port_alignment_option(CoreOption::PortAlignmentDefault, "port alignment default")
    }

    pub fn port_alignment_east(&self) -> Option<PortAlignment> {
        self.port_alignment_option(CoreOption::PortAlignmentEast, "port alignment east")
    }

    pub fn port_alignment_north(&self) -> Option<PortAlignment> {
        self.port_alignment_option(CoreOption::PortAlignmentNorth, "port alignment north")
    }

    pub fn port_alignment_south(&self) -> Option<PortAlignment> {
        self.port_alignment_option(CoreOption::PortAlignmentSouth, "port alignment south")
    }

    pub fn port_alignment_west(&self) -> Option<PortAlignment> {
        self.port_alignment_option(CoreOption::PortAlignmentWest, "port alignment west")
    }

    pub fn port_labels_next_to_port_if_possible(&self) -> bool {
        self.bool_option(
            CoreOption::PortLabelsNextToPortIfPossible,
            "port labels next to port if possible",
        )
    }

    pub fn port_labels_treat_as_group(&self) -> bool {
        self.bool_option(
            CoreOption::PortLabelsTreatAsGroup,
            "port labels treat as group",
        )
    }

    pub fn separate_connected_components(&self) -> bool {
        self.bool_option(
            CoreOption::SeparateConnectedComponents,
            "separate connected components",
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

    pub fn spacing_component_component(&self) -> f64 {
        match self.get(CoreOption::SpacingComponentComponent) {
            Some(PropertyValue::Number(spacing)) => *spacing,
            Some(value) => {
                unreachable!("component-component spacing stored incompatible value: {value:?}")
            }
            _ => DEFAULT_COMPONENT_COMPONENT_SPACING,
        }
    }

    pub fn spacing_node_self_loop(&self) -> f64 {
        match self.get(CoreOption::SpacingNodeSelfLoop) {
            Some(PropertyValue::Number(spacing)) => *spacing,
            Some(value) => {
                unreachable!("node self-loop spacing stored incompatible value: {value:?}")
            }
            _ => DEFAULT_NODE_SELF_LOOP_SPACING,
        }
    }

    pub fn spacing_port_port(&self) -> f64 {
        match self.get(CoreOption::SpacingPortPort) {
            Some(PropertyValue::Number(spacing)) => *spacing,
            Some(value) => unreachable!("port-port spacing stored incompatible value: {value:?}"),
            _ => DEFAULT_PORT_PORT_SPACING,
        }
    }

    pub fn topdown_layout(&self) -> bool {
        self.bool_option(CoreOption::TopdownLayout, "topdown layout")
    }

    pub fn unnecessary_bendpoints(&self) -> bool {
        self.bool_option(CoreOption::UnnecessaryBendpoints, "unnecessary bendpoints")
    }

    pub fn wrapping_improve_cuts(&self) -> bool {
        self.bool_option(CoreOption::WrappingImproveCuts, "improve cuts")
    }

    pub fn wrapping_improve_wrapped_edges(&self) -> bool {
        self.bool_option(
            CoreOption::WrappingImproveWrappedEdges,
            "improve wrapped edges",
        )
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

    fn integer_option(&self, option: CoreOption, name: &str) -> Option<i64> {
        match self.get(option) {
            Some(PropertyValue::Integer(value)) => Some(*value),
            Some(value) => unreachable!("{name} stored incompatible value: {value:?}"),
            _ => None,
        }
    }

    fn number_option(&self, option: CoreOption, name: &str) -> Option<f64> {
        match self.get(option) {
            Some(PropertyValue::Number(value)) => Some(*value),
            Some(value) => unreachable!("{name} stored incompatible value: {value:?}"),
            _ => None,
        }
    }

    fn group_ordering_strategy_option(
        &self,
        option: CoreOption,
        name: &str,
    ) -> Option<GroupOrderingStrategy> {
        match self.get(option) {
            Some(PropertyValue::GroupOrderingStrategy(strategy)) => Some(*strategy),
            Some(value) => unreachable!("{name} stored incompatible value: {value:?}"),
            _ => None,
        }
    }

    fn set_port_alignment_option(
        &mut self,
        option: CoreOption,
        port_alignment: PortAlignment,
    ) -> Option<PropertyValue> {
        self.values
            .insert(option, PropertyValue::PortAlignment(port_alignment))
    }

    fn port_alignment_option(&self, option: CoreOption, name: &str) -> Option<PortAlignment> {
        match self.get(option) {
            Some(PropertyValue::PortAlignment(port_alignment)) => Some(*port_alignment),
            Some(value) => unreachable!("{name} stored incompatible value: {value:?}"),
            _ => None,
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
        assert!(!properties.merge_hierarchy_edges());
        assert!(!properties.separate_connected_components());
        assert!(!properties.semi_interactive_crossing_minimization());
        assert!(!properties.topdown_layout());
        assert!(!properties.unnecessary_bendpoints());
        assert!(!properties.wrapping_improve_cuts());
        assert!(!properties.wrapping_improve_wrapped_edges());

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
        properties.set_merge_hierarchy_edges(true);
        properties.set_separate_connected_components(true);
        properties.set_semi_interactive_crossing_minimization(true);
        properties.set_topdown_layout(true);
        properties.set_unnecessary_bendpoints(true);
        properties.set_wrapping_improve_cuts(true);
        properties.set_wrapping_improve_wrapped_edges(true);

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
        assert!(properties.merge_hierarchy_edges());
        assert!(properties.separate_connected_components());
        assert!(properties.semi_interactive_crossing_minimization());
        assert!(properties.topdown_layout());
        assert!(properties.unnecessary_bendpoints());
        assert!(properties.wrapping_improve_cuts());
        assert!(properties.wrapping_improve_wrapped_edges());
    }

    #[test]
    fn node_boolean_options_default_to_false_and_can_be_set() {
        let mut properties = Properties::default();

        assert!(!properties.comment_box());
        assert!(!properties.hypernode());
        assert!(!properties.inside_self_loops());
        assert!(!properties.no_model_order());
        assert!(!properties.no_layout());
        assert!(!properties.layer_unzipping_minimize_edge_length());
        assert!(!properties.layer_unzipping_reset_on_long_edges());
        assert!(!properties.port_labels_next_to_port_if_possible());
        assert!(!properties.port_labels_treat_as_group());

        properties.set_comment_box(true);
        properties.set_hypernode(true);
        properties.set_inside_self_loops(true);
        properties.set_no_model_order(true);
        properties.set_no_layout(true);
        properties.set_layer_unzipping_minimize_edge_length(true);
        properties.set_layer_unzipping_reset_on_long_edges(true);
        properties.set_port_labels_next_to_port_if_possible(true);
        properties.set_port_labels_treat_as_group(true);

        assert!(properties.comment_box());
        assert!(properties.hypernode());
        assert!(properties.inside_self_loops());
        assert!(properties.no_model_order());
        assert!(properties.no_layout());
        assert!(properties.layer_unzipping_minimize_edge_length());
        assert!(properties.layer_unzipping_reset_on_long_edges());
        assert!(properties.port_labels_next_to_port_if_possible());
        assert!(properties.port_labels_treat_as_group());
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
    fn port_constraints_default_to_undefined_and_can_be_set() {
        let mut properties = Properties::default();

        assert_eq!(properties.port_constraints(), PortConstraints::Undefined);
        properties.set_port_constraints(PortConstraints::FixedOrder);

        assert_eq!(properties.port_constraints(), PortConstraints::FixedOrder);
    }

    #[test]
    fn port_alignment_options_can_be_set() {
        let mut properties = Properties::default();

        assert_eq!(properties.port_alignment_default(), None);
        assert_eq!(properties.port_alignment_east(), None);
        assert_eq!(properties.port_alignment_north(), None);
        assert_eq!(properties.port_alignment_south(), None);
        assert_eq!(properties.port_alignment_west(), None);

        properties.set_port_alignment_default(PortAlignment::Justified);
        properties.set_port_alignment_east(PortAlignment::Begin);
        properties.set_port_alignment_north(PortAlignment::Center);
        properties.set_port_alignment_south(PortAlignment::Distributed);
        properties.set_port_alignment_west(PortAlignment::End);

        assert_eq!(
            properties.port_alignment_default(),
            Some(PortAlignment::Justified)
        );
        assert_eq!(properties.port_alignment_east(), Some(PortAlignment::Begin));
        assert_eq!(
            properties.port_alignment_north(),
            Some(PortAlignment::Center)
        );
        assert_eq!(
            properties.port_alignment_south(),
            Some(PortAlignment::Distributed)
        );
        assert_eq!(properties.port_alignment_west(), Some(PortAlignment::End));
    }

    #[test]
    fn model_order_options_can_be_set() {
        let mut properties = Properties::default();

        assert_eq!(properties.consider_model_order_components(), None);
        assert_eq!(properties.consider_model_order_strategy(), None);

        properties.set_consider_model_order_components(ComponentOrderingStrategy::ModelOrder);
        properties.set_consider_model_order_strategy(ModelOrderStrategy::PreferNodes);

        assert_eq!(
            properties.consider_model_order_components(),
            Some(ComponentOrderingStrategy::ModelOrder)
        );
        assert_eq!(
            properties.consider_model_order_strategy(),
            Some(ModelOrderStrategy::PreferNodes)
        );
    }

    #[test]
    fn greedy_switch_options_can_be_set() {
        let mut properties = Properties::default();

        assert_eq!(properties.greedy_switch_activation_threshold(), None);
        assert_eq!(properties.greedy_switch_type(), None);
        assert_eq!(properties.greedy_switch_hierarchical_type(), None);

        properties.set_greedy_switch_activation_threshold(42);
        properties.set_greedy_switch_type(GreedySwitchType::OneSided);
        properties.set_greedy_switch_hierarchical_type(GreedySwitchType::TwoSided);

        assert_eq!(properties.greedy_switch_activation_threshold(), Some(42));
        assert_eq!(
            properties.greedy_switch_type(),
            Some(GreedySwitchType::OneSided)
        );
        assert_eq!(
            properties.greedy_switch_hierarchical_type(),
            Some(GreedySwitchType::TwoSided)
        );
    }

    #[test]
    fn model_order_group_options_can_be_set() {
        let mut properties = Properties::default();

        assert_eq!(properties.crossing_counter_node_influence(), None);
        assert_eq!(properties.crossing_counter_port_influence(), None);
        assert_eq!(properties.cycle_breaking_group_order_strategy(), None);
        assert_eq!(properties.cycle_breaking_preferred_source_id(), None);
        assert_eq!(properties.cycle_breaking_preferred_target_id(), None);
        assert_eq!(
            properties.crossing_minimization_group_order_strategy(),
            None
        );
        assert_eq!(properties.long_edge_ordering_strategy(), None);

        properties.set_crossing_counter_node_influence(0.25);
        properties.set_crossing_counter_port_influence(0.5);
        properties.set_cycle_breaking_group_order_strategy(GroupOrderingStrategy::ModelOrder);
        properties.set_cycle_breaking_preferred_source_id(-3);
        properties.set_cycle_breaking_preferred_target_id(9);
        properties.set_crossing_minimization_group_order_strategy(GroupOrderingStrategy::Enforced);
        properties.set_long_edge_ordering_strategy(LongEdgeOrderingStrategy::Equal);

        assert_eq!(properties.crossing_counter_node_influence(), Some(0.25));
        assert_eq!(properties.crossing_counter_port_influence(), Some(0.5));
        assert_eq!(
            properties.cycle_breaking_group_order_strategy(),
            Some(GroupOrderingStrategy::ModelOrder)
        );
        assert_eq!(properties.cycle_breaking_preferred_source_id(), Some(-3));
        assert_eq!(properties.cycle_breaking_preferred_target_id(), Some(9));
        assert_eq!(
            properties.crossing_minimization_group_order_strategy(),
            Some(GroupOrderingStrategy::Enforced)
        );
        assert_eq!(
            properties.long_edge_ordering_strategy(),
            Some(LongEdgeOrderingStrategy::Equal)
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
        assert_eq!(
            properties.spacing_component_component(),
            DEFAULT_COMPONENT_COMPONENT_SPACING
        );
        assert_eq!(
            properties.spacing_node_self_loop(),
            DEFAULT_NODE_SELF_LOOP_SPACING
        );
        assert_eq!(properties.spacing_port_port(), DEFAULT_PORT_PORT_SPACING);
    }

    #[test]
    fn spacing_values_can_be_overridden() {
        let mut properties = Properties::default();

        properties.set_spacing_node_node(42.0);
        properties.set_spacing_layer_node_node(300.0);
        properties.set_spacing_edge_node(12.0);
        properties.set_spacing_edge_edge(24.0);
        properties.set_spacing_component_component(36.0);
        properties.set_spacing_node_self_loop(48.0);
        properties.set_spacing_port_port(60.0);

        assert_eq!(properties.spacing_node_node(), 42.0);
        assert_eq!(properties.spacing_layer_node_node(), 300.0);
        assert_eq!(properties.spacing_edge_node(), 12.0);
        assert_eq!(properties.spacing_edge_edge(), 24.0);
        assert_eq!(properties.spacing_component_component(), 36.0);
        assert_eq!(properties.spacing_node_self_loop(), 48.0);
        assert_eq!(properties.spacing_port_port(), 60.0);
    }
}
