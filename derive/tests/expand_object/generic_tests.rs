use dynamic_graphql::dynamic::DynamicRequestExt;
use dynamic_graphql::{
    App, ExpandObject, ExpandObjectFields, FieldValue, Object, ParentType, SimpleObject,
};

use crate::schema_utils::normalize_schema;

#[test]
fn test_impl_expand_object_with_generic() {
    trait GetName {
        fn get_name(&self) -> String;
    }

    #[derive(SimpleObject)]
    struct Example {
        field: String,
    }
    impl GetName for Example {
        fn get_name(&self) -> String {
            "foo".to_string()
        }
    }

    #[derive(ExpandObject)]
    struct ExpandExample<'a, T: GetName + Object>(&'a T);

    assert_eq!(
        <<ExpandExample<Example> as ParentType>::Type as Object>::NAME,
        "Example"
    );
    assert_eq!(
        <ExpandExample<Example> as ExpandObject>::NAME,
        "ExpandExample"
    );
    let example = Example {
        field: "field".to_string(),
    };
    let expand_example = ExpandExample(&example);
    assert_eq!(expand_example.parent().field, "field");
    assert_eq!(expand_example.parent().get_name(), "foo");
    let expand_example: ExpandExample<Example> = (&example).into();
    assert_eq!(expand_example.parent().field, "field");
}

#[test]
fn test_impl_expand_object_with_where() {
    trait GetName {
        fn get_name(&self) -> String;
    }

    #[derive(SimpleObject)]
    struct Example {
        field: String,
    }
    impl GetName for Example {
        fn get_name(&self) -> String {
            "foo".to_string()
        }
    }

    #[derive(ExpandObject)]
    struct ExpandExample<'a, T>(&'a T)
    where
        T: GetName + Object;

    assert_eq!(
        <<ExpandExample<Example> as ParentType>::Type as Object>::NAME,
        "Example"
    );
    assert_eq!(
        <ExpandExample<Example> as ExpandObject>::NAME,
        "ExpandExample"
    );
    let example = Example {
        field: "field".to_string(),
    };
    let expand_example = ExpandExample(&example);
    assert_eq!(expand_example.parent().field, "field");
    assert_eq!(expand_example.parent().get_name(), "foo");
    let expand_example: ExpandExample<Example> = (&example).into();
    assert_eq!(expand_example.parent().field, "field");
}

#[test]
fn test_schema_with_generic() {
    #[derive(SimpleObject)]
    struct Foo {
        field: String,
    }

    impl GetName for Foo {
        fn get_name(&self) -> String {
            "foo".to_string()
        }
    }

    #[derive(SimpleObject)]
    struct Bar {
        field: String,
    }

    impl GetName for Bar {
        fn get_name(&self) -> String {
            "bar".to_string()
        }
    }

    trait GetName {
        fn get_name(&self) -> String;
    }

    #[derive(ExpandObject)]
    struct WithName<'a, T>(&'a T)
    where
        T: GetName + Object;

    #[ExpandObjectFields]
    impl<'a, T> WithName<'a, T>
    where
        T: GetName + Object + 'static,
    {
        fn name(&self) -> String {
            self.parent().get_name()
        }
    }

    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        foo: Foo,
        bar: Bar,
    }

    #[derive(App)]
    struct App(Query, Bar, Foo);

    let schema = App::create_schema().finish().unwrap();
    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"
            type Bar {
              field: String!
            }

            type Foo {
              field: String!
            }

            type Query {
              foo: Foo!
              bar: Bar!
            }

            schema {
              query: Query
            }

            "#
        ),
    );

    #[derive(App)]
    struct AppWithName(Query, Bar, Foo, WithName<'static, Foo>);

    let schema = AppWithName::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"
            type Bar {
              field: String!
            }

            type Foo {
              field: String!
              name: String!
            }

            type Query {
              foo: Foo!
              bar: Bar!
            }

            schema {
              query: Query
            }

            "#
        ),
    );

    #[derive(App)]
    struct AppBothWithName(
        Query,
        Bar,
        Foo,
        WithName<'static, Foo>,
        WithName<'static, Bar>,
    );

    let schema = AppBothWithName::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"
            type Bar {
              field: String!
              name: String!
            }

            type Foo {
              field: String!
              name: String!
            }

            type Query {
              foo: Foo!
              bar: Bar!
            }

            schema {
              query: Query
            }

            "#
        ),
    );
}

#[tokio::test]
async fn test_query_with_generic() {
    #[derive(SimpleObject)]
    struct Foo {
        field: String,
    }

    impl GetName for Foo {
        fn get_name(&self) -> String {
            format!("foo: {}", self.field)
        }
    }

    trait GetName {
        fn get_name(&self) -> String;
    }

    #[derive(ExpandObject)]
    struct WithName<'a, T>(&'a T)
    where
        T: GetName + Object;

    #[ExpandObjectFields]
    impl<'a, T> WithName<'a, T>
    where
        T: GetName + Object + 'static,
    {
        fn name(&self) -> String {
            self.parent().get_name()
        }
    }

    #[derive(App)]
    struct AppBothWithName(Query, Foo, WithName<'static, Foo>);

    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        foo: Foo,
    }

    let schema = AppBothWithName::create_schema().finish().unwrap();

    let query = r#"
        query {
            foo {
                field
                name
            }
        }
    "#;

    let root = Query {
        foo: Foo {
            field: "field".to_string(),
        },
    };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));

    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(
        data,
        serde_json::json!(
            {
                "foo": {
                    "field": "field",
                    "name": "foo: field",
                }
            }
        )
    );
}

#[tokio::test]
async fn test_query_with_generic_and_args() {
    #[derive(SimpleObject)]
    struct Foo {
        field: String,
        #[graphql(skip)]
        greeting: String,
    }

    impl GetGreeting for Foo {
        fn get_greeting(&self) -> String {
            self.greeting.clone()
        }
    }

    trait GetGreeting {
        fn get_greeting(&self) -> String;
    }

    #[derive(ExpandObject)]
    struct WithGreeting<'a, T>(&'a T)
    where
        T: GetGreeting + Object;

    #[ExpandObjectFields]
    impl<'a, T> WithGreeting<'a, T>
    where
        T: GetGreeting + Object + 'static,
    {
        fn hello(&self, name: String) -> String {
            let greeting = self.parent().get_greeting();
            format!("{} {}", greeting, name)
        }
    }

    #[derive(App)]
    struct App(Query, Foo, WithGreeting<'static, Foo>);

    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        foo: Foo,
    }

    let schema = App::create_schema().finish().unwrap();

    let query = r#"
        query {
            foo {
                field
                hello(name: "world")
            }
        }
    "#;

    let root = Query {
        foo: Foo {
            field: "foo".to_string(),
            greeting: "Hi".to_string(),
        },
    };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));

    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(
        data,
        serde_json::json!(
            {
                "foo": {
                    "field": "foo",
                    "hello": "Hi world",
                }
            }
        )
    );
}
