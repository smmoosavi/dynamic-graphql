mod schema_utils;

use syn::Pat::Struct;
use crate::schema_utils::normalize_schema;
use dynamic_graphql::{OutputType, Registry, ResolveOwned, ResolveRef};
use dynamic_graphql::{App, ResolvedObject, ResolvedObjectFields, SimpleObject};

#[test]
fn test_app() {
    trait Pageable {
        type Item: OutputType + 'static;
        fn items(&self, page: i32) -> Vec<Self::Item>;
    }

    #[derive(ResolvedObject)]
    struct BaseList<T>(T) where
        T: Pageable + 'static,
        T: Send + Sync,
        T::Item: OutputType + 'static,
        T::Item: Send + Sync,
        T::Item: for<'a> ResolveRef<'a> + for<'a> ResolveOwned<'a>;

    #[ResolvedObjectFields]
    impl<T> BaseList<T> where
        T: Pageable + 'static,
        T: Send + Sync,
        T::Item: OutputType + 'static,
        T::Item: Send + Sync,
        T::Item: for<'a> ResolveRef<'a> + for<'a> ResolveOwned<'a>,
    {
        fn items(&self, page: Option<i32>) -> Vec<T::Item> {
            self.0.items(page.unwrap_or(0))
        }
    }

    struct FooStore;
    impl Pageable for FooStore {
        type Item = Foo;
        fn items(&self, page: i32) -> Vec<Self::Item> {
            let start = page * 10;
            let end = start + 10;
            (start..end).map(|i| Foo {
                value: i.to_string(),
            }).collect()
        }
    }

    #[derive(SimpleObject)]
    struct Foo {
        value: String,
    }

    #[derive(NewObject)]
    struct FooList(BaseList<FooStore>);

    #[derive(ResolvedObject)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        fn foo_list(&self) -> FooList {
            FooList(BaseList(FooStore))
        }
    }



    #[derive(App)]
    struct App(Query, BaseList<FooStore>, Foo);

    let registry = dynamic_graphql::Registry::new();
    let registry = registry.register::<App>().set_root("Query");
    let schema = registry.create_schema().finish().unwrap();

    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"

            type BaseList {
              items(page: Int): [Foo!]!
            }

            type Foo {
              value: String!
            }

            type Query {
              fooList: FooList!
            }

            schema {
              query: Query
            }

            "#
        ),
    );
}