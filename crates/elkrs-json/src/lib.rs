//! ELK-style JSON import and export.

use std::collections::BTreeMap;

use elkrs_core::geometry::{Point, Size};
use elkrs_core::graph::{
    ElementId, ElementRef, ElkEdge, ElkEdgeSection, ElkGraph, ElkLabel, ElkNode, ElkPort,
};
use elkrs_core::options::{
    Algorithm, CoreOption, Direction, EdgeRouting, HierarchyHandling, PortConstraints, PortSide,
    PropertyValue,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

const ALGORITHM_KEY: &str = "org.eclipse.elk.algorithm";
const LEGACY_ALGORITHM_KEY: &str = "elk.algorithm";
const COMMENT_BOX_KEY: &str = "org.eclipse.elk.commentBox";
const CONNECTED_COMPONENTS_COMPACTION_KEY: &str =
    "org.eclipse.elk.layered.compaction.connectedComponents";
const CONSIDER_PORT_ORDER_KEY: &str = "org.eclipse.elk.layered.considerModelOrder.portModelOrder";
const DEBUG_MODE_KEY: &str = "org.eclipse.elk.debugMode";
const DIRECTION_KEY: &str = "org.eclipse.elk.direction";
const LEGACY_DIRECTION_KEY: &str = "elk.direction";
const EDGE_ROUTING_KEY: &str = "org.eclipse.elk.edgeRouting";
const FAVOR_STRAIGHT_EDGES_KEY: &str = "org.eclipse.elk.layered.nodePlacement.favorStraightEdges";
const FEEDBACK_EDGES_KEY: &str = "org.eclipse.elk.layered.feedbackEdges";
const FIXED_GRAPH_SIZE_KEY: &str = "org.eclipse.elk.nodeSize.fixedGraphSize";
const FORCE_NODE_MODEL_ORDER_KEY: &str =
    "org.eclipse.elk.layered.crossingMinimization.forceNodeModelOrder";
const GENERATE_POSITION_AND_LAYER_IDS_KEY: &str =
    "org.eclipse.elk.layered.generatePositionAndLayerIds";
const HIGH_DEGREE_NODE_TREATMENT_KEY: &str = "org.eclipse.elk.layered.highDegreeNodes.treatment";
const HIERARCHY_HANDLING_KEY: &str = "org.eclipse.elk.hierarchyHandling";
const HYPERNODE_KEY: &str = "org.eclipse.elk.hypernode";
const INSIDE_SELF_LOOPS_KEY: &str = "org.eclipse.elk.insideSelfLoops.activate";
const INTERACTIVE_LAYOUT_KEY: &str = "org.eclipse.elk.interactiveLayout";
const LAYER_UNZIPPING_MINIMIZE_EDGE_LENGTH_KEY: &str =
    "org.eclipse.elk.layered.layerUnzipping.minimizeEdgeLength";
const LAYER_UNZIPPING_RESET_ON_LONG_EDGES_KEY: &str =
    "org.eclipse.elk.layered.layerUnzipping.resetOnLongEdges";
const LAYOUT_PARTITIONING_KEY: &str = "org.eclipse.elk.partitioning.activate";
const MERGE_EDGES_KEY: &str = "org.eclipse.elk.layered.mergeEdges";
const MERGE_HIERARCHY_EDGES_KEY: &str = "org.eclipse.elk.layered.mergeHierarchyEdges";
const NO_LAYOUT_KEY: &str = "org.eclipse.elk.noLayout";
const NO_MODEL_ORDER_KEY: &str = "org.eclipse.elk.layered.considerModelOrder.noModelOrder";
const NODE_NODE_SPACING_KEY: &str = "org.eclipse.elk.spacing.nodeNode";
const LEGACY_NODE_NODE_SPACING_KEY: &str = "elk.spacing.nodeNode";
const SPACING_BASE_VALUE_KEY: &str = "org.eclipse.elk.layered.spacing.baseValue";
const COMMENT_COMMENT_SPACING_KEY: &str = "org.eclipse.elk.spacing.commentComment";
const COMMENT_NODE_SPACING_KEY: &str = "org.eclipse.elk.spacing.commentNode";
const COMPONENT_COMPONENT_SPACING_KEY: &str = "org.eclipse.elk.spacing.componentComponent";
const LAYER_NODE_NODE_SPACING_KEY: &str = "org.eclipse.elk.layered.spacing.nodeNodeBetweenLayers";
const LEGACY_LAYER_NODE_NODE_SPACING_KEY: &str = "elk.spacing.layerNodeNode";
const EDGE_NODE_SPACING_KEY: &str = "org.eclipse.elk.spacing.edgeNode";
const LEGACY_EDGE_NODE_SPACING_KEY: &str = "elk.spacing.edgeNode";
const EDGE_EDGE_SPACING_KEY: &str = "org.eclipse.elk.spacing.edgeEdge";
const LEGACY_EDGE_EDGE_SPACING_KEY: &str = "elk.spacing.edgeEdge";
const EDGE_LABEL_SPACING_KEY: &str = "org.eclipse.elk.spacing.edgeLabel";
const LAYER_EDGE_NODE_SPACING_KEY: &str = "org.eclipse.elk.layered.spacing.edgeNodeBetweenLayers";
const LAYER_EDGE_EDGE_SPACING_KEY: &str = "org.eclipse.elk.layered.spacing.edgeEdgeBetweenLayers";
const LABEL_LABEL_SPACING_KEY: &str = "org.eclipse.elk.spacing.labelLabel";
const LABEL_NODE_SPACING_KEY: &str = "org.eclipse.elk.spacing.labelNode";
const LABEL_PORT_HORIZONTAL_SPACING_KEY: &str = "org.eclipse.elk.spacing.labelPortHorizontal";
const LABEL_PORT_VERTICAL_SPACING_KEY: &str = "org.eclipse.elk.spacing.labelPortVertical";
const NODE_SELF_LOOP_SPACING_KEY: &str = "org.eclipse.elk.spacing.nodeSelfLoop";
const PORT_PORT_SPACING_KEY: &str = "org.eclipse.elk.spacing.portPort";
const PORT_CONSTRAINTS_KEY: &str = "org.eclipse.elk.portConstraints";
const PORT_SIDE_KEY: &str = "org.eclipse.elk.port.side";
const LEGACY_PORT_SIDE_KEY: &str = "side";
const PORT_LABELS_NEXT_TO_PORT_IF_POSSIBLE_KEY: &str =
    "org.eclipse.elk.portLabels.nextToPortIfPossible";
const PORT_LABELS_TREAT_AS_GROUP_KEY: &str = "org.eclipse.elk.portLabels.treatAsGroup";
const SEPARATE_CONNECTED_COMPONENTS_KEY: &str = "org.eclipse.elk.separateConnectedComponents";
const SEMI_INTERACTIVE_CROSSING_MINIMIZATION_KEY: &str =
    "org.eclipse.elk.layered.crossingMinimization.semiInteractive";
const TOPDOWN_LAYOUT_KEY: &str = "org.eclipse.elk.topdownLayout";
const UNNECESSARY_BENDPOINTS_KEY: &str = "org.eclipse.elk.layered.unnecessaryBendpoints";
const WRAPPING_ADDITIONAL_EDGE_SPACING_KEY: &str =
    "org.eclipse.elk.layered.wrapping.additionalEdgeSpacing";
const WRAPPING_IMPROVE_CUTS_KEY: &str = "org.eclipse.elk.layered.wrapping.multiEdge.improveCuts";
const WRAPPING_IMPROVE_WRAPPED_EDGES_KEY: &str =
    "org.eclipse.elk.layered.wrapping.multiEdge.improveWrappedEdges";

#[derive(Debug, Error)]
pub enum JsonError {
    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error("invalid ELK JSON: {0}")]
    Invalid(String),
}

