use crate::schema_utils::normalize_schema;
use dynamic_graphql::dynamic::DynamicRequestExt;
use dynamic_graphql::FieldValue;
use dynamic_graphql::Object;

#[test]
fn test_impl_object() {
    #[allow(dead_code)]
    #[derive(Object)]
    struct Example {
        pub string: String,
    }
    assert_eq!(<Example as dynamic_graphql::Object>::NAME, "Example");
}

#[test]
fn test_impl_object_with_name() {
    #[allow(dead_code)]
    #[derive(Object)]
    #[graphql(name = "Other")]
    struct Example {
        pub string: String,
    }
    assert_eq!(<Example as dynamic_graphql::Object>::NAME, "Other");
}

#[test]
fn test_impl_resolvers() {
    #[allow(dead_code)]
    #[derive(Object)]
    struct Example {
        pub string: String,
    }
    let example = Example {
        string: "Hello".to_string(),
    };
    let s = example.resolve_string();
    assert_eq!(s, &"Hello".to_string());
}

#[test]
fn test_schema() {
    #[allow(dead_code)]
    #[derive(Object)]
    struct Query {
        pub string: String,
    }
    let registry = dynamic_graphql::Registry::new();
    let registry = registry.register::<Query>().set_root("Query");
    let schema = registry.create_schema();
    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"
            type Query {
              string: String!
            }
            schema {
              query: Query
            }
            "#
        ),
    );
}

#[test]
fn test_schema_with_rename() {
    #[allow(dead_code)]
    #[derive(Object)]
    #[graphql(name = "Other")]
    struct Query {
        pub string: String,
    }
    let registry = dynamic_graphql::Registry::new();
    let registry = registry.register::<Query>().set_root("Other");
    let schema = registry.create_schema();
    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"
            type Other {
              string: String!
            }
            schema {
              query: Other
            }
            "#
        ),
    );
}

#[test]
fn test_schema_with_skip() {
    #[allow(dead_code)]
    #[derive(Object)]
    struct Query {
        pub string: String,
        #[graphql(skip)]
        pub other: String,
    }
    let registry = dynamic_graphql::Registry::new();
    let registry = registry.register::<Query>().set_root("Query");
    let schema = registry.create_schema();
    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"
            type Query {
              string: String!
            }
            schema {
              query: Query
            }
            "#
        ),
    );
}

#[tokio::test]
async fn test_query() {
    #[allow(dead_code)]
    #[derive(Object)]
    struct Query {
        pub string: String,
    }
    let registry = dynamic_graphql::Registry::new();
    let registry = registry.register::<Query>().set_root("Query");
    let schema = registry.create_schema();
    let query = r#"
        query {
            string
        }
    "#;
    let root = Query {
        string: "Hello".to_string(),
    };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "string": "Hello" }));
}

#[tokio::test]
async fn test_optional() {
    #[allow(dead_code)]
    #[derive(Object)]
    struct Query {
        pub maybe_string: Option<String>,
    }
    let registry = dynamic_graphql::Registry::new();
    let registry = registry.register::<Query>().set_root("Query");
    let schema = registry.create_schema();

    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"
            type Query {
              maybe_string: String
            }
            schema {
              query: Query
            }
            "#
        ),
    );

    let query = r#"
        query {
            maybe_string
        }
    "#;

    let root = Query {
        maybe_string: Some("Hello".to_string()),
    };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "maybe_string": "Hello" }));

    let root = Query { maybe_string: None };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "maybe_string": null }));
}
