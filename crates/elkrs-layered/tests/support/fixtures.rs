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
