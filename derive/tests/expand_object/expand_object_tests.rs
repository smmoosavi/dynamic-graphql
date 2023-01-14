use crate::schema_utils::normalize_schema;
use dynamic_graphql::dynamic::DynamicRequestExt;
use dynamic_graphql::{App, ExpandObject, ExpandObjectFields, FieldValue, Object, SimpleObject};

#[test]
fn test_impl_expand_object() {
    #[derive(SimpleObject)]
    struct Example {
        field: String,
    }

    #[derive(ExpandObject)]
    struct ExpandExample<'a>(&'a Example);

    assert_eq!(
        <<ExpandExample as ExpandObject>::Target as Object>::NAME,
        "Example"
    );
    assert_eq!(<ExpandExample as ExpandObject>::NAME, "ExpandExample");

    let example = Example {
        field: "field".to_string(),
    };
    let expand_example = ExpandExample(&example);
    assert_eq!(expand_example.parent().field, "field");
    let expand_example: ExpandExample = (&example).into();
    assert_eq!(expand_example.parent().field, "field");
}

#[test]
fn test_schema() {
    #[derive(SimpleObject)]
    struct Example {
        field: String,
    }

    #[derive(ExpandObject)]
    struct ExampleQuery<'a>(&'a Query);

    #[ExpandObjectFields]
    impl ExampleQuery<'_> {
        fn the_example(&self) -> Example {
            Example {
                field: "field".to_string(),
            }
        }
    }

    #[derive(App)]
    struct ExampleApp<'a>(Example, ExampleQuery<'a>);

    #[derive(SimpleObject)]
    struct Query {
        foo: String,
    }

    #[derive(App)]
    struct App<'a>(Query, ExampleApp<'a>);

    let registry = dynamic_graphql::Registry::new();
    let registry = registry.register::<App>().set_root("Query");
    let schema = registry.create_schema().finish().unwrap();
    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"

            type Example {
              field: String!
            }

            type Query {
              foo: String!
              theExample: Example!
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
    #[derive(SimpleObject)]
    struct Example {
        field: String,
    }

    #[derive(ExpandObject)]
    struct ExampleQuery<'a>(&'a Query);

    #[ExpandObjectFields]
    #[graphql(rename_fields = "snake_case")]
    impl ExampleQuery<'_> {
        fn the_example(&self) -> Example {
            Example {
                field: "field".to_string(),
            }
        }
        #[graphql(name = "other")]
        fn example(&self) -> Example {
            Example {
                field: "field".to_string(),
            }
        }
    }

    #[derive(App)]
    struct ExampleApp<'a>(Example, ExampleQuery<'a>);

    #[derive(SimpleObject)]
    struct Query {
        foo: String,
    }

    #[derive(App)]
    struct App<'a>(Query, ExampleApp<'a>);

    let registry = dynamic_graphql::Registry::new();
    let registry = registry.register::<App>().set_root("Query");
    let schema = registry.create_schema().finish().unwrap();
    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"

            type Example {
              field: String!
            }

            type Query {
              foo: String!
              the_example: Example!
              other: Example!
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
    #[derive(SimpleObject)]
    struct Example {
        field: String,
    }

    #[derive(ExpandObject)]
    struct ExampleQuery<'a>(&'a Query);

    #[ExpandObjectFields]
    impl ExampleQuery<'_> {
        fn example(&self) -> Example {
            Example {
                field: "field".to_string(),
            }
        }
    }

    #[derive(App)]
    struct ExampleApp<'a>(Example, ExampleQuery<'a>);

    #[derive(SimpleObject)]
    struct Query {
        foo: String,
    }

    #[derive(App)]
    struct App<'a>(Query, ExampleApp<'a>);

    let registry = dynamic_graphql::Registry::new();
    let registry = registry.register::<App>().set_root("Query");
    let schema = registry.create_schema().finish().unwrap();

    let query = r#"
        query {
            example {
                field
            }
        }
    "#;

    let root = Query {
        foo: "foo".to_string(),
    };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));

    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(
        data,
        serde_json::json!(
            {
                "example": {
                    "field": "field"
                }
            }
        )
    );
}
