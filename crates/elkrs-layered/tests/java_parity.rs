mod support;

use std::env;
use std::io::Write;
use std::process::{Command, Stdio};

use elkrs_json::{from_str, to_string_pretty};
use elkrs_layered::{LayeredLayout, LayoutAlgorithm};

use support::fixtures::{parity_fixtures, ParityFixtureStatus};
use support::quality::layout_metrics;

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
        LayeredLayout.layout(&mut rust_graph).unwrap_or_else(|error| {
            panic!(
                "Rust layout for fixture {} ({}) should succeed: {error}",
                fixture.id, fixture.name
            )
        });

        assert_eq!(
            java_graph.nodes.len(),
            rust_graph.nodes.len(),
            "fixture {} ({}) should preserve node count",
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
        assert_eq!(
            java_metrics.node_overlaps, rust_metrics.node_overlaps,
            "fixture {} ({}) node overlap parity mismatch: java={java_metrics:?}, rust={rust_metrics:?}",
            fixture.id,
            fixture.name
        );
        assert_eq!(
            java_metrics.edges_through_nodes, rust_metrics.edges_through_nodes,
            "fixture {} ({}) route-through-node parity mismatch: java={java_metrics:?}, rust={rust_metrics:?}",
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
