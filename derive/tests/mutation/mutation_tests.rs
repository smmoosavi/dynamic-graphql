use dynamic_graphql::dynamic::DynamicRequestExt;
use dynamic_graphql::{
    App, ExpandObject, FieldValue, Mutation, MutationFields, MutationRoot, Object, ParentType,
    SimpleObject,
};

use crate::schema_utils::normalize_schema;

#[test]
fn test_mutation_root() {
    #[derive(MutationRoot)]
    struct MutationRoot;

    assert_eq!(<MutationRoot as Object>::NAME, "MutationRoot");
}

#[test]
fn test_mutation_root_with_rename() {
    #[derive(MutationRoot)]
    #[graphql(name = "Mutation")]
    struct MutationRoot;

    assert_eq!(<MutationRoot as Object>::NAME, "Mutation");
}

#[test]
fn test_mutation() {
    #[derive(MutationRoot)]
    struct MutationRoot;

    #[derive(Mutation)]
    struct MyMutation(MutationRoot);

    assert_eq!(<MyMutation as ExpandObject>::NAME, "MyMutation");
    assert_eq!(
        <<MyMutation as ParentType>::Type as Object>::NAME,
        "MutationRoot"
    );
}

#[test]
fn test_schema() {
    #[derive(MutationRoot)]
    struct MutationRoot;

    #[derive(Mutation)]
    struct MyMutation(MutationRoot);

    #[MutationFields]
    impl MyMutation {
        fn the_example() -> String {
            "field".to_string()
        }
    }

    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        foo: String,
    }

    #[derive(App)]
    struct App(Query, MutationRoot, MyMutation);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"

                type MutationRoot {
                  theExample: String!
                }

                type Query {
                  foo: String!
                }

                schema {
                  query: Query
                  mutation: MutationRoot
                }

            "#
        ),
    );
}

#[test]
fn test_schema_with_rename() {
    #[derive(MutationRoot)]
    #[graphql(name = "Mutation")]
    struct MutationRoot;

    #[derive(Mutation)]
    struct MyMutation(MutationRoot);

    #[MutationFields]
    impl MyMutation {
        fn the_example() -> String {
            "field".to_string()
        }
    }

    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        foo: String,
    }

    #[derive(App)]
    struct App(Query, MutationRoot, MyMutation);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"

                type Mutation {
                  theExample: String!
                }

                type Query {
                  foo: String!
                }

                schema {
                  query: Query
                  mutation: Mutation
                }

            "#
        ),
    );
}

#[test]
fn test_schema_with_doc() {
    /// The Root of all Mutations
    #[derive(MutationRoot)]
    struct MutationRoot;

    #[derive(Mutation)]
    struct MyMutation(MutationRoot);

    #[MutationFields]
    impl MyMutation {
        fn the_example() -> String {
            "field".to_string()
        }
    }

    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        foo: String,
    }

    #[derive(App)]
    struct App(Query, MutationRoot, MyMutation);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"

                """The Root of all Mutations"""
                type MutationRoot {
                  theExample: String!
                }

                type Query {
                  foo: String!
                }

                schema {
                  query: Query
                  mutation: MutationRoot
                }

            "#
        ),
    );
}

#[tokio::test]
async fn test_query() {
    #[derive(MutationRoot)]
    struct MutationRoot;

    #[derive(Mutation)]
    struct MyMutation(MutationRoot);

    #[MutationFields]
    impl MyMutation {
        fn the_example() -> String {
            "field".to_string()
        }
    }

    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        foo: String,
    }

    #[derive(App)]
    struct App(Query, MutationRoot, MyMutation);

    let schema = App::create_schema().finish().unwrap();

    let query = r#"
        mutation {
            theExample
        }
    "#;

    let root = Query {
        foo: "foo".to_string(),
    };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));

    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(
        data,
        serde_json::json!(
            {
                "theExample": "field"
            }
        )
    );
}

mod in_mod {
    use dynamic_graphql::dynamic::DynamicRequestExt;
    use dynamic_graphql::{App, FieldValue, Mutation, MutationFields, SimpleObject};

    mod root {
        use dynamic_graphql::MutationRoot;

        #[derive(MutationRoot)]
        pub struct MutationRoot;
    }

    #[derive(Mutation)]
    struct MyMutation(root::MutationRoot);

    #[MutationFields]
    impl MyMutation {
        fn the_example() -> String {
            "field".to_string()
        }
    }

    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        foo: String,
    }

    #[derive(App)]
    struct App(Query, root::MutationRoot, MyMutation);

    #[tokio::test]
    async fn test_in_mod() {
        let schema = App::create_schema().finish().unwrap();

        let query = r#"
            mutation {
                theExample
            }
        "#;

        let root = Query {
            foo: "foo".to_string(),
        };
        let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));

        let res = schema.execute(req).await;
        let data = res.data.into_json().unwrap();

        assert_eq!(
            data,
            serde_json::json!(
                {
                    "theExample": "field"
                }
            )
        );
    }
}
