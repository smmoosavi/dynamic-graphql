use crate::schema_utils::normalize_schema;
use dynamic_graphql::dynamic::DynamicRequestExt;
use dynamic_graphql::{FieldValue, Interface, ResolvedObject, ResolvedObjectFields, SimpleObject};
use dynamic_graphql_derive::App;

#[tokio::test]
async fn interface_as_output_value() {
    #[Interface(NodeInterface)]
    trait Node {
        fn the_id(&self) -> String;
    }

    #[derive(SimpleObject)]
    #[graphql(implement = "NodeInterface")]
    struct FooNode {
        other_field: String,
    }

    impl Node for FooNode {
        fn the_id(&self) -> String {
            "foo".to_string()
        }
    }

    #[derive(ResolvedObject)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        async fn node(&self) -> NodeInterface {
            NodeInterface::new_owned(FooNode {
                other_field: "bar".to_string(),
            })
        }
    }

    #[derive(App)]
    struct App(Query, NodeInterface<'static>, FooNode);

    let registry = dynamic_graphql::Registry::new();
    let registry = registry.register::<App>().set_root("Query");
    let schema = registry.create_schema().finish().unwrap();
    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"

                type FooNode implements Node {
                    otherField: String!
                    theId: String!
                }

                interface Node {
                    theId: String!
                }

                type Query {
                    node: Node!
                }

                schema {
                    query: Query
                }

            "#
        ),
    );

    let query = r#"

        query {
            node {
                theId
                ... on FooNode {
                    otherField
                }
            }
        }

    "#;

    let root = Query;
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    assert_eq!(
        res.data.into_json().unwrap(),
        serde_json::json!({
            "node": {
                "theId": "foo",
                "otherField": "bar",
            }
        })
    );
}
