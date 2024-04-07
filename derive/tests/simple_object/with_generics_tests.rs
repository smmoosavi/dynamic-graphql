use std::borrow::Cow;

use dynamic_graphql::dynamic::DynamicRequestExt;
use dynamic_graphql::internal::OutputTypeName;
use dynamic_graphql::internal::ResolveRef;
use dynamic_graphql::internal::TypeName;
use dynamic_graphql::App;
use dynamic_graphql::FieldValue;
use dynamic_graphql::SimpleObject;

use crate::schema_utils::normalize_schema;

#[tokio::test]
async fn test_query_simple_generic() {
    #[derive(SimpleObject)]
    struct Foo {
        pub value: String,
    }

    #[allow(dead_code)]
    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query<F>
    where
        F: OutputTypeName + 'static,
        F: Send + Sync,
        F: for<'a> ResolveRef<'a>,
    {
        pub field: F,
    }

    #[derive(App)]
    struct App(Query<Foo>, Foo);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @"");

    let query = r#"
        query {
            field {
                value
            }
        }
    "#;
    let root = Query {
        field: Foo {
            value: "foo".to_string(),
        },
    };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "field": { "value": "foo" } }));
}

#[tokio::test]
async fn test_query_generic_with_type_name() {
    #[derive(SimpleObject)]
    struct Foo {
        pub foo: String,
    }

    #[derive(SimpleObject)]
    struct Bar {
        pub bar: String,
    }

    #[derive(SimpleObject)]
    #[graphql(get_type_name)]
    struct Box<T>
    where
        T: OutputTypeName + 'static,
        T: Send + Sync,
        T: for<'a> ResolveRef<'a>,
    {
        pub inner: T,
    }

    impl<T> TypeName for Box<T>
    where
        T: OutputTypeName + 'static,
        T: Send + Sync,
        T: for<'a> ResolveRef<'a>,
    {
        fn get_type_name() -> Cow<'static, str> {
            format!("Box{}", T::get_type_name()).into()
        }
    }

    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        foo: Box<Foo>,
        bar: Box<Bar>,
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();
    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @"");

    let query = r#"
        query {
            foo {
                inner {
                    foo
                }
            }
            bar {
                inner {
                    bar
                }
            }
        }
    "#;
    let root = Query {
        foo: Box {
            inner: Foo {
                foo: "foo".to_string(),
            },
        },
        bar: Box {
            inner: Bar {
                bar: "bar".to_string(),
            },
        },
    };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(
        data,
        serde_json::json!({
            "foo": {
                "inner": {
                    "foo": "foo"
                }
            },
            "bar": {
                "inner": {
                    "bar": "bar"
                }
            }
        })
    );
}