pub fn from_str(input: &str) -> Result<ElkGraph, JsonError> {
    JsonGraph::try_into(serde_json::from_str::<JsonGraph>(input)?)
}

pub fn to_string_pretty(graph: &ElkGraph) -> Result<String, JsonError> {
    Ok(serde_json::to_string_pretty(&JsonGraph::from_graph(
        graph,
    )?)?)
}

#[derive(Debug, Deserialize, Serialize)]
struct JsonGraph {
    id: String,
    #[serde(
        default,
        rename = "layoutOptions",
        skip_serializing_if = "BTreeMap::is_empty"
    )]
    layout_options: BTreeMap<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    children: Vec<JsonNode>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    edges: Vec<JsonEdge>,
}

#[derive(Debug, Deserialize, Serialize)]
struct JsonNode {
    id: String,
    #[serde(
        default,
        rename = "layoutOptions",
        skip_serializing_if = "BTreeMap::is_empty"
    )]
    layout_options: BTreeMap<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "is_default_f64")]
    x: f64,
    #[serde(default, skip_serializing_if = "is_default_f64")]
    y: f64,
    #[serde(default, skip_serializing_if = "is_default_f64")]
    width: f64,
    #[serde(default, skip_serializing_if = "is_default_f64")]
    height: f64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    labels: Vec<JsonLabel>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    ports: Vec<JsonPort>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    children: Vec<JsonNode>,
}

#[derive(Debug, Deserialize, Serialize)]
struct JsonPort {
    id: String,
    #[serde(
        default,
        rename = "layoutOptions",
        skip_serializing_if = "BTreeMap::is_empty"
    )]
    layout_options: BTreeMap<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "is_default_f64")]
    x: f64,
    #[serde(default, skip_serializing_if = "is_default_f64")]
    y: f64,
    #[serde(default, skip_serializing_if = "is_default_f64")]
    width: f64,
    #[serde(default, skip_serializing_if = "is_default_f64")]
    height: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    side: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct JsonEdge {
    id: String,
    sources: Vec<String>,
    targets: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    labels: Vec<JsonLabel>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    sections: Vec<JsonSection>,
}

#[derive(Debug, Deserialize, Serialize)]
struct JsonLabel {
    text: String,
    #[serde(default, skip_serializing_if = "is_default_f64")]
    x: f64,
    #[serde(default, skip_serializing_if = "is_default_f64")]
    y: f64,
    #[serde(default, skip_serializing_if = "is_default_f64")]
    width: f64,
    #[serde(default, skip_serializing_if = "is_default_f64")]
    height: f64,
}

#[derive(Debug, Deserialize, Serialize)]
struct JsonSection {
    #[serde(rename = "startPoint")]
    start_point: JsonPoint,
    #[serde(default, rename = "bendPoints", skip_serializing_if = "Vec::is_empty")]
    bend_points: Vec<JsonPoint>,
    #[serde(rename = "endPoint")]
    end_point: JsonPoint,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
struct JsonPoint {
    x: f64,
    y: f64,
}

impl TryFrom<JsonGraph> for ElkGraph {
    type Error = JsonError;

    fn try_from(json: JsonGraph) -> Result<Self, Self::Error> {
        let mut graph = ElkGraph::new(json.id.as_str());
        apply_layout_options(&mut graph, &json.layout_options)?;
        for child in json.children {
            graph.add_node(child.try_into()?);
        }
        for edge in json.edges {
            graph.add_edge(edge.into_edge(&graph)?);
        }
        Ok(graph)
    }
}

impl TryFrom<JsonNode> for ElkNode {
    type Error = JsonError;

    fn try_from(json: JsonNode) -> Result<Self, Self::Error> {
        let mut node = ElkNode::new(json.id.as_str());
        apply_node_layout_options(&mut node, &json.layout_options)?;
        node.position = Point::new(json.x, json.y);
        node.size = Size::new(json.width, json.height);
        node.labels = json.labels.into_iter().map(JsonLabel::into_label).collect();
        for port in json.ports {
            node.add_port(port.try_into()?);
        }
        for child in json.children {
            node.add_child(child.try_into()?);
        }
        Ok(node)
    }
}

impl TryFrom<JsonPort> for ElkPort {
    type Error = JsonError;

    fn try_from(json: JsonPort) -> Result<Self, Self::Error> {
        let mut port = ElkPort::new(json.id.as_str());
        port.position = Point::new(json.x, json.y);
        port.size = Size::new(json.width, json.height);
        port.side = port_side_from_json(&json)?;
        Ok(port)
    }
}

impl JsonGraph {
    fn from_graph(graph: &ElkGraph) -> Result<Self, JsonError> {
        Ok(Self {
            id: graph.id.as_str().to_string(),
            layout_options: layout_options_from_graph(graph),
            children: graph.nodes.values().map(JsonNode::from_node).collect(),
            edges: graph.edges.values().map(JsonEdge::from_edge).collect(),
        })
    }
}

impl JsonNode {
    fn from_node(node: &ElkNode) -> Self {
        Self {
            id: node.id.as_str().to_string(),
            layout_options: layout_options_from_node(node),
            x: node.position.x,
            y: node.position.y,
            width: node.size.width,
            height: node.size.height,
            labels: node.labels.iter().map(JsonLabel::from_label).collect(),
            ports: node.ports.values().map(JsonPort::from_port).collect(),
            children: node.children.values().map(JsonNode::from_node).collect(),
        }
    }
}

impl JsonPort {
    fn from_port(port: &ElkPort) -> Self {
        Self {
            id: port.id.as_str().to_string(),
            layout_options: layout_options_from_port(port),
            x: port.position.x,
            y: port.position.y,
            width: port.size.width,
            height: port.size.height,
            side: None,
        }
    }
}

impl JsonEdge {
    fn into_edge(self, graph: &ElkGraph) -> Result<ElkEdge, JsonError> {
        let source = single_endpoint(&self.sources, "sources")?;
        let target = single_endpoint(&self.targets, "targets")?;
        let mut edge = ElkEdge::new(
            self.id.as_str(),
            resolve_element_ref(graph, source)?,
            resolve_element_ref(graph, target)?,
        );
        edge.labels = self.labels.into_iter().map(JsonLabel::into_label).collect();
        edge.sections = self
            .sections
            .into_iter()
            .map(JsonSection::into_section)
            .collect();
        Ok(edge)
    }

