mod support;

use elkrs_layered::{LayeredLayout, LayoutAlgorithm};

use support::fixtures::{parity_fixtures, ParityFixtureStatus};
use support::quality::layout_metrics;

#[test]
fn all_declared_parity_fixtures_produce_structurally_valid_layouts() {
    let fixtures = parity_fixtures();

    assert!(
        !fixtures.is_empty(),
        "expected at least one declared parity fixture"
    );
    assert!(
        fixtures
            .iter()
            .any(|fixture| fixture.status == ParityFixtureStatus::RustOnly),
        "expected at least one Rust-only parity fixture"
    );

    for fixture in fixtures {
        let mut graph = (fixture.build)();

        LayeredLayout.layout(&mut graph).unwrap_or_else(|error| {
            panic!(
                "fixture {} ({}) should lay out successfully: {error}",
                fixture.id, fixture.name
            )
        });

        let metrics = layout_metrics(&graph);
        assert_eq!(
            metrics.unrouted_edges, 0,
            "fixture {} ({}) should route every edge: {metrics:?}",
            fixture.id, fixture.name
        );
        assert!(
            metrics.route_segments >= graph.edges.len(),
            "fixture {} ({}) should have at least one route segment per edge: {metrics:?}",
            fixture.id,
            fixture.name
        );
        assert_eq!(
            metrics.containment_violations, 0,
            "fixture {} ({}) should not violate compound containment: {metrics:?}",
            fixture.id, fixture.name
        );
        assert_eq!(
            metrics.node_overlaps, 0,
            "fixture {} ({}) should not overlap nodes: {metrics:?}",
            fixture.id, fixture.name
        );
        assert_eq!(
            metrics.edges_through_nodes, 0,
            "fixture {} ({}) should not route through unrelated nodes: {metrics:?}",
            fixture.id, fixture.name
        );
        assert_eq!(
            metrics.port_anchor_mismatches, 0,
            "fixture {} ({}) should preserve port anchors: {metrics:?}",
            fixture.id, fixture.name
        );
    }
}
