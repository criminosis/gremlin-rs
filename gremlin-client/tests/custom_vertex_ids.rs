use std::collections::HashMap;

use common::io::{drop_vertices, expect_janusgraph_client};
use gremlin_client::{
    process::traversal::{traversal, __},
    structure::T,
    GKey, GValue, IoProtocol,
};

use rstest::*;
use rstest_reuse::apply;
use serial_test::serial;

mod common;

//Custom vertex ids are a feature offered by JanusGraph
//https://docs.janusgraph.org/advanced-topics/custom-vertex-id/

#[apply(common::serializers)]
#[serial(test_mapping_custom_vertex_id)]
#[cfg(feature = "derive")]
fn test_mapping_custom_vertex_id(protocol: IoProtocol) {
    if protocol == IoProtocol::GraphSONV2 {
        //GraphSONV2 doesn't support the non-string key of the merge step,
        //so skip it in testing
        return;
    }
    let client = expect_janusgraph_client(protocol);
    use chrono::{DateTime, TimeZone, Utc};
    use gremlin_client::derive::FromGMap;
    use gremlin_client::process::traversal::{Bytecode, TraversalBuilder};
    use std::convert::TryFrom;

    drop_vertices(&client, "test_mapping_custom_vertex_id").unwrap();

    let g = traversal().with_remote(client);

    let uuid = uuid::Uuid::new_v4();
    let mark = g
        .add_v("test_mapping_custom_vertex_id")
        .property(T::Id, "test_mapping_custom_id")
        .property("name", "Mark")
        .property("age", 22)
        .property("time", 22 as i64)
        .property("score", 3.2)
        .property("uuid", uuid.clone())
        .property("datetime", chrono::Utc.timestamp(1551825863, 0))
        .property("date", 1551825863 as i64)
        .value_map(true)
        .by(TraversalBuilder::new(Bytecode::new()).unfold())
        .next();
    assert_eq!(mark.is_ok(), true);

    #[derive(Debug, PartialEq, FromGMap)]
    struct Person {
        id: String,
        label: String,
        name: String,
        age: i32,
        time: i64,
        datetime: DateTime<Utc>,
        uuid: uuid::Uuid,
        optional: Option<String>,
    }
    let person = Person::try_from(mark.unwrap().unwrap()).expect("Should get person");

    assert_eq!(
        Person {
            id: String::from("test_mapping_custom_id"),
            label: String::from("test_mapping_custom_vertex_id"),
            name: String::from("Mark"),
            age: 22,
            time: 22,
            datetime: chrono::Utc.timestamp(1551825863, 0),
            uuid: uuid,
            optional: None
        },
        person
    );
}

#[apply(common::serializers)]
#[serial(test_merge_v_custom_id)]
fn test_merge_v_custom_id(protocol: IoProtocol) {
    if protocol == IoProtocol::GraphSONV2 {
        //GraphSONV2 doesn't support the non-string key of the merge step,
        //so skip it in testing
        return;
    }
    let client = expect_janusgraph_client(protocol);
    let expected_label = "test_merge_v_custom_id";
    drop_vertices(&client, expected_label).expect("Failed to drop vertices");
    let g = traversal().with_remote(client);
    let expected_id = "test_merge_v_custom_id";
    let mut start_step_map: HashMap<GKey, GValue> = HashMap::new();
    start_step_map.insert(T::Id.into(), expected_id.into());
    start_step_map.insert(T::Label.into(), expected_label.into());
    let actual_vertex = g
        .merge_v(start_step_map)
        .next()
        .expect("Should get a response")
        .expect("Should return a vertex");
    match actual_vertex.id() {
        gremlin_client::GID::String(actual) => assert_eq!(expected_id, actual),
        other => panic!("Didn't get expected id type {:?}", other),
    }

    assert_eq!(expected_label, actual_vertex.label());

    //Now try it as a mid-traversal step (inject is the start step)
    let expected_id = "foo";
    let expected_property = "propValue";

    let mut map_to_inject: HashMap<GKey, GValue> = HashMap::new();
    let mut lookup_map: HashMap<GKey, GValue> = HashMap::new();
    lookup_map.insert(T::Id.into(), expected_id.into());
    lookup_map.insert(T::Label.into(), "myvertexlabel".into());
    let mut property_map: HashMap<GKey, GValue> = HashMap::new();
    property_map.insert("propertyKey".into(), expected_property.into());
    map_to_inject.insert("lookup".into(), lookup_map.into());
    map_to_inject.insert("properties".into(), property_map.into());

    let actual_vertex = g
        .inject(vec![map_to_inject.into()])
        .unfold()
        .as_("payload")
        .merge_v(__.select("lookup"))
        .property(
            "propertyKey",
            __.select("payload")
                .select("properties")
                .select("propertyKey"),
        )
        .next()
        .expect("Should get response")
        .expect("Should have returned a vertex");

    match actual_vertex.id() {
        gremlin_client::GID::String(actual) => assert_eq!(expected_id, actual),
        other => panic!("Didn't get expected id type {:?}", other),
    }

    let actual_property: &String = actual_vertex
        .property("propertyKey")
        .expect("Should have property")
        .get()
        .unwrap();
    assert_eq!(expected_property, actual_property);
}

#[apply(common::serializers)]
#[serial(test_merge_v_custom_id)]
fn test_add_v_custom_id(protocol: IoProtocol) {
    let client = expect_janusgraph_client(protocol);
    let expected_id = "test_add_v_custom_id";
    let test_vertex_label = "test_add_v_custom_id";
    drop_vertices(&client, test_vertex_label).expect("Failed to drop vertices");
    let g = traversal().with_remote(client);
    let actual_vertex = g
        .add_v(test_vertex_label)
        .property(T::Id, expected_id)
        .next()
        .expect("Should get a response")
        .expect("Should return a vertex");
    match actual_vertex.id() {
        gremlin_client::GID::String(actual) => assert_eq!(expected_id, actual),
        other => panic!("Didn't get expected id type {:?}", other),
    }
}
