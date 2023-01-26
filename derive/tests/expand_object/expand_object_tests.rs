use dynamic_graphql::dynamic::DynamicRequestExt;
use dynamic_graphql::{
    App, ExpandObject, ExpandObjectFields, FieldValue, Object, ParentType, SimpleObject,
};
use dynamic_graphql_derive::InputObject;

use crate::schema_utils::normalize_schema;

#[test]
fn test_impl_expand_object() {
    #[derive(SimpleObject)]
    struct Example {
        field: String,
    }

    #[derive(ExpandObject)]
    struct ExpandExample<'a>(&'a Example);

    assert_eq!(
        <<ExpandExample as ParentType>::Type as Object>::NAME,
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
    struct ExampleApp(Example, ExampleQuery<'static>);

    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        foo: String,
    }

    #[derive(App)]
    struct App(Query, ExampleApp);

    let schema = App::create_schema().finish().unwrap();
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
    struct ExampleApp(Example, ExampleQuery<'static>);

    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        foo: String,
    }

    #[derive(App)]
    struct App(Query, ExampleApp);

    let schema = App::create_schema().finish().unwrap();
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

#[test]
fn test_schema_with_skip() {
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
        #[graphql(skip)]
        #[allow(dead_code)]
        fn other(&self) -> Example {
            Example {
                field: "field".to_string(),
            }
        }
    }

    #[derive(App)]
    struct ExampleApp(Example, ExampleQuery<'static>);

    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        foo: String,
    }

    #[derive(App)]
    struct App(Query, ExampleApp);

    let schema = App::create_schema().finish().unwrap();
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
fn test_schema_with_deprecation() {
    #[derive(SimpleObject)]
    struct Example {
        field: String,
    }

    #[derive(ExpandObject)]
    struct ExampleQuery<'a>(&'a Query);

    #[ExpandObjectFields]
    impl ExampleQuery<'_> {
        #[graphql(deprecation)]
        fn example(&self) -> Example {
            Example {
                field: "field".to_string(),
            }
        }
        #[graphql(deprecation = "this is the old one")]
        fn old(&self) -> Example {
            Example {
                field: "field".to_string(),
            }
        }
    }

    #[derive(App)]
    struct ExampleApp(Example, ExampleQuery<'static>);

    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        foo: String,
    }

    #[derive(App)]
    struct App(Query, ExampleApp);

    let schema = App::create_schema().finish().unwrap();
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
              example: Example! @deprecated
              old: Example! @deprecated(reason: "this is the old one")
            }

            schema {
              query: Query
            }
            "#
        ),
    );
}

#[test]
fn test_schema_with_description() {
    #[derive(SimpleObject)]
    struct Example {
        field: String,
    }

    #[derive(ExpandObject)]
    struct ExampleQuery<'a>(&'a Query);

    #[ExpandObjectFields]
    impl ExampleQuery<'_> {
        /// this is the example
        fn the_example(&self) -> Example {
            Example {
                field: "field".to_string(),
            }
        }
    }

    #[derive(App)]
    struct ExampleApp(Example, ExampleQuery<'static>);

    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        foo: String,
    }

    #[derive(App)]
    struct App(Query, ExampleApp);

    let schema = App::create_schema().finish().unwrap();
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
              """
                this is the example
              """
              theExample: Example!
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
    struct ExampleApp(Example, ExampleQuery<'static>);

    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        foo: String,
    }

    #[derive(App)]
    struct App(Query, ExampleApp);

    let schema = App::create_schema().finish().unwrap();

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

#[tokio::test]
async fn test_auto_register() {
    #[allow(dead_code)]
    #[derive(InputObject)]
    struct ExampleInput {
        field: String,
    }

    #[derive(SimpleObject)]
    struct Example {
        field: String,
    }

    #[derive(ExpandObject)]
    struct ExampleQuery<'a>(&'a Query);

    #[ExpandObjectFields]
    impl ExampleQuery<'_> {
        fn example(&self, _input: Option<ExampleInput>) -> Example {
            Example {
                field: "field".to_string(),
            }
        }
    }

    #[derive(App)]
    struct ExampleApp(ExampleQuery<'static>);

    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        foo: String,
    }

    #[derive(App)]
    struct App(Query, ExampleApp);

    let schema = App::create_schema().finish().unwrap();
    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"
                type Example {
                  field: String!
                }

                input ExampleInput {
                  field: String!
                }

                type Query {
                  foo: String!
                  example(input: ExampleInput): Example!
                }

                schema {
                  query: Query
                }
            "#
        ),
    );

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

mod test_in_mod {
    use dynamic_graphql::dynamic::DynamicRequestExt;
    use dynamic_graphql::FieldValue;

    pub mod query {
        use dynamic_graphql::SimpleObject;

        #[derive(SimpleObject)]
        #[graphql(root)]
        pub struct Query;
    }

    pub mod foo {
        use dynamic_graphql::ExpandObject;

        #[derive(ExpandObject)]
        pub struct FooQuery<'a>(&'a super::query::Query);

        mod deep {
            use dynamic_graphql::ExpandObjectFields;

            #[ExpandObjectFields]
            impl super::FooQuery<'_> {
                fn foo(&self) -> String {
                    "foo".to_string()
                }
            }
        }
    }

    #[derive(dynamic_graphql::App)]
    pub struct App(query::Query, foo::FooQuery<'static>);

    #[tokio::test]
    async fn test_query() {
        let schema = App::create_schema().finish().unwrap();

        let query = r#"
            query {
                foo
            }
        "#;

        let root = query::Query;
        let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));

        let res = schema.execute(req).await;
        let data = res.data.into_json().unwrap();

        assert_eq!(
            data,
            serde_json::json!(
                {
                    "foo": "foo"
                }
            )
        );
    }
}
