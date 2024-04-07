use dynamic_graphql::dynamic::DynamicRequestExt;
use dynamic_graphql::App;
use dynamic_graphql::FieldValue;
use dynamic_graphql::Interface;
use dynamic_graphql::SimpleObject;
use dynamic_graphql::Union;

use crate::schema_utils::normalize_schema;

#[tokio::test]
async fn test_query() {
    #[Interface]
    trait Named {
        fn name(&self) -> &str;
    }

    #[derive(SimpleObject)]
    #[graphql(mark(Named))]
    struct Cat {
        name: String,
        life: i32,
    }

    #[derive(SimpleObject)]
    #[graphql(mark(Named))]
    struct Dog {
        name: String,
        power: i32,
    }

    #[derive(SimpleObject)]
    struct Snake {
        length: i32,
    }

    #[allow(dead_code)]
    #[derive(Union)]
    enum Animal {
        Dog(Dog),
        Cat(Cat),
        Snake(Snake),
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
    insta::assert_snapshot!(normalize_schema(&sdl), @"");

    let query = r#"
        query {
            pet {
                ... on Named {
                    __typename
                    name
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
        serde_json::json!({ "pet": { "__typename": "Dog", "name": "dog" }})
    );

    let root = Query {
        pet: Animal::Cat(Cat {
            name: "cat".to_string(),
            life: 100,
        }),
    };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(
        data,
        serde_json::json!({ "pet": { "__typename": "Cat", "name": "cat" }})
    );

    let root = Query {
        pet: Animal::Snake(Snake { length: 100 }),
    };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "pet": {}}));
}
