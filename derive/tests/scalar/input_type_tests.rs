use dynamic_graphql::dynamic::DynamicRequestExt;
use dynamic_graphql::App;
use dynamic_graphql::FieldValue;
use dynamic_graphql::InputObject;
use dynamic_graphql::MaybeUndefined;
use dynamic_graphql::ResolvedObject;
use dynamic_graphql::ResolvedObjectFields;
use dynamic_graphql::Scalar;
use dynamic_graphql::ScalarValue;
use dynamic_graphql::Value;
use dynamic_graphql::Variables;

use crate::scalar::common::IP;
use crate::scalar::common::{MyString, StringValue};

#[tokio::test]
async fn test_query() {
    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        async fn value(value: MyString) -> String {
            value.0
        }
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();
    let root = Query;
    let query = r#"
        query {
            value(value: "foo")
        }
    "#;

    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();
    assert_eq!(data, serde_json::json!({ "value": "foo" }));
}

#[tokio::test]
async fn test_query_validation() {
    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        async fn value(value: IP) -> String {
            value.0.to_string()
        }
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();
    let root = Query;
    let query = r#"
        query {
            value(value: "192.168.10.10")
        }
    "#;

    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();
    assert_eq!(data, serde_json::json!({ "value": "192.168.10.10" }));

    let query = r#"
        query
        {
            value(value: "invalid")
        }
    "#;

    let root = Query;
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;

    assert_eq!(res.errors.len(), 1);
    assert_eq!(
        res.errors[0].message,
        r#"Invalid value for argument "value": Failed to parse "IP": invalid IP address syntax"#,
    );
}

#[tokio::test]
async fn test_query_validator() {
    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    fn validate_foo(value: &Value) -> bool {
        match value {
            Value::String(s) => s.len() <= 5,
            _ => false,
        }
    }

    #[derive(Scalar)]
    #[graphql(validator(validate_foo))]
    struct Foo(String);

    #[derive(InputObject)]
    struct FooInput {
        value: Foo,
    }

    impl ScalarValue for Foo {
        fn from_value(value: Value) -> dynamic_graphql::Result<Self>
        where
            Self: Sized,
        {
            StringValue::try_from(value).map(|v| Foo(v.0))
        }

        fn to_value(&self) -> Value {
            Value::String(self.0.clone())
        }
    }

    #[ResolvedObjectFields]
    impl Query {
        async fn value(value: Foo) -> String {
            value.0
        }
        async fn with_result(value: dynamic_graphql::Result<Foo>) -> String {
            match value {
                Ok(v) => v.0,
                Err(e) => format!("Err({})", e.message),
            }
        }
        async fn with_maybe_undefined(value: MaybeUndefined<Foo>) -> String {
            match value {
                MaybeUndefined::Value(v) => v.0,
                MaybeUndefined::Undefined => "undefined".to_string(),
                MaybeUndefined::Null => "null".to_string(),
            }
        }
        async fn with_input_object(input: FooInput) -> String {
            input.value.0
        }
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();
    let query = r#"
        query {
            value(value: "12345")
        }
    "#;

    let res = schema.execute(query).await;
    let data = res.data.into_json().unwrap();
    assert_eq!(data, serde_json::json!({ "value": "12345" }));

    let query = r#"
        query
        {
            value(value: "invalid")
        }
    "#;

    let res = schema.execute(query).await;

    assert_eq!(res.errors.len(), 1);
    assert_eq!(
        res.errors[0].message,
        r#"Invalid value for argument "value", expected type "Foo""#,
    );

    let query = r#"
        query
        {
            withResult(value: "invalid")
        }
    "#;

    let res = schema.execute(query).await;

    assert_eq!(res.errors.len(), 1);
    assert_eq!(
        res.errors[0].message,
        r#"Invalid value for argument "value", expected type "Foo""#,
    );

    let query = r#"
        query
        {
            withResult(value: "12345")
        }
    "#;

    let res = schema.execute(query).await;

    let data = res.data.into_json().unwrap();
    assert_eq!(data, serde_json::json!({ "withResult": "12345" }));

    let query = r#"
        query
        {
            withMaybeUndefined(value: "12345")
        }
    "#;

    let res = schema.execute(query).await;

    let data = res.data.into_json().unwrap();
    assert_eq!(data, serde_json::json!({ "withMaybeUndefined": "12345" }));

    let query = r#"
        query
        {
            withMaybeUndefined(value: null)
        }
    "#;

    let res = schema.execute(query).await;

    let data = res.data.into_json().unwrap();
    assert_eq!(data, serde_json::json!({ "withMaybeUndefined": "null" }));

    let query = r#"
        query
        {
            withMaybeUndefined
        }
    "#;

    let res = schema.execute(query).await;

    let data = res.data.into_json().unwrap();
    assert_eq!(
        data,
        serde_json::json!({ "withMaybeUndefined": "undefined" })
    );

    let query = r#"
        query
        {
            withMaybeUndefined(value: "invalid")
        }
    "#;

    let res = schema.execute(query).await;

    assert_eq!(res.errors.len(), 1);
    assert_eq!(
        res.errors[0].message,
        r#"Invalid value for argument "value", expected type "Foo""#,
    );

    let query = r#"
        query
        {
            withInputObject(input: { value: "12345" })
        }
    "#;

    let res = schema.execute(query).await;

    let data = res.data.into_json().unwrap();
    assert_eq!(data, serde_json::json!({ "withInputObject": "12345" }));

    let query = r#"
        query
        {
            withInputObject(input: { value: "invalid" })
        }
    "#;

    let res = schema.execute(query).await;

    assert_eq!(res.errors.len(), 1);
    assert_eq!(
        res.errors[0].message,
        r#"Invalid value for argument "input.value", expected type "Foo""#,
    );
}

#[tokio::test]
async fn test_query_validation_with_result() {
    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        async fn value(value: dynamic_graphql::Result<IP>) -> String {
            match value {
                Ok(ip) => format!("OK({})", ip.0),
                Err(e) => format!("Err({})", e.message),
            }
        }
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();
    let root = Query;
    let query = r#"
        query {
            value(value: "192.168.10.10")
        }
    "#;

    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();
    assert_eq!(data, serde_json::json!({ "value": "OK(192.168.10.10)" }));

    let query = r#"
        query
        {
            value(value: "invalid")
        }
    "#;

    let root = Query;
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(
        data,
        serde_json::json!({ "value": r#"Err(Failed to parse "IP": invalid IP address syntax)"# })
    );
}

#[tokio::test]
async fn test_query_input_object() {
    #[derive(InputObject)]
    struct MyInput {
        value: MyString,
    }

    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        async fn example(input: MyInput) -> String {
            input.value.0
        }
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();
    let root = Query;
    let query = r#"
        query($input: MyInput!) {
            example(input: $input)
        }
    "#;

    let variables = serde_json::json!({
        "input": {
            "value": "foo"
        }
    });
    let req = dynamic_graphql::Request::new(query)
        .variables(Variables::from_json(variables))
        .root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();
    assert_eq!(data, serde_json::json!({ "example": "foo" }));
}

#[tokio::test]
async fn test_query_option() {
    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        async fn value(input: Option<MyString>) -> String {
            match input {
                None => "None".to_string(),
                Some(s) => format!("Some({})", s.0),
            }
        }
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();
    let root = Query;
    let query = r#"
        query {
            value(input: "foo")
        }
    "#;

    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();
    assert_eq!(data, serde_json::json!({ "value": "Some(foo)" }));

    let root = Query;
    let query = r#"
        query {
            value
        }
    "#;

    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();
    assert_eq!(data, serde_json::json!({ "value": "None" }));
}
