use elkrs_json::{from_str, JsonError};

#[test]
fn malformed_json_returns_json_error() {
    let error = from_str("{").unwrap_err();

    assert!(matches!(error, JsonError::Json(_)));
}

#[test]
fn unknown_edge_endpoint_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "children": [{ "id": "source" }],
          "edges": [{ "id": "edge", "sources": ["source"], "targets": ["missing"] }]
        }"#,
        "unknown endpoint id: missing",
    );
}

#[test]
fn ambiguous_port_endpoint_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "children": [
            { "id": "left", "ports": [{ "id": "p" }] },
            { "id": "right", "ports": [{ "id": "p" }] },
            { "id": "target" }
          ],
          "edges": [{ "id": "edge", "sources": ["p"], "targets": ["target"] }]
        }"#,
        "ambiguous port endpoint id: p",
    );
}

#[test]
fn edge_with_multiple_sources_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "children": [{ "id": "a" }, { "id": "b" }, { "id": "target" }],
          "edges": [{ "id": "edge", "sources": ["a", "b"], "targets": ["target"] }]
        }"#,
        "edge sources must contain exactly one endpoint",
    );
}

#[test]
fn edge_with_multiple_targets_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "children": [{ "id": "source" }, { "id": "a" }, { "id": "b" }],
          "edges": [{ "id": "edge", "sources": ["source"], "targets": ["a", "b"] }]
        }"#,
        "edge targets must contain exactly one endpoint",
    );
}

#[test]
fn unsupported_direction_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "layoutOptions": { "org.eclipse.elk.direction": "SIDEWAYS" }
        }"#,
        "unsupported org.eclipse.elk.direction value: SIDEWAYS",
    );
}

#[test]
fn non_string_direction_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "layoutOptions": { "org.eclipse.elk.direction": 7 }
        }"#,
        "org.eclipse.elk.direction must be a string",
    );
}

#[test]
fn non_string_algorithm_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "layoutOptions": { "org.eclipse.elk.algorithm": 7 }
        }"#,
        "org.eclipse.elk.algorithm must be a string",
    );
}

#[test]
fn unsupported_edge_routing_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "layoutOptions": { "org.eclipse.elk.edgeRouting": "SPLINES" }
        }"#,
        "unsupported org.eclipse.elk.edgeRouting value: SPLINES",
    );
}

#[test]
fn non_string_edge_routing_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "layoutOptions": { "org.eclipse.elk.edgeRouting": 7 }
        }"#,
        "org.eclipse.elk.edgeRouting must be a string",
    );
}

#[test]
fn non_number_node_spacing_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "layoutOptions": { "org.eclipse.elk.spacing.nodeNode": "wide" }
        }"#,
        "org.eclipse.elk.spacing.nodeNode must be a number",
    );
}

#[test]
fn non_number_layer_spacing_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "layoutOptions": { "elk.spacing.layerNodeNode": false }
        }"#,
        "elk.spacing.layerNodeNode must be a number",
    );
}

#[test]
fn non_finite_string_spacing_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "layoutOptions": { "org.eclipse.elk.layered.spacing.nodeNodeBetweenLayers": "NaN" }
        }"#,
        "org.eclipse.elk.layered.spacing.nodeNodeBetweenLayers must be a number",
    );
}

#[test]
fn non_number_edge_spacing_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "layoutOptions": { "org.eclipse.elk.spacing.edgeNode": "wide" }
        }"#,
        "org.eclipse.elk.spacing.edgeNode must be a number",
    );
}

#[test]
fn negative_edge_spacing_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "layoutOptions": { "org.eclipse.elk.spacing.edgeEdge": -1 }
        }"#,
        "org.eclipse.elk.spacing.edgeEdge must be non-negative",
    );
}

#[test]
fn unsupported_port_side_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "children": [
            { "id": "source", "ports": [{ "id": "out", "side": "DIAGONAL" }] }
          ]
        }"#,
        "unsupported side value: DIAGONAL",
    );
}

#[test]
fn unsupported_port_side_layout_option_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "children": [
            {
              "id": "source",
              "ports": [
                {
                  "id": "out",
                  "layoutOptions": { "org.eclipse.elk.port.side": "DIAGONAL" }
                }
              ]
            }
          ]
        }"#,
        "unsupported org.eclipse.elk.port.side value: DIAGONAL",
    );
}

#[test]
fn non_string_port_side_layout_option_returns_invalid_error() {
    assert_invalid_contains(
        r#"{
          "id": "root",
          "children": [
            {
              "id": "source",
              "ports": [
                {
                  "id": "out",
                  "layoutOptions": { "org.eclipse.elk.port.side": 7 }
                }
              ]
            }
          ]
        }"#,
        "org.eclipse.elk.port.side must be a string",
    );
}

fn assert_invalid_contains(input: &str, expected: &str) {
    let error = from_str(input).unwrap_err();

    assert!(
        matches!(error, JsonError::Invalid(ref message) if message.contains(expected)),
        "expected JsonError::Invalid containing {expected:?}, got {error:?}",
    );
}
