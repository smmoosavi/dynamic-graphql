use dynamic_graphql::dynamic::DynamicRequestExt;
use dynamic_graphql::App;
use dynamic_graphql::{FieldValue, InputObject};
use dynamic_graphql::{ResolvedObject, ResolvedObjectFields};

use crate::schema_utils::normalize_schema;

#[test]
fn test_impl_object() {
    #[allow(dead_code)]
    #[derive(InputObject)]
    struct ExampleInput {
        pub string: String,
    }
    assert_eq!(
        <ExampleInput as dynamic_graphql::InputObject>::NAME,
        "ExampleInput"
    );
}

#[test]
fn test_impl_object_with_name() {
    #[allow(dead_code)]
    #[derive(InputObject)]
    #[graphql(name = "OtherInput")]
    struct ExampleInput {
        pub string: String,
    }
    assert_eq!(
        <ExampleInput as dynamic_graphql::InputObject>::NAME,
        "OtherInput"
    );
}

#[tokio::test]
async fn test_schema() {
    #[derive(InputObject)]
    struct ExampleInput {
        pub the_string: String,
    }

    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        async fn example(&self, input: ExampleInput) -> String {
            input.the_string
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
                    theString: String!
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
        query {
            example(input: { theString: "hello" })
        }
    "#;

    let root = Query;
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;

    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "example": "hello" }));
}

#[tokio::test]
async fn test_schema_with_rename() {
    #[derive(InputObject)]
    #[graphql(name = "OtherInput")]
    struct ExampleInput {
        #[graphql(name = "other")]
        pub string: String,
    }

    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        async fn example(&self, input: ExampleInput) -> String {
            input.string
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
                input OtherInput {
                    other: String!
                }
                type Query {
                    example(input: OtherInput!): String!
                }
                schema {
                    query: Query
                }
            "#,
        ),
    );
    let query = r#"
        query {
            example(input: { other: "hello" })
        }
    "#;

    let root = Query;
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;

    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "example": "hello" }));
}

#[tokio::test]
async fn test_schema_with_skip() {
    #[allow(dead_code)]
    #[derive(InputObject)]
    struct ExampleInput {
        pub string: String,
        #[graphql(skip)]
        pub other: String,
    }

    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        async fn example(&self, input: ExampleInput) -> String {
            input.string
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
                    string: String!
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
        query {
            example(input: { string: "hello" })
        }
    "#;

    let root = Query;
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;

    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "example": "hello" }));
}
#[test]
fn test_schema_with_doc() {
    /// the example input object
    #[allow(dead_code)]
    #[derive(InputObject)]
    struct ExampleInput {
        /// the string input field
        pub string: String,
    }

    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        async fn example(&self, input: ExampleInput) -> String {
            input.string
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
                """
                the example input object
                """
                input ExampleInput {
                    """
                    the string input field
                    """
                    string: String!
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
}

#[tokio::test]
async fn test_rename_fields() {
    #[derive(InputObject)]
    #[graphql(rename_fields = "snake_case")]
    struct ExampleInput {
        pub the_string: String,
    }

    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        async fn example(&self, input: ExampleInput) -> String {
            input.the_string
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
                    the_string: String!
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
        query {
            example(input: { the_string: "hello" })
        }
    "#;

    let root = Query;
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;

    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "example": "hello" }));
}

mod in_mod {
    use dynamic_graphql::dynamic::DynamicRequestExt;
    use dynamic_graphql::{App, FieldValue, ResolvedObject, ResolvedObjectFields};

    use crate::schema_utils::normalize_schema;

    mod foo {
        use dynamic_graphql::InputObject;

        #[derive(InputObject)]
        pub struct FooInput {
            pub string: String,
        }
    }
    mod example {
        use dynamic_graphql::InputObject;

        #[derive(InputObject)]
        pub struct ExampleInput {
            pub foo: super::foo::FooInput,
        }
    }
    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        async fn example(&self, input: example::ExampleInput) -> String {
            input.foo.string
        }
    }

    #[derive(App)]
    struct App(Query, foo::FooInput, example::ExampleInput);

    #[tokio::test]
    async fn test_schema() {
        let schema = App::create_schema().finish().unwrap();

        let sdl = schema.sdl();

        assert_eq!(
            normalize_schema(&sdl),
            normalize_schema(
                r#"
                    input ExampleInput {
                        foo: FooInput!
                    }
                    input FooInput {
                        string: String!
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
            query {
                example(input: { foo: { string: "hello" } })
            }
        "#;
        let root = Query;
        let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));

        let res = schema.execute(req).await;
        let data = res.data.into_json().unwrap();

        assert_eq!(data, serde_json::json!({ "example": "hello" }));
    }
}
