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
async fn interface_string_ref_types() {
    #[Interface]
    trait Node {
        fn id_ref(&self) -> &String;
        fn id_owned(&self) -> String;
        fn id_cow_borrowed(&self) -> std::borrow::Cow<'_, String>;
        fn id_cow_owned(&self) -> std::borrow::Cow<'_, String>;
    }

    #[derive(SimpleObject)]
    #[graphql(implements(Node))]
    struct FooNode {
        other_field: String,
        #[graphql(skip)]
        id: String,
    }

    impl Node for FooNode {
        fn id_ref(&self) -> &String {
            &self.id
        }
        fn id_owned(&self) -> String {
            self.id.clone()
        }
        fn id_cow_borrowed(&self) -> std::borrow::Cow<'_, String> {
            std::borrow::Cow::Borrowed(&self.id)
        }
        fn id_cow_owned(&self) -> std::borrow::Cow<'_, String> {
            std::borrow::Cow::Owned(self.id.clone())
        }
    }

    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        async fn node(&self) -> Instance<dyn Node> {
            Instance::new_owned(FooNode {
                other_field: "foo".to_string(),
                id: "foo id".to_string(),
            })
        }
    }

    #[derive(App)]
    struct App(Query, FooNode);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"

                type FooNode implements Node {
                    otherField: String!
                    idRef: String!
                    idOwned: String!
                    idCowBorrowed: String!
                    idCowOwned: String!
                }

                interface Node {
                    idRef: String!
                    idOwned: String!
                    idCowBorrowed: String!
                    idCowOwned: String!
                }

                type Query {
                    node: Node!
                }

                schema {
                    query: Query
                }

            "#
        ),
    );

    let query = r#"

        query {
            node {
                idRef
                idOwned
                idCowBorrowed
                idCowOwned

                ... on FooNode {
                    otherField
                }
            }
        }

    "#;

    let root = Query;
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    assert_eq!(
        res.data.into_json().unwrap(),
        serde_json::json!({
            "node": {
                "idRef": "foo id",
                "idOwned": "foo id",
                "idCowBorrowed": "foo id",
                "idCowOwned": "foo id",
                "otherField": "foo",
            }
        })
    );
}

#[tokio::test]
async fn interface_object_ref_types() {
    #[derive(SimpleObject, Clone)]
    struct Bar {
        value: String,
    }

    #[Interface]
    trait Baz {
        fn bar_ref(&self) -> &Bar;
        fn bar_owned(&self) -> Bar;
        fn bar_cow_borrowed(&self) -> std::borrow::Cow<'_, Bar>;
        fn bar_cow_owned(&self) -> std::borrow::Cow<'_, Bar>;
    }

    #[derive(SimpleObject)]
    #[graphql(implements(Baz))]
    struct FooNode {
        other_field: String,
        #[graphql(skip)]
        bar: Bar,
    }

    impl Baz for FooNode {
        fn bar_ref(&self) -> &Bar {
            &self.bar
        }
        fn bar_owned(&self) -> Bar {
            self.bar.clone()
        }
        fn bar_cow_borrowed(&self) -> std::borrow::Cow<'_, Bar> {
            std::borrow::Cow::Borrowed(&self.bar)
        }
        fn bar_cow_owned(&self) -> std::borrow::Cow<'_, Bar> {
            std::borrow::Cow::Owned(self.bar.clone())
        }
    }

    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        async fn baz(&self) -> Instance<dyn Baz> {
            Instance::new_owned(FooNode {
                other_field: "foo".to_string(),
                bar: Bar {
                    value: "bar".to_string(),
                },
            })
        }
    }

    #[derive(App)]
    struct App(Query, FooNode, Bar);

    let schema = App::create_schema().finish().unwrap();
    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"
                type Bar {
                  value: String!
                }

                interface Baz {
                  barRef: Bar!
                  barOwned: Bar!
                  barCowBorrowed: Bar!
                  barCowOwned: Bar!
                }

                type FooNode implements Baz {
                  otherField: String!
                  barRef: Bar!
                  barOwned: Bar!
                  barCowBorrowed: Bar!
                  barCowOwned: Bar!
                }

                type Query {
                  baz: Baz!
                }

                schema {
                  query: Query
                }

            "#
        ),
    );

    let query = r#"

        query {
            baz {
                barRef { value }
                barOwned { value }
                barCowBorrowed { value }
                barCowOwned { value }

                ... on FooNode {
                    otherField
                }
            }
        }

    "#;

    let root = Query;
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;

    assert_eq!(
        res.data.into_json().unwrap(),
        serde_json::json!({
            "baz": {
                "barRef": { "value": "bar" },
                "barOwned": { "value": "bar" },
                "barCowBorrowed": { "value": "bar" },
                "barCowOwned": { "value": "bar" },
                "otherField": "foo",
            }
        })
    );
}
