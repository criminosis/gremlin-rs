use gremlin_client::Map;

pub fn assert_map_property(element_map: &Map, expected_key: &str, expected_value: &str) {
    let actual_prop_value: &String = element_map
        .get(expected_key)
        .unwrap_or_else(|| panic!("Didn't have expected key {}", expected_key))
        .get()
        .expect("Should be String");
    assert_eq!(expected_value, actual_prop_value);
}

#[allow(dead_code)]
pub mod io {
    use gremlin_client::{ConnectionOptions, Edge, IoProtocol, GremlinClient, GremlinResult, Vertex};

    pub fn connect() -> GremlinResult<GremlinClient> {
        GremlinClient::connect(("localhost", 8182))
    }

    fn connect_janusgraph_client() -> GremlinResult<GremlinClient> {
        GremlinClient::connect(("localhost", 8184))
    }

    pub fn connect_serializer(serializer: IoProtocol) -> GremlinResult<GremlinClient> {
        let port = match serializer {
            IoProtocol::GraphSONV2 => 8182,
            IoProtocol::GraphSONV3 => 8182,
        };
        GremlinClient::connect(
            ConnectionOptions::builder()
                .host("localhost")
                .port(port)
                .serializer(serializer.clone())
                .deserializer(serializer)
                .build(),
        )
    }

    pub fn expect_client() -> GremlinClient {
        connect().expect("It should connect")
    }

    pub fn expect_janusgraph_client() -> GremlinClient {
        connect_janusgraph_client().expect("It should connect")
    }

    pub fn expect_client_serializer(serializer: IoProtocol) -> GremlinClient {
        connect_serializer(serializer).expect("It should connect")
    }

    pub fn graph() -> GremlinClient {
        let client = expect_client();

        client
    }

    pub fn graph_serializer(serializer: IoProtocol) -> GremlinClient {
        let client = expect_client_serializer(serializer);

        client
    }

    pub fn create_vertex(graph: &GremlinClient, name: &str) -> Vertex {
        create_vertex_with_label(graph, "person", name)
    }

    pub fn drop_vertices(graph: &GremlinClient, label: &str) -> GremlinResult<()> {
        graph
            .execute("g.V().hasLabel(_label).drop()", &[("_label", &label)])
            .map(|_| ())
    }

    pub fn drop_edges(graph: &GremlinClient, label: &str) -> GremlinResult<()> {
        graph
            .execute("g.E().hasLabel(_label).drop()", &[("_label", &label)])
            .map(|_| ())
    }

    pub fn create_vertex_with_label(graph: &GremlinClient, label: &str, name: &str) -> Vertex {
        graph
            .execute(
                "g.addV(_label).property('name',name)",
                &[("_label", &label), ("name", &name)],
            )
            .expect("it should execute addV")
            .filter_map(Result::ok)
            .map(|f| f.take::<Vertex>())
            .collect::<Result<Vec<Vertex>, _>>()
            .expect("It should be ok")
            .pop()
            .expect("It should contain 1 element")
    }

    pub fn create_edge(graph: &GremlinClient, v: &Vertex, v1: &Vertex, name: &str) -> Edge {
        graph
            .execute(
                "g.V(v1).as('a').V(v2).as('b').addE(rel).from('a').to('b')",
                &[("rel", &name), ("v1", v.id()), ("v2", v1.id())],
            )
            .expect("it should execute addE")
            .filter_map(Result::ok)
            .map(|f| f.take::<Edge>())
            .collect::<Result<Vec<Edge>, _>>()
            .expect("It should be ok")
            .pop()
            .expect("It should contain 1 element")
    }
}

#[cfg(feature = "async_gremlin")]
#[allow(dead_code)]
pub mod aio {
    use gremlin_client::aio::GremlinClient;

    use gremlin_client::{ConnectionOptions, Edge, IoProtocol, GremlinResult, Vertex};

    #[cfg(feature = "async-std-runtime")]
    use async_std::prelude::*;

    #[cfg(feature = "tokio-runtime")]
    use tokio_stream::StreamExt;

    pub async fn connect() -> GremlinClient {
        GremlinClient::connect(("localhost", 8182))
            .await
            .expect("It should connect")
    }

    pub async fn connect_serializer(serializer: IoProtocol) -> GremlinClient {
        let port = match serializer {
            IoProtocol::GraphSONV2 => 8182,
            IoProtocol::GraphSONV3 => 8182,
        };
        GremlinClient::connect(
            ConnectionOptions::builder()
                .host("localhost")
                .port(port)
                .serializer(serializer.clone())
                .deserializer(serializer)
                .build(),
        )
        .await
        .expect("It should connect")
    }

    pub async fn create_vertex(graph: &GremlinClient, name: &str) -> Vertex {
        create_vertex_with_label(graph, "person", name).await
    }

    pub async fn create_vertex_with_label(
        graph: &GremlinClient,
        label: &str,
        name: &str,
    ) -> Vertex {
        graph
            .execute(
                "g.addV(_label).property('name',name)",
                &[("_label", &label), ("name", &name)],
            )
            .await
            .expect("it should execute addV")
            .filter_map(Result::ok)
            .map(|f| f.take::<Vertex>())
            .collect::<Result<Vec<Vertex>, _>>()
            .await
            .expect("It should be ok")
            .pop()
            .expect("It should contain 1 element")
    }

    pub async fn drop_vertices(graph: &GremlinClient, label: &str) -> GremlinResult<()> {
        graph
            .execute("g.V().hasLabel(_label).drop()", &[("_label", &label)])
            .await
            .map(|_| ())
    }

    pub async fn drop_edges(graph: &GremlinClient, label: &str) -> GremlinResult<()> {
        graph
            .execute("g.E().hasLabel(_label).drop()", &[("_label", &label)])
            .await
            .map(|_| ())
    }

    pub async fn create_edge(graph: &GremlinClient, v: &Vertex, v1: &Vertex, name: &str) -> Edge {
        graph
            .execute(
                "g.V(v1).as('a').V(v2).as('b').addE(rel).from('a').to('b')",
                &[("rel", &name), ("v1", v.id()), ("v2", v1.id())],
            )
            .await
            .expect("it should execute addE")
            .filter_map(Result::ok)
            .map(|f| f.take::<Edge>())
            .collect::<Result<Vec<Edge>, _>>()
            .await
            .expect("It should be ok")
            .pop()
            .expect("It should contain 1 element")
    }
}
