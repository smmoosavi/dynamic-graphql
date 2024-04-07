use dynamic_graphql::dynamic::DynamicRequestExt;
use dynamic_graphql::App;
use dynamic_graphql::FieldValue;
use dynamic_graphql::Instance;
use dynamic_graphql::Interface;
use dynamic_graphql::ResolvedObject;
use dynamic_graphql::ResolvedObjectFields;
use dynamic_graphql::SimpleObject;

use crate::schema_utils::normalize_schema;

#[tokio::test]
async fn interface_as_output_value_for_simple_object_with_implement() {
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
        async fn node(&self) -> Instance<dyn Node> {
            Instance::new_owned(FooNode {
                other_field: "foo".to_string(),
            })
        }
    }

    #[derive(App)]
    struct App(Query, FooNode);

    let schema = App::create_schema().finish().unwrap();
    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r###"
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
    "###);

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
async fn interface_as_output_value_for_simple_object_with_mark_with() {
    #[Interface]
    trait Node {
        fn the_id(&self) -> String;
    }

    #[derive(SimpleObject)]
    #[graphql(mark(Node))]
    struct FooNode {
        the_id: String,
        other_field: String,
    }

    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        async fn node(&self) -> Instance<dyn Node> {
            Instance::new_owned(FooNode {
                the_id: "foo".to_string(),
                other_field: "foo".to_string(),
            })
        }
    }

    #[derive(App)]
    struct App(Query, FooNode);

    let schema = App::create_schema().finish().unwrap();
    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r###"
    type FooNode implements Node {
      theId: String!
      otherField: String!
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
    "###);

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
async fn interface_as_output_value_for_resolved_object_with_implement() {
    #[Interface]
    trait Node {
        fn the_id(&self) -> String;
    }

    #[derive(ResolvedObject)]
    #[graphql(implements(Node))]
    struct FooNode;

    #[ResolvedObjectFields]
    impl FooNode {
        async fn other_field(&self) -> String {
            "foo".to_string()
        }
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
        async fn node(&self) -> Instance<dyn Node> {
            Instance::new_owned(FooNode)
        }
    }

    #[derive(App)]
    struct App(Query, FooNode);

    let schema = App::create_schema().finish().unwrap();
    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r###"
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
    "###);

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
async fn interface_as_output_value_for_resolved_object_with_mark_with() {
    #[Interface]
    trait Node {
        fn the_id(&self) -> String;
    }

    #[derive(ResolvedObject)]
    #[graphql(mark(Node))]
    struct FooNode;

    #[ResolvedObjectFields]
    impl FooNode {
        fn the_id(&self) -> String {
            "foo".to_string()
        }
        async fn other_field(&self) -> String {
            "foo".to_string()
        }
    }

    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        async fn node(&self) -> Instance<dyn Node> {
            Instance::new_owned(FooNode)
        }
    }

    #[derive(App)]
    struct App(Query, FooNode);

    let schema = App::create_schema().finish().unwrap();
    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r###"
    type FooNode implements Node {
      theId: String!
      otherField: String!
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
    "###);

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
