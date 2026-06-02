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
fn component_spacing_row_has_java_fixture_evidence() {
    let fixtures = parity_fixtures();

    for row_id in ["LAYERED-P4-003", "LAYERED-META-OPTION-135"] {
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

#[test]
fn hierarchy_crossing_graph_row_has_java_fixture_evidence() {
    let fixtures = parity_fixtures();
    let row_id = "LAYERED-GRAPH-009";

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
fn node_label_and_size_rows_have_java_fixture_evidence() {
    let fixtures = parity_fixtures();

    for row_id in [
        "LAYERED-GRAPH-006",
        "LAYERED-OPT-009",
        "LAYERED-META-OPTION-109",
        "LAYERED-META-OPTION-111",
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
fn layer_assignment_rows_have_java_fixture_evidence() {
    let fixtures = parity_fixtures();

    for row_id in [
        "LAYERED-P2-002",
        "LAYERED-META-OPTION-057",
        "LAYERED-META-OPTION-069",
        "LAYERED-META-OPTION-074",
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
fn layer_assignment_exclusion_rows_document_compatibility_boundaries() {
    for row_id in [
        "LAYERED-P2-003",
        "LAYERED-META-OPTION-048",
        "LAYERED-META-OPTION-058",
        "LAYERED-META-OPTION-059",
        "LAYERED-META-OPTION-060",
        "LAYERED-META-OPTION-062",
        "LAYERED-META-OPTION-063",
        "LAYERED-META-OPTION-064",
        "LAYERED-META-OPTION-065",
        "LAYERED-META-OPTION-066",
        "LAYERED-META-OPTION-067",
        "LAYERED-META-OPTION-068",
        "LAYERED-META-OPTION-070",
        "LAYERED-META-OPTION-071",
        "LAYERED-META-OPTION-072",
        "LAYERED-META-OPTION-073",
        "LAYERED-META-OPTION-114",
        "LAYERED-META-OPTION-115",
        "LAYERED-META-OPTION-130",
    ] {
        assert!(
            row_next_plan(PARITY_MATRIX, row_id)
                .is_some_and(|next_plan| next_plan.contains("1.0.0 compatibility exclusion")),
            "{row_id} should document the 1.0.0 compatibility exclusion"
        );
    }
}

#[test]
fn node_label_placement_row_documents_compatibility_boundary() {
    assert_eq!(
        row_status(PARITY_MATRIX, "LAYERED-META-OPTION-108"),
        Some("parsed"),
        "node-label placement should be parsed for Java-compatible sizing fixtures without claiming full placement semantics"
    );
    assert!(
        row_next_plan(PARITY_MATRIX, "LAYERED-META-OPTION-108")
            .is_some_and(|next_plan| next_plan.contains("1.0.0 compatibility exclusion")),
        "node-label placement row should document the 1.0.0 compatibility exclusion"
    );
}

#[test]
fn alignment_aspect_ratio_metadata_rows_are_diagnostic() {
    for row_id in ["LAYERED-META-OPTION-001", "LAYERED-META-OPTION-002"] {
        assert_eq!(
            row_status(PARITY_MATRIX, row_id),
            Some("diagnostic"),
            "{row_id} should be marked as diagnostic in the parity matrix"
        );
    }
}

#[test]
fn high_degree_node_numeric_metadata_rows_are_diagnostic() {
    for row_id in ["LAYERED-META-OPTION-058", "LAYERED-META-OPTION-060"] {
        assert_eq!(
            row_status(PARITY_MATRIX, row_id),
            Some("diagnostic"),
            "{row_id} should be marked as diagnostic in the parity matrix"
        );
    }
}

#[test]
fn layer_unzipping_layer_split_metadata_row_is_diagnostic() {
    assert_eq!(
        row_status(PARITY_MATRIX, "LAYERED-META-OPTION-062"),
        Some("diagnostic"),
        "LAYERED-META-OPTION-062 should be marked as diagnostic in the parity matrix"
    );
}

#[test]
fn layer_unzipping_strategy_metadata_row_is_diagnostic() {
    assert_eq!(
        row_status(PARITY_MATRIX, "LAYERED-META-OPTION-065"),
        Some("diagnostic"),
        "LAYERED-META-OPTION-065 should be marked as diagnostic in the parity matrix"
    );
}

#[test]
fn interactive_reference_point_metadata_row_is_diagnostic() {
    assert_eq!(
        row_status(PARITY_MATRIX, "LAYERED-META-OPTION-061"),
        Some("diagnostic"),
        "LAYERED-META-OPTION-061 should be marked as diagnostic in the parity matrix"
    );
}

#[test]
fn min_width_and_node_promotion_metadata_rows_are_diagnostic() {
    for row_id in [
        "LAYERED-META-OPTION-070",
        "LAYERED-META-OPTION-071",
        "LAYERED-META-OPTION-072",
        "LAYERED-META-OPTION-073",
    ] {
        assert_eq!(
            row_status(PARITY_MATRIX, row_id),
            Some("diagnostic"),
            "{row_id} should be marked as diagnostic in the parity matrix"
        );
    }
}

#[test]
fn hierarchy_topdown_rows_document_compatibility_exclusions() {
    for (row_id, status) in [
        ("LAYERED-GRAPH-010", "unsupported"),
        ("LAYERED-OPT-007", "diagnostic"),
        ("LAYERED-META-FEATURE-001", "unsupported"),
        ("LAYERED-META-OPTION-011", "diagnostic"),
        ("LAYERED-META-OPTION-012", "diagnostic"),
        ("LAYERED-META-OPTION-148", "unsupported"),
        ("LAYERED-META-OPTION-149", "unsupported"),
        ("LAYERED-META-OPTION-150", "unsupported"),
        ("LAYERED-META-OPTION-151", "unsupported"),
        ("LAYERED-META-OPTION-152", "diagnostic"),
    ] {
        assert_eq!(
            row_status(PARITY_MATRIX, row_id),
            Some(status),
            "{row_id} should keep its documented compatibility status"
        );
        let next_plan = row_next_plan(PARITY_MATRIX, row_id)
            .unwrap_or_else(|| panic!("{row_id} should have a matrix row"));
        assert!(
            next_plan.contains("1.0.0 compatibility exclusion"),
            "{row_id} should document its 1.0.0 compatibility exclusion boundary"
        );
    }
}

#[test]
fn port_constraint_rows_document_compatibility_boundaries() {
    for row_id in [
        "LAYERED-GRAPH-007",
        "LAYERED-P5-002",
        "LAYERED-META-FEATURE-006",
        "LAYERED-META-OPTION-119",
        "LAYERED-META-OPTION-146",
    ] {
        assert_eq!(
            row_status(PARITY_MATRIX, row_id),
            Some("java-parity"),
            "{row_id} should stay promoted for Java-backed explicit port anchors or spacing"
        );
    }

    for (row_id, status) in [
        ("LAYERED-OPT-008", "diagnostic"),
        ("LAYERED-META-OPTION-017", "diagnostic"),
        ("LAYERED-META-OPTION-084", "unsupported"),
        ("LAYERED-META-OPTION-116", "unsupported"),
        ("LAYERED-META-OPTION-117", "diagnostic"),
        ("LAYERED-META-OPTION-118", "diagnostic"),
        ("LAYERED-META-OPTION-120", "diagnostic"),
        ("LAYERED-META-OPTION-121", "diagnostic"),
        ("LAYERED-META-OPTION-122", "diagnostic"),
        ("LAYERED-META-OPTION-123", "diagnostic"),
        ("LAYERED-META-OPTION-124", "diagnostic"),
        ("LAYERED-META-OPTION-125", "diagnostic"),
        ("LAYERED-META-OPTION-147", "unsupported"),
    ] {
        assert_eq!(
            row_status(PARITY_MATRIX, row_id),
            Some(status),
            "{row_id} should keep its documented port compatibility status"
        );
        let next_plan = row_next_plan(PARITY_MATRIX, row_id)
            .unwrap_or_else(|| panic!("{row_id} should have a matrix row"));
        assert!(
            next_plan.contains("1.0.0 compatibility exclusion"),
            "{row_id} should document its 1.0.0 port compatibility boundary"
        );
    }
}

#[test]
fn routing_variant_rows_document_compatibility_boundaries() {
    for (row_id, status) in [
        ("LAYERED-GRAPH-004", "unsupported"),
        ("LAYERED-P5-004", "diagnostic"),
        ("LAYERED-P5-005", "diagnostic"),
        ("LAYERED-P5-006", "unsupported"),
        ("LAYERED-META-FEATURE-004", "unsupported"),
        ("LAYERED-META-OPTION-007", "diagnostic"),
        ("LAYERED-META-OPTION-010", "diagnostic"),
        ("LAYERED-META-OPTION-013", "diagnostic"),
        ("LAYERED-META-OPTION-014", "diagnostic"),
        ("LAYERED-META-OPTION-016", "unsupported"),
        ("LAYERED-META-OPTION-051", "unsupported"),
        ("LAYERED-META-OPTION-052", "unsupported"),
        ("LAYERED-META-OPTION-053", "unsupported"),
        ("LAYERED-META-OPTION-054", "unsupported"),
        ("LAYERED-META-OPTION-055", "unsupported"),
        ("LAYERED-META-OPTION-075", "diagnostic"),
        ("LAYERED-META-OPTION-076", "diagnostic"),
        ("LAYERED-META-OPTION-085", "unsupported"),
        ("LAYERED-META-OPTION-086", "unsupported"),
        ("LAYERED-META-OPTION-087", "unsupported"),
        ("LAYERED-META-OPTION-093", "diagnostic"),
        ("LAYERED-META-OPTION-139", "unsupported"),
    ] {
        assert_eq!(
            row_status(PARITY_MATRIX, row_id),
            Some(status),
            "{row_id} should keep its documented routing compatibility status"
        );
        let next_plan = row_next_plan(PARITY_MATRIX, row_id)
            .unwrap_or_else(|| panic!("{row_id} should have a matrix row"));
        assert!(
            next_plan.contains("1.0.0 compatibility exclusion"),
            "{row_id} should document its 1.0.0 routing compatibility boundary"
        );
    }
}

#[test]
fn json_contract_rows_are_closed_for_supported_surface() {
    for row_id in ["LAYERED-JSON-001", "LAYERED-JSON-002"] {
        assert_eq!(
            row_status(PARITY_MATRIX, row_id),
            Some("semantic"),
            "{row_id} should be semantic for the supported Rust JSON contract"
        );
        let proof =
            row_current_proof(PARITY_MATRIX, row_id).unwrap_or_else(|| panic!("{row_id} row"));
        assert!(
            proof.contains("elkrs-json"),
            "{row_id} should cite elkrs-json evidence"
        );
        let next_plan =
            row_next_plan(PARITY_MATRIX, row_id).unwrap_or_else(|| panic!("{row_id} row"));
        assert!(
            next_plan.contains("Complete for supported 1.0.0 JSON contract"),
            "{row_id} should close the supported JSON contract"
        );
        for stale in ["deferred", "open", "#45", "plan"] {
            assert!(
                !next_plan.contains(stale),
                "{row_id} should not keep stale {stale:?} closeout wording"
            );
        }
    }
}

#[test]
fn json_contract_diagnostic_rows_cite_json_evidence() {
    for row_id in [
        "LAYERED-P1-002",
        "LAYERED-META-OPTION-003",
        "LAYERED-META-OPTION-005",
        "LAYERED-META-OPTION-015",
        "LAYERED-META-OPTION-061",
        "LAYERED-META-OPTION-088",
        "LAYERED-META-OPTION-089",
        "LAYERED-META-OPTION-090",
        "LAYERED-META-OPTION-094",
        "LAYERED-META-OPTION-106",
        "LAYERED-META-OPTION-119",
    ] {
        let proof =
            row_current_proof(PARITY_MATRIX, row_id).unwrap_or_else(|| panic!("{row_id} row"));
        assert!(
            proof.contains("elkrs-json"),
            "{row_id} should cite JSON import/export or validation evidence"
        );
        let next_plan =
            row_next_plan(PARITY_MATRIX, row_id).unwrap_or_else(|| panic!("{row_id} row"));
        assert!(
            next_plan.contains("1.0.0 compatibility exclusion")
                || next_plan.contains("Complete for supported 1.0.0 JSON contract"),
            "{row_id} should be closed by a compatibility exclusion or supported JSON contract"
        );
        for stale in [
            "not implemented yet",
            "semantics remain open",
            "broader",
            "deferred",
        ] {
            assert!(
                !next_plan.contains(stale),
                "{row_id} should not keep stale {stale:?} wording"
            );
        }
    }
}

#[test]
fn json_contract_unsupported_rows_document_exclusions() {
    for row_id in [
        "LAYERED-GRAPH-004",
        "LAYERED-GRAPH-010",
        "LAYERED-P4-002",
        "LAYERED-P5-006",
        "LAYERED-META-FEATURE-001",
        "LAYERED-META-FEATURE-004",
        "LAYERED-META-OPTION-004",
        "LAYERED-META-OPTION-008",
        "LAYERED-META-OPTION-009",
        "LAYERED-META-OPTION-016",
        "LAYERED-META-OPTION-019",
        "LAYERED-META-OPTION-020",
        "LAYERED-META-OPTION-047",
        "LAYERED-META-OPTION-048",
        "LAYERED-META-OPTION-049",
        "LAYERED-META-OPTION-050",
        "LAYERED-META-OPTION-051",
        "LAYERED-META-OPTION-052",
        "LAYERED-META-OPTION-053",
        "LAYERED-META-OPTION-054",
        "LAYERED-META-OPTION-055",
        "LAYERED-META-OPTION-077",
        "LAYERED-META-OPTION-078",
        "LAYERED-META-OPTION-080",
        "LAYERED-META-OPTION-081",
        "LAYERED-META-OPTION-082",
        "LAYERED-META-OPTION-083",
        "LAYERED-META-OPTION-084",
        "LAYERED-META-OPTION-085",
        "LAYERED-META-OPTION-086",
        "LAYERED-META-OPTION-087",
        "LAYERED-META-OPTION-095",
        "LAYERED-META-OPTION-096",
        "LAYERED-META-OPTION-097",
        "LAYERED-META-OPTION-098",
        "LAYERED-META-OPTION-099",
        "LAYERED-META-OPTION-102",
        "LAYERED-META-OPTION-103",
        "LAYERED-META-OPTION-104",
        "LAYERED-META-OPTION-105",
        "LAYERED-META-OPTION-107",
        "LAYERED-META-OPTION-112",
        "LAYERED-META-OPTION-113",
        "LAYERED-META-OPTION-116",
        "LAYERED-META-OPTION-127",
        "LAYERED-META-OPTION-129",
        "LAYERED-META-OPTION-130",
        "LAYERED-META-OPTION-139",
        "LAYERED-META-OPTION-147",
        "LAYERED-META-OPTION-148",
        "LAYERED-META-OPTION-149",
        "LAYERED-META-OPTION-150",
        "LAYERED-META-OPTION-151",
    ] {
        let next_plan =
            row_next_plan(PARITY_MATRIX, row_id).unwrap_or_else(|| panic!("{row_id} row"));
        assert!(
            next_plan.contains("1.0.0 compatibility exclusion"),
            "{row_id} should document the JSON compatibility exclusion"
        );
    }
}

#[test]
fn crossing_constraint_rows_document_compatibility_boundaries() {
    for row_id in [
        "LAYERED-P3-002",
        "LAYERED-META-OPTION-021",
        "LAYERED-META-OPTION-022",
        "LAYERED-META-OPTION-023",
        "LAYERED-META-OPTION-024",
        "LAYERED-META-OPTION-025",
        "LAYERED-META-OPTION-026",
        "LAYERED-META-OPTION-027",
        "LAYERED-META-OPTION-028",
        "LAYERED-META-OPTION-029",
        "LAYERED-META-OPTION-030",
        "LAYERED-META-OPTION-031",
        "LAYERED-META-OPTION-032",
        "LAYERED-META-OPTION-033",
        "LAYERED-META-OPTION-034",
        "LAYERED-META-OPTION-035",
        "LAYERED-META-OPTION-036",
        "LAYERED-META-OPTION-037",
        "LAYERED-META-OPTION-038",
        "LAYERED-META-OPTION-039",
        "LAYERED-META-OPTION-040",
        "LAYERED-META-OPTION-041",
        "LAYERED-META-OPTION-042",
        "LAYERED-META-OPTION-043",
        "LAYERED-META-OPTION-044",
        "LAYERED-META-OPTION-045",
        "LAYERED-META-OPTION-046",
        "LAYERED-META-OPTION-092",
        "LAYERED-META-OPTION-131",
    ] {
        assert_eq!(
            row_status(PARITY_MATRIX, row_id),
            Some("diagnostic"),
            "{row_id} should keep its documented crossing-constraint diagnostic status"
        );
        let next_plan = row_next_plan(PARITY_MATRIX, row_id)
            .unwrap_or_else(|| panic!("{row_id} should have a matrix row"));
        assert!(
            next_plan.contains("1.0.0 compatibility exclusion"),
            "{row_id} should document its 1.0.0 crossing compatibility boundary"
        );
    }
}

#[test]
fn placement_compaction_wrapping_rows_document_compatibility_boundaries() {
    assert_eq!(
        row_status(PARITY_MATRIX, "LAYERED-P4-003"),
        Some("java-parity"),
        "LAYERED-P4-003 should stay promoted for Java-backed disconnected component spacing"
    );

    for (row_id, status) in [
        ("LAYERED-P4-002", "unsupported"),
        ("LAYERED-P4-004", "diagnostic"),
        ("LAYERED-META-OPTION-001", "diagnostic"),
        ("LAYERED-META-OPTION-002", "diagnostic"),
        ("LAYERED-META-OPTION-004", "unsupported"),
        ("LAYERED-META-OPTION-018", "diagnostic"),
        ("LAYERED-META-OPTION-019", "unsupported"),
        ("LAYERED-META-OPTION-020", "unsupported"),
        ("LAYERED-META-OPTION-077", "unsupported"),
        ("LAYERED-META-OPTION-078", "unsupported"),
        ("LAYERED-META-OPTION-079", "diagnostic"),
        ("LAYERED-META-OPTION-080", "unsupported"),
        ("LAYERED-META-OPTION-081", "unsupported"),
        ("LAYERED-META-OPTION-082", "unsupported"),
        ("LAYERED-META-OPTION-083", "unsupported"),
        ("LAYERED-META-OPTION-095", "unsupported"),
        ("LAYERED-META-OPTION-096", "unsupported"),
        ("LAYERED-META-OPTION-097", "unsupported"),
        ("LAYERED-META-OPTION-098", "unsupported"),
        ("LAYERED-META-OPTION-099", "unsupported"),
        ("LAYERED-META-OPTION-100", "diagnostic"),
        ("LAYERED-META-OPTION-101", "diagnostic"),
        ("LAYERED-META-OPTION-102", "unsupported"),
        ("LAYERED-META-OPTION-103", "unsupported"),
        ("LAYERED-META-OPTION-104", "unsupported"),
        ("LAYERED-META-OPTION-105", "unsupported"),
        ("LAYERED-META-OPTION-113", "unsupported"),
        ("LAYERED-META-OPTION-129", "unsupported"),
        ("LAYERED-META-OPTION-132", "diagnostic"),
    ] {
        assert_eq!(
            row_status(PARITY_MATRIX, row_id),
            Some(status),
            "{row_id} should keep its documented placement compatibility status"
        );
        let next_plan = row_next_plan(PARITY_MATRIX, row_id)
            .unwrap_or_else(|| panic!("{row_id} should have a matrix row"));
        assert!(
            next_plan.contains("1.0.0 compatibility exclusion"),
            "{row_id} should document its 1.0.0 placement compatibility boundary"
        );
    }
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

fn row_current_proof<'a>(matrix: &'a str, row_id: &str) -> Option<&'a str> {
    matrix.lines().find_map(|line| {
        let mut columns = line.split('|').map(str::trim);
        columns.next()?;
        let id = columns.next()?;
        if id != row_id {
            return None;
        }
        columns.next()?;
        columns.next()?;
        columns.next()?;
        columns.next()
    })
}

fn row_next_plan<'a>(matrix: &'a str, row_id: &str) -> Option<&'a str> {
    matrix.lines().find_map(|line| {
        let mut columns = line.split('|').map(str::trim);
        columns.next()?;
        let id = columns.next()?;
        if id != row_id {
            return None;
        }
        columns.next()?;
        columns.next()?;
        columns.next()?;
        columns.next()?;
        columns.next()
    })
}
