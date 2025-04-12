use std::borrow::Cow;

use dynamic_graphql::dynamic::DynamicRequestExt;
use dynamic_graphql::internal::TypeName;
use dynamic_graphql::internal::Union;
use dynamic_graphql::App;
use dynamic_graphql::FieldValue;
use dynamic_graphql::ResolvedObject;
use dynamic_graphql::ResolvedObjectFields;
use dynamic_graphql::SimpleObject;
use dynamic_graphql::Union;

use crate::schema_utils::normalize_schema;

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

    assert_eq!(<Animal as Union>::get_union_type_name(), "Animal");
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

    assert_eq!(<Animal as Union>::get_union_type_name(), "Other");
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
    #[graphql(root)]
    struct Query {
        pet: Animal,
    }

    #[derive(App)]
    struct App(Query, Animal, Dog, Cat);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r###"
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

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    "###);
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
    #[graphql(root)]
    struct Query {
        pet: Animal,
    }

    #[derive(App)]
    struct App(Query, Animal, Dog, Cat);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r###"
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

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    "###);
}

#[test]
fn test_schema_with_type_name() {
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
    #[graphql(get_type_name)]
    enum Animal {
        Dog(Dog),
        Cat(Cat),
    }

    impl TypeName for Animal {
        fn get_type_name() -> Cow<'static, str> {
            "Other".into()
        }
    }

    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        pet: Animal,
    }

    #[derive(App)]
    struct App(Query, Animal, Dog, Cat);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r###"
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

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    "###);
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
    #[graphql(root)]
    struct Query {
        pet: Animal,
    }

    #[derive(App)]
    struct App(Query, Animal, Dog, Cat);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r#"
    type Cat {
      name: String!
      life: Int!
    }

    type Dog {
      name: String!
      power: Int!
    }

    "Some animal"
    union Other = Dog | Cat

    type Query {
      pet: Other!
    }

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    "#);
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
    #[graphql(root)]
    struct Query {
        pet: Animal,
    }

    #[derive(App)]
    struct App(Query, Animal, Dog, Cat);

    let schema = App::create_schema().finish().unwrap();

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
    #[graphql(root)]
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

    let schema = App::create_schema().finish().unwrap();

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

#[tokio::test]
async fn test_auto_register() {
    #[derive(SimpleObject)]
    struct Bird {
        name: String,
        fly: bool,
    }

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
    #[graphql(register(Bird))]
    enum Animal {
        Dog(Dog),
        Cat(Cat),
    }

    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        pet: Animal,
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();
    let sdl = schema.sdl();

    insta::assert_snapshot!(normalize_schema(&sdl), @r###"
    union Animal = Dog | Cat

    type Bird {
      name: String!
      fly: Boolean!
    }

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

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    "###);
}

mod in_mod {
    use dog::Dog;
    use dynamic_graphql::dynamic::DynamicRequestExt;
    use dynamic_graphql::App;
    use dynamic_graphql::FieldValue;
    use dynamic_graphql::SimpleObject;
    use dynamic_graphql::Union;

    use crate::schema_utils::normalize_schema;

    mod cat {
        use dynamic_graphql::SimpleObject;

        #[derive(SimpleObject)]
        pub struct Cat {
            pub name: String,
            pub life: i32,
        }
    }

    mod dog {
        use dynamic_graphql::SimpleObject;

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
    #[graphql(root)]
    struct Query {
        pet: Animal,
    }

    #[derive(App)]
    struct App(Query, Animal, Dog, cat::Cat);

    #[tokio::test]
    async fn test_schema() {
        let schema = App::create_schema().finish().unwrap();

        let sdl = schema.sdl();
        insta::assert_snapshot!(normalize_schema(&sdl), @r###"
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

        directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

        directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

        schema {
          query: Query
        }
        "###);

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
