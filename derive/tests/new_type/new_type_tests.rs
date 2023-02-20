use crate::schema_utils::normalize_schema;
use dynamic_graphql_derive::{App, ResolvedObject, ResolvedObjectFields, SimpleObject};
use the_newtype::Newtype;

#[tokio::test]
async fn test_query() {
    #[derive(SimpleObject)]
    struct Foo {
        value: String,
    }

    // #[derive(Newtype)]
    struct Example(Foo);

    impl dynamic_graphql::Newtype for Example {
        type Inner = Foo;
    }

    impl From<Example> for Foo {
        fn from(example: Example) -> Self {
            example.0
        }
    }
    impl AsRef<Foo> for Example {
        fn as_ref(&self) -> &Foo {
            &self.0
        }
    }

    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query {
        example: Example,
    }

    #[ResolvedObjectFields]
    impl Query {
        async fn example(&self) -> Example {
            Example(Foo {
                value: "Hello, world!".to_string(),
            })
        }
        async fn example_ref(&self) -> &Example {
            &self.example
        }
    }

    #[derive(App)]
    struct App(Query);

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
              example: Foo!
            }

            schema {
              query: Query
            }

        "#
        ),
    );
}
