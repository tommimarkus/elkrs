#![allow(dead_code)]

use elkrs_core::geometry::{Point, Size};
use elkrs_core::graph::{ElementId, ElementRef, ElkEdge, ElkGraph, ElkNode, ElkPort};
use elkrs_core::options::PortSide;

pub fn chain() -> ElkGraph {
    let mut graph = ElkGraph::new("root");
    for id in ["a", "b", "c"] {
        graph.add_node(node(id, 40.0, 30.0));
    }
    graph.add_edge(edge("ab", "a", "b"));
    graph.add_edge(edge("bc", "b", "c"));
    graph
}

pub fn diamond() -> ElkGraph {
    let mut graph = ElkGraph::new("root");
    for id in ["a", "b", "c", "d"] {
        graph.add_node(node(id, 50.0, 30.0));
    }
    graph.add_edge(edge("ab", "a", "b"));
    graph.add_edge(edge("ac", "a", "c"));
    graph.add_edge(edge("bd", "b", "d"));
    graph.add_edge(edge("cd", "c", "d"));
    graph
}

pub fn fan_in() -> ElkGraph {
    let mut graph = ElkGraph::new("root");
    for id in ["a", "b", "c", "d"] {
        graph.add_node(node(id, 50.0, 30.0));
    }
    graph.add_edge(edge("ad", "a", "d"));
    graph.add_edge(edge("bd", "b", "d"));
    graph.add_edge(edge("cd", "c", "d"));
    graph
}

pub fn fan_out() -> ElkGraph {
    let mut graph = ElkGraph::new("root");
    for id in ["a", "b", "c", "d"] {
        graph.add_node(node(id, 50.0, 30.0));
    }
    graph.add_edge(edge("ab", "a", "b"));
    graph.add_edge(edge("ac", "a", "c"));
    graph.add_edge(edge("ad", "a", "d"));
    graph
}

pub fn two_layer_crossing() -> ElkGraph {
    let mut graph = ElkGraph::new("root");
    for id in ["a", "b", "d", "c"] {
        graph.add_node(node(id, 50.0, 30.0));
    }
    graph.add_edge(edge("a-c", "a", "c"));
    graph.add_edge(edge("b-d", "b", "d"));
    graph
}

pub fn nested_group() -> ElkGraph {
    let mut group = node("group", 240.0, 160.0);
    group.add_child(node("child-a", 50.0, 30.0));
    group.add_child(node("child-b", 50.0, 30.0));
    let mut graph = ElkGraph::new("root");
    graph.add_node(group);
    graph
}

pub fn cross_group_edge() -> ElkGraph {
    let mut group = node("group", 240.0, 160.0);
    group.add_child(node("child", 50.0, 30.0));
    let mut graph = ElkGraph::new("root");
    graph.add_node(group);
    graph.add_node(node("external", 50.0, 30.0));
    graph.add_edge(edge("child-external", "child", "external"));
    graph
}

pub fn port_heavy() -> ElkGraph {
    let mut source = node("source", 100.0, 50.0);
    source.add_port(port(
        "out-top",
        PortSide::East,
        Point::new(90.0, 5.0),
        Size::new(10.0, 10.0),
    ));
    source.add_port(port(
        "out-bottom",
        PortSide::East,
        Point::new(90.0, 35.0),
        Size::new(10.0, 10.0),
    ));

    let mut target = node("target", 100.0, 50.0);
    target.add_port(port(
        "in-top",
        PortSide::West,
        Point::new(0.0, 5.0),
        Size::new(10.0, 10.0),
    ));
    target.add_port(port(
        "in-bottom",
        PortSide::West,
        Point::new(0.0, 35.0),
        Size::new(10.0, 10.0),
    ));

    let mut graph = ElkGraph::new("root");
    graph.add_node(source);
    graph.add_node(target);
    graph.add_edge(ElkEdge::new(
        "top",
        ElementRef::Port {
            node: ElementId::from("source"),
            port: ElementId::from("out-top"),
        },
        ElementRef::Port {
            node: ElementId::from("target"),
            port: ElementId::from("in-top"),
        },
    ));
    graph.add_edge(ElkEdge::new(
        "bottom",
        ElementRef::Port {
            node: ElementId::from("source"),
            port: ElementId::from("out-bottom"),
        },
        ElementRef::Port {
            node: ElementId::from("target"),
            port: ElementId::from("in-bottom"),
        },
    ));
    graph
}

pub fn multi_edge_pair() -> ElkGraph {
    let mut graph = ElkGraph::new("root");
    graph.add_node(node("a", 60.0, 30.0));
    graph.add_node(node("b", 60.0, 30.0));
    graph.add_edge(edge("ab-1", "a", "b"));
    graph.add_edge(edge("ab-2", "a", "b"));
    graph
}

pub fn self_loop() -> ElkGraph {
    let mut graph = ElkGraph::new("root");
    graph.add_node(node("a", 80.0, 40.0));
    graph.add_edge(edge("aa", "a", "a"));
    graph
}

