use crate::schema_utils::normalize_schema;
use dynamic_graphql::dynamic::DynamicRequestExt;
use dynamic_graphql::{App, FieldValue, OutputType, ResolveOwned, ResolveRef, SimpleObject};

#[tokio::test]
async fn test_query_simple_generic() {
    #[derive(SimpleObject)]
    struct Foo {
        pub value: String,
    }

    #[allow(dead_code)]
    #[derive(SimpleObject)]
    struct Query<F>
    where
        F: OutputType + 'static,
        F: Send + Sync,
        F: for<'a> ResolveRef<'a> + for<'a> ResolveOwned<'a>,
    {
        pub field: F,
    }

    #[derive(App)]
    struct App(Query<Foo>, Foo);

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
                  field: Foo!
                }

                schema {
                  query: Query
                }

            "#
        ),
    );

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
