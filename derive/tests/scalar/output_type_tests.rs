use dynamic_graphql::App;
use dynamic_graphql::FieldValue;
use dynamic_graphql::Scalar;
use dynamic_graphql::ScalarValue;
use dynamic_graphql::SimpleObject;
use dynamic_graphql::dynamic::DynamicRequestExt;
use dynamic_graphql::value;

use crate::scalar::common::IP;
use crate::scalar::common::MyString;

#[tokio::test]
async fn test_query() {
    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        value: MyString,
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();
    let root = Query {
        value: MyString("foo".to_string()),
    };
    let query = r#"
        query {
            value
        }
    "#;

    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();
    assert_eq!(data, serde_json::json!({ "value": "foo" }));
}

#[tokio::test]
async fn test_query_with_ip() {
    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        value: IP,
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();
    let root = Query {
        value: IP("192.168.10.10".parse().unwrap()),
    };
    let query = r#"
        query {
            value
        }
    "#;

    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();
    assert_eq!(data, serde_json::json!({ "value": "192.168.10.10" }));
}

#[tokio::test]
async fn test_query_with_object_output() {
    #[derive(Scalar)]
    struct FooValue(String);

    impl ScalarValue for FooValue {
        fn from_value(_value: dynamic_graphql::Value) -> dynamic_graphql::Result<Self>
        where
            Self: Sized,
        {
            unimplemented!()
        }

        fn to_value(&self) -> dynamic_graphql::Value {
            let value = self.0.clone();
            value! ({
                "foo": value,
            })
        }
    }

    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        value: FooValue,
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();
    let root = Query {
        value: FooValue("1000".to_string()),
    };
    let query = r#"
        query {
            value
        }
    "#;

    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();
    assert_eq!(data, serde_json::json!({ "value": { "foo": "1000" } }));
}

#[tokio::test]
async fn test_query_with_option() {
    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        value: Option<MyString>,
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();
    let root = Query {
        value: Some(MyString("foo".to_string())),
    };
    let query = r#"
        query {
            value
        }
    "#;

    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();
    assert_eq!(data, serde_json::json!({ "value": "foo" }));

    let empty_root = Query { value: None };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(empty_root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();
    assert_eq!(data, serde_json::json!({ "value": null }));
}

#[tokio::test]
async fn test_query_with_vector() {
    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        value: Vec<MyString>,
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();
    let root = Query {
        value: vec![MyString("foo".to_string())],
    };
    let query = r#"
        query {
            value
        }
    "#;

    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();
    assert_eq!(data, serde_json::json!({ "value": ["foo"] }));

    let empty_root = Query { value: vec![] };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(empty_root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();
    assert_eq!(data, serde_json::json!({ "value": [] }));
}