    fn from_edge(edge: &ElkEdge) -> Self {
        Self {
            id: edge.id.as_str().to_string(),
            sources: vec![element_ref_id(&edge.source).to_string()],
            targets: vec![element_ref_id(&edge.target).to_string()],
            labels: edge.labels.iter().map(JsonLabel::from_label).collect(),
            sections: edge
                .sections
                .iter()
                .map(JsonSection::from_section)
                .collect(),
        }
    }
}

impl JsonLabel {
    fn into_label(self) -> ElkLabel {
        ElkLabel {
            text: self.text,
            position: Point::new(self.x, self.y),
            size: Size::new(self.width, self.height),
        }
    }

    fn from_label(label: &ElkLabel) -> Self {
        Self {
            text: label.text.clone(),
            x: label.position.x,
            y: label.position.y,
            width: label.size.width,
            height: label.size.height,
        }
    }
}

impl JsonSection {
    fn into_section(self) -> ElkEdgeSection {
        let mut points = Vec::with_capacity(self.bend_points.len() + 2);
        points.push(self.start_point.into());
        points.extend(self.bend_points.into_iter().map(Point::from));
        points.push(self.end_point.into());
        ElkEdgeSection { points }
    }

    fn from_section(section: &ElkEdgeSection) -> Self {
        let start_point = section
            .points
            .first()
            .copied()
            .unwrap_or(Point::new(0.0, 0.0));
        let end_point = section.points.last().copied().unwrap_or(start_point);
        let bend_points = section
            .points
            .iter()
            .copied()
            .skip(1)
            .take(section.points.len().saturating_sub(2))
            .map(JsonPoint::from)
            .collect();
        Self {
            start_point: JsonPoint::from(start_point),
            bend_points,
            end_point: JsonPoint::from(end_point),
        }
    }
}

impl From<JsonPoint> for Point {
    fn from(point: JsonPoint) -> Self {
        Self::new(point.x, point.y)
    }
}

impl From<Point> for JsonPoint {
    fn from(point: Point) -> Self {
        Self {
            x: point.x,
            y: point.y,
        }
    }
}

fn apply_layout_options(
    graph: &mut ElkGraph,
    options: &BTreeMap<String, serde_json::Value>,
) -> Result<(), JsonError> {
    for (key, value) in options {
        match key.as_str() {
            ALGORITHM_KEY | LEGACY_ALGORITHM_KEY => {
                graph.properties.set_algorithm(parse_algorithm(value, key)?)
            }
            CONNECTED_COMPONENTS_COMPACTION_KEY => graph
                .properties
                .set_connected_components_compaction(boolean(value, key)?),
            CONSIDER_PORT_ORDER_KEY => graph
                .properties
                .set_consider_port_order(boolean(value, key)?),
            DEBUG_MODE_KEY => graph.properties.set_debug_mode(boolean(value, key)?),
            EDGE_ROUTING_KEY => {
                if let Some(edge_routing) = parse_edge_routing(value, key)? {
                    graph.properties.set_edge_routing(edge_routing)
                } else {
                    None
                }
            }
            FAVOR_STRAIGHT_EDGES_KEY => graph
                .properties
                .set_favor_straight_edges(boolean(value, key)?),
            FEEDBACK_EDGES_KEY => graph.properties.set_feedback_edges(boolean(value, key)?),
            FIXED_GRAPH_SIZE_KEY => graph.properties.set_fixed_graph_size(boolean(value, key)?),
            FORCE_NODE_MODEL_ORDER_KEY => graph
                .properties
                .set_force_node_model_order(boolean(value, key)?),
            GENERATE_POSITION_AND_LAYER_IDS_KEY => graph
                .properties
                .set_generate_position_and_layer_ids(boolean(value, key)?),
            HIGH_DEGREE_NODE_TREATMENT_KEY => graph
                .properties
                .set_high_degree_node_treatment(boolean(value, key)?),
            HIERARCHY_HANDLING_KEY => {
                if let Some(hierarchy_handling) = parse_hierarchy_handling(value, key)? {
                    graph.properties.set_hierarchy_handling(hierarchy_handling)
                } else {
                    None
                }
            }
            INTERACTIVE_LAYOUT_KEY => graph
                .properties
                .set_interactive_layout(boolean(value, key)?),
            LAYOUT_PARTITIONING_KEY => graph
                .properties
                .set_layout_partitioning(boolean(value, key)?),
            MERGE_EDGES_KEY => graph.properties.set_merge_edges(boolean(value, key)?),
            MERGE_HIERARCHY_EDGES_KEY => graph
                .properties
                .set_merge_hierarchy_edges(boolean(value, key)?),
            DIRECTION_KEY | LEGACY_DIRECTION_KEY => {
                graph.properties.set_direction(parse_direction(value, key)?)
            }
            NODE_NODE_SPACING_KEY | LEGACY_NODE_NODE_SPACING_KEY => {
                graph.properties.set_spacing_node_node(number(value, key)?)
            }
            SPACING_BASE_VALUE_KEY => graph
                .properties
                .set_spacing_base_value(non_negative_number(value, key)?),
            COMMENT_COMMENT_SPACING_KEY => graph
                .properties
                .set_spacing_comment_comment(non_negative_number(value, key)?),
            COMMENT_NODE_SPACING_KEY => graph
                .properties
                .set_spacing_comment_node(non_negative_number(value, key)?),
            COMPONENT_COMPONENT_SPACING_KEY => graph
                .properties
                .set_spacing_component_component(non_negative_number(value, key)?),
            LAYER_NODE_NODE_SPACING_KEY | LEGACY_LAYER_NODE_NODE_SPACING_KEY => graph
                .properties
                .set_spacing_layer_node_node(number(value, key)?),
            EDGE_NODE_SPACING_KEY | LEGACY_EDGE_NODE_SPACING_KEY => graph
                .properties
                .set_spacing_edge_node(non_negative_number(value, key)?),
            EDGE_EDGE_SPACING_KEY | LEGACY_EDGE_EDGE_SPACING_KEY => graph
                .properties
                .set_spacing_edge_edge(non_negative_number(value, key)?),
            EDGE_LABEL_SPACING_KEY => graph
                .properties
                .set_spacing_edge_label(non_negative_number(value, key)?),
            LAYER_EDGE_NODE_SPACING_KEY => graph
                .properties
                .set_spacing_edge_node_between_layers(non_negative_number(value, key)?),
            LAYER_EDGE_EDGE_SPACING_KEY => graph
                .properties
                .set_spacing_edge_edge_between_layers(non_negative_number(value, key)?),
            LABEL_LABEL_SPACING_KEY => graph
                .properties
                .set_spacing_label_label(non_negative_number(value, key)?),
            LABEL_NODE_SPACING_KEY => graph
                .properties
                .set_spacing_label_node(non_negative_number(value, key)?),
            LABEL_PORT_HORIZONTAL_SPACING_KEY => graph
                .properties
                .set_spacing_label_port_horizontal(non_negative_number(value, key)?),
            LABEL_PORT_VERTICAL_SPACING_KEY => graph
                .properties
                .set_spacing_label_port_vertical(non_negative_number(value, key)?),
            NODE_SELF_LOOP_SPACING_KEY => graph
                .properties
                .set_spacing_node_self_loop(non_negative_number(value, key)?),
            PORT_PORT_SPACING_KEY => graph
                .properties
                .set_spacing_port_port(non_negative_number(value, key)?),
            SEPARATE_CONNECTED_COMPONENTS_KEY => graph
                .properties
                .set_separate_connected_components(boolean(value, key)?),
            SEMI_INTERACTIVE_CROSSING_MINIMIZATION_KEY => graph
                .properties
                .set_semi_interactive_crossing_minimization(boolean(value, key)?),
            TOPDOWN_LAYOUT_KEY => graph.properties.set_topdown_layout(boolean(value, key)?),
            UNNECESSARY_BENDPOINTS_KEY => graph
                .properties
                .set_unnecessary_bendpoints(boolean(value, key)?),
            WRAPPING_ADDITIONAL_EDGE_SPACING_KEY => graph
                .properties
                .set_wrapping_additional_edge_spacing(non_negative_number(value, key)?),
            WRAPPING_IMPROVE_CUTS_KEY => graph
                .properties
                .set_wrapping_improve_cuts(boolean(value, key)?),
            WRAPPING_IMPROVE_WRAPPED_EDGES_KEY => graph
                .properties
                .set_wrapping_improve_wrapped_edges(boolean(value, key)?),
            _ => continue,
        };
    }
    Ok(())
}

fn layout_options_from_graph(graph: &ElkGraph) -> BTreeMap<String, serde_json::Value> {
    let mut options = BTreeMap::new();
    if let Some(PropertyValue::Algorithm(algorithm)) = graph.properties.get(CoreOption::Algorithm) {
        options.insert(
            ALGORITHM_KEY.to_string(),
            serde_json::Value::String(format_algorithm(algorithm)),
        );
    }
    insert_boolean_option(
        &mut options,
        &graph.properties,
        CoreOption::ConnectedComponentsCompaction,
        CONNECTED_COMPONENTS_COMPACTION_KEY,
    );
    insert_boolean_option(
        &mut options,
        &graph.properties,
        CoreOption::ConsiderPortOrder,
        CONSIDER_PORT_ORDER_KEY,
    );
    if let Some(PropertyValue::Bool(debug_mode)) = graph.properties.get(CoreOption::DebugMode) {
        options.insert(
            DEBUG_MODE_KEY.to_string(),
            serde_json::Value::Bool(*debug_mode),
        );
    }
    if let Some(PropertyValue::Direction(direction)) = graph.properties.get(CoreOption::Direction) {
        options.insert(
            DIRECTION_KEY.to_string(),
            serde_json::Value::String(format_direction(*direction).to_string()),
        );
    }
    if let Some(PropertyValue::EdgeRouting(edge_routing)) =
        graph.properties.get(CoreOption::EdgeRouting)
    {
        options.insert(
            EDGE_ROUTING_KEY.to_string(),
            serde_json::Value::String(format_edge_routing(*edge_routing).to_string()),
        );
    }
    insert_boolean_option(
        &mut options,
        &graph.properties,
        CoreOption::FavorStraightEdges,
        FAVOR_STRAIGHT_EDGES_KEY,
    );
    if let Some(PropertyValue::Bool(feedback_edges)) =
        graph.properties.get(CoreOption::FeedbackEdges)
    {
        options.insert(
            FEEDBACK_EDGES_KEY.to_string(),
            serde_json::Value::Bool(*feedback_edges),
        );
    }
    insert_boolean_option(
        &mut options,
        &graph.properties,
        CoreOption::FixedGraphSize,
        FIXED_GRAPH_SIZE_KEY,
    );
    insert_boolean_option(
        &mut options,
        &graph.properties,
        CoreOption::ForceNodeModelOrder,
        FORCE_NODE_MODEL_ORDER_KEY,
    );
    insert_boolean_option(
        &mut options,
        &graph.properties,
        CoreOption::GeneratePositionAndLayerIds,
        GENERATE_POSITION_AND_LAYER_IDS_KEY,
    );
    insert_boolean_option(
        &mut options,
        &graph.properties,
        CoreOption::HighDegreeNodeTreatment,
        HIGH_DEGREE_NODE_TREATMENT_KEY,
    );
    if let Some(PropertyValue::HierarchyHandling(hierarchy_handling)) =
        graph.properties.get(CoreOption::HierarchyHandling)
    {
        insert_hierarchy_handling(&mut options, *hierarchy_handling);
    }
    insert_boolean_option(
        &mut options,
        &graph.properties,
        CoreOption::InteractiveLayout,
        INTERACTIVE_LAYOUT_KEY,
    );
    insert_boolean_option(
        &mut options,
        &graph.properties,
        CoreOption::LayoutPartitioning,
        LAYOUT_PARTITIONING_KEY,
    );
    insert_boolean_option(
        &mut options,
        &graph.properties,
        CoreOption::MergeEdges,
        MERGE_EDGES_KEY,
    );
    insert_boolean_option(
        &mut options,
        &graph.properties,
        CoreOption::MergeHierarchyEdges,
        MERGE_HIERARCHY_EDGES_KEY,
    );
    if let Some(PropertyValue::Number(spacing)) = graph.properties.get(CoreOption::SpacingNodeNode)
    {
        options.insert(NODE_NODE_SPACING_KEY.to_string(), (*spacing).into());
    }
    if let Some(PropertyValue::Number(spacing)) =
        graph.properties.get(CoreOption::SpacingLayerNodeNode)
    {
        options.insert(LAYER_NODE_NODE_SPACING_KEY.to_string(), (*spacing).into());
    }
    if let Some(PropertyValue::Number(spacing)) = graph.properties.get(CoreOption::SpacingBaseValue)
    {
        options.insert(SPACING_BASE_VALUE_KEY.to_string(), (*spacing).into());
    }
    if let Some(PropertyValue::Number(spacing)) =
        graph.properties.get(CoreOption::SpacingCommentComment)
    {
        options.insert(COMMENT_COMMENT_SPACING_KEY.to_string(), (*spacing).into());
    }
    if let Some(PropertyValue::Number(spacing)) =
        graph.properties.get(CoreOption::SpacingCommentNode)
    {
        options.insert(COMMENT_NODE_SPACING_KEY.to_string(), (*spacing).into());
    }
    if let Some(PropertyValue::Number(spacing)) =
        graph.properties.get(CoreOption::SpacingComponentComponent)
    {
        options.insert(
            COMPONENT_COMPONENT_SPACING_KEY.to_string(),
            (*spacing).into(),
        );
    }
    if let Some(PropertyValue::Number(spacing)) = graph.properties.get(CoreOption::SpacingEdgeNode)
    {
        options.insert(EDGE_NODE_SPACING_KEY.to_string(), (*spacing).into());
    }
    if let Some(PropertyValue::Number(spacing)) = graph.properties.get(CoreOption::SpacingEdgeEdge)
    {
        options.insert(EDGE_EDGE_SPACING_KEY.to_string(), (*spacing).into());
    }
    if let Some(PropertyValue::Number(spacing)) = graph.properties.get(CoreOption::SpacingEdgeLabel)
    {
        options.insert(EDGE_LABEL_SPACING_KEY.to_string(), (*spacing).into());
    }
    if let Some(PropertyValue::Number(spacing)) = graph
        .properties
        .get(CoreOption::SpacingEdgeNodeBetweenLayers)
    {
        options.insert(LAYER_EDGE_NODE_SPACING_KEY.to_string(), (*spacing).into());
    }
    if let Some(PropertyValue::Number(spacing)) = graph
        .properties
        .get(CoreOption::SpacingEdgeEdgeBetweenLayers)
    {
        options.insert(LAYER_EDGE_EDGE_SPACING_KEY.to_string(), (*spacing).into());
    }
    if let Some(PropertyValue::Number(spacing)) =
        graph.properties.get(CoreOption::SpacingLabelLabel)
    {
        options.insert(LABEL_LABEL_SPACING_KEY.to_string(), (*spacing).into());
    }
    if let Some(PropertyValue::Number(spacing)) = graph.properties.get(CoreOption::SpacingLabelNode)
    {
        options.insert(LABEL_NODE_SPACING_KEY.to_string(), (*spacing).into());
    }
    if let Some(PropertyValue::Number(spacing)) =
        graph.properties.get(CoreOption::SpacingLabelPortHorizontal)
    {
        options.insert(
            LABEL_PORT_HORIZONTAL_SPACING_KEY.to_string(),
            (*spacing).into(),
        );
    }
    if let Some(PropertyValue::Number(spacing)) =
        graph.properties.get(CoreOption::SpacingLabelPortVertical)
    {
        options.insert(
            LABEL_PORT_VERTICAL_SPACING_KEY.to_string(),
            (*spacing).into(),
        );
    }
    if let Some(PropertyValue::Number(spacing)) =
        graph.properties.get(CoreOption::SpacingNodeSelfLoop)
    {
        options.insert(NODE_SELF_LOOP_SPACING_KEY.to_string(), (*spacing).into());
    }
    if let Some(PropertyValue::Number(spacing)) = graph.properties.get(CoreOption::SpacingPortPort)
    {
        options.insert(PORT_PORT_SPACING_KEY.to_string(), (*spacing).into());
    }
    insert_boolean_option(
        &mut options,
        &graph.properties,
        CoreOption::SeparateConnectedComponents,
        SEPARATE_CONNECTED_COMPONENTS_KEY,
    );
    insert_boolean_option(
        &mut options,
        &graph.properties,
        CoreOption::SemiInteractiveCrossingMinimization,
        SEMI_INTERACTIVE_CROSSING_MINIMIZATION_KEY,
    );
    insert_boolean_option(
        &mut options,
        &graph.properties,
        CoreOption::TopdownLayout,
        TOPDOWN_LAYOUT_KEY,
    );
    insert_boolean_option(
        &mut options,
        &graph.properties,
        CoreOption::UnnecessaryBendpoints,
        UNNECESSARY_BENDPOINTS_KEY,
    );
    if let Some(PropertyValue::Number(spacing)) = graph
        .properties
        .get(CoreOption::WrappingAdditionalEdgeSpacing)
    {
        options.insert(
            WRAPPING_ADDITIONAL_EDGE_SPACING_KEY.to_string(),
            (*spacing).into(),
        );
    }
    insert_boolean_option(
        &mut options,
        &graph.properties,
        CoreOption::WrappingImproveCuts,
        WRAPPING_IMPROVE_CUTS_KEY,
    );
    insert_boolean_option(
        &mut options,
        &graph.properties,
        CoreOption::WrappingImproveWrappedEdges,
        WRAPPING_IMPROVE_WRAPPED_EDGES_KEY,
    );
    options
}

fn apply_node_layout_options(
    node: &mut ElkNode,
    options: &BTreeMap<String, serde_json::Value>,
) -> Result<(), JsonError> {
    for (key, value) in options {
        match key.as_str() {
            COMMENT_BOX_KEY => {
                node.properties.set_comment_box(boolean(value, key)?);
            }
            CONNECTED_COMPONENTS_COMPACTION_KEY => {
                node.properties
                    .set_connected_components_compaction(boolean(value, key)?);
            }
            CONSIDER_PORT_ORDER_KEY => {
                node.properties
                    .set_consider_port_order(boolean(value, key)?);
            }
            DEBUG_MODE_KEY => {
                node.properties.set_debug_mode(boolean(value, key)?);
            }
            EDGE_ROUTING_KEY => {
                if let Some(edge_routing) = parse_edge_routing(value, key)? {
                    node.properties.set_edge_routing(edge_routing);
                }
            }
            FAVOR_STRAIGHT_EDGES_KEY => {
                node.properties
                    .set_favor_straight_edges(boolean(value, key)?);
            }
            FEEDBACK_EDGES_KEY => {
                node.properties.set_feedback_edges(boolean(value, key)?);
            }
            FIXED_GRAPH_SIZE_KEY => {
                node.properties.set_fixed_graph_size(boolean(value, key)?);
            }
            FORCE_NODE_MODEL_ORDER_KEY => {
                node.properties
                    .set_force_node_model_order(boolean(value, key)?);
            }
            GENERATE_POSITION_AND_LAYER_IDS_KEY => {
                node.properties
                    .set_generate_position_and_layer_ids(boolean(value, key)?);
            }
            HIGH_DEGREE_NODE_TREATMENT_KEY => {
                node.properties
                    .set_high_degree_node_treatment(boolean(value, key)?);
            }
            HIERARCHY_HANDLING_KEY => {
                if let Some(hierarchy_handling) = parse_hierarchy_handling(value, key)? {
                    node.properties.set_hierarchy_handling(hierarchy_handling);
                }
            }
            HYPERNODE_KEY => {
                node.properties.set_hypernode(boolean(value, key)?);
            }
            INSIDE_SELF_LOOPS_KEY => {
                node.properties.set_inside_self_loops(boolean(value, key)?);
            }
            INTERACTIVE_LAYOUT_KEY => {
                node.properties.set_interactive_layout(boolean(value, key)?);
            }
            LAYER_UNZIPPING_MINIMIZE_EDGE_LENGTH_KEY => {
                node.properties
                    .set_layer_unzipping_minimize_edge_length(boolean(value, key)?);
            }
            LAYER_UNZIPPING_RESET_ON_LONG_EDGES_KEY => {
                node.properties
                    .set_layer_unzipping_reset_on_long_edges(boolean(value, key)?);
            }
            LAYOUT_PARTITIONING_KEY => {
                node.properties
                    .set_layout_partitioning(boolean(value, key)?);
            }
            MERGE_EDGES_KEY => {
                node.properties.set_merge_edges(boolean(value, key)?);
            }
            MERGE_HIERARCHY_EDGES_KEY => {
                node.properties
                    .set_merge_hierarchy_edges(boolean(value, key)?);
            }
            NO_LAYOUT_KEY => {
                node.properties.set_no_layout(boolean(value, key)?);
            }
            NO_MODEL_ORDER_KEY => {
                node.properties.set_no_model_order(boolean(value, key)?);
            }
            PORT_CONSTRAINTS_KEY => {
                if let Some(port_constraints) = parse_port_constraints(value, key)? {
                    node.properties.set_port_constraints(port_constraints);
                }
            }
            PORT_LABELS_NEXT_TO_PORT_IF_POSSIBLE_KEY => {
                node.properties
                    .set_port_labels_next_to_port_if_possible(boolean(value, key)?);
            }
            PORT_LABELS_TREAT_AS_GROUP_KEY => {
                node.properties
                    .set_port_labels_treat_as_group(boolean(value, key)?);
            }
            PORT_PORT_SPACING_KEY => {
                node.properties
                    .set_spacing_port_port(non_negative_number(value, key)?);
            }
            SEPARATE_CONNECTED_COMPONENTS_KEY => {
                node.properties
                    .set_separate_connected_components(boolean(value, key)?);
            }
            SEMI_INTERACTIVE_CROSSING_MINIMIZATION_KEY => {
                node.properties
                    .set_semi_interactive_crossing_minimization(boolean(value, key)?);
            }
            TOPDOWN_LAYOUT_KEY => {
                node.properties.set_topdown_layout(boolean(value, key)?);
            }
            UNNECESSARY_BENDPOINTS_KEY => {
                node.properties
                    .set_unnecessary_bendpoints(boolean(value, key)?);
            }
            WRAPPING_IMPROVE_CUTS_KEY => {
                node.properties
                    .set_wrapping_improve_cuts(boolean(value, key)?);
            }
            WRAPPING_IMPROVE_WRAPPED_EDGES_KEY => {
                node.properties
                    .set_wrapping_improve_wrapped_edges(boolean(value, key)?);
            }
            _ => continue,
        }
    }
    Ok(())
}

fn layout_options_from_node(node: &ElkNode) -> BTreeMap<String, serde_json::Value> {
    let mut options = BTreeMap::new();
    insert_boolean_option(
        &mut options,
        &node.properties,
        CoreOption::CommentBox,
        COMMENT_BOX_KEY,
    );
    insert_boolean_option(
        &mut options,
        &node.properties,
        CoreOption::ConnectedComponentsCompaction,
        CONNECTED_COMPONENTS_COMPACTION_KEY,
    );
    insert_boolean_option(
        &mut options,
        &node.properties,
        CoreOption::ConsiderPortOrder,
        CONSIDER_PORT_ORDER_KEY,
    );
    if let Some(PropertyValue::Bool(debug_mode)) = node.properties.get(CoreOption::DebugMode) {
        options.insert(
            DEBUG_MODE_KEY.to_string(),
            serde_json::Value::Bool(*debug_mode),
        );
    }
    if let Some(PropertyValue::EdgeRouting(edge_routing)) =
        node.properties.get(CoreOption::EdgeRouting)
    {
        options.insert(
            EDGE_ROUTING_KEY.to_string(),
            serde_json::Value::String(format_edge_routing(*edge_routing).to_string()),
        );
    }
    insert_boolean_option(
        &mut options,
        &node.properties,
        CoreOption::FavorStraightEdges,
        FAVOR_STRAIGHT_EDGES_KEY,
    );
    if let Some(PropertyValue::Bool(feedback_edges)) =
        node.properties.get(CoreOption::FeedbackEdges)
    {
        options.insert(
            FEEDBACK_EDGES_KEY.to_string(),
            serde_json::Value::Bool(*feedback_edges),
        );
    }
    insert_boolean_option(
        &mut options,
        &node.properties,
        CoreOption::FixedGraphSize,
        FIXED_GRAPH_SIZE_KEY,
    );
    insert_boolean_option(
        &mut options,
        &node.properties,
        CoreOption::ForceNodeModelOrder,
        FORCE_NODE_MODEL_ORDER_KEY,
    );
    insert_boolean_option(
        &mut options,
        &node.properties,
        CoreOption::GeneratePositionAndLayerIds,
        GENERATE_POSITION_AND_LAYER_IDS_KEY,
    );
    insert_boolean_option(
        &mut options,
        &node.properties,
        CoreOption::HighDegreeNodeTreatment,
        HIGH_DEGREE_NODE_TREATMENT_KEY,
    );
    if let Some(PropertyValue::HierarchyHandling(hierarchy_handling)) =
        node.properties.get(CoreOption::HierarchyHandling)
    {
        insert_hierarchy_handling(&mut options, *hierarchy_handling);
    }
    insert_boolean_option(
        &mut options,
        &node.properties,
        CoreOption::Hypernode,
        HYPERNODE_KEY,
    );
    insert_boolean_option(
        &mut options,
        &node.properties,
        CoreOption::InsideSelfLoops,
        INSIDE_SELF_LOOPS_KEY,
    );
    insert_boolean_option(
        &mut options,
        &node.properties,
        CoreOption::InteractiveLayout,
        INTERACTIVE_LAYOUT_KEY,
    );
    insert_boolean_option(
        &mut options,
        &node.properties,
        CoreOption::LayerUnzippingMinimizeEdgeLength,
        LAYER_UNZIPPING_MINIMIZE_EDGE_LENGTH_KEY,
    );
    insert_boolean_option(
        &mut options,
        &node.properties,
        CoreOption::LayerUnzippingResetOnLongEdges,
        LAYER_UNZIPPING_RESET_ON_LONG_EDGES_KEY,
    );
    insert_boolean_option(
        &mut options,
        &node.properties,
        CoreOption::LayoutPartitioning,
        LAYOUT_PARTITIONING_KEY,
    );
    insert_boolean_option(
        &mut options,
        &node.properties,
        CoreOption::MergeEdges,
        MERGE_EDGES_KEY,
    );
    insert_boolean_option(
        &mut options,
        &node.properties,
        CoreOption::MergeHierarchyEdges,
        MERGE_HIERARCHY_EDGES_KEY,
    );
    insert_boolean_option(
        &mut options,
        &node.properties,
        CoreOption::NoLayout,
        NO_LAYOUT_KEY,
    );
    insert_boolean_option(
        &mut options,
        &node.properties,
        CoreOption::NoModelOrder,
        NO_MODEL_ORDER_KEY,
    );
    if let Some(PropertyValue::PortConstraints(port_constraints)) =
        node.properties.get(CoreOption::PortConstraints)
    {
        options.insert(
            PORT_CONSTRAINTS_KEY.to_string(),
            serde_json::Value::String(format_port_constraints(*port_constraints).to_string()),
        );
    }
    insert_boolean_option(
        &mut options,
        &node.properties,
        CoreOption::PortLabelsNextToPortIfPossible,
        PORT_LABELS_NEXT_TO_PORT_IF_POSSIBLE_KEY,
    );
    insert_boolean_option(
        &mut options,
        &node.properties,
        CoreOption::PortLabelsTreatAsGroup,
        PORT_LABELS_TREAT_AS_GROUP_KEY,
    );
    if let Some(PropertyValue::Number(spacing)) = node.properties.get(CoreOption::SpacingPortPort) {
        options.insert(PORT_PORT_SPACING_KEY.to_string(), (*spacing).into());
    }
    insert_boolean_option(
        &mut options,
        &node.properties,
        CoreOption::SeparateConnectedComponents,
        SEPARATE_CONNECTED_COMPONENTS_KEY,
    );
    insert_boolean_option(
        &mut options,
        &node.properties,
        CoreOption::SemiInteractiveCrossingMinimization,
        SEMI_INTERACTIVE_CROSSING_MINIMIZATION_KEY,
    );
    insert_boolean_option(
        &mut options,
        &node.properties,
        CoreOption::TopdownLayout,
        TOPDOWN_LAYOUT_KEY,
    );
    insert_boolean_option(
        &mut options,
        &node.properties,
        CoreOption::UnnecessaryBendpoints,
        UNNECESSARY_BENDPOINTS_KEY,
    );
    insert_boolean_option(
        &mut options,
        &node.properties,
        CoreOption::WrappingImproveCuts,
        WRAPPING_IMPROVE_CUTS_KEY,
    );
    insert_boolean_option(
        &mut options,
        &node.properties,
        CoreOption::WrappingImproveWrappedEdges,
        WRAPPING_IMPROVE_WRAPPED_EDGES_KEY,
    );
    options
}

fn insert_boolean_option(
    options: &mut BTreeMap<String, serde_json::Value>,
    properties: &elkrs_core::options::Properties,
    option: CoreOption,
    key: &str,
) {
    if let Some(PropertyValue::Bool(enabled)) = properties.get(option) {
        options.insert(key.to_string(), serde_json::Value::Bool(*enabled));
    }
}

fn insert_hierarchy_handling(
    options: &mut BTreeMap<String, serde_json::Value>,
    hierarchy_handling: HierarchyHandling,
) {
    options.insert(
        HIERARCHY_HANDLING_KEY.to_string(),
        serde_json::Value::String(format_hierarchy_handling(hierarchy_handling).to_string()),
    );
}

fn port_side_from_json(port: &JsonPort) -> Result<Option<PortSide>, JsonError> {
    if let Some(value) = port.layout_options.get(PORT_SIDE_KEY) {
        return parse_port_side(string(value, PORT_SIDE_KEY)?, PORT_SIDE_KEY);
    }
    if let Some(side) = port.side.as_deref() {
        return parse_port_side(side, LEGACY_PORT_SIDE_KEY);
    }
    Ok(None)
}

fn layout_options_from_port(port: &ElkPort) -> BTreeMap<String, serde_json::Value> {
    let mut options = BTreeMap::new();
    if let Some(side) = port.side {
        options.insert(
            PORT_SIDE_KEY.to_string(),
            serde_json::Value::String(format_port_side(side)),
        );
    }
    options
}

fn resolve_element_ref(graph: &ElkGraph, id: &str) -> Result<ElementRef, JsonError> {
    let element_id = ElementId::from(id);
    if find_node(graph, &element_id).is_some() {
        return Ok(ElementRef::Node(element_id));
    }
    let owners = port_owners(graph, &ElementId::from(id));
    match owners.as_slice() {
        [owner] => Ok(ElementRef::Port {
            node: (*owner).clone(),
            port: ElementId::from(id),
        }),
        [] => Err(JsonError::Invalid(format!("unknown endpoint id: {id}"))),
        _ => Err(JsonError::Invalid(format!(
            "ambiguous port endpoint id: {id}"
        ))),
    }
}

fn single_endpoint<'a>(endpoints: &'a [String], field: &str) -> Result<&'a str, JsonError> {
    match endpoints {
        [endpoint] => Ok(endpoint),
        _ => Err(JsonError::Invalid(format!(
            "edge {field} must contain exactly one endpoint"
        ))),
    }
}

