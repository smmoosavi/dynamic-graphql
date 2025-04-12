use dynamic_graphql::App;
use dynamic_graphql::FieldValue;
use dynamic_graphql::MaybeUndefined;
use dynamic_graphql::ResolvedObject;
use dynamic_graphql::ResolvedObjectFields;
use dynamic_graphql::Variables;
use dynamic_graphql::dynamic::DynamicRequestExt;
use dynamic_graphql_derive::OneOfInput;

use crate::schema_utils::normalize_schema;

#[tokio::test]
async fn test_maybe_undefined() {
    #[derive(OneOfInput)]
    enum ExampleInput {
        Str(MaybeUndefined<String>),
        Int(i32),
    }

    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        async fn example(&self, input: ExampleInput) -> String {
            match input {
                ExampleInput::Str(s) => match s {
                    MaybeUndefined::Undefined => "str: undefined".to_string(),
                    MaybeUndefined::Null => "str: null".to_string(),
                    MaybeUndefined::Value(value) => format!("str: value: {}", value),
                },
                ExampleInput::Int(i) => {
                    format!("int: {}", i)
                }
            }
        }
    }

    #[derive(App)]
    struct App(Query, ExampleInput);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r"
    input ExampleInput @oneOf {
      str: String
      int: Int
    }

    type Query {
      example(input: ExampleInput!): String!
    }

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @oneOf on INPUT_OBJECT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    ");

    let query = r#"
        query($input: ExampleInput) {
            example(input: $input)
        }
    "#;

    let root = Query;
    let variables = serde_json::json!({ "input": {  } });
    let req = dynamic_graphql::Request::new(query)
        .variables(Variables::from_json(variables))
        .root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;

    let err = res.errors;

    assert_eq!(err.len(), 1);
    insta::assert_snapshot!(err[0].message, @r#"Invalid value for argument "input", Oneof input objects requires have exactly one field"#);

    let root = Query;
    let variables = serde_json::json!({ "input": { "str": null } });
    let req = dynamic_graphql::Request::new(query)
        .variables(Variables::from_json(variables))
        .root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let err = res.errors;

    assert_eq!(err.len(), 1);
    insta::assert_snapshot!(err[0].message, @r#"Invalid value for argument "input", Oneof Input Objects require that exactly one field must be supplied and that field must not be null"#);

    let root = Query;
    let variables = serde_json::json!({ "input": { "str": "value", "int": 42 } });
    let req = dynamic_graphql::Request::new(query)
        .variables(Variables::from_json(variables))
        .root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let err = res.errors;

    assert_eq!(err.len(), 1);
    insta::assert_snapshot!(err[0].message, @r#"Invalid value for argument "input", Oneof input objects requires have exactly one field"#);

    let root = Query;
    let variables = serde_json::json!({ "input": { "str": "value" } });
    let req = dynamic_graphql::Request::new(query)
        .variables(Variables::from_json(variables))
        .root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;

    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "example": "str: value: value" }));

    let root = Query;
    let variables = serde_json::json!({ "input": { "int": 42 } });
    let req = dynamic_graphql::Request::new(query)
        .variables(Variables::from_json(variables))
        .root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;

    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "example": "int: 42" }));
}

#[tokio::test]
async fn test_option() {
    #[derive(OneOfInput)]
    enum ExampleInput {
        Str(Option<String>),
        Int(i32),
    }

    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        async fn example(&self, input: ExampleInput) -> String {
            match input {
                ExampleInput::Str(s) => match s {
                    None => "str: None".to_string(),
                    Some(value) => format!("str: some: {}", value),
                },
                ExampleInput::Int(i) => {
                    format!("int: {}", i)
                }
            }
        }
    }

    #[derive(App)]
    struct App(Query, ExampleInput);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r"
    input ExampleInput @oneOf {
      str: String
      int: Int
    }

    type Query {
      example(input: ExampleInput!): String!
    }

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @oneOf on INPUT_OBJECT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    ");

    let query = r#"
        query($input: ExampleInput) {
            example(input: $input)
        }
    "#;

    let root = Query;
    let variables = serde_json::json!({ "input": {  } });
    let req = dynamic_graphql::Request::new(query)
        .variables(Variables::from_json(variables))
        .root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;

    let err = res.errors;

    assert_eq!(err.len(), 1);
    insta::assert_snapshot!(err[0].message, @r#"Invalid value for argument "input", Oneof input objects requires have exactly one field"#);

    let root = Query;
    let variables = serde_json::json!({ "input": { "str": null } });
    let req = dynamic_graphql::Request::new(query)
        .variables(Variables::from_json(variables))
        .root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let err = res.errors;

    assert_eq!(err.len(), 1);
    insta::assert_snapshot!(err[0].message, @r#"Invalid value for argument "input", Oneof Input Objects require that exactly one field must be supplied and that field must not be null"#);

    let root = Query;
    let variables = serde_json::json!({ "input": { "str": "value", "int": 42 } });
    let req = dynamic_graphql::Request::new(query)
        .variables(Variables::from_json(variables))
        .root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let err = res.errors;

    assert_eq!(err.len(), 1);
    insta::assert_snapshot!(err[0].message, @r#"Invalid value for argument "input", Oneof input objects requires have exactly one field"#);

    let root = Query;
    let variables = serde_json::json!({ "input": { "str": "value" } });
    let req = dynamic_graphql::Request::new(query)
        .variables(Variables::from_json(variables))
        .root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;

    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "example": "str: some: value" }));

    let root = Query;
    let variables = serde_json::json!({ "input": { "int": 42 } });
    let req = dynamic_graphql::Request::new(query)
        .variables(Variables::from_json(variables))
        .root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;

    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "example": "int: 42" }));
}
