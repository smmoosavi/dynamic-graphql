use dynamic_graphql::dynamic::DynamicRequestExt;
use dynamic_graphql::{App, Register, TypeName};
use dynamic_graphql::{FieldValue, Object};
use dynamic_graphql::{ResolvedObject, ResolvedObjectFields};
use dynamic_graphql_derive::{InputObject, SimpleObject};
use std::borrow::Cow;

use crate::schema_utils::normalize_schema;

#[test]
fn test_impl_resolved_object() {
    #[derive(ResolvedObject)]
    struct Example;

    impl Register for Example {}

    assert_eq!(<Example as Object>::get_object_type_name(), "Example");
}

#[test]
fn test_impl_resolved_object_with_name() {
    #[derive(ResolvedObject)]
    #[graphql(name = "Other")]
    struct Example;

    impl Register for Example {}

    assert_eq!(<Example as Object>::get_object_type_name(), "Other");
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
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        #[graphql(name = "other")]
        fn string(&self) -> String {
            "Hello".to_string()
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
fn test_schema_with_type_name() {
    #[derive(ResolvedObject)]
    #[graphql(root)]
    #[graphql(get_type_name)]
    struct Query;

    impl TypeName for Query {
        fn get_type_name() -> Cow<'static, str> {
            "Other".into()
        }
    }

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
            type Other {
              theString: String!
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
    #[graphql(root)]
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

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();

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
    #[graphql(root)]
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

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();

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
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        /// this is the string field
        fn string(&self) -> String {
            "Hello".to_string()
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
    #[graphql(root)]
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

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();

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
    #[graphql(root)]
    struct the_query;

    #[ResolvedObjectFields]
    #[graphql(rename_fields = "snake_case")]
    impl the_query {
        fn the_string(&self) -> String {
            "Hello".to_string()
        }
    }

    #[derive(App)]
    struct App(the_query);

    let schema = App::create_schema().finish().unwrap();

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
    #[graphql(root)]
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

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();

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

#[tokio::test]
async fn test_auto_register() {
    #[derive(SimpleObject)]
    struct Foo {
        value: String,
    }

    #[derive(SimpleObject)]
    struct Example {
        value: String,
    }
    #[derive(InputObject)]
    struct ExampleInput {
        value: String,
    }

    #[derive(ResolvedObject)]
    #[graphql(register(Foo))]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        fn example(&self, input: ExampleInput) -> Example {
            Example { value: input.value }
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

                type Example {
                  value: String!
                }

                input ExampleInput {
                  value: String!
                }

                type Foo {
                  value: String!
                }

                type Query {
                  example(input: ExampleInput!): Example!
                }

                schema {
                  query: Query
                }

            "#
        ),
    );
    let query = r#"
    query {
        example(input: {value: "Hello"}) {
            value
        }
    }
    "#;

    let root = Query;
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));

    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(
        data,
        serde_json::json!(
            {
                "example": {
                    "value": "Hello",
                }
            }
        )
    );
}

mod in_mod {
    use dynamic_graphql::App;
    use dynamic_graphql::ResolvedObject;

    use crate::schema_utils::normalize_schema;

    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    mod deep {
        use dynamic_graphql::ResolvedObjectFields;

        #[ResolvedObjectFields]
        impl super::Query {
            fn the_string(&self) -> String {
                "Hello".to_string()
            }
        }
    }

    #[test]
    fn test_schema() {
        #[derive(App)]
        struct App(Query);

        let schema = App::create_schema().finish().unwrap();

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
