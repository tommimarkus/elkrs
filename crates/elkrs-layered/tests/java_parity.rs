mod support;

use std::env;
use std::io::Write;
use std::process::{Command, Stdio};

use elkrs_core::graph::{ElkGraph, ElkNode};
use elkrs_json::{from_str, to_string_pretty};
use elkrs_layered::{LayeredLayout, LayoutAlgorithm};

use support::fixtures::{parity_fixtures, ParityAssertion, ParityFixtureStatus};
use support::quality::{layout_metrics, major_axis_edge_node_clearance, LayoutMetrics};

#[test]
#[ignore = "requires ELKRS_JAVA_ELK_COMMAND to point at a Java ELK JSON command"]
fn java_elk_parity_matches_structural_metrics_for_comparable_fixtures() {
    let command = env::var("ELKRS_JAVA_ELK_COMMAND")
        .expect("set ELKRS_JAVA_ELK_COMMAND to a Java ELK JSON command");

    let comparable = parity_fixtures()
        .into_iter()
        .filter(|fixture| fixture.status == ParityFixtureStatus::JavaComparable)
        .collect::<Vec<_>>();

    assert!(
        !comparable.is_empty(),
        "expected at least one Java-comparable parity fixture"
    );

    for fixture in comparable {
        let fixture_graph = (fixture.build)();
        let input = to_string_pretty(&fixture_graph).unwrap();
        let java_output = run_java_elk_command(&command, &input);
        let java_graph = from_str(&java_output).unwrap_or_else(|error| {
            panic!(
                "Java ELK output for fixture {} ({}) should be importable: {error}",
                fixture.id, fixture.name
            )
        });

        let mut rust_graph = fixture_graph;
        LayeredLayout
            .layout(&mut rust_graph)
            .unwrap_or_else(|error| {
                panic!(
                    "Rust layout for fixture {} ({}) should succeed: {error}",
                    fixture.id, fixture.name
                )
            });

        assert_eq!(
            node_count(&java_graph),
            node_count(&rust_graph),
            "fixture {} ({}) should preserve recursive node count",
            fixture.id,
            fixture.name
        );
        assert_eq!(
            java_graph.edges.len(),
            rust_graph.edges.len(),
            "fixture {} ({}) should preserve edge count",
            fixture.id,
            fixture.name
        );

        let java_metrics = layout_metrics(&java_graph);
        let rust_metrics = layout_metrics(&rust_graph);
        for assertion in fixture.assertions {
            assert_parity_fixture_assertion(
                assertion,
                &java_graph,
                &rust_graph,
                fixture.id,
                fixture.name,
            );
        }
        assert_structural_metric_parity(&java_metrics, &rust_metrics, fixture.id, fixture.name);
        assert_eq!(
            java_metrics.unrouted_edges, 0,
            "fixture {} ({}) Java output should route every edge: {java_metrics:?}",
            fixture.id, fixture.name
        );
        assert_eq!(
            rust_metrics.unrouted_edges, 0,
            "fixture {} ({}) Rust output should route every edge: {rust_metrics:?}",
            fixture.id, fixture.name
        );
        assert!(
            java_metrics.route_segments >= java_graph.edges.len(),
            "fixture {} ({}) Java output should have at least one routed segment per edge: {java_metrics:?}",
            fixture.id,
            fixture.name
        );
        assert!(
            rust_metrics.route_segments >= rust_graph.edges.len(),
            "fixture {} ({}) should have at least one routed segment per edge: {rust_metrics:?}",
            fixture.id,
            fixture.name
        );
    }
}

fn assert_structural_metric_parity(
    java_metrics: &LayoutMetrics,
    rust_metrics: &LayoutMetrics,
    fixture_id: &str,
    fixture_name: &str,
) {
    assert_eq!(
        java_metrics.node_overlaps, rust_metrics.node_overlaps,
        "fixture {fixture_id} ({fixture_name}) node overlap parity mismatch: java={java_metrics:?}, rust={rust_metrics:?}"
    );
    assert_eq!(
        java_metrics.containment_violations, rust_metrics.containment_violations,
        "fixture {fixture_id} ({fixture_name}) containment parity mismatch: java={java_metrics:?}, rust={rust_metrics:?}"
    );
    assert_eq!(
        java_metrics.edges_through_nodes, rust_metrics.edges_through_nodes,
        "fixture {fixture_id} ({fixture_name}) route-through-node parity mismatch: java={java_metrics:?}, rust={rust_metrics:?}"
    );
    assert_eq!(
        java_metrics.crossings, rust_metrics.crossings,
        "fixture {fixture_id} ({fixture_name}) crossing parity mismatch: java={java_metrics:?}, rust={rust_metrics:?}"
    );
    assert_eq!(
        java_metrics.port_anchor_mismatches, rust_metrics.port_anchor_mismatches,
        "fixture {fixture_id} ({fixture_name}) port anchor parity mismatch: java={java_metrics:?}, rust={rust_metrics:?}"
    );
}

