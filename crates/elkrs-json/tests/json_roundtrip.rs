use elkrs_core::geometry::{Point, Size};
use elkrs_core::graph::{ElementId, ElementRef};
use elkrs_core::options::Direction;
use elkrs_json::{from_str, to_string_pretty};
use serde_json::Value;

#[test]
fn round_trips_self_loop_edge() {
    let input = r#"{
      "id": "root",
      "children": [
        { "id": "a" }
      ],
      "edges": [
        { "id": "aa", "sources": ["a"], "targets": ["a"] }
      ]
    }"#;

    let graph = from_str(input).unwrap();
    let edge = &graph.edges[&ElementId::from("aa")];

    assert_eq!(edge.source, ElementRef::Node(ElementId::from("a")));
    assert_eq!(edge.target, ElementRef::Node(ElementId::from("a")));

    let output = to_string_pretty(&graph).unwrap();
    let reparsed = from_str(&output).unwrap();

    assert_eq!(reparsed, graph);
}

#[test]
fn round_trips_parallel_edges() {
    let input = r#"{
      "id": "root",
      "children": [
        { "id": "a" },
        { "id": "b" }
      ],
      "edges": [
        { "id": "ab-1", "sources": ["a"], "targets": ["b"] },
        { "id": "ab-2", "sources": ["a"], "targets": ["b"] }
      ]
    }"#;

    let graph = from_str(input).unwrap();
    let first = &graph.edges[&ElementId::from("ab-1")];
    let second = &graph.edges[&ElementId::from("ab-2")];

    assert_eq!(graph.edges.len(), 2);
    assert_eq!(first.source, ElementRef::Node(ElementId::from("a")));
    assert_eq!(first.target, ElementRef::Node(ElementId::from("b")));
    assert_eq!(second.source, first.source);
    assert_eq!(second.target, first.target);

    let output = to_string_pretty(&graph).unwrap();
    let reparsed = from_str(&output).unwrap();

    assert_eq!(reparsed, graph);
}

#[test]
fn round_trips_graph_with_ports_options_and_edge_sections() {
    let input = r#"{
      "id": "root",
      "layoutOptions": {
        "elk.direction": "DOWN",
        "elk.spacing.nodeNode": 42,
        "elk.spacing.layerNodeNode": 84,
        "elk.spacing.edgeNode": 21,
        "elk.spacing.edgeEdge": 9
      },
      "children": [
        {
          "id": "source",
          "x": 1,
          "y": 2,
          "width": 100,
          "height": 40,
          "ports": [
            {
              "id": "out",
              "x": 90,
              "y": 10,
              "width": 10,
              "height": 10,
              "layoutOptions": { "org.eclipse.elk.port.side": "EAST" }
            }
          ]
        },
        {
          "id": "target",
          "width": 80,
          "height": 30,
          "ports": [
            {
              "id": "in",
              "x": 0,
              "y": 10,
              "width": 10,
              "height": 10,
              "layoutOptions": { "org.eclipse.elk.port.side": "WEST" }
            }
          ]
        }
      ],
      "edges": [
        {
          "id": "edge",
          "sources": ["out"],
          "targets": ["in"],
          "sections": [
            {
              "startPoint": { "x": 101, "y": 17 },
              "bendPoints": [{ "x": 120, "y": 17 }],
              "endPoint": { "x": 0, "y": 15 }
            }
          ]
        }
      ]
    }"#;

    let graph = from_str(input).unwrap();

    assert_eq!(graph.id.as_str(), "root");
    assert_eq!(graph.properties.direction(), Direction::Down);
    assert_eq!(graph.properties.spacing_node_node(), 42.0);
    assert_eq!(graph.properties.spacing_layer_node_node(), 84.0);
    assert_eq!(graph.properties.spacing_edge_node(), 21.0);
    assert_eq!(graph.properties.spacing_edge_edge(), 9.0);
    assert_eq!(
        graph.edges[&ElementId::from("edge")].source,
        ElementRef::Port {
            node: ElementId::from("source"),
            port: ElementId::from("out")
        }
    );
    assert_eq!(
        graph.edges[&ElementId::from("edge")].sections[0].points,
        vec![
            Point::new(101.0, 17.0),
            Point::new(120.0, 17.0),
            Point::new(0.0, 15.0)
        ]
    );

    let output = to_string_pretty(&graph).unwrap();
    let reparsed = from_str(&output).unwrap();

    assert_eq!(reparsed, graph);
}

