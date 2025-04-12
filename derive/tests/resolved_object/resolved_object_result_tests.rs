use dynamic_graphql::dynamic::DynamicRequestExt;
use dynamic_graphql::App;
use dynamic_graphql::FieldValue;
use dynamic_graphql::ResolvedObject;
use dynamic_graphql::ResolvedObjectFields;
use dynamic_graphql::SimpleObject;

use crate::schema_utils::normalize_schema;

#[derive(thiserror::Error, Debug)]
enum MyError {
    #[error("Not found")]
    NotFound,
}

#[test]
fn test_schema() {
    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        fn string(&self) -> Result<String, MyError> {
            Ok("Hello".to_string())
        }
        fn maybe_string(&self) -> Result<Option<String>, MyError> {
            Ok(Some("Hello".to_string()))
        }
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r"
    type Query {
      string: String!
      maybeString: String
    }

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    ");
}

#[tokio::test]
async fn test_query() {
    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query {
        fail: bool,
    }

    #[ResolvedObjectFields]
    impl Query {
        async fn string(&self) -> Result<String, MyError> {
            if self.fail {
                Err(MyError::NotFound)
            } else {
                Ok("Hello".to_string())
            }
        }
        async fn maybe_string(&self) -> Result<Option<String>, MyError> {
            if self.fail {
                Err(MyError::NotFound)
            } else {
                Ok(Some("Hello".to_string()))
            }
        }
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();

    let query = r#"
        query {
            string
            maybeString
        }
    "#;
    let root = Query { fail: false };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    assert_eq!(
        res.data.into_json().unwrap(),
        serde_json::json!({
            "string": "Hello",
            "maybeString": "Hello",
        })
    );

    let root = Query { fail: true };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    assert_eq!(res.data.into_json().unwrap(), serde_json::json!(null));
    assert_eq!(res.errors.len(), 1);
    assert_eq!(res.errors[0].message, "Not found");
}

#[tokio::test]
async fn test_ref_query() {
    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query {
        fail: bool,
        value: String,
    }

    #[ResolvedObjectFields]
    impl Query {
        async fn string(&self) -> Result<&String, MyError> {
            if self.fail {
                Err(MyError::NotFound)
            } else {
                Ok(&self.value)
            }
        }
        async fn maybe_string(&self) -> Result<Option<&String>, MyError> {
            if self.fail {
                Err(MyError::NotFound)
            } else {
                Ok(Some(&self.value))
            }
        }
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();

    let query = r#"
        query {
            string
            maybeString
        }
    "#;
    let root = Query {
        fail: false,
        value: "Hello".to_string(),
    };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    assert_eq!(
        res.data.into_json().unwrap(),
        serde_json::json!({
            "string": "Hello",
            "maybeString": "Hello",
        })
    );

    let root = Query {
        fail: true,
        value: "Hello".to_string(),
    };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    assert_eq!(res.data.into_json().unwrap(), serde_json::json!(null));
    assert_eq!(res.errors.len(), 1);
    assert_eq!(res.errors[0].message, "Not found");
}

#[tokio::test]
async fn test_ref_object_query() {
    #[derive(SimpleObject)]
    struct Foo {
        value: String,
    }

    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query {
        fail: bool,
        value: Foo,
    }

    #[ResolvedObjectFields]
    impl Query {
        async fn foo(&self) -> Result<&Foo, MyError> {
            if self.fail {
                Err(MyError::NotFound)
            } else {
                Ok(&self.value)
            }
        }
        async fn maybe_foo(&self) -> Result<Option<&Foo>, MyError> {
            if self.fail {
                Err(MyError::NotFound)
            } else {
                Ok(Some(&self.value))
            }
        }
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();

    let query = r#"
        query {
            foo { value }
            maybeFoo { value }
        }
    "#;
    let root = Query {
        fail: false,
        value: Foo {
            value: "Hello".to_string(),
        },
    };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    assert_eq!(
        res.data.into_json().unwrap(),
        serde_json::json!({
            "foo": { "value": "Hello" },
            "maybeFoo": { "value": "Hello" },
        })
    );

    let root = Query {
        fail: true,
        value: Foo {
            value: "Hello".to_string(),
        },
    };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    assert_eq!(res.data.into_json().unwrap(), serde_json::json!(null));
    assert_eq!(res.errors.len(), 1);
    assert_eq!(res.errors[0].message, "Not found");
}
