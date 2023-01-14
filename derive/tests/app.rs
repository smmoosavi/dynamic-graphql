mod schema_utils;

use crate::schema_utils::normalize_schema;
use dynamic_graphql::Registry;
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
fn test_app_with_generic() {
    #[derive(SimpleObject)]
    struct Foo {
        value: String,
    }
    #[derive(SimpleObject)]
    struct Query {
        #[graphql(name = "other")]
        pub string: Foo,
    }

    trait GetFoo {
        fn get_foo(&self) -> Foo;
    }

    impl GetFoo for () {
        fn get_foo(&self) -> Foo {
            Foo {
                value: "foo".to_string(),
            }
        }
    }

    struct Other<T: GetFoo>(T);
    impl<T: GetFoo> dynamic_graphql::Register for Other<T> {
        fn register(registry: Registry) -> Registry {
            registry
        }
    }

    #[derive(App)]
    struct App<T: GetFoo>(Query, Foo, Other<T>);

    let registry = dynamic_graphql::Registry::new();
    let registry = registry.register::<App<()>>().set_root("Query");
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
fn test_app_with_lifetime() {
    #[derive(SimpleObject)]
    struct Foo {
        value: String,
    }
    #[derive(SimpleObject)]
    struct Query {
        #[graphql(name = "other")]
        pub string: Foo,
    }

    struct Other<'a>(&'a Foo);
    impl<'a> dynamic_graphql::Register for Other<'a> {
        fn register(registry: Registry) -> Registry {
            registry
        }
    }

    #[derive(App)]
    struct App<'a>(Query, Foo, Other<'a>);

    let registry = dynamic_graphql::Registry::new();
    let registry = registry.register::<App<'_>>().set_root("Query");
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
fn test_app_with_generic_and_lifetime() {
    #[derive(SimpleObject)]
    struct Foo {
        value: String,
    }
    #[derive(SimpleObject)]
    struct Query {
        #[graphql(name = "other")]
        pub string: Foo,
    }

    trait GetFoo {
        fn get_foo(&self) -> Foo;
    }

    impl GetFoo for () {
        fn get_foo(&self) -> Foo {
            Foo {
                value: "foo".to_string(),
            }
        }
    }

    struct Other<'a, T>(&'a T)
    where
        T: GetFoo;
    impl<'a, T: GetFoo> dynamic_graphql::Register for Other<'a, T> {
        fn register(registry: Registry) -> Registry {
            registry
        }
    }

    #[derive(App)]
    struct App<'a, T>(Query, Foo, Other<'a, T>)
    where
        T: GetFoo + 'a;

    let registry = dynamic_graphql::Registry::new();
    let registry = registry.register::<App<'_, ()>>().set_root("Query");
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
