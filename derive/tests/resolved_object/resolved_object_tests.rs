use crate::schema_utils::normalize_schema;
use dynamic_graphql::dynamic::DynamicRequestExt;
use dynamic_graphql::{FieldValue, Object};
use dynamic_graphql::{ResolvedObject, ResolvedObjectFields};

#[test]
fn test_impl_resolved_object() {
    #[derive(ResolvedObject)]
    struct Example;

    assert_eq!(<Example as Object>::NAME, "Example");
}

#[test]
fn test_impl_resolved_object_with_name() {
    #[derive(ResolvedObject)]
    #[graphql(name = "Other")]
    struct Example;

    assert_eq!(<Example as Object>::NAME, "Other");
}

#[test]
fn test_schema() {
    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        fn the_string(&self) -> String {
            "Hello".to_string()
        }
    }

    let registry = dynamic_graphql::Registry::new();
    let registry = registry.register::<Query>();
    let schema = registry.create_schema().finish().unwrap();
    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"
            type Query {
              theString: String!
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
    #[derive(ResolvedObject)]
    #[graphql(name = "Other")]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        #[graphql(name = "other")]
        fn string(&self) -> String {
            "Hello".to_string()
        }
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
              other: String!
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
    #[derive(ResolvedObject)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        fn string(&self) -> String {
            "Hello".to_string()
        }
        #[graphql(skip)]
        #[allow(dead_code)]
        fn other(&self) -> String {
            "Hello".to_string()
        }
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

#[tokio::test]
async fn test_query() {
    #[derive(ResolvedObject)]
    struct Query {
        value: String,
    }

    #[ResolvedObjectFields]
    impl Query {
        fn string(&self) -> String {
            "Hello".to_string()
        }
        fn value(&self) -> &String {
            &self.value
        }
        fn other(&self) -> &str {
            &self.value
        }
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
              value: String!
              other: String!
            }
            schema {
              query: Query
            }
            "#
        ),
    );
    let query = r#"
    query {
      string
      value
      other
    }
    "#;

    let root = Query {
        value: "Hello".to_string(),
    };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));

    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(
        data,
        serde_json::json!(
            {
                "string": "Hello",
                "value": "Hello",
                "other": "Hello",
            }
        )
    );
}

#[test]
fn test_schema_with_doc() {
    /// this is the query object
    #[derive(ResolvedObject)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        /// this is the string field
        fn string(&self) -> String {
            "Hello".to_string()
        }
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
    #[derive(ResolvedObject)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        #[graphql(deprecation)]
        fn deprecated(&self) -> String {
            "Hello".to_string()
        }
        #[graphql(deprecation = "this is the old one")]
        fn with_reason(&self) -> String {
            "Hello".to_string()
        }
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
    #[derive(ResolvedObject)]
    #[allow(non_camel_case_types)]
    struct the_query;

    #[ResolvedObjectFields]
    #[graphql(rename_fields = "snake_case")]
    impl the_query {
        fn the_string(&self) -> String {
            "Hello".to_string()
        }
    }

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

#[tokio::test]
async fn test_async_query() {
    #[derive(ResolvedObject)]
    struct Query {
        value: String,
    }

    #[ResolvedObjectFields]
    impl Query {
        async fn string(&self) -> String {
            "Hello".to_string()
        }
        async fn value(&self) -> &String {
            &self.value
        }
        async fn other(&self) -> &str {
            &self.value
        }
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
              value: String!
              other: String!
            }
            schema {
              query: Query
            }
            "#
        ),
    );
    let query = r#"
    query {
      string
      value
      other
    }
    "#;

    let root = Query {
        value: "Hello".to_string(),
    };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));

    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(
        data,
        serde_json::json!(
            {
                "string": "Hello",
                "value": "Hello",
                "other": "Hello",
            }
        )
    );
}

mod in_mod {
    use crate::schema_utils::normalize_schema;
    use dynamic_graphql::{ResolvedObject, ResolvedObjectFields};

    #[derive(ResolvedObject)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        fn the_string(&self) -> String {
            "Hello".to_string()
        }
    }

    #[test]
    fn test_schema() {
        let registry = dynamic_graphql::Registry::new();
        let registry = registry.register::<Query>().set_root("Query");
        let schema = registry.create_schema().finish().unwrap();
        let sdl = schema.sdl();
        assert_eq!(
            normalize_schema(&sdl),
            normalize_schema(
                r#"
                    type Query {
                      theString: String!
                    }
                    schema {
                      query: Query
                    }
                "#
            ),
        );
    }
}
