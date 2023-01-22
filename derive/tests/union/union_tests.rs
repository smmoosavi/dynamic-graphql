use crate::schema_utils::normalize_schema;
use dynamic_graphql::dynamic::DynamicRequestExt;
use dynamic_graphql::{FieldValue, Union};
use dynamic_graphql_derive::{App, ResolvedObject, ResolvedObjectFields, SimpleObject};

#[test]
fn test_impl_union() {
    #[derive(SimpleObject)]
    struct Cat {
        name: String,
        life: i32,
    }

    #[derive(SimpleObject)]
    struct Dog {
        name: String,
        power: i32,
    }

    #[allow(dead_code)]
    #[derive(Union)]
    enum Animal {
        Dog(Dog),
        Cat(Cat),
    }

    assert_eq!(<Animal as Union>::NAME, "Animal");
}

#[test]
fn test_impl_union_with_rename() {
    #[derive(SimpleObject)]
    struct Cat {
        name: String,
        life: i32,
    }

    #[derive(SimpleObject)]
    struct Dog {
        name: String,
        power: i32,
    }

    #[allow(dead_code)]
    #[derive(Union)]
    #[graphql(name = "Other")]
    enum Animal {
        Dog(Dog),
        Cat(Cat),
    }

    assert_eq!(<Animal as Union>::NAME, "Other");
}

#[test]
fn test_schema() {
    #[derive(SimpleObject)]
    struct Cat {
        name: String,
        life: i32,
    }

    #[derive(SimpleObject)]
    struct Dog {
        name: String,
        power: i32,
    }

    #[allow(dead_code)]
    #[derive(Union)]
    enum Animal {
        Dog(Dog),
        Cat(Cat),
    }

    #[derive(SimpleObject)]
    struct Query {
        pet: Animal,
    }

    #[derive(App)]
    struct App(Query, Animal, Dog, Cat);

    let registry = dynamic_graphql::Registry::new();
    let registry = registry.register::<App>().set_root("Query");
    let schema = registry.create_schema().finish().unwrap();
    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"
                union Animal = Dog | Cat

                type Cat {
                  name: String!
                  life: Int!
                }

                type Dog {
                  name: String!
                  power: Int!
                }

                type Query {
                  pet: Animal!
                }

                schema {
                  query: Query
                }
            "#
        )
    );
}

#[test]
fn test_schema_with_rename() {
    #[derive(SimpleObject)]
    struct Cat {
        name: String,
        life: i32,
    }

    #[derive(SimpleObject)]
    struct Dog {
        name: String,
        power: i32,
    }

    #[allow(dead_code)]
    #[derive(Union)]
    #[graphql(name = "Other")]
    enum Animal {
        Dog(Dog),
        Cat(Cat),
    }

    #[derive(SimpleObject)]
    struct Query {
        pet: Animal,
    }

    #[derive(App)]
    struct App(Query, Animal, Dog, Cat);

    let registry = dynamic_graphql::Registry::new();
    let registry = registry.register::<App>().set_root("Query");
    let schema = registry.create_schema().finish().unwrap();
    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"
                type Cat {
                  name: String!
                  life: Int!
                }

                type Dog {
                  name: String!
                  power: Int!
                }

                union Other = Dog | Cat

                type Query {
                  pet: Other!
                }

                schema {
                  query: Query
                }
            "#
        )
    );
}

#[test]
fn test_schema_with_doc() {
    #[derive(SimpleObject)]
    struct Cat {
        name: String,
        life: i32,
    }

    #[derive(SimpleObject)]
    struct Dog {
        name: String,
        power: i32,
    }

    /// Some animal
    #[allow(dead_code)]
    #[derive(Union)]
    #[graphql(name = "Other")]
    enum Animal {
        Dog(Dog),
        Cat(Cat),
    }

    #[derive(SimpleObject)]
    struct Query {
        pet: Animal,
    }

    #[derive(App)]
    struct App(Query, Animal, Dog, Cat);

    let registry = dynamic_graphql::Registry::new();
    let registry = registry.register::<App>().set_root("Query");
    let schema = registry.create_schema().finish().unwrap();
    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"
                type Cat {
                  name: String!
                  life: Int!
                }

                type Dog {
                  name: String!
                  power: Int!
                }

                """
                  Some animal
                """
                union Other = Dog | Cat

                type Query {
                  pet: Other!
                }

                schema {
                  query: Query
                }
            "#
        )
    );
}

