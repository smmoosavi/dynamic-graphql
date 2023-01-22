use crate::schema_utils::normalize_schema;
use dynamic_graphql::{App, Interface, SimpleObject};

#[test]
fn test_impl_interface() {
    #[Interface(NodeInterface)]
    trait Node {
        fn id(&self) -> String;
    }

    assert_eq!(<NodeInterface as Interface>::NAME, "Node");
}

#[test]
fn test_impl_interface_with_name() {
    #[Interface(NodeInterface)]
    #[graphql(name = "Other")]
    trait Node {
        fn id(&self) -> String;
    }

    assert_eq!(<NodeInterface as Interface>::NAME, "Other");
}

#[test]
fn test_schema() {
    #[Interface(NodeInterface)]
    trait Node {
        fn the_id(&self) -> String;
    }

    #[derive(SimpleObject)]
    struct Query {
        foo: String,
    }

    #[derive(App)]
    struct App(Query, NodeInterface<'static>);

    let registry = dynamic_graphql::Registry::new();
    let registry = registry.register::<App>().set_root("Query");
    let schema = registry.create_schema().finish().unwrap();
    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"

            interface Node {
              theId: String!
            }

            type Query {
              foo: String!
            }

            schema {
              query: Query
            }

            "#
        ),
    );
}

#[test]
fn test_schema_with_name() {
    #[Interface(NodeInterface)]
    #[graphql(name = "Other")]
    trait Node {
        #[graphql(name = "id")]
        fn get_id(&self) -> String;
    }

    #[derive(SimpleObject)]
    struct Query {
        foo: String,
    }

    #[derive(App)]
    struct App(Query, NodeInterface<'static>);

    let registry = dynamic_graphql::Registry::new();
    let registry = registry.register::<App>().set_root("Query");
    let schema = registry.create_schema().finish().unwrap();
    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"

            interface Other {
              id: String!
            }

            type Query {
              foo: String!
            }

            schema {
              query: Query
            }

            "#
        ),
    );
}

#[test]
fn test_schema_with_rename() {
    #[Interface(NodeInterface)]
    #[graphql(rename_fields = "snake_case")]
    trait Node {
        #[graphql(name = "id")]
        fn get_id(&self) -> String;

        fn the_id(&self) -> String;
    }

    #[derive(SimpleObject)]
    struct Query {
        foo: String,
    }

    #[derive(App)]
    struct App(Query, NodeInterface<'static>);

    let registry = dynamic_graphql::Registry::new();
    let registry = registry.register::<App>().set_root("Query");
    let schema = registry.create_schema().finish().unwrap();
    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"

            interface Node {
              id: String!
              the_id: String!
            }

            type Query {
              foo: String!
            }

            schema {
              query: Query
            }

            "#
        ),
    );
}

#[test]
fn test_schema_description() {
    /// the interface
    #[Interface(NodeInterface)]
    trait Node {
        /// the id
        fn the_id(&self) -> String;
    }

    #[derive(SimpleObject)]
    struct Query {
        foo: String,
    }

    #[derive(App)]
    struct App(Query, NodeInterface<'static>);

    let registry = dynamic_graphql::Registry::new();
    let registry = registry.register::<App>().set_root("Query");
    let schema = registry.create_schema().finish().unwrap();
    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"

            """
              the interface
            """
            interface Node {
              """
                the id
              """
              theId: String!
            }

            type Query {
              foo: String!
            }

            schema {
              query: Query
            }

            "#
        ),
    );
}

#[test]
fn test_schema_with_deprecation() {
    #[Interface(NodeInterface)]
    trait Node {
        #[graphql(deprecation)]
        fn the_id(&self) -> String;

        #[graphql(deprecation = "deprecated")]
        fn old(&self) -> String;
    }

    #[derive(SimpleObject)]
    struct Query {
        foo: String,
    }

    #[derive(App)]
    struct App(Query, NodeInterface<'static>);

    let registry = dynamic_graphql::Registry::new();
    let registry = registry.register::<App>().set_root("Query");
    let schema = registry.create_schema().finish().unwrap();
    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"

            interface Node {
                theId: String! @deprecated
                old: String! @deprecated(reason: "deprecated")
            }

            type Query {
              foo: String!
            }

            schema {
              query: Query
            }

            "#
        ),
    );
}

#[test]
fn test_schema_with_skip() {
    #[Interface(NodeInterface)]
    trait Node {
        fn the_id(&self) -> String;
        #[graphql(skip)]
        fn old(&self) -> String;
    }

    #[derive(SimpleObject)]
    struct Query {
        foo: String,
    }

    #[derive(App)]
    struct App(Query, NodeInterface<'static>);

    let registry = dynamic_graphql::Registry::new();
    let registry = registry.register::<App>().set_root("Query");
    let schema = registry.create_schema().finish().unwrap();
    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"

            interface Node {
              theId: String!
            }

            type Query {
              foo: String!
            }

            schema {
              query: Query
            }

            "#
        ),
    );
}

mod in_mod {
    mod node {
        use dynamic_graphql::Interface;

        #[Interface(NodeInterface)]
        pub trait Node {
            fn id(&self) -> String;
        }
    }

    mod foo {
        use super::node::Node;
        use crate::schema_utils::normalize_schema;
        use dynamic_graphql::dynamic::DynamicRequestExt;
        use dynamic_graphql::{FieldValue, SimpleObject};
        use dynamic_graphql_derive::{ResolvedObject, ResolvedObjectFields};

        #[derive(SimpleObject)]
        #[graphql(mark_with = "super::node::NodeInterface")]
        struct Bar {
            id: String,
            other: String,
        }

        #[derive(SimpleObject)]
        #[graphql(implement = "super::node::NodeInterface")]
        struct Foo {
            other: String,
        }

        impl Node for Foo {
            fn id(&self) -> String {
                "foo".to_string()
            }
        }

        #[derive(ResolvedObject)]
        pub struct Query;

        #[ResolvedObjectFields]
        impl Query {
            async fn foo(&self) -> super::node::NodeInterface {
                super::node::NodeInterface::new_owned(Foo {
                    other: "foo".to_string(),
                })
            }
            async fn bar(&self) -> super::node::NodeInterface {
                super::node::NodeInterface::new_owned(Bar {
                    id: "bar".to_string(),
                    other: "bar".to_string(),
                })
            }
        }

        #[derive(dynamic_graphql::App)]
        pub struct App(Query, super::node::NodeInterface<'static>, Bar, Foo);

        #[tokio::test]
        async fn test_in_mode() {
            let registry = dynamic_graphql::Registry::new();
            let registry = registry.register::<App>().set_root("Query");
            let schema = registry.create_schema().finish().unwrap();
            let sdl = schema.sdl();
            assert_eq!(
                normalize_schema(&sdl),
                normalize_schema(
                    r#"
                        type Bar implements Node {
                          id: String!
                          other: String!
                        }

                        type Foo implements Node {
                          other: String!
                          id: String!
                        }

                        interface Node {
                          id: String!
                        }

                        type Query {
                          foo: Node!
                          bar: Node!
                        }

                        schema {
                          query: Query
                        }
                "#
                ),
            );

            let query = r#"
                query {
                    foo {
                        id
                        ... on Foo {
                            other
                        }
                    }
                    bar {
                        id
                        ... on Bar {
                            other
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
                    "foo": { "id": "foo", "other": "foo" },
                    "bar": { "id": "bar", "other": "bar" },
                })
            );
        }
    }
}
