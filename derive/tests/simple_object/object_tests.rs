use crate::schema_utils::normalize_schema;
use dynamic_graphql::dynamic::DynamicRequestExt;
use dynamic_graphql::FieldValue;
use dynamic_graphql::SimpleObject;

#[test]
fn test_impl_object() {
    #[allow(dead_code)]
    #[derive(SimpleObject)]
    struct Example {
        pub string: String,
    }
    assert_eq!(<Example as dynamic_graphql::Object>::NAME, "Example");
}

#[test]
fn test_impl_object_with_name() {
    #[allow(dead_code)]
    #[derive(SimpleObject)]
    #[graphql(name = "Other")]
    struct Example {
        pub string: String,
    }
    assert_eq!(<Example as dynamic_graphql::Object>::NAME, "Other");
}

#[test]
fn test_impl_resolvers() {
    #[allow(dead_code)]
    #[derive(SimpleObject)]
    struct Example {
        pub string: String,
    }
    let example = Example {
        string: "Hello".to_string(),
    };
    let s = example.__resolve_string();
    assert_eq!(s, &"Hello".to_string());
}

#[test]
fn test_schema() {
    #[allow(dead_code)]
    #[derive(SimpleObject)]
    struct Query {
        pub string: String,
    }
    let registry = dynamic_graphql::Registry::new();
    let registry = registry.register::<Query>().set_root("Query");
    let schema = registry.create_schema().finish().unwrap();
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
    #[derive(SimpleObject)]
    #[graphql(name = "Other")]
    struct Query {
        pub string: String,
    }
    let registry = dynamic_graphql::Registry::new();
    let registry = registry.register::<Query>().set_root("Other");
    let schema = registry.create_schema().finish().unwrap();
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
    #[derive(SimpleObject)]
    struct Query {
        pub string: String,
        #[graphql(skip)]
        pub other: String,
    }
    let registry = dynamic_graphql::Registry::new();
    let registry = registry.register::<Query>().set_root("Query");
    let schema = registry.create_schema().finish().unwrap();
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
fn test_schema_with_rename_field() {
    #[allow(dead_code)]
    #[derive(SimpleObject)]
    struct Query {
        #[graphql(name = "other")]
        pub string: String,
    }
    let registry = dynamic_graphql::Registry::new();
    let registry = registry.register::<Query>().set_root("Query");
    let schema = registry.create_schema().finish().unwrap();
    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"
            type Query {
              other: String!
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
    #[derive(SimpleObject)]
    struct Query {
        pub string: String,
    }
    let registry = dynamic_graphql::Registry::new();
    let registry = registry.register::<Query>().set_root("Query");
    let schema = registry.create_schema().finish().unwrap();
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
    #[derive(SimpleObject)]
    struct Query {
        pub maybe_string: Option<String>,
    }
    let registry = dynamic_graphql::Registry::new();
    let registry = registry.register::<Query>().set_root("Query");
    let schema = registry.create_schema().finish().unwrap();

    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"
            type Query {
              maybeString: String
            }
            schema {
              query: Query
            }
            "#
        ),
    );

    let query = r#"
        query {
            maybeString
        }
    "#;

    let root = Query {
        maybe_string: Some("Hello".to_string()),
    };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "maybeString": "Hello" }));

    let root = Query { maybe_string: None };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "maybeString": null }));
}

#[test]
fn test_schema_with_doc() {
    /// this is the query object
    #[allow(dead_code)]
    #[derive(SimpleObject)]
    struct Query {
        /// this is the string field
        pub string: String,
    }
    let registry = dynamic_graphql::Registry::new();
    let registry = registry.register::<Query>().set_root("Query");
    let schema = registry.create_schema().finish().unwrap();
    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"
            """
              this is the query object
            """
            type Query {
              """
                this is the string field
              """
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
fn test_schema_with_deprecation() {
    #[allow(dead_code)]
    #[derive(SimpleObject)]
    struct Query {
        #[graphql(deprecation)]
        pub deprecated: String,
        #[graphql(deprecation = "this is the old one")]
        pub with_reason: String,
    }
    let registry = dynamic_graphql::Registry::new();
    let registry = registry.register::<Query>().set_root("Query");
    let schema = registry.create_schema().finish().unwrap();
    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"
                type Query {
                  deprecated: String! @deprecated
                  withReason: String! @deprecated(reason: "this is the old one")
                }

                schema {
                  query: Query
                }
            "#
        ),
    );
}

#[test]
fn test_rename_fields() {
    #[derive(SimpleObject)]
    #[graphql(rename_fields = "snake_case")]
    #[allow(non_camel_case_types)]
    struct the_query {
        pub the_string: String,
    }
    assert_eq!(<the_query as dynamic_graphql::Object>::NAME, "TheQuery");
    let registry = dynamic_graphql::Registry::new();
    let registry = registry.register::<the_query>().set_root("TheQuery");
    let schema = registry.create_schema().finish().unwrap();
    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"
            type TheQuery {
              the_string: String!
            }
            schema {
              query: TheQuery
            }
            "#
        ),
    );
}