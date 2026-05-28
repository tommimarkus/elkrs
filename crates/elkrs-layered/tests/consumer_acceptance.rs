mod support;

use elkrs_layered::{LayeredLayout, LayoutAlgorithm};

use support::fixtures::consumer_compound_ports;
use support::quality::{
    containment_violation_count, node_overlap_count, port_anchor_mismatch_count,
    route_segment_count,
};

#[test]
fn consumer_compound_ports_fixture_meets_current_acceptance_metrics() {
    let mut graph = consumer_compound_ports();

    LayeredLayout.layout(&mut graph).unwrap();

    assert_eq!(node_overlap_count(&graph), 0);
    assert_eq!(containment_violation_count(&graph), 0);
    assert_eq!(port_anchor_mismatch_count(&graph), 0);
    assert!(route_segment_count(&graph) >= graph.edges.len());
}
