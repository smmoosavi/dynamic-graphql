use std::borrow::Cow;

use dynamic_graphql::dynamic::DynamicRequestExt;
use dynamic_graphql::internal::Object;
use dynamic_graphql::internal::OutputTypeName;
use dynamic_graphql::internal::ResolveOwned;
use dynamic_graphql::internal::ResolveRef;
use dynamic_graphql::internal::TypeName;
use dynamic_graphql::App;
use dynamic_graphql::FieldValue;
use dynamic_graphql::SimpleObject;
use dynamic_graphql::Union;
use syn::__private::str;

use crate::schema_utils::normalize_schema;

#[tokio::test]
async fn test_query_simple_generic() {
    #[derive(SimpleObject)]
    struct Foo {
        pub foo: String,
    }

    #[derive(SimpleObject)]
    struct Bar {
        pub bar: String,
    }

    #[derive(SimpleObject)]
    #[graphql(get_type_name)]
    struct Box<T>
    where
        T: OutputTypeName + 'static,
        T: Send + Sync,
        T: for<'a> ResolveRef<'a>,
    {
        pub inner: T,
    }

    impl<T> TypeName for Box<T>
    where
        T: OutputTypeName + 'static,
        T: Send + Sync,
        T: for<'a> ResolveRef<'a>,
    {
        fn get_type_name() -> Cow<'static, str> {
            format!("Box{}", T::get_type_name()).into()
        }
    }

    #[derive(Union)]
    #[graphql(get_type_name)]
    #[allow(dead_code)]
    enum AB<A, B>
    where
        A: Object + 'static,
        A: Send + Sync,
        A: for<'a> ResolveRef<'a> + for<'a> ResolveOwned<'a>,
        B: Object + 'static,
        B: Send + Sync,
        B: for<'a> ResolveRef<'a> + for<'a> ResolveOwned<'a>,
    {
        A(A),
        B(B),
        ABoxed(Box<A>),
        BBoxed(Box<B>),
    }

    impl<A, B> TypeName for AB<A, B>
    where
        A: Object + 'static,
        A: Send + Sync,
        A: for<'a> ResolveRef<'a> + for<'a> ResolveOwned<'a>,
        B: Object + 'static,
        B: Send + Sync,
        B: for<'a> ResolveRef<'a> + for<'a> ResolveOwned<'a>,
    {
        fn get_type_name() -> Cow<'static, str> {
            format!("{}{}", A::get_type_name(), B::get_type_name()).into()
        }
    }

    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        #[graphql(name = "box")]
        pub field: AB<Foo, Bar>,
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();
    let sdl = schema.sdl();
    insta::assert_snapshot!(
        normalize_schema(&sdl),@r###"

    type Bar {
      bar: String!
    }

    type BoxBar {
      inner: Bar!
    }

    type BoxFoo {
      inner: Foo!
    }

    type Foo {
      foo: String!
    }

    union FooBar = Foo | Bar | BoxFoo | BoxBar

    type Query {
      box: FooBar!
    }

    schema {
      query: Query
    }
    "###);

    let query = r#"
        query {
            box {
                __typename

               ... on Foo {
                    foo
                }

                ... on Bar {
                    bar
                }

                ... on BoxFoo {
                    inner {
                        foo
                    }
                }

                ... on BoxBar {
                    inner {
                        bar
                    }
                }
            }
        }
    "#;
    let root = Query {
        field: AB::ABoxed(Box {
            inner: Foo {
                foo: "foo".to_string(),
            },
        }),
    };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(
        data,
        serde_json::json!({
            "box": {
                "__typename": "BoxFoo",
                "inner": {
                    "foo": "foo"
                }
            }
        })
    );
}
