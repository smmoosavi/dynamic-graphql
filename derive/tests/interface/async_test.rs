use async_trait::async_trait;
use dynamic_graphql::dynamic::DynamicRequestExt;
use dynamic_graphql::App;
use dynamic_graphql::FieldValue;
use dynamic_graphql::Instance;
use dynamic_graphql::Interface;
use dynamic_graphql::ResolvedObject;
use dynamic_graphql::ResolvedObjectFields;
use dynamic_graphql::SimpleObject;

use crate::schema_utils::normalize_schema;

#[tokio::test]
async fn test_async_trait() {
    #[Interface]
    #[async_trait]
    trait Foo {
        fn sync_value(&self) -> String;
        async fn async_value(&self) -> String;
    }

    #[derive(SimpleObject)]
    #[graphql(implements(Foo))]
    struct FooValue;

    #[async_trait]
    impl Foo for FooValue {
        fn sync_value(&self) -> String {
            "sync_value".to_string()
        }

        async fn async_value(&self) -> String {
            "async_value".to_string()
        }
    }

    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        async fn foo(&self) -> Instance<dyn Foo> {
            Instance::new_owned(FooValue)
        }
    }

    #[derive(App)]
    struct App(Query, FooValue);

    let schema = App::create_schema().finish().unwrap();
    let sdl = schema.sdl();

    insta::assert_snapshot!(normalize_schema(&sdl), @"");

    let query = r#"

        query {
            foo {
                syncValue
                asyncValue
            }
        }

    "#;

    let root = Query;
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;

    assert_eq!(
        res.data.into_json().unwrap(),
        serde_json::json!({
            "foo": {
                "syncValue": "sync_value",
                "asyncValue": "async_value"
            }
        })
    );
}