#[test]
fn round_trips_multi_section_edge_without_bendpoints() {
    let input = r#"{
      "id": "root",
      "children": [
        { "id": "source" },
        { "id": "target" }
      ],
      "edges": [
        {
          "id": "edge",
          "sources": ["source"],
          "targets": ["target"],
          "sections": [
            {
              "startPoint": { "x": 10, "y": 20 },
              "endPoint": { "x": 30, "y": 20 }
            },
            {
              "startPoint": { "x": 30, "y": 20 },
              "bendPoints": [{ "x": 40, "y": 35 }],
              "endPoint": { "x": 50, "y": 35 }
            }
          ]
        }
      ]
    }"#;

    let graph = from_str(input).unwrap();
    let edge = &graph.edges[&ElementId::from("edge")];

    assert_eq!(edge.sections.len(), 2);
    assert_eq!(
        edge.sections[0].points,
        vec![Point::new(10.0, 20.0), Point::new(30.0, 20.0)]
    );
    assert_eq!(
        edge.sections[1].points,
        vec![
            Point::new(30.0, 20.0),
            Point::new(40.0, 35.0),
            Point::new(50.0, 35.0)
        ]
    );

    let output = to_string_pretty(&graph).unwrap();
    let serialized: Value = serde_json::from_str(&output).unwrap();

    assert_eq!(
        serialized["edges"][0]["sections"].as_array().unwrap().len(),
        2
    );
    assert!(serialized["edges"][0]["sections"][0]
        .get("bendPoints")
        .is_none());

    let reparsed = from_str(&output).unwrap();

    assert_eq!(reparsed, graph);
}

#[test]
fn round_trips_node_and_edge_label_text() {
    let input = r#"{
      "id": "root",
      "children": [
        {
          "id": "source",
          "labels": [{ "text": "Source node", "x": 3, "y": 5, "width": 70, "height": 12 }]
        },
        {
          "id": "target",
          "labels": [{ "text": "Target node", "x": 7, "y": 11, "width": 80, "height": 14 }]
        }
      ],
      "edges": [
        {
          "id": "edge",
          "sources": ["source"],
          "targets": ["target"],
          "labels": [{ "text": "Edge label", "x": 13, "y": 17, "width": 90, "height": 16 }]
        }
      ]
    }"#;

    let graph = from_str(input).unwrap();

    assert_eq!(
        graph.nodes[&ElementId::from("source")].labels[0].text,
        "Source node"
    );
    assert_eq!(
        graph.nodes[&ElementId::from("source")].labels[0].position,
        Point::new(3.0, 5.0)
    );
    assert_eq!(
        graph.nodes[&ElementId::from("source")].labels[0].size,
        Size::new(70.0, 12.0)
    );
    assert_eq!(
        graph.nodes[&ElementId::from("target")].labels[0].text,
        "Target node"
    );
    assert_eq!(
        graph.nodes[&ElementId::from("target")].labels[0].position,
        Point::new(7.0, 11.0)
    );
    assert_eq!(
        graph.nodes[&ElementId::from("target")].labels[0].size,
        Size::new(80.0, 14.0)
    );
    assert_eq!(
        graph.edges[&ElementId::from("edge")].labels[0].text,
        "Edge label"
    );
    assert_eq!(
        graph.edges[&ElementId::from("edge")].labels[0].position,
        Point::new(13.0, 17.0)
    );
    assert_eq!(
        graph.edges[&ElementId::from("edge")].labels[0].size,
        Size::new(90.0, 16.0)
    );

    let output = to_string_pretty(&graph).unwrap();
    let output_json: Value = serde_json::from_str(&output).unwrap();
    assert_eq!(
        output_json["children"][0]["labels"][0]["x"],
        Value::from(3.0)
    );
    assert_eq!(
        output_json["children"][0]["labels"][0]["width"],
        Value::from(70.0)
    );
    assert_eq!(output_json["edges"][0]["labels"][0]["x"], Value::from(13.0));
    assert_eq!(
        output_json["edges"][0]["labels"][0]["width"],
        Value::from(90.0)
    );

    let reparsed = from_str(&output).unwrap();

    assert_eq!(reparsed, graph);
}

