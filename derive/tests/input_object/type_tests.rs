use dynamic_graphql::dynamic::DynamicRequestExt;
use dynamic_graphql::App;
use dynamic_graphql::{FieldValue, InputObject, Variables};
use dynamic_graphql::{ResolvedObject, ResolvedObjectFields};

use crate::schema_utils::normalize_schema;

#[tokio::test]
async fn test_types() {
    #[allow(dead_code)]
    #[derive(InputObject, Debug)]
    struct ExampleInput {
        pub by_string: String,
        pub by_id: dynamic_graphql::ID,
        pub by_i8: i8,
        pub by_i16: i16,
        pub by_i32: i32,
        pub by_i64: i64,
        pub by_isize: isize,
        pub by_u8: u8,
        pub by_u16: u16,
        pub by_u32: u32,
        pub by_u64: u64,
        pub by_usize: usize,
        pub by_f32: f32,
        pub by_f64: f64,
        pub by_bool: bool,
    }

    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        async fn example(&self, input: ExampleInput) -> String {
            format!("{:#?}", input)
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
                    byString: String!
                    byId: ID!
                    byI8: Int!
                    byI16: Int!
                    byI32: Int!
                    byI64: Int!
                    byIsize: Int!
                    byU8: Int!
                    byU16: Int!
                    byU32: Int!
                    byU64: Int!
                    byUsize: Int!
                    byF32: Float!
                    byF64: Float!
                    byBool: Boolean!
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
    let variables = serde_json::json!({ "input": {
        "byString": "0",
        "byId": "0",
        "byI8": 0,
        "byI16": 0,
        "byI32": 0,
        "byI64": 0,
        "byIsize": 0,
        "byU8": 0,
        "byU16": 0,
        "byU32": 0,
        "byU64": 0,
        "byUsize": 0,
        "byF32": 0,
        "byF64": 0,
        "byBool": false,
    } });
    let req = dynamic_graphql::Request::new(query)
        .variables(Variables::from_json(variables))
        .root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;

    let data = res.data.into_json().unwrap();

    let example = data.get("example").unwrap().as_str().unwrap();

    assert_eq!(
        example,
        r#"ExampleInput {
    by_string: "0",
    by_id: ID(
        "0",
    ),
    by_i8: 0,
    by_i16: 0,
    by_i32: 0,
    by_i64: 0,
    by_isize: 0,
    by_u8: 0,
    by_u16: 0,
    by_u32: 0,
    by_u64: 0,
    by_usize: 0,
    by_f32: 0.0,
    by_f64: 0.0,
    by_bool: false,
}"#
    );
}

#[tokio::test]
async fn test_object_type() {
    #[derive(InputObject)]
    struct FooInput {
        pub value: String,
    }
    #[derive(InputObject)]
    struct ExampleInput {
        pub foo: FooInput,
    }

    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        async fn example(&self, input: ExampleInput) -> String {
            input.foo.value
        }
    }

    #[derive(App)]
    struct App(Query, ExampleInput, FooInput);

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
                  value: String!
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
            example(input: { foo: { value: "hello" } })
        }
    "#;

    let root = Query;
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;

    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "example": "hello" }));
}