#[test]
fn recursive_node_count_includes_nested_children() {
    let graph = support::fixtures::nested_group();

    assert_eq!(node_count(&graph), 3);
}

#[test]
fn structural_metric_parity_checks_containment_violations() {
    let java_metrics = metrics_with_containment_violations(1);
    let rust_metrics = metrics_with_containment_violations(0);

    assert!(
        std::panic::catch_unwind(|| {
            assert_structural_metric_parity(&java_metrics, &rust_metrics, "fixture-id", "fixture")
        })
        .is_err(),
        "containment violation mismatches should fail Java parity"
    );
}

#[test]
fn structural_metric_parity_checks_port_anchor_mismatches() {
    let java_metrics = metrics_with_port_anchor_mismatches(1);
    let rust_metrics = metrics_with_port_anchor_mismatches(0);

    assert!(
        std::panic::catch_unwind(|| {
            assert_structural_metric_parity(&java_metrics, &rust_metrics, "fixture-id", "fixture")
        })
        .is_err(),
        "port anchor mismatch differences should fail Java parity"
    );
}

fn run_java_elk_command(command: &str, input: &str) -> String {
    let mut child = Command::new(command)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap_or_else(|error| panic!("failed to start {command}: {error}"));

    child
        .stdin
        .as_mut()
        .expect("stdin should be piped")
        .write_all(input.as_bytes())
        .expect("failed to write fixture JSON to Java ELK command");

    let output = child
        .wait_with_output()
        .expect("failed to wait for Java ELK command");

    assert!(
        output.status.success(),
        "Java ELK command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8(output.stdout).expect("Java ELK command stdout should be UTF-8 JSON")
}

fn node_count(graph: &ElkGraph) -> usize {
    graph.nodes.values().map(node_count_in_subtree).sum()
}

fn node_count_in_subtree(node: &ElkNode) -> usize {
    1 + node
        .children
        .values()
        .map(node_count_in_subtree)
        .sum::<usize>()
}

fn metrics_with_containment_violations(containment_violations: usize) -> LayoutMetrics {
    LayoutMetrics {
        node_overlaps: 0,
        containment_violations,
        route_segments: 1,
        unrouted_edges: 0,
        edges_through_nodes: 0,
        crossings: 0,
        port_anchor_mismatches: 0,
    }
}

fn metrics_with_port_anchor_mismatches(port_anchor_mismatches: usize) -> LayoutMetrics {
    LayoutMetrics {
        node_overlaps: 0,
        containment_violations: 0,
        route_segments: 1,
        unrouted_edges: 0,
        edges_through_nodes: 0,
        crossings: 0,
        port_anchor_mismatches,
    }
}

fn assert_parity_fixture_assertion(
    assertion: &ParityAssertion,
    java_graph: &elkrs_core::graph::ElkGraph,
    rust_graph: &elkrs_core::graph::ElkGraph,
    fixture_id: &str,
    fixture_name: &str,
) {
    match *assertion {
        ParityAssertion::EdgeNodeClearance {
            edge_id,
            node_id,
            minimum,
        } => {
            let java_clearance = major_axis_edge_node_clearance(java_graph, edge_id, node_id)
                .unwrap_or_else(|| {
                    panic!(
                        "fixture {fixture_id} ({fixture_name}) Java output should route edge {edge_id} across node {node_id}'s layer"
                    )
                });
            let rust_clearance = major_axis_edge_node_clearance(rust_graph, edge_id, node_id)
                .unwrap_or_else(|| {
                    panic!(
                        "fixture {fixture_id} ({fixture_name}) Rust output should route edge {edge_id} across node {node_id}'s layer"
                    )
                });

            assert!(
                java_clearance + f64::EPSILON >= minimum,
                "fixture {fixture_id} ({fixture_name}) Java edge-node clearance for edge {edge_id} vs node {node_id} should be at least {minimum}: {java_clearance}"
            );
            assert!(
                rust_clearance + f64::EPSILON >= minimum,
                "fixture {fixture_id} ({fixture_name}) Rust edge-node clearance for edge {edge_id} vs node {node_id} should be at least {minimum}: {rust_clearance}"
            );
        }
    }
}