#[test]
fn unknown_json_fields_are_ignored_and_not_reemitted() {
    let input = r#"{
      "id": "root",
      "unknownGraphField": "drop",
      "layoutOptions": {
        "elk.direction": "DOWN",
        "org.eclipse.elk.unknownGraphOption": true
      },
      "children": [
        {
          "id": "node",
          "unknownNodeField": "drop",
          "layoutOptions": {
            "org.eclipse.elk.noLayout": true,
            "org.eclipse.elk.unknownNodeOption": "drop"
          },
          "labels": [
            { "text": "Node label", "unknownLabelField": true }
          ],
          "ports": [
            {
              "id": "out",
              "unknownPortField": "drop",
              "layoutOptions": {
                "org.eclipse.elk.port.side": "EAST",
                "org.eclipse.elk.unknownPortOption": "drop"
              }
            }
          ]
        }
      ],
      "edges": [
        {
          "id": "edge",
          "sources": ["out"],
          "targets": ["node"],
          "unknownEdgeField": "drop",
          "layoutOptions": {
            "org.eclipse.elk.edge.thickness": 2.5,
            "org.eclipse.elk.unknownEdgeOption": "drop"
          },
          "labels": [
            { "text": "Edge label", "unknownLabelField": false }
          ],
          "sections": [
            {
              "startPoint": { "x": 1, "y": 2, "unknownPointField": true },
              "bendPoints": [{ "x": 3, "y": 4, "unknownPointField": true }],
              "endPoint": { "x": 5, "y": 6, "unknownPointField": true },
              "unknownSectionField": "drop"
            }
          ]
        }
      ]
    }"#;

    let graph = from_str(input).unwrap();
    let output = to_string_pretty(&graph).unwrap();
    let serialized: Value = serde_json::from_str(&output).unwrap();
    let node = &serialized["children"][0];
    let port = &node["ports"][0];
    let edge = &serialized["edges"][0];
    let node_label = &node["labels"][0];
    let edge_label = &edge["labels"][0];
    let section = &edge["sections"][0];

    assert_eq!(
        serialized["layoutOptions"]["org.eclipse.elk.direction"],
        Value::String("DOWN".to_owned())
    );
    assert_eq!(
        node["layoutOptions"]["org.eclipse.elk.noLayout"],
        Value::Bool(true)
    );
    assert_eq!(
        port["layoutOptions"]["org.eclipse.elk.port.side"],
        Value::String("EAST".to_owned())
    );
    assert_eq!(
        edge["layoutOptions"]["org.eclipse.elk.edge.thickness"],
        Value::from(2.5)
    );
    assert_eq!(node_label["text"], Value::String("Node label".to_owned()));
    assert_eq!(edge_label["text"], Value::String("Edge label".to_owned()));

    assert!(serialized.get("unknownGraphField").is_none());
    assert!(serialized["layoutOptions"]
        .get("org.eclipse.elk.unknownGraphOption")
        .is_none());
    assert!(node.get("unknownNodeField").is_none());
    assert!(node["layoutOptions"]
        .get("org.eclipse.elk.unknownNodeOption")
        .is_none());
    assert!(node_label.get("unknownLabelField").is_none());
    assert!(port.get("unknownPortField").is_none());
    assert!(port["layoutOptions"]
        .get("org.eclipse.elk.unknownPortOption")
        .is_none());
    assert!(edge.get("unknownEdgeField").is_none());
    assert!(edge["layoutOptions"]
        .get("org.eclipse.elk.unknownEdgeOption")
        .is_none());
    assert!(edge_label.get("unknownLabelField").is_none());
    assert!(section.get("unknownSectionField").is_none());
    assert!(section["startPoint"].get("unknownPointField").is_none());
    assert!(section["bendPoints"][0].get("unknownPointField").is_none());
    assert!(section["endPoint"].get("unknownPointField").is_none());
}
