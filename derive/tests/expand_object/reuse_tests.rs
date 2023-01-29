use dynamic_graphql::dynamic::DynamicRequestExt;
use dynamic_graphql::{
    App, ExpandObject, ExpandObjectFields, FieldValue, Object, OutputType, ResolveOwned,
    SimpleObject,
};

use crate::schema_utils::normalize_schema;

#[tokio::test]
async fn test_base_list() {
    trait Pageable {
        type Item: OutputType + 'static;
        fn items(&self, page: i32) -> Vec<Self::Item>;
    }

    #[derive(ExpandObject)]
    struct BaseList<'a, T>(&'a T)
    where
        T: Pageable + 'static,
        T: Send + Sync,
        T: Object,
        T::Item: OutputType + 'static,
        T::Item: Send + Sync,
        T::Item: for<'r> ResolveOwned<'r>;

    #[ExpandObjectFields]
    impl<'a, T> BaseList<'a, T>
    where
        T: Pageable + 'static,
        T: Send + Sync,
        T: Object,
        T::Item: OutputType + 'static,
        T::Item: Send + Sync,
        T::Item: for<'r> ResolveOwned<'r>,
    {
        fn items(&self, page: Option<i32>) -> Vec<T::Item> {
            self.0.items(page.unwrap_or(0))
        }
    }

    // foo app
    #[derive(SimpleObject)]
    struct Foo {
        value: String,
    }

    #[derive(SimpleObject)]
    struct FooList;
    impl Pageable for FooList {
        type Item = Foo;
        fn items(&self, page: i32) -> Vec<Self::Item> {
            let start = page * 10;
            let end = start + 10;
            (start..end)
                .map(|i| Foo {
                    value: i.to_string(),
                })
                .collect()
        }
    }

    #[derive(ExpandObject)]
    struct FooQuery<'a>(&'a Query);

    #[ExpandObjectFields]
    impl<'a> FooQuery<'a> {
        fn foo_list(&self) -> FooList {
            FooList
        }
    }

    #[derive(App)]
    struct FooApp(FooQuery<'static>, FooList, BaseList<'static, FooList>, Foo);

    // main app

    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query;

    #[derive(App)]
    struct App(Query, FooApp);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"

                type Foo {
                  value: String!
                }

                type FooList {
                  items(page: Int): [Foo!]!
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

    let query = r#"
        query {
            fooList {
                items(page: 1) {
                    value
                }
            }
        }
    "#;
    let root = Query;
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(
        data,
        serde_json::json!({
            "fooList": {
                "items": [
                    { "value": "10" },
                    { "value": "11" },
                    { "value": "12" },
                    { "value": "13" },
                    { "value": "14" },
                    { "value": "15" },
                    { "value": "16" },
                    { "value": "17" },
                    { "value": "18" },
                    { "value": "19" },
                ]
            }
        })
    );
}
