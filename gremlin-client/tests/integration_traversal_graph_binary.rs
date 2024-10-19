use common::io::graph_serializer;
use gremlin_client::{
    process::traversal::{traversal, Scope},
    GValue, IoProtocol,
};

mod common;

#[test]
fn demo() {
    let g = traversal().with_remote(graph_serializer(IoProtocol::GraphBinaryV1));
    let y = g.inject(1).sum(Scope::Global).next().unwrap();
    panic!("Got {:?}", y);
}
