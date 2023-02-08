use crate::schema_utils::normalize_schema;
use dynamic_graphql::dynamic::DynamicRequestExt;
use dynamic_graphql::{FieldValue, OutputTypeName, ResolveOwned, TypeName};
use dynamic_graphql_derive::{
    App, ExpandObject, ExpandObjectFields, ResolvedObject, ResolvedObjectFields, SimpleObject,
};
use std::borrow::Cow;

#[tokio::test]
async fn test_base_list() {
    trait Pageable {
        type Item: OutputTypeName + 'static;
        fn items(&self, page: i32) -> Vec<Self::Item>;
    }

    #[derive(ResolvedObject)]
    #[graphql(get_type_name)]
    struct BaseList<T>(T)
    where
        T: Pageable + 'static,
        T: Send + Sync,
        T::Item: OutputTypeName + 'static,
        T::Item: Send + Sync,
        T::Item: for<'r> ResolveOwned<'r>;

    impl<T> TypeName for BaseList<T>
    where
        T: Pageable + 'static,
        T: Send + Sync,
        T::Item: OutputTypeName + 'static,
        T::Item: Send + Sync,
        T::Item: for<'r> ResolveOwned<'r>,
    {
        fn get_type_name() -> Cow<'static, str> {
            format!("{}List", T::Item::get_type_name()).into()
        }
    }

    #[ResolvedObjectFields]
    impl<T> BaseList<T>
    where
        T: Pageable + 'static,
        T: Send + Sync,
        T::Item: OutputTypeName + 'static,
        T::Item: Send + Sync,
        T::Item: for<'r> ResolveOwned<'r>,
    {
        fn items(&self, page: Option<i32>) -> Vec<T::Item> {
            self.0.items(page.unwrap_or(0))
        }
    }

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

    #[derive(SimpleObject)]
    struct Foo {
        value: String,
    }

    #[derive(ExpandObject)]
    struct FooQuery(Query);

    #[ExpandObjectFields]
    impl FooQuery {
        fn foo_list() -> BaseList<FooList> {
            BaseList(FooList)
        }
    }

    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query;

    #[derive(App)]
    struct App(Query, FooQuery);

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
