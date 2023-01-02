use dynamic_graphql::dynamic::DynamicRequestExt;
use dynamic_graphql::FieldValue;
use dynamic_graphql_derive::Object;
use schema_utils::normalize_schema;

#[tokio::test]
async fn test_list() {
    #[allow(dead_code)]
    #[derive(Object)]
    struct Query {
        pub strings: Vec<String>,
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
              strings: [String!]!
            }
            schema {
              query: Query
            }
            "#
        ),
    );

    let query = r#"
        query {
            strings
        }
    "#;

    let root = Query {
        strings: vec!["Hello".to_string()],
    };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "strings": [ "Hello" ] }));

    let root = Query { strings: vec![] };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "strings": [] }));
}

#[tokio::test]
#[ignore]
async fn test_optional_list() {
    #[allow(dead_code)]
    #[derive(Object)]
    struct Query {
        pub maybe_list_of_strings: Option<Vec<String>>,
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
              maybe_list_of_strings: [String!]
            }
            schema {
              query: Query
            }
            "#
        ),
    );

    let query = r#"
        query {
            maybe_list_of_strings
        }
    "#;

    let root = Query {
        maybe_list_of_strings: Some(vec!["Hello".to_string()]),
    };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(
        data,
        serde_json::json!({ "maybe_list_of_strings": [ "Hello" ] })
    );

    let root = Query {
        maybe_list_of_strings: None,
    };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "maybe_list_of_strings": null }));
}

#[tokio::test]
async fn test_list_of_optional() {
    #[allow(dead_code)]
    #[derive(Object)]
    struct Query {
        pub list_of_maybe_strings: Vec<Option<String>>,
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
              list_of_maybe_strings: [String]!
            }
            schema {
              query: Query
            }
            "#
        ),
    );

    let query = r#"
        query {
            list_of_maybe_strings
        }
    "#;

    let root = Query {
        list_of_maybe_strings: vec![Some("Hello".to_string()), None],
    };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(
        data,
        serde_json::json!({ "list_of_maybe_strings": [ "Hello", null ] })
    );
}

#[tokio::test]
#[ignore]
async fn test_optional_list_of_optional() {
    #[allow(dead_code)]
    #[derive(Object)]
    struct Query {
        pub maybe_list_of_maybe_strings: Option<Vec<Option<String>>>,
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
              maybe_list_of_maybe_strings: [String]
            }
            schema {
              query: Query
            }
            "#
        ),
    );

    let query = r#"
        query {
            maybe_list_of_maybe_strings
        }
    "#;

    let root = Query {
        maybe_list_of_maybe_strings: Some(vec![Some("Hello".to_string()), None]),
    };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(
        data,
        serde_json::json!({ "maybe_list_of_maybe_strings": [ "Hello", null ] })
    );

    let root = Query {
        maybe_list_of_maybe_strings: None,
    };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(
        data,
        serde_json::json!({ "maybe_list_of_maybe_strings": null })
    );
}
