use crate::scalar::json::{JsonObject, JsonValue, KeyValue};
use crate::schema_utils::normalize_schema;
use dynamic_graphql_derive::{App, ResolvedObject, ResolvedObjectFields};

#[tokio::test]
async fn json_input_output() {
    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        async fn value(value: JsonValue) -> JsonValue {
            value
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
    scalar JsonValue

    type Query {
      value(value: JsonValue!): JsonValue!
    }

    schema {
      query: Query
    }
"#
        )
    );

    let query = r#"
        query {
            string: value(value: "foo")
            bool: value(value: true)
            int: value(value: 42)
            float: value(value: 8.2)
            list: value(value: [1, 2, 3])
            object: value(value: { foo: "bar" })
            nested: value(value: { foo: { bar: ["baz"] } })
        }
    "#;

    let req = dynamic_graphql::Request::new(query);
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();
    assert_eq!(
        data,
        serde_json::json!({
            "string": "foo",
            "bool": true,
            "int": 42,
            "float": 8.2,
            "list": [1, 2, 3],
            "object": { "foo": "bar" },
            "nested": { "foo": { "bar": ["baz"] } },
        })
    );
}

#[tokio::test]
async fn json_object_test() {
    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        async fn value(value: JsonObject) -> JsonObject {
            value
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
    scalar JsonObject

    type Query {
      value(value: JsonObject!): JsonObject!
    }

    schema {
      query: Query
    }
"#
        )
    );

    let query = r#"
        query {
            number: value(value: { foo: 42 })
            object: value(value: { foo: "bar" })
            nested: value(value: { foo: { bar: ["baz"] } })
        }
    "#;

    let req = dynamic_graphql::Request::new(query);
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();
    assert_eq!(
        data,
        serde_json::json!({
            "number": { "foo": 42 },
            "object": { "foo": "bar" },
            "nested": { "foo": { "bar": ["baz"] } },
        })
    );

    let req = dynamic_graphql::Request::new(
        r#"
        query {
            value(value: 42)
        }
    "#,
    );

    let res = schema.execute(req).await;
    assert_eq!(
        res.errors[0].message,
        r#"Invalid value for argument "value": Failed to parse "JsonObject": Expected an object value"#
    );
}

#[tokio::test]
async fn json_key_value() {
    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        async fn value(value: KeyValue) -> KeyValue {
            value
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
    scalar KeyValue

    type Query {
      value(value: KeyValue!): KeyValue!
    }

    schema {
      query: Query
    }
"#
        )
    );

    let query = r#"
        query {
            number: value(value: { foo: "42" })
            object: value(value: { foo: "bar", baz: "qux" })
        }
    "#;

    let req = dynamic_graphql::Request::new(query);
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();
    assert_eq!(
        data,
        serde_json::json!({
            "number": { "foo": "42" },
            "object": { "foo": "bar", "baz": "qux" },
        })
    );

    let req = dynamic_graphql::Request::new(
        r#"
        query {
            value(value: 42)
        }
    "#,
    );

    let res = schema.execute(req).await;
    assert_eq!(
        res.errors[0].message,
        r#"Invalid value for argument "value": Failed to parse "KeyValue": Expected an object value"#
    );

    let req = dynamic_graphql::Request::new(
        r#"
        query {
            value(value: { foo: 42 })
        }
    "#,
    );

    let res = schema.execute(req).await;
    assert_eq!(
        res.errors[0].message,
        r#"Invalid value for argument "value": Failed to parse "KeyValue": Expected a string value"#
    );
}
