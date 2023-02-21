use dynamic_graphql::dynamic::DynamicRequestExt;
use dynamic_graphql::App;
use dynamic_graphql::FieldValue;
use dynamic_graphql::InputObject;
use dynamic_graphql::ResolvedObject;
use dynamic_graphql::ResolvedObjectFields;
use dynamic_graphql::Variables;

use crate::scalar::common::MyString;
use crate::scalar::common::IP;

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
