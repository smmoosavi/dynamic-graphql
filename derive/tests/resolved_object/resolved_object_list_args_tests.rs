use dynamic_graphql::dynamic::DynamicRequestExt;
use dynamic_graphql::App;
use dynamic_graphql::{FieldValue, ResolvedObject, ResolvedObjectFields};

use crate::schema_utils::normalize_schema;

#[tokio::test]
async fn test_schema() {
    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        fn hello(names: Vec<String>) -> String {
            format!("Hello {}", names.join(", "))
        }
        fn hello_with_ref(names: &[String]) -> String {
            format!("Hello {}", names.join(", "))
        }
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"
            type Query {
                hello(names: [String!]!): String!
                helloWithRef(names: [String!]!): String!
            }
            schema {
              query: Query
            }
            "#
        ),
    );

    let query = r#"
        query {
            hello(names: ["world", "rust"])
            helloWithRef(names: ["world", "rust"])
        }
    "#;

    let root = Query;
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));

    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();
    assert_eq!(
        data,
        serde_json::json!({
            "hello": "Hello world, rust",
            "helloWithRef": "Hello world, rust",
        }),
    );
}

#[tokio::test]
async fn test_schema_optional_arg() {
    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        fn hello_with_optional_arg(names: Option<Vec<String>>) -> String {
            match names {
                None => "Anybody there?".to_string(),
                Some(names) => {
                    format!("Hello {}", names.join(", "))
                }
            }
        }
        fn hello_with_optional_arg_ref(names: &Option<Vec<String>>) -> String {
            match names {
                None => "Anybody there?".to_string(),
                Some(names) => {
                    format!("Hello {}", names.join(", "))
                }
            }
        }
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"
            type Query {
                helloWithOptionalArg(names: [String!]): String!
                helloWithOptionalArgRef(names: [String!]): String!
            }
            schema {
              query: Query
            }
            "#
        ),
    );
    let query = r#"
        query($names: [String!]) {
            helloWithOptionalArg(names: $names)
            helloWithOptionalArgRef(names: $names)
        }
    "#;

    let root = Query;
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));

    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();
    assert_eq!(
        data,
        serde_json::json!({
            "helloWithOptionalArg": "Anybody there?",
            "helloWithOptionalArgRef": "Anybody there?",
        }),
    );

    let query = r#"
        query {
            helloWithOptionalArg(names: ["world", "rust"])
            helloWithOptionalArgRef(names: ["world", "rust"])
        }
    "#;

    let root = Query;
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));

    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();
    assert_eq!(
        data,
        serde_json::json!({
            "helloWithOptionalArg": "Hello world, rust",
            "helloWithOptionalArgRef": "Hello world, rust",
        }),
    );
}

#[tokio::test]
async fn test_schema_optional_item() {
    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        fn hello_with_optional_item(names: Vec<Option<String>>) -> String {
            let names = names
                .into_iter()
                .map(|name| name.unwrap_or_else(|| "nobody".to_string()))
                .collect::<Vec<_>>();
            format!("Hello {}", names.join(", "))
        }
        fn hello_with_optional_item_ref(names: &[Option<String>]) -> String {
            let names = names
                .iter()
                .map(|name| name.to_owned().unwrap_or_else(|| "nobody".to_string()))
                .collect::<Vec<_>>();
            format!("Hello {}", names.join(", "))
        }
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"
            type Query {
                helloWithOptionalItem(names: [String]!): String!
                helloWithOptionalItemRef(names: [String]!): String!
            }
            schema {
              query: Query
            }
            "#
        ),
    );
    let query = r#"
        query {
            helloWithOptionalItem(names: ["world", null, "rust"])
            helloWithOptionalItemRef(names: ["world", null, "rust"])
        }
    "#;

    let root = Query;
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));

    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();
    assert_eq!(
        data,
        serde_json::json!({
            "helloWithOptionalItem": "Hello world, nobody, rust",
            "helloWithOptionalItemRef": "Hello world, nobody, rust",
        }),
    );
}

#[tokio::test]
async fn test_schema_optional_item_and_arg() {
    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        fn hello_with_optional_item_and_arg(names: Option<Vec<Option<String>>>) -> String {
            match names {
                None => "Anybody there?".to_string(),
                Some(names) => {
                    let names = names
                        .into_iter()
                        .map(|name| name.unwrap_or_else(|| "nobody".to_string()))
                        .collect::<Vec<_>>();
                    format!("Hello {}", names.join(", "))
                }
            }
        }
        fn hello_with_optional_item_and_arg_ref(names: &Option<Vec<Option<String>>>) -> String {
            match names {
                None => "Anybody there?".to_string(),
                Some(names) => {
                    let names = names
                        .iter()
                        .map(|name| name.to_owned().unwrap_or_else(|| "nobody".to_string()))
                        .collect::<Vec<_>>();
                    format!("Hello {}", names.join(", "))
                }
            }
        }
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"
            type Query {
                helloWithOptionalItemAndArg(names: [String]): String!
                helloWithOptionalItemAndArgRef(names: [String]): String!
            }
            schema {
              query: Query
            }
            "#
        ),
    );
    let query = r#"
        query($names: [String]) {
            helloWithOptionalItemAndArg(names: $names)
            helloWithOptionalItemAndArgRef(names: $names)
        }
    "#;

    let root = Query;
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));

    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();
    assert_eq!(
        data,
        serde_json::json!({
            "helloWithOptionalItemAndArg": "Anybody there?",
            "helloWithOptionalItemAndArgRef": "Anybody there?",
        }),
    );
    let query = r#"
        query {
            helloWithOptionalItemAndArg(names: ["world", null, "rust"])
            helloWithOptionalItemAndArgRef(names: ["world", null, "rust"])
        }
    "#;

    let root = Query;
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));

    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();
    assert_eq!(
        data,
        serde_json::json!({
            "helloWithOptionalItemAndArg": "Hello world, nobody, rust",
            "helloWithOptionalItemAndArgRef": "Hello world, nobody, rust",
        }),
    );
}
