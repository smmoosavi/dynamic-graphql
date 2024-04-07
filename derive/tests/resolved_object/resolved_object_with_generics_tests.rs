use std::marker::PhantomData;

use dynamic_graphql::dynamic::DynamicRequestExt;
use dynamic_graphql::internal::OutputTypeName;
use dynamic_graphql::internal::ResolveRef;
use dynamic_graphql::App;
use dynamic_graphql::FieldValue;
use dynamic_graphql::ResolvedObject;
use dynamic_graphql::ResolvedObjectFields;
use dynamic_graphql::SimpleObject;

use crate::schema_utils::normalize_schema;

#[tokio::test]
async fn test_query_static_generic() {
    trait Greeter {
        fn greet(name: String) -> String;
    }

    struct Hi;
    impl Greeter for Hi {
        fn greet(name: String) -> String {
            format!("Hi, {}!", name)
        }
    }

    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query<G: Greeter + Send + Sync + 'static>(PhantomData<G>);

    #[ResolvedObjectFields]
    impl<G: Greeter + Send + Sync + 'static> Query<G> {
        fn greet(&self, name: String) -> String {
            G::greet(name)
        }
    }

    #[derive(App)]
    struct App(Query<Hi>);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r###"
    type Query {
      greet(name: String!): String!
    }

    schema {
      query: Query
    }
    "###);

    let query = r#"
        query {
            greet(name: "World")
        }
    "#;
    let root = Query::<Hi>(PhantomData);
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "greet": "Hi, World!" }));
}

#[tokio::test]
async fn test_query_generic_with_self() {
    trait Greeter {
        fn greet(&self, name: String) -> String;
    }

    struct Greet {
        value: String,
    }
    impl Greeter for Greet {
        fn greet(&self, name: String) -> String {
            format!("{}, {}!", self.value, name)
        }
    }

    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query<G: Greeter + Send + Sync + 'static>(G);

    #[ResolvedObjectFields]
    impl<G: Greeter + Send + Sync + 'static> Query<G> {
        fn greet(&self, name: String) -> String {
            G::greet(&self.0, name)
        }
    }

    #[derive(App)]
    struct App(Query<Greet>);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r###"
    type Query {
      greet(name: String!): String!
    }

    schema {
      query: Query
    }
    "###);

    let query = r#"
        query {
            greet(name: "World")
        }
    "#;
    let root = Query::<Greet>(Greet {
        value: "Hello".to_string(),
    });
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "greet": "Hello, World!" }));
}

#[tokio::test]
async fn test_query_graphql_generic() {
    #[derive(SimpleObject)]
    struct Foo {
        value: String,
    }

    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query<G>(G)
    where
        G: OutputTypeName + 'static,
        G: Send + Sync,
        G: for<'a> ResolveRef<'a>;

    #[ResolvedObjectFields]
    impl<G> Query<G>
    where
        G: OutputTypeName + 'static,
        G: Send + Sync,
        G: for<'a> ResolveRef<'a>,
    {
        fn the_g(&self) -> &G {
            &self.0
        }
    }

    #[derive(App)]
    struct App(Query<Foo>, Foo);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r###"
    type Foo {
      value: String!
    }

    type Query {
      theG: Foo!
    }

    schema {
      query: Query
    }
    "###);

    let query = r#"
        query {
            theG {
                value
            }
        }
    "#;
    let root = Query::<Foo>(Foo {
        value: "Hello".to_string(),
    });
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "theG": { "value": "Hello" } }));
}