fn find_node<'a>(graph: &'a ElkGraph, id: &ElementId) -> Option<&'a ElkNode> {
    graph
        .nodes
        .values()
        .find_map(|node| find_node_in_subtree(node, id))
}

fn find_node_in_subtree<'a>(node: &'a ElkNode, id: &ElementId) -> Option<&'a ElkNode> {
    if node.id == *id {
        return Some(node);
    }
    node.children
        .values()
        .find_map(|child| find_node_in_subtree(child, id))
}

fn port_owners(graph: &ElkGraph, port_id: &ElementId) -> Vec<ElementId> {
    let mut owners = Vec::new();
    for node in graph.nodes.values() {
        collect_port_owners(node, port_id, &mut owners);
    }
    owners
}

fn collect_port_owners(node: &ElkNode, port_id: &ElementId, owners: &mut Vec<ElementId>) {
    if node.ports.contains_key(port_id) {
        owners.push(node.id.clone());
    }
    for child in node.children.values() {
        collect_port_owners(child, port_id, owners);
    }
}

fn element_ref_id(endpoint: &ElementRef) -> &str {
    match endpoint {
        ElementRef::Node(node) => node.as_str(),
        ElementRef::Port { port, .. } => port.as_str(),
    }
}

fn parse_direction(value: &serde_json::Value, key: &str) -> Result<Direction, JsonError> {
    match string(value, key)? {
        "RIGHT" => Ok(Direction::Right),
        "LEFT" => Ok(Direction::Left),
        "DOWN" => Ok(Direction::Down),
        "UP" => Ok(Direction::Up),
        other => Err(JsonError::Invalid(format!(
            "unsupported {key} value: {other}"
        ))),
    }
}