#[tokio::test]
async fn test_query() {
    #[derive(SimpleObject)]
    struct Cat {
        name: String,
        life: i32,
    }

    #[derive(SimpleObject)]
    struct Dog {
        name: String,
        power: i32,
    }

    #[allow(dead_code)]
    #[derive(Union)]
    #[graphql(name = "Other")]
    enum Animal {
        Dog(Dog),
        Cat(Cat),
    }

    #[derive(SimpleObject)]
    struct Query {
        pet: Animal,
    }

    #[derive(App)]
    struct App(Query, Animal, Dog, Cat);

    let registry = dynamic_graphql::Registry::new();
    let registry = registry.register::<App>().set_root("Query");
    let schema = registry.create_schema().finish().unwrap();

    let root = Query {
        pet: Animal::Dog(Dog {
            name: "dog".to_string(),
            power: 100,
        }),
    };

    let query = r#"
        query {
            pet {
                ... on Dog {
                    name
                    power
                }
                ... on Cat {
                    name
                    life
                }
            }
        }
    "#;

    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(
        data,
        serde_json::json!({ "pet": { "name": "dog", "power": 100 } })
    );
}

#[tokio::test]
async fn test_query_owned() {
    #[derive(SimpleObject)]
    struct Cat {
        name: String,
        life: i32,
    }

    #[derive(SimpleObject)]
    struct Dog {
        name: String,
        power: i32,
    }

    #[allow(dead_code)]
    #[derive(Union)]
    #[graphql(name = "Other")]
    enum Animal {
        Dog(Dog),
        Cat(Cat),
    }

    #[derive(ResolvedObject)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        async fn pet(&self) -> Animal {
            Animal::Dog(Dog {
                name: "dog".to_string(),
                power: 100,
            })
        }
    }

    #[derive(App)]
    struct App(Query, Animal, Dog, Cat);

    let registry = dynamic_graphql::Registry::new();
    let registry = registry.register::<App>().set_root("Query");
    let schema = registry.create_schema().finish().unwrap();

    let root = Query;

    let query = r#"
        query {
            pet {
                ... on Dog {
                    name
                    power
                }
                ... on Cat {
                    name
                    life
                }
            }
        }
    "#;

    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(
        data,
        serde_json::json!({ "pet": { "name": "dog", "power": 100 } })
    );
}

mod in_mod {
    use dog::Dog;
    use dynamic_graphql::dynamic::DynamicRequestExt;
    use dynamic_graphql::FieldValue;
    use dynamic_graphql_derive::{App, SimpleObject, Union};
    use crate::schema_utils::normalize_schema;

    mod cat {
        use dynamic_graphql_derive::SimpleObject;

        #[derive(SimpleObject)]
        pub struct Cat {
            pub name: String,
            pub life: i32,
        }
    }

    mod dog {
        use dynamic_graphql_derive::SimpleObject;

        #[derive(SimpleObject)]
        pub struct Dog {
            pub name: String,
            pub power: i32,
        }
    }

    #[allow(dead_code)]
    #[derive(Union)]
    enum Animal {
        Dog(Dog),
        Cat(cat::Cat),
    }

    #[derive(SimpleObject)]
    struct Query {
        pet: Animal,
    }

    #[derive(App)]
    struct App(Query, Animal, Dog, cat::Cat);

    #[tokio::test]
    async fn test_schema() {
        let registry = dynamic_graphql::Registry::new();
        let registry = registry.register::<App>().set_root("Query");
        let schema = registry.create_schema().finish().unwrap();
        let sdl = schema.sdl();
        assert_eq!(
            normalize_schema(&sdl),
            normalize_schema(
                r#"

                    union Animal = Dog | Cat

                    type Cat {
                      name: String!
                      life: Int!
                    }

                    type Dog {
                      name: String!
                      power: Int!
                    }

                    type Query {
                      pet: Animal!
                    }

                    schema {
                      query: Query
                    }

                "#
            ),
        );

        let query = r#"
        query {
            pet {
                ... on Dog {
                    name
                    power
                }
                ... on Cat {
                    name
                    life
                }
            }
        }
    "#;

        let root = Query {
            pet: Animal::Dog(Dog {
                name: "dog".to_string(),
                power: 100,
            }),
        };

        let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
        let res = schema.execute(req).await;
        let data = res.data.into_json().unwrap();

        assert_eq!(
            data,
            serde_json::json!({ "pet": { "name": "dog", "power": 100 } })
        );
    }
}
