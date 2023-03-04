use dynamic_graphql::internal::Registry;
use dynamic_graphql::SimpleObject;
use dynamic_graphql::{dynamic, value};

use crate::schema_utils::normalize_schema;

mod schema_utils;

#[tokio::test]
async fn test_apply() {
    #[derive(SimpleObject)]
    struct Foo {
        value: String,
    }

    let registry = Registry::new().register::<Foo>();

    let schema = dynamic::Schema::build("Query", None, None);
    let schema = registry.apply_into_schema_builder(schema);

    let query = dynamic::Object::new("Query");
    let query = query.field(dynamic::Field::new(
        "foo",
        dynamic::TypeRef::named("Foo"),
        |_ctx| {
            dynamic::FieldFuture::new(async move {
                Ok(Some(dynamic::FieldValue::owned_any(Foo {
                    value: "the foo".to_string(),
                })))
            })
        },
    ));
    let schema = schema.register(query);

    let schema = schema.finish().unwrap();
    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"
            type Foo {
              value: String!
            }

            type Query {
              foo: Foo
            }

            schema {
              query: Query
            }
            "#
        ),
    );

    let result = schema
        .execute("{ foo { value } }")
        .await
        .into_result()
        .unwrap();
    assert_eq!(result.data, value!({ "foo": { "value": "the foo" } }));
}
