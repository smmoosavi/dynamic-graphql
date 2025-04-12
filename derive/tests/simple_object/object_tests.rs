use std::borrow::Cow;

use dynamic_graphql::dynamic::DynamicRequestExt;
use dynamic_graphql::internal::Object;
use dynamic_graphql::internal::TypeName;
use dynamic_graphql::App;
use dynamic_graphql::FieldValue;
use dynamic_graphql::SimpleObject;

use crate::schema_utils::normalize_schema;

#[test]
fn test_impl_object() {
    #[allow(dead_code)]
    #[derive(SimpleObject)]
    struct Example {
        pub string: String,
    }
    assert_eq!(<Example as Object>::get_object_type_name(), "Example");
}

#[test]
fn test_impl_object_with_name() {
    #[allow(dead_code)]
    #[derive(SimpleObject)]
    #[graphql(name = "Other")]
    struct Example {
        pub string: String,
    }
    assert_eq!(<Example as Object>::get_object_type_name(), "Other");
}

#[test]
fn test_impl_resolvers() {
    #[allow(dead_code)]
    #[derive(SimpleObject)]
    struct Example {
        pub string: String,
    }
    let example = Example {
        string: "Hello".to_string(),
    };
    let s = example.__resolve_string();
    assert_eq!(s, &"Hello".to_string());
}

#[test]
fn test_schema() {
    #[allow(dead_code)]
    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        pub string: String,
    }
    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();
    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r###"
    type Query {
      string: String!
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
    #[allow(dead_code)]
    #[derive(SimpleObject)]
    #[graphql(name = "Other")]
    #[graphql(root)]
    struct Query {
        pub string: String,
    }
    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();
    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r###"
    type Other {
      string: String!
    }

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Other
    }
    "###);
}

#[test]
fn test_schema_with_type_name() {
    #[allow(dead_code)]
    #[derive(SimpleObject)]
    #[graphql(root)]
    #[graphql(get_type_name)]
    struct Query {
        pub string: String,
    }

    impl TypeName for Query {
        fn get_type_name() -> Cow<'static, str> {
            "Other".into()
        }
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();
    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r###"
    type Other {
      string: String!
    }

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Other
    }
    "###);
}

#[test]
fn test_schema_with_skip() {
    #[allow(dead_code)]
    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        pub string: String,
        #[graphql(skip)]
        pub other: String,
    }
    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r###"
    type Query {
      string: String!
    }

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    "###);
}

#[test]
fn test_schema_with_rename_field() {
    #[allow(dead_code)]
    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        #[graphql(name = "other")]
        pub string: String,
    }
    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r###"
    type Query {
      other: String!
    }

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    "###);
}

#[tokio::test]
async fn test_query() {
    #[allow(dead_code)]
    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        pub string: String,
    }
    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();
    let query = r#"
        query {
            string
        }
    "#;
    let root = Query {
        string: "Hello".to_string(),
    };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "string": "Hello" }));
}

#[tokio::test]
async fn test_optional() {
    #[allow(dead_code)]
    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        pub maybe_string: Option<String>,
    }
    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r###"
    type Query {
      maybeString: String
    }

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    "###);

    let query = r#"
        query {
            maybeString
        }
    "#;

    let root = Query {
        maybe_string: Some("Hello".to_string()),
    };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "maybeString": "Hello" }));

    let root = Query { maybe_string: None };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "maybeString": null }));
}

#[test]
fn test_schema_with_doc() {
    /// this is the query object
    #[allow(dead_code)]
    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        /// this is the string field
        pub string: String,
    }
    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();
    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r#"
    "this is the query object"
    type Query {
      "this is the string field"
      string: String!
    }

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    "#);
}

#[test]
fn test_schema_with_deprecation() {
    #[allow(dead_code)]
    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        #[graphql(deprecation)]
        pub deprecated: String,
        #[graphql(deprecation = "this is the old one")]
        pub with_reason: String,
    }
    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r###"
    type Query {
      deprecated: String! @deprecated
      withReason: String! @deprecated(reason: "this is the old one")
    }

    directive @deprecated(reason: String = "No longer supported") on FIELD_DEFINITION | ARGUMENT_DEFINITION | INPUT_FIELD_DEFINITION | ENUM_VALUE

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    "###);
}

#[test]
fn test_rename_fields() {
    #[derive(SimpleObject)]
    #[graphql(rename_fields = "snake_case")]
    #[allow(non_camel_case_types)]
    #[graphql(root)]
    struct the_query {
        pub the_string: String,
    }
    assert_eq!(<the_query as Object>::get_object_type_name(), "TheQuery");
    #[derive(App)]
    struct App(the_query);

    let schema = App::create_schema().finish().unwrap();
    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r###"
    type TheQuery {
      the_string: String!
    }

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: TheQuery
    }
    "###);
}

#[tokio::test]
async fn test_auto_register() {
    #[derive(SimpleObject)]
    struct Foo {
        pub string: String,
    }

    #[derive(SimpleObject)]
    struct Bar {
        pub foo: Foo,
    }

    #[derive(SimpleObject)]
    #[graphql(register(Foo))]
    #[graphql(register(Bar))]
    struct Example {
        pub string: String,
    }

    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        pub example: Example,
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();
    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r###"
    type Bar {
      foo: Foo!
    }

    type Example {
      string: String!
    }

    type Foo {
      string: String!
    }

    type Query {
      example: Example!
    }

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    "###);
}

mod in_mod {
    use dynamic_graphql::dynamic::DynamicRequestExt;
    use dynamic_graphql::App;
    use dynamic_graphql::FieldValue;
    use dynamic_graphql::SimpleObject;

    #[derive(SimpleObject)]
    #[graphql(root)]
    pub struct Query {
        pub string: String,
    }

    #[tokio::test]
    async fn test_query() {
        #[derive(App)]
        struct App(Query);

        let schema = App::create_schema().finish().unwrap();
        let query = r#"
            query {
                string
            }
        "#;
        let root = Query {
            string: "Hello".to_string(),
        };
        let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
        let res = schema.execute(req).await;
        let data = res.data.into_json().unwrap();

        assert_eq!(data, serde_json::json!({ "string": "Hello" }));
    }
}
