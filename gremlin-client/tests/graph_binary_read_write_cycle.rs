use std::array::IntoIter;
use std::collections::{HashSet, HashMap};

use chrono::{DateTime, TimeZone, Utc};
use common::io::graph_serializer;
use gremlin_client::{
    process::traversal::{
        traversal, Bytecode, GraphTraversal, GraphTraversalSource, Scope, SyncTerminator,
    },
    GValue, IoProtocol,
};
use rstest::rstest;
use uuid::Uuid;
use std::iter::FromIterator;

mod common;

fn get_graph_source() -> GraphTraversalSource<SyncTerminator> {
    traversal().with_remote(graph_serializer(IoProtocol::GraphBinaryV1))
}

#[rstest]
#[case::int(1i32)]
#[case::long(1i64)]
#[case::string("abc")]
#[case::date(Utc.timestamp_millis(9001))]
#[case::double(0.1f64)]
#[case::float(0.1f32)]
#[case::int_list(Vec::from_iter([GValue::Int64(2)]))]
#[case::map_str_int(HashMap::<_, _>::from_iter(IntoIter::new([(String::from("abc"), GValue::Int32(1))])))]
#[case::int_set(GValue::Set(Vec::from_iter([GValue::Int64(2)]).into()))]
#[case::uuid(Uuid::new_v4())]
fn simple_value_rw_cycle<T: Into<GValue>>(#[case] payload: T) {
    let payload = payload.into();
    assert_eq!(
        get_graph_source().inject(payload.clone()).next().unwrap(),
        Some(payload)
    )
}

// #[test]
// fn edge_rw_cycle() {
//     todo!()
// }

// #[test]
// fn path_rw_cycle() {
//     todo!()
// }

// #[test]
// fn property_rw_cycle() {
//     todo!()
// }

// #[test]
// fn vertex_rw_cycle() {
//     todo!()
// }

// #[test]
// fn vertex_property_rw_cycle() {
//     todo!()
// }

// #[test]
// fn scope_rw_cycle() {
//     todo!()
// }

// #[test]
// fn traverser_rw_cycle() {
//     todo!()
// }