fn parse_edge_routing(
    value: &serde_json::Value,
    key: &str,
) -> Result<Option<EdgeRouting>, JsonError> {
    match string(value, key)? {
        "ORTHOGONAL" => Ok(Some(EdgeRouting::Orthogonal)),
        "POLYLINE" => Ok(Some(EdgeRouting::Polyline)),
        "SPLINES" => Ok(Some(EdgeRouting::Splines)),
        "UNDEFINED" => Ok(None),
        other => Err(JsonError::Invalid(format!(
            "unsupported {key} value: {other}"
        ))),
    }
}

fn parse_hierarchy_handling(
    value: &serde_json::Value,
    key: &str,
) -> Result<Option<HierarchyHandling>, JsonError> {
    match string(value, key)? {
        "INCLUDE_CHILDREN" => Ok(Some(HierarchyHandling::IncludeChildren)),
        "SEPARATE_CHILDREN" => Ok(Some(HierarchyHandling::SeparateChildren)),
        "INHERIT" => Ok(None),
        other => Err(JsonError::Invalid(format!(
            "unsupported {key} value: {other}"
        ))),
    }
}

fn parse_port_constraints(
    value: &serde_json::Value,
    key: &str,
) -> Result<Option<PortConstraints>, JsonError> {
    match string(value, key)? {
        "FIXED_ORDER" => Ok(Some(PortConstraints::FixedOrder)),
        "FIXED_POS" => Ok(Some(PortConstraints::FixedPos)),
        "FIXED_RATIO" => Ok(Some(PortConstraints::FixedRatio)),
        "FIXED_SIDE" => Ok(Some(PortConstraints::FixedSide)),
        "FREE" => Ok(Some(PortConstraints::Free)),
        "UNDEFINED" => Ok(None),
        other => Err(JsonError::Invalid(format!(
            "unsupported {key} value: {other}"
        ))),
    }
}

