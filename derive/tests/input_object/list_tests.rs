use crate::schema_utils::normalize_schema;
use dynamic_graphql::dynamic::DynamicRequestExt;
use dynamic_graphql::{FieldValue, InputObject, Variables};
use dynamic_graphql::{ResolvedObject, ResolvedObjectFields};
use dynamic_graphql_derive::App;

#[tokio::test]
async fn test_option() {
    #[derive(InputObject)]
    struct ExampleInput {
        pub the_string: Option<String>,
    }

    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        async fn example(&self, input: Option<ExampleInput>) -> String {
            match input {
                None => "None".to_string(),
                Some(e) => match e.the_string {
                    None => "Some(None)".to_string(),
                    Some(value) => format!("Some({})", value),
                },
            }
        }
    }

    #[derive(App)]
    struct App(Query, ExampleInput);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"
                input ExampleInput {
                    theString: String
                }
                type Query {
                    example(input: ExampleInput): String!
                }
                schema {
                    query: Query
                }
            "#,
        ),
    );

    let query = r#"
        query($input: ExampleInput) {
            example(input: $input)
        }
    "#;

    let root = Query;
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;

    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "example": "None" }));

    let root = Query;
    let variables = serde_json::json!({ "input": { "theString": null } });
    let req = dynamic_graphql::Request::new(query)
        .variables(Variables::from_json(variables))
        .root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;

    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "example": "Some(None)" }));

    let root = Query;
    let variables = serde_json::json!({ "input": { "theString": "value" } });
    let req = dynamic_graphql::Request::new(query)
        .variables(Variables::from_json(variables))
        .root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;

    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "example": "Some(value)" }));
}

#[tokio::test]
async fn test_list() {
    #[derive(InputObject)]
    struct ExampleInput {
        pub names: Vec<String>,
    }

    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        async fn example(&self, input: ExampleInput) -> String {
            if input.names.is_empty() {
                "empty".to_string()
            } else {
                input.names.join(", ")
            }
        }
    }

    #[derive(App)]
    struct App(Query, ExampleInput);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"
                input ExampleInput {
                    names: [String!]!
                }
                type Query {
                    example(input: ExampleInput!): String!
                }
                schema {
                    query: Query
                }
            "#,
        ),
    );

    let query = r#"
        query($input: ExampleInput) {
            example(input: $input)
        }
    "#;

    let root = Query;
    let variables = serde_json::json!({ "input": { "names": [] } });
    let req = dynamic_graphql::Request::new(query)
        .variables(Variables::from_json(variables))
        .root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;

    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "example": "empty" }));

    let root = Query;
    let variables = serde_json::json!({ "input": { "names": ["world", "rust"] } });
    let req = dynamic_graphql::Request::new(query)
        .variables(Variables::from_json(variables))
        .root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;

    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "example": "world, rust" }));
}

#[tokio::test]
async fn test_optional_list() {
    #[derive(InputObject)]
    struct ExampleInput {
        pub names: Option<Vec<String>>,
    }

    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        async fn example(&self, input: ExampleInput) -> String {
            match input.names {
                None => "None".to_string(),
                Some(names) => {
                    if names.is_empty() {
                        "Some(empty)".to_string()
                    } else {
                        format!("Some({})", names.join(", "))
                    }
                }
            }
        }
    }

    #[derive(App)]
    struct App(Query, ExampleInput);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"
                input ExampleInput {
                    names: [String!]
                }
                type Query {
                    example(input: ExampleInput!): String!
                }
                schema {
                    query: Query
                }
            "#,
        ),
    );

    let query = r#"
        query($input: ExampleInput!) {
            example(input: $input)
        }
    "#;

    let root = Query;
    let variables = serde_json::json!({ "input": { "names": null } });
    let req = dynamic_graphql::Request::new(query)
        .variables(Variables::from_json(variables))
        .root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;

    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "example": "None" }));

    let root = Query;
    let variables = serde_json::json!({ "input": { "names": [] } });
    let req = dynamic_graphql::Request::new(query)
        .variables(Variables::from_json(variables))
        .root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;

    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "example": "Some(empty)" }));

    let root = Query;
    let variables = serde_json::json!({ "input": { "names": ["world", "rust"] } });
    let req = dynamic_graphql::Request::new(query)
        .variables(Variables::from_json(variables))
        .root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;

    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "example": "Some(world, rust)" }));
}

#[tokio::test]
async fn test_optional_items() {
    #[derive(InputObject)]
    struct ExampleInput {
        pub names: Vec<Option<String>>,
    }

    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        async fn example(&self, input: ExampleInput) -> String {
            if input.names.is_empty() {
                "empty".to_string()
            } else {
                input
                    .names
                    .into_iter()
                    .map(|name| name.unwrap_or_else(|| "None".to_string()))
                    .collect::<Vec<_>>()
                    .join(", ")
            }
        }
    }

    #[derive(App)]
    struct App(Query, ExampleInput);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"
                input ExampleInput {
                    names: [String]!
                }
                type Query {
                    example(input: ExampleInput!): String!
                }
                schema {
                    query: Query
                }
            "#,
        ),
    );

    let query = r#"
        query($input: ExampleInput!) {
            example(input: $input)
        }
    "#;

    let root = Query;
    let variables = serde_json::json!({ "input": { "names": [] } });
    let req = dynamic_graphql::Request::new(query)
        .variables(Variables::from_json(variables))
        .root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;

    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "example": "empty" }));

    let root = Query;
    let variables = serde_json::json!({ "input": { "names": ["world", null] } });
    let req = dynamic_graphql::Request::new(query)
        .variables(Variables::from_json(variables))
        .root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;

    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "example": "world, None" }));
}

#[tokio::test]
async fn test_optional_items_and_value() {
    #[derive(InputObject)]
    struct ExampleInput {
        pub names: Option<Vec<Option<String>>>,
    }

    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        async fn example(&self, input: ExampleInput) -> String {
            match input.names {
                None => "None".to_string(),
                Some(names) => {
                    if names.is_empty() {
                        "empty".to_string()
                    } else {
                        names
                            .into_iter()
                            .map(|name| name.unwrap_or_else(|| "None".to_string()))
                            .collect::<Vec<_>>()
                            .join(", ")
                    }
                }
            }
        }
    }

    #[derive(App)]
    struct App(Query, ExampleInput);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"
                input ExampleInput {
                    names: [String]
                }
                type Query {
                    example(input: ExampleInput!): String!
                }
                schema {
                    query: Query
                }
            "#,
        ),
    );

    let query = r#"
        query($input: ExampleInput!) {
            example(input: $input)
        }
    "#;

    let root = Query;
    let variables = serde_json::json!({ "input": { "names": null } });
    let req = dynamic_graphql::Request::new(query)
        .variables(Variables::from_json(variables))
        .root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;

    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "example": "None" }));

    let root = Query;
    let variables = serde_json::json!({ "input": { "names": [] } });
    let req = dynamic_graphql::Request::new(query)
        .variables(Variables::from_json(variables))
        .root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;

    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "example": "empty" }));

    let root = Query;
    let variables = serde_json::json!({ "input": { "names": ["world", null] } });
    let req = dynamic_graphql::Request::new(query)
        .variables(Variables::from_json(variables))
        .root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;

    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "example": "world, None" }));
}
