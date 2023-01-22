use dynamic_graphql::dynamic::DynamicRequestExt;
use dynamic_graphql::App;
use dynamic_graphql::FieldValue;
use dynamic_graphql::SimpleObject;

use crate::schema_utils::normalize_schema;

#[tokio::test]
async fn test_types() {
    #[allow(dead_code)]
    #[derive(SimpleObject, Default)]
    #[graphql(root)]
    struct Query {
        pub string: String,
        pub str: &'static str,
        pub id: dynamic_graphql::ID,
        pub i8: i8,
        pub i16: i16,
        pub i32: i32,
        pub i64: i64,
        pub isize: isize,
        pub u8: u8,
        pub u16: u16,
        pub u32: u32,
        pub u64: u64,
        pub usize: usize,
        pub f32: f32,
        pub f64: f64,
        pub bool: bool,
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
              str: String!
              id: ID!
              i8: Int!
              i16: Int!
              i32: Int!
              i64: Int!
              isize: Int!
              u8: Int!
              u16: Int!
              u32: Int!
              u64: Int!
              usize: Int!
              f32: Float!
              f64: Float!
              bool: Boolean!
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
            str
            id
            i8
            i16
            i32
            i64
            isize
            u8
            u16
            u32
            u64
            usize
            f32
            f64
            bool
        }
    "#;

    let root = Query::default();
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(
        data,
        serde_json::json!({
            "string": "",
            "str": "",
            "id": "",
            "i8": 0,
            "i16": 0,
            "i32": 0,
            "i64": 0,
            "isize": 0,
            "u8": 0,
            "u16": 0,
            "u32": 0,
            "u64": 0,
            "usize": 0,
            "f32": 0.0,
            "f64": 0.0,
            "bool": false,
        })
    );
}

#[tokio::test]
async fn test_optional_types() {
    #[allow(dead_code)]
    #[derive(SimpleObject, Default)]
    #[graphql(root)]
    struct Query {
        pub string: Option<String>,
        pub str: Option<&'static str>,
        pub id: Option<dynamic_graphql::ID>,
        pub i8: Option<i8>,
        pub i16: Option<i16>,
        pub i32: Option<i32>,
        pub i64: Option<i64>,
        pub isize: Option<isize>,
        pub u8: Option<u8>,
        pub u16: Option<u16>,
        pub u32: Option<u32>,
        pub u64: Option<u64>,
        pub usize: Option<usize>,
        pub f32: Option<f32>,
        pub f64: Option<f64>,
        pub bool: Option<bool>,
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
              string: String
              str: String
              id: ID
              i8: Int
              i16: Int
              i32: Int
              i64: Int
              isize: Int
              u8: Int
              u16: Int
              u32: Int
              u64: Int
              usize: Int
              f32: Float
              f64: Float
              bool: Boolean
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
            str
            id
            i8
            i16
            i32
            i64
            isize
            u8
            u16
            u32
            u64
            usize
            f32
            f64
            bool
        }
    "#;

    let root = Query {
        string: Some("the string".to_string()),
        str: Some("the str"),
        id: Some("the id".into()),
        i8: Some(42),
        i16: Some(42),
        i32: Some(42),
        i64: Some(42),
        isize: Some(42),
        u8: Some(42),
        u16: Some(42),
        u32: Some(42),
        u64: Some(42),
        usize: Some(42),
        f32: Some(42.5),
        f64: Some(42.5),
        bool: Some(true),
    };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(
        data,
        serde_json::json!({
            "string": "the string",
            "str": "the str",
            "id": "the id",
            "i8": 42,
            "i16": 42,
            "i32": 42,
            "i64": 42,
            "isize": 42,
            "u8": 42,
            "u16": 42,
            "u32": 42,
            "u64": 42,
            "usize": 42,
            "f32": 42.5,
            "f64": 42.5,
            "bool": true,
        })
    );

    let root = Query::default();
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(
        data,
        serde_json::json!({
            "string": null,
            "str": null,
            "id": null,
            "i8": null,
            "i16": null,
            "i32": null,
            "i64": null,
            "isize": null,
            "u8": null,
            "u16": null,
            "u32": null,
            "u64": null,
            "usize": null,
            "f32": null,
            "f64": null,
            "bool": null,
        })
    );
}

#[tokio::test]
async fn test_object_output() {
    #[derive(SimpleObject, Default)]
    struct Foo {
        pub value: String,
    }

    #[derive(SimpleObject, Default)]
    #[graphql(root)]
    struct Query {
        pub foo: Foo,
    }

    #[derive(App)]
    struct App(Query, Foo);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"
            type Foo {
              value: String!
            }

            type Query {
              foo: Foo!
            }

            schema {
              query: Query
            }
        "#
        ),
    );

    let query = r#"
        query {
            foo { value }
        }
    "#;

    let root = Query {
        foo: Foo {
            value: "the foo".to_string(),
        },
    };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "foo": { "value": "the foo" } }));
}
