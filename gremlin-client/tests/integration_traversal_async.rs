mod common;

#[cfg(feature = "async_gremlin")]
mod aio {
    use rstest::*;
    use rstest_reuse::{self, *};

    use serial_test::serial;

    use gremlin_client::{aio::GremlinClient, process::traversal::traversal};

    use super::common::aio::{connect_serializer, create_vertex_with_label, drop_vertices};

    #[cfg(feature = "async-std-runtime")]
    use async_std::prelude::*;

    #[cfg(feature = "tokio-runtime")]
    use tokio_stream::StreamExt;

    use gremlin_client::{IoProtocol, Vertex};

    #[rstest]
    #[case::graphson_v2(connect_serializer(IoProtocol::GraphSONV2))]
    #[case::graphson_v3(connect_serializer(IoProtocol::GraphSONV3))]
    #[case::graph_binary_v1(connect_serializer(IoProtocol::GraphBinaryV1))]
    #[awt]
    #[cfg_attr(feature = "async-std-runtime", async_std::test)]
    #[cfg_attr(feature = "tokio-runtime", tokio::test)]
    #[serial(test_simple_vertex_traversal_with_multiple_id)]
    async fn test_simple_vertex_traversal_with_multiple_id(
        #[future]
        #[case]
        client: GremlinClient,
    ) {
        drop_vertices(&client, "test_simple_vertex_traversal_async")
            .await
            .unwrap();

        let vertex =
            create_vertex_with_label(&client, "test_simple_vertex_traversal_async", "Traversal")
                .await;
        let vertex2 =
            create_vertex_with_label(&client, "test_simple_vertex_traversal_async", "Traversal")
                .await;

        let g = traversal().with_remote_async(client);

        let results = g
            .v(vec![vertex.id(), vertex2.id()])
            .to_list()
            .await
            .unwrap();

        assert_eq!(2, results.len());

        assert_eq!(vertex.id(), results[0].id());
        assert_eq!(vertex2.id(), results[1].id());

        let has_next = g
            .v(())
            .has_label("test_simple_vertex_traversal_async")
            .has_next()
            .await
            .expect("It should return");

        assert_eq!(true, has_next);

        let next = g
            .v(())
            .has_label("test_simple_vertex_traversal_async")
            .next()
            .await
            .expect("It should execute one traversal")
            .expect("It should return one element");

        assert_eq!("test_simple_vertex_traversal_async", next.label());

        let vertices = g
            .v(())
            .has_label("test_simple_vertex_traversal_async")
            .iter()
            .await
            .expect("It should get the iterator")
            .collect::<Result<Vec<Vertex>, _>>()
            .await
            .expect("It should collect elements");

        assert_eq!(2, vertices.len());
    }
}