fn format_hierarchy_handling(hierarchy_handling: HierarchyHandling) -> &'static str {
    match hierarchy_handling {
        HierarchyHandling::IncludeChildren => "INCLUDE_CHILDREN",
        HierarchyHandling::SeparateChildren => "SEPARATE_CHILDREN",
    }
}

fn format_port_constraints(port_constraints: PortConstraints) -> &'static str {
    match port_constraints {
        PortConstraints::FixedOrder => "FIXED_ORDER",
        PortConstraints::FixedPos => "FIXED_POS",
        PortConstraints::FixedRatio => "FIXED_RATIO",
        PortConstraints::FixedSide => "FIXED_SIDE",
        PortConstraints::Free => "FREE",
        PortConstraints::Undefined => "UNDEFINED",
    }
}

fn format_edge_routing(edge_routing: EdgeRouting) -> &'static str {
    match edge_routing {
        EdgeRouting::Orthogonal => "ORTHOGONAL",
        EdgeRouting::Polyline => "POLYLINE",
        EdgeRouting::Splines => "SPLINES",
    }
}

fn format_direction(direction: Direction) -> &'static str {
    match direction {
        Direction::Right => "RIGHT",
        Direction::Left => "LEFT",
        Direction::Down => "DOWN",
        Direction::Up => "UP",
    }
}

