use dynamic_graphql::Registry;
use dynamic_graphql_derive::{App, SimpleObject};

use crate::schema_utils::normalize_schema;

mod schema_utils;

#[test]
fn test_app() {
    #[derive(SimpleObject)]
    struct Foo {
        value: String,
    }
    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        #[graphql(name = "other")]
        pub string: Foo,
    }

    #[derive(App)]
    struct App(Query, Foo);

    let schema = App::create_schema().finish().unwrap();
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
    #[graphql(root)]
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

    let schema = App::<()>::create_schema().finish().unwrap();
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
    #[graphql(root)]
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

    let schema = App::create_schema().finish().unwrap();

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
    #[graphql(root)]
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

    let schema = App::<()>::create_schema().finish().unwrap();
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
    #[graphql(root)]
    struct Query {
        pub foo: Foo,
        pub bar: Bar,
    }

    #[derive(App)]
    struct App(Query, FooBar);

    let schema = App::create_schema().finish().unwrap();
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

mod test_in_mod {
    use dynamic_graphql::App;
    use dynamic_graphql::SimpleObject;

    use crate::schema_utils::normalize_schema;

    mod foo {
        use dynamic_graphql::SimpleObject;

        #[derive(SimpleObject)]
        pub struct Foo {
            value: String,
        }
    }

    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        pub foo: foo::Foo,
    }

    #[derive(App)]
    struct App(Query, foo::Foo);

    #[tokio::test]
    async fn test() {
        let schema = App::create_schema().finish().unwrap();
        let sdl = schema.sdl();
        assert_eq!(
            normalize_schema(&sdl),
            normalize_schema(
                r#"
                type Foo {
                  value: String!
                }

                type Query {
                  foo: Foo!
                }

                schema {
                  query: Query
                }
                "#
            ),
        );
    }
}
