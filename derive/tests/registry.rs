use std::borrow::Cow;

use dynamic_graphql::App;
use dynamic_graphql::Context;
use dynamic_graphql::SimpleObject;
use dynamic_graphql::dynamic;
use dynamic_graphql::dynamic::DynamicRequestExt;
use dynamic_graphql::internal::OutputTypeName;
use dynamic_graphql::internal::Register;
use dynamic_graphql::internal::Registry;
use dynamic_graphql::internal::ResolveOwned;
use dynamic_graphql::internal::ResolveRef;
use dynamic_graphql::internal::TypeName;
use dynamic_graphql::value;

use crate::schema_utils::normalize_schema;

mod schema_utils;

#[tokio::test]
async fn test_app() {
    #[derive(SimpleObject)]
    struct Foo {
        value: String,
    }

    struct Query;

    impl Register for Query {
        fn register(registry: Registry) -> Registry {
            let registry = registry.register::<Foo>();
            let object = dynamic::Object::new("Query");
            let object = object.field(dynamic::Field::new(
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

            registry.register_type(object).set_root("Query")
        }
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();
    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r"
    type Foo {
      value: String!
    }

    type Query {
      foo: Foo
    }

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    ");

    let result = schema
        .execute("{ foo { value } }")
        .await
        .into_result()
        .unwrap();
    assert_eq!(result.data, value!({ "foo": { "value": "the foo" } }));
}

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
    insta::assert_snapshot!(normalize_schema(&sdl), @r"
    type Foo {
      value: String!
    }

    type Query {
      foo: Foo
    }

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    ");

    let result = schema
        .execute("{ foo { value } }")
        .await
        .into_result()
        .unwrap();
    assert_eq!(result.data, value!({ "foo": { "value": "the foo" } }));
}

#[tokio::test]
async fn define_custom_type() {
    // define Foo type
    struct Foo {
        value: String,
    }

    impl TypeName for Foo {
        fn get_type_name() -> Cow<'static, str> {
            "Foo".into()
        }
    }

    impl OutputTypeName for Foo {}
    impl Register for Foo {
        fn register(registry: Registry) -> Registry {
            let object = dynamic::Object::new("Foo");
            let object = object.field(dynamic::Field::new(
                "value",
                dynamic::TypeRef::named_nn("String"),
                |ctx| {
                    let query: &Foo = ctx.parent_value.downcast_ref().unwrap();
                    dynamic::FieldFuture::new(async move {
                        Ok(Some(dynamic::FieldValue::value(query.value.clone())))
                    })
                },
            ));
            registry.register_type(object)
        }
    }

    impl<'a> ResolveOwned<'a> for Foo {
        fn resolve_owned(
            self,
            _ctx: &Context,
        ) -> dynamic_graphql::Result<Option<dynamic::FieldValue<'a>>> {
            Ok(Some(dynamic::FieldValue::owned_any(self)))
        }
    }
    impl<'a> ResolveRef<'a> for Foo {
        fn resolve_ref(
            &'a self,
            _ctx: &Context,
        ) -> dynamic_graphql::Result<Option<dynamic::FieldValue<'a>>> {
            Ok(Some(dynamic::FieldValue::borrowed_any(self)))
        }
    }

    // use Foo type

    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        foo: Foo,
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();

    // use schema
    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r"
    type Foo {
      value: String!
    }

    type Query {
      foo: Foo!
    }

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    ");

    let root = Query {
        foo: Foo {
            value: "the foo".to_string(),
        },
    };
    let query = "{ foo { value } }";
    let req = dynamic_graphql::Request::new(query).root_value(dynamic::FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    assert_eq!(res.data, value!({ "foo": { "value": "the foo" } }));
}
