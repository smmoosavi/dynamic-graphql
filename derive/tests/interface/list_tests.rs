use dynamic_graphql::App;
use dynamic_graphql::FieldValue;
use dynamic_graphql::Instance;
use dynamic_graphql::Interface;
use dynamic_graphql::ResolvedObject;
use dynamic_graphql::ResolvedObjectFields;
use dynamic_graphql::SimpleObject;
use dynamic_graphql::dynamic::DynamicRequestExt;

use crate::schema_utils::normalize_schema;

#[tokio::test]
async fn test_interface_as_optional_value() {
    #[Interface]
    trait Node {
        fn the_id(&self) -> String;
    }

    #[derive(SimpleObject)]
    #[graphql(implements(Node))]
    struct FooNode {
        other_field: String,
    }

    impl Node for FooNode {
        fn the_id(&self) -> String {
            "foo".to_string()
        }
    }

    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        async fn node(&self) -> Option<Instance<dyn Node>> {
            Some(Instance::new_owned(FooNode {
                other_field: "foo".to_string(),
            }))
        }
    }

    #[derive(App)]
    struct App(Query, FooNode);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r"
    type FooNode implements Node {
      otherField: String!
      theId: String!
    }

    interface Node {
      theId: String!
    }

    type Query {
      node: Node
    }

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    ");

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
                "otherField": "foo",
            }
        })
    );
}

#[tokio::test]
async fn test_interface_as_list_value() {
    #[Interface]
    trait Node {
        fn the_id(&self) -> String;
    }

    #[derive(SimpleObject)]
    #[graphql(implements(Node))]
    struct FooNode {
        other_field: String,
    }

    impl Node for FooNode {
        fn the_id(&self) -> String {
            "foo".to_string()
        }
    }

    #[derive(SimpleObject)]
    #[graphql(implements(Node))]
    struct BarNode {
        another_field: String,
    }

    impl Node for BarNode {
        fn the_id(&self) -> String {
            "foo".to_string()
        }
    }

    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        async fn nodes(&self) -> Vec<Instance<dyn Node>> {
            vec![
                Instance::new_owned(FooNode {
                    other_field: "foo".to_string(),
                }),
                Instance::new_owned(BarNode {
                    another_field: "bar".to_string(),
                }),
            ]
        }
    }

    #[derive(App)]
    struct App(Query, FooNode, BarNode);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r"
    type BarNode implements Node {
      anotherField: String!
      theId: String!
    }

    type FooNode implements Node {
      otherField: String!
      theId: String!
    }

    interface Node {
      theId: String!
    }

    type Query {
      nodes: [Node!]!
    }

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    ");

    let query = r#"

        query {
            nodes {
                theId
                ... on FooNode {
                    otherField
                }
                ... on BarNode {
                    anotherField
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
            "nodes": [
                {
                    "theId": "foo",
                    "otherField": "foo",
                },
                {
                    "theId": "foo",
                    "anotherField": "bar",
                },
            ]
        })
    );
}