pub fn edge_node_spacing_obstacle() -> ElkGraph {
    let mut graph = ElkGraph::new("root");
    graph.properties.set_spacing_edge_node(48.0);
    graph.add_node(node("a-source", 40.0, 30.0));
    graph.add_node(node("b-obstacle", 40.0, 80.0));
    graph.add_node(node("c-target", 40.0, 30.0));
    graph.add_edge(edge("direct", "a-source", "c-target"));
    graph.add_edge(edge("source-obstacle", "a-source", "b-obstacle"));
    graph.add_edge(edge("obstacle-target", "b-obstacle", "c-target"));
    graph
}

pub fn consumer_compound_ports() -> ElkGraph {
    let mut client = node("a-client", 80.0, 40.0);
    client.add_port(port(
        "client-out",
        PortSide::East,
        Point::new(70.0, 15.0),
        Size::new(10.0, 10.0),
    ));

    let mut api = node("b-api", 90.0, 40.0);
    api.add_port(port(
        "api-in",
        PortSide::West,
        Point::new(0.0, 15.0),
        Size::new(10.0, 10.0),
    ));
    api.add_port(port(
        "api-out",
        PortSide::East,
        Point::new(80.0, 15.0),
        Size::new(10.0, 10.0),
    ));

    let mut worker = node("c-worker", 90.0, 40.0);
    worker.add_port(port(
        "worker-in",
        PortSide::West,
        Point::new(0.0, 15.0),
        Size::new(10.0, 10.0),
    ));

    let mut group = node("core-services", 320.0, 220.0);
    group.add_child(api);
    group.add_child(worker);

    let mut graph = ElkGraph::new("root");
    graph.add_node(client);
    graph.add_node(group);
    graph.add_edge(ElkEdge::new(
        "client-api",
        ElementRef::Port {
            node: ElementId::from("a-client"),
            port: ElementId::from("client-out"),
        },
        ElementRef::Port {
            node: ElementId::from("b-api"),
            port: ElementId::from("api-in"),
        },
    ));
    graph.add_edge(ElkEdge::new(
        "api-worker",
        ElementRef::Port {
            node: ElementId::from("b-api"),
            port: ElementId::from("api-out"),
        },
        ElementRef::Port {
            node: ElementId::from("c-worker"),
            port: ElementId::from("worker-in"),
        },
    ));
    graph
}

pub fn node(id: &str, width: f64, height: f64) -> ElkNode {
    let mut node = ElkNode::new(id);
    node.size = Size::new(width, height);
    node
}

pub fn edge(id: &str, source: &str, target: &str) -> ElkEdge {
    ElkEdge::new(
        id,
        ElementRef::Node(ElementId::from(source)),
        ElementRef::Node(ElementId::from(target)),
    )
}

pub fn port(id: &str, side: PortSide, position: Point, size: Size) -> ElkPort {
    let mut port = ElkPort::new(id);
    port.side = Some(side);
    port.position = position;
    port.size = size;
    port
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParityFixtureStatus {
    RustOnly,
    JavaComparable,
}

pub struct ParityFixture {
    pub id: &'static str,
    pub name: &'static str,
    pub status: ParityFixtureStatus,
    pub build: fn() -> ElkGraph,
    pub assertions: &'static [ParityAssertion],
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParityAssertion {
    EdgeNodeClearance {
        edge_id: &'static str,
        node_id: &'static str,
        minimum: f64,
    },
}

pub fn parity_fixtures() -> Vec<ParityFixture> {
    vec![
        ParityFixture {
            id: "LAYERED-GRAPH-001",
            name: "chain",
            status: ParityFixtureStatus::JavaComparable,
            build: chain,
            assertions: &[],
        },
        ParityFixture {
            id: "LAYERED-GRAPH-002",
            name: "multi-edge-pair",
            status: ParityFixtureStatus::JavaComparable,
            build: multi_edge_pair,
            assertions: &[],
        },
        ParityFixture {
            id: "LAYERED-GRAPH-003",
            name: "self-loop",
            status: ParityFixtureStatus::JavaComparable,
            build: self_loop,
            assertions: &[],
        },
        ParityFixture {
            id: "LAYERED-P3-001",
            name: "two-layer-crossing",
            status: ParityFixtureStatus::JavaComparable,
            build: two_layer_crossing,
            assertions: &[],
        },
        ParityFixture {
            id: "LAYERED-OPT-005",
            name: "edge-node-spacing-obstacle",
            status: ParityFixtureStatus::JavaComparable,
            build: edge_node_spacing_obstacle,
            assertions: &[ParityAssertion::EdgeNodeClearance {
                edge_id: "direct",
                node_id: "b-obstacle",
                minimum: 48.0,
            }],
        },
        ParityFixture {
            id: "LAYERED-GRAPH-008",
            name: "nested-group",
            status: ParityFixtureStatus::JavaComparable,
            build: nested_group,
            assertions: &[],
        },
        ParityFixture {
            id: "LAYERED-GRAPH-009",
            name: "consumer-compound-ports",
            status: ParityFixtureStatus::RustOnly,
            build: consumer_compound_ports,
            assertions: &[],
        },
        ParityFixture {
            id: "LAYERED-P5-002",
            name: "port-heavy",
            status: ParityFixtureStatus::JavaComparable,
            build: port_heavy,
            assertions: &[],
        },
    ]
}
