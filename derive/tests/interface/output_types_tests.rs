use crate::schema_utils::normalize_schema;
use dynamic_graphql::dynamic::DynamicRequestExt;
use dynamic_graphql::FieldValue;
use dynamic_graphql_derive::{App, Interface, ResolvedObject, ResolvedObjectFields, SimpleObject};

#[tokio::test]
async fn interface_as_output_value_for_simple_object_with_implement() {
    #[Interface(NodeInterface)]
    trait Node {
        fn id_ref(&self) -> &String;
        fn id_owned(&self) -> String;
        fn id_cow_borrowed(&self) -> std::borrow::Cow<'_, String>;
        fn id_cow_owned(&self) -> std::borrow::Cow<'_, String>;
    }

    #[derive(SimpleObject)]
    #[graphql(implement = "NodeInterface")]
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
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        async fn node(&self) -> NodeInterface {
            NodeInterface::new_owned(FooNode {
                other_field: "foo".to_string(),
                id: "foo id".to_string(),
            })
        }
    }

    #[derive(App)]
    struct App(Query, NodeInterface<'static>, FooNode);

    let registry = dynamic_graphql::Registry::new();
    let registry = registry.register::<App>().set_root("Query");
    let schema = registry.create_schema().finish().unwrap();
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
