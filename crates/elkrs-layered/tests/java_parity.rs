mod support;

use std::env;
use std::io::Write;
use std::process::{Command, Stdio};

use elkrs_json::{from_str, to_string_pretty};
use elkrs_layered::{LayeredLayout, LayoutAlgorithm};

use support::fixtures::chain;
use support::quality::{edge_through_node_count, node_overlap_count, route_segment_count};

#[test]
fn java_elk_chain_parity_matches_structural_metrics_when_configured() {
    let Ok(command) = env::var("ELKRS_JAVA_ELK_COMMAND") else {
        return;
    };

    let fixture = chain();
    let input = to_string_pretty(&fixture).unwrap();
    let java_output = run_java_elk_command(&command, &input);
    let java_graph = from_str(&java_output).unwrap();

    let mut rust_graph = fixture;
    LayeredLayout.layout(&mut rust_graph).unwrap();

    assert_eq!(java_graph.nodes.len(), rust_graph.nodes.len());
    assert_eq!(java_graph.edges.len(), rust_graph.edges.len());
    assert_eq!(
        node_overlap_count(&java_graph),
        node_overlap_count(&rust_graph)
    );
    assert_eq!(
        edge_through_node_count(&java_graph),
        edge_through_node_count(&rust_graph)
    );
    assert!(route_segment_count(&java_graph) >= java_graph.edges.len());
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
