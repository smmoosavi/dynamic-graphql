mod schema_utils;

use crate::schema_utils::normalize_schema;
use dynamic_graphql_derive::{App, SimpleObject};

#[test]
fn test_app() {
    #[derive(SimpleObject)]
    struct Foo {
        value: String,
    }
    #[derive(SimpleObject)]
    struct Query {
        #[graphql(name = "other")]
        pub string: Foo,
    }

    #[derive(App)]
    struct App(Query, Foo);

    let registry = dynamic_graphql::Registry::new();
    let registry = registry.register::<App>().set_root("Query");
    let schema = registry.create_schema().finish().unwrap();
    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"
            type Foo {
              value: String!
            }

            type Query {
              other: Foo!
            }

            schema {
              query: Query
            }
            "#
        ),
    );
}

#[test]
fn test_nested_app() {
    #[derive(SimpleObject)]
    struct Foo {
        value: String,
    }

    #[derive(SimpleObject)]
    struct Bar {
        value: String,
    }

    #[derive(App)]
    struct FooBar(Foo, Bar);

    #[derive(SimpleObject)]
    struct Query {
        pub foo: Foo,
        pub bar: Bar,
    }

    #[derive(App)]
    struct App(Query, FooBar);

    let registry = dynamic_graphql::Registry::new();
    let registry = registry.register::<App>().set_root("Query");
    let schema = registry.create_schema().finish().unwrap();
    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"
            type Bar {
              value: String!
            }

            type Foo {
              value: String!
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
