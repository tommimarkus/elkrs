mod support;

use elkrs_layered::{LayeredLayout, LayoutAlgorithm};

use support::fixtures::{parity_fixtures, ParityFixtureStatus};
use support::quality::layout_metrics;

const PARITY_MATRIX: &str = include_str!("../../../docs/parity/elk-layered-v0.11.0.md");

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

#[test]
fn java_backed_fixture_rows_are_marked_java_parity() {
    let fixtures = parity_fixtures();
    let java_comparable = fixtures
        .iter()
        .filter(|fixture| fixture.status == ParityFixtureStatus::JavaComparable)
        .collect::<Vec<_>>();

    assert!(
        !java_comparable.is_empty(),
        "expected at least one Java-comparable parity fixture"
    );

    for fixture in java_comparable {
        assert_eq!(
            row_status(PARITY_MATRIX, fixture.id),
            Some("java-parity"),
            "{} ({}) should be marked as java-parity in the parity matrix",
            fixture.id,
            fixture.name
        );
    }
}

#[test]
fn edge_edge_spacing_rows_have_java_fixture_evidence() {
    let fixtures = parity_fixtures();

    for row_id in ["LAYERED-OPT-006", "LAYERED-META-OPTION-136"] {
        assert!(
            fixtures.iter().any(|fixture| {
                fixture.id == row_id && fixture.status == ParityFixtureStatus::JavaComparable
            }),
            "{row_id} should have a Java-comparable parity fixture"
        );
        assert_eq!(
            row_status(PARITY_MATRIX, row_id),
            Some("java-parity"),
            "{row_id} should be marked as java-parity in the parity matrix"
        );
    }
}

#[test]
fn node_self_loop_spacing_row_has_java_fixture_evidence() {
    let fixtures = parity_fixtures();
    let row_id = "LAYERED-META-OPTION-145";

    assert!(
        fixtures.iter().any(|fixture| {
            fixture.id == row_id && fixture.status == ParityFixtureStatus::JavaComparable
        }),
        "{row_id} should have a Java-comparable parity fixture"
    );
    assert_eq!(
        row_status(PARITY_MATRIX, row_id),
        Some("java-parity"),
        "{row_id} should be marked as java-parity in the parity matrix"
    );
}

#[test]
fn port_spacing_row_has_java_fixture_evidence() {
    let fixtures = parity_fixtures();
    let row_id = "LAYERED-META-OPTION-146";

    assert!(
        fixtures.iter().any(|fixture| {
            fixture.id == row_id && fixture.status == ParityFixtureStatus::JavaComparable
        }),
        "{row_id} should have a Java-comparable parity fixture"
    );
    assert_eq!(
        row_status(PARITY_MATRIX, row_id),
        Some("java-parity"),
        "{row_id} should be marked as java-parity in the parity matrix"
    );
}

#[test]
fn spacing_metadata_rows_have_java_fixture_evidence() {
    let fixtures = parity_fixtures();

    for row_id in [
        "LAYERED-META-OPTION-091",
        "LAYERED-META-OPTION-138",
        "LAYERED-META-OPTION-144",
    ] {
        assert!(
            fixtures.iter().any(|fixture| {
                fixture.id == row_id && fixture.status == ParityFixtureStatus::JavaComparable
            }),
            "{row_id} should have a Java-comparable parity fixture"
        );
        assert_eq!(
            row_status(PARITY_MATRIX, row_id),
            Some("java-parity"),
            "{row_id} should be marked as java-parity in the parity matrix"
        );
    }
}

#[test]
fn graph_feature_metadata_rows_have_java_fixture_evidence() {
    let fixtures = parity_fixtures();

    for row_id in [
        "LAYERED-META-FEATURE-002",
        "LAYERED-META-FEATURE-005",
        "LAYERED-META-FEATURE-006",
        "LAYERED-META-FEATURE-007",
    ] {
        assert!(
            fixtures.iter().any(|fixture| {
                fixture.id == row_id && fixture.status == ParityFixtureStatus::JavaComparable
            }),
            "{row_id} should have a Java-comparable parity fixture"
        );
        assert_eq!(
            row_status(PARITY_MATRIX, row_id),
            Some("java-parity"),
            "{row_id} should be marked as java-parity in the parity matrix"
        );
    }
}

#[test]
fn direction_metadata_row_has_java_fixture_evidence() {
    let fixtures = parity_fixtures();
    let row_id = "LAYERED-META-OPTION-006";

    assert!(
        fixtures.iter().any(|fixture| {
            fixture.id == row_id && fixture.status == ParityFixtureStatus::JavaComparable
        }),
        "{row_id} should have a Java-comparable parity fixture"
    );
    assert_eq!(
        row_status(PARITY_MATRIX, row_id),
        Some("java-parity"),
        "{row_id} should be marked as java-parity in the parity matrix"
    );
}

#[test]
fn port_endpoint_graph_row_has_java_fixture_evidence() {
    let fixtures = parity_fixtures();
    let row_id = "LAYERED-GRAPH-007";

    assert!(
        fixtures.iter().any(|fixture| {
            fixture.id == row_id && fixture.status == ParityFixtureStatus::JavaComparable
        }),
        "{row_id} should have a Java-comparable parity fixture"
    );
    assert_eq!(
        row_status(PARITY_MATRIX, row_id),
        Some("java-parity"),
        "{row_id} should be marked as java-parity in the parity matrix"
    );
}

fn row_status<'a>(matrix: &'a str, row_id: &str) -> Option<&'a str> {
    matrix.lines().find_map(|line| {
        let mut columns = line.split('|').map(str::trim);
        columns.next()?;
        let id = columns.next()?;
        if id != row_id {
            return None;
        }
        columns.next()?;
        columns.next()?;
        Some(columns.next()?.trim_matches('`'))
    })
}
