use elkrs_core::geometry::Point;
use elkrs_core::graph::{ElementId, ElementRef};
use elkrs_core::options::Direction;
use elkrs_json::{from_str, to_string_pretty};

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
            { "id": "out", "x": 90, "y": 10, "width": 10, "height": 10, "side": "EAST" }
          ]
        },
        {
          "id": "target",
          "width": 80,
          "height": 30,
          "ports": [
            { "id": "in", "x": 0, "y": 10, "width": 10, "height": 10, "side": "WEST" }
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