fn parse_port_side(value: &str, key: &str) -> Result<Option<PortSide>, JsonError> {
    match value {
        "NORTH" => Ok(Some(PortSide::North)),
        "EAST" => Ok(Some(PortSide::East)),
        "SOUTH" => Ok(Some(PortSide::South)),
        "WEST" => Ok(Some(PortSide::West)),
        "UNDEFINED" => Ok(None),
        other => Err(JsonError::Invalid(format!(
            "unsupported {key} value: {other}"
        ))),
    }
}

fn format_port_side(side: PortSide) -> String {
    match side {
        PortSide::North => "NORTH",
        PortSide::East => "EAST",
        PortSide::South => "SOUTH",
        PortSide::West => "WEST",
    }
    .to_string()
}

fn string<'a>(value: &'a serde_json::Value, key: &str) -> Result<&'a str, JsonError> {
    value
        .as_str()
        .ok_or_else(|| JsonError::Invalid(format!("{key} must be a string")))
}

fn parse_algorithm(value: &serde_json::Value, key: &str) -> Result<Algorithm, JsonError> {
    Ok(match string(value, key)? {
        "layered" | "org.eclipse.elk.layered" => Algorithm::Layered,
        other => Algorithm::Other(other.to_owned()),
    })
}

fn format_algorithm(algorithm: &Algorithm) -> String {
    match algorithm {
        Algorithm::Layered => "layered".to_owned(),
        Algorithm::Other(value) => value.clone(),
    }
}

fn number(value: &serde_json::Value, key: &str) -> Result<f64, JsonError> {
    let number = match value {
        serde_json::Value::Number(number) => number.as_f64(),
        serde_json::Value::String(number) => number.parse::<f64>().ok(),
        _ => None,
    }
    .filter(|number| number.is_finite());

    number.ok_or_else(|| JsonError::Invalid(format!("{key} must be a number")))
}

fn non_negative_number(value: &serde_json::Value, key: &str) -> Result<f64, JsonError> {
    let number = number(value, key)?;
    if number >= 0.0 {
        Ok(number)
    } else {
        Err(JsonError::Invalid(format!("{key} must be non-negative")))
    }
}

fn boolean(value: &serde_json::Value, key: &str) -> Result<bool, JsonError> {
    value
        .as_bool()
        .ok_or_else(|| JsonError::Invalid(format!("{key} must be a boolean")))
}

fn is_default_f64(value: &f64) -> bool {
    *value == 0.0
}
