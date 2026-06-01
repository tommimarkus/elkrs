//! ELK-style JSON import and export.

use std::collections::BTreeMap;

use elkrs_core::geometry::{Point, Size};
use elkrs_core::graph::{
    ElementId, ElementRef, ElkEdge, ElkEdgeSection, ElkGraph, ElkNode, ElkPort,
};
use elkrs_core::options::{CoreOption, Direction, PortSide, PropertyValue};
use serde::{Deserialize, Serialize};
use thiserror::Error;

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
    #[serde(default, skip_serializing_if = "is_default_f64")]
    x: f64,
    #[serde(default, skip_serializing_if = "is_default_f64")]
    y: f64,
    #[serde(default, skip_serializing_if = "is_default_f64")]
    width: f64,
    #[serde(default, skip_serializing_if = "is_default_f64")]
    height: f64,
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
    sections: Vec<JsonSection>,
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
        node.position = Point::new(json.x, json.y);
        node.size = Size::new(json.width, json.height);
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
            x: node.position.x,
            y: node.position.y,
            width: node.size.width,
            height: node.size.height,
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
            sections: edge
                .sections
                .iter()
                .map(JsonSection::from_section)
                .collect(),
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
            "elk.direction" => graph.properties.set_direction(parse_direction(value)?),
            "elk.spacing.nodeNode" => graph.properties.set_spacing_node_node(number(value, key)?),
            "elk.spacing.layerNodeNode" => graph
                .properties
                .set_spacing_layer_node_node(number(value, key)?),
            "elk.spacing.edgeNode" => graph
                .properties
                .set_spacing_edge_node(non_negative_number(value, key)?),
            "elk.spacing.edgeEdge" => graph
                .properties
                .set_spacing_edge_edge(non_negative_number(value, key)?),
            _ => continue,
        };
    }
    Ok(())
}

fn layout_options_from_graph(graph: &ElkGraph) -> BTreeMap<String, serde_json::Value> {
    let mut options = BTreeMap::new();
    if let Some(PropertyValue::Direction(direction)) = graph.properties.get(CoreOption::Direction) {
        options.insert(
            "elk.direction".to_string(),
            serde_json::Value::String(format_direction(*direction).to_string()),
        );
    }
    if let Some(PropertyValue::Number(spacing)) = graph.properties.get(CoreOption::SpacingNodeNode)
    {
        options.insert("elk.spacing.nodeNode".to_string(), (*spacing).into());
    }
    if let Some(PropertyValue::Number(spacing)) =
        graph.properties.get(CoreOption::SpacingLayerNodeNode)
    {
        options.insert("elk.spacing.layerNodeNode".to_string(), (*spacing).into());
    }
    if let Some(PropertyValue::Number(spacing)) = graph.properties.get(CoreOption::SpacingEdgeNode)
    {
        options.insert("elk.spacing.edgeNode".to_string(), (*spacing).into());
    }
    if let Some(PropertyValue::Number(spacing)) = graph.properties.get(CoreOption::SpacingEdgeEdge)
    {
        options.insert("elk.spacing.edgeEdge".to_string(), (*spacing).into());
    }
    options
}

fn port_side_from_json(port: &JsonPort) -> Result<Option<PortSide>, JsonError> {
    if let Some(value) = port.layout_options.get("org.eclipse.elk.port.side") {
        return Ok(Some(parse_port_side(string(
            value,
            "org.eclipse.elk.port.side",
        )?)?));
    }
    port.side.as_deref().map(parse_port_side).transpose()
}

fn layout_options_from_port(port: &ElkPort) -> BTreeMap<String, serde_json::Value> {
    let mut options = BTreeMap::new();
    if let Some(side) = port.side {
        options.insert(
            "org.eclipse.elk.port.side".to_string(),
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

fn parse_direction(value: &serde_json::Value) -> Result<Direction, JsonError> {
    match string(value, "elk.direction")? {
        "RIGHT" => Ok(Direction::Right),
        "LEFT" => Ok(Direction::Left),
        "DOWN" => Ok(Direction::Down),
        "UP" => Ok(Direction::Up),
        other => Err(JsonError::Invalid(format!(
            "unsupported elk.direction value: {other}"
        ))),
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

fn parse_port_side(value: &str) -> Result<PortSide, JsonError> {
    match value {
        "NORTH" => Ok(PortSide::North),
        "EAST" => Ok(PortSide::East),
        "SOUTH" => Ok(PortSide::South),
        "WEST" => Ok(PortSide::West),
        other => Err(JsonError::Invalid(format!(
            "unsupported port side value: {other}"
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

fn number(value: &serde_json::Value, key: &str) -> Result<f64, JsonError> {
    value
        .as_f64()
        .ok_or_else(|| JsonError::Invalid(format!("{key} must be a number")))
}

fn non_negative_number(value: &serde_json::Value, key: &str) -> Result<f64, JsonError> {
    let number = number(value, key)?;
    if number >= 0.0 {
        Ok(number)
    } else {
        Err(JsonError::Invalid(format!("{key} must be non-negative")))
    }
}

fn is_default_f64(value: &f64) -> bool {
    *value == 0.0
}
