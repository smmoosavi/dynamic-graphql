use std::borrow::Cow;

use dynamic_graphql::FieldValue;
use dynamic_graphql::dynamic::DynamicRequestExt;
use dynamic_graphql::internal::InputObject;
use dynamic_graphql::internal::TypeName;
use dynamic_graphql_derive::App;
use dynamic_graphql_derive::InputObject;
use dynamic_graphql_derive::OneOfInput;
use dynamic_graphql_derive::ResolvedObject;
use dynamic_graphql_derive::ResolvedObjectFields;
use dynamic_graphql_derive::SimpleObject;

use crate::schema_utils::normalize_schema;

#[test]
fn test_impl_object() {
    #[allow(dead_code)]
    #[derive(OneOfInput)]
    enum ExampleInput {
        Str(String),
        Int(i32),
    }
    assert_eq!(
        <ExampleInput as InputObject>::get_input_object_type_name(),
        "ExampleInput"
    );
}

#[test]
fn test_impl_object_with_name() {
    #[allow(dead_code)]
    #[derive(OneOfInput)]
    #[graphql(name = "OtherInput")]
    enum ExampleInput {
        Str(String),
        Int(i32),
    }
    assert_eq!(
        <ExampleInput as InputObject>::get_input_object_type_name(),
        "OtherInput"
    );
}

#[tokio::test]
async fn test_schema() {
    #[derive(OneOfInput)]
    enum ExampleInput {
        Str(String),
        Int(i32),
    }

    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        async fn example(&self, input: ExampleInput) -> String {
            match input {
                ExampleInput::Str(s) => {
                    format!("String: {}", s)
                }
                ExampleInput::Int(i) => {
                    format!("Int: {}", i)
                }
            }
        }
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r"
    input ExampleInput @oneOf {
      str: String
      int: Int
    }

    type Query {
      example(input: ExampleInput!): String!
    }

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @oneOf on INPUT_OBJECT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    ");

    let query = r#"
        query {
            example(input: { str: "hello" })
        }
    "#;

    let root = Query;
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;

    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "example": "String: hello" }));
}

#[tokio::test]
async fn test_schema_with_rename() {
    #[derive(OneOfInput)]
    #[graphql(name = "OtherInput")]
    enum ExampleInput {
        Str(String),
        #[graphql(name = "number")]
        Int(i32),
    }

    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        async fn example(&self, input: ExampleInput) -> String {
            match input {
                ExampleInput::Str(s) => {
                    format!("String: {}", s)
                }
                ExampleInput::Int(i) => {
                    format!("Int: {}", i)
                }
            }
        }
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r"
    input OtherInput @oneOf {
      str: String
      number: Int
    }

    type Query {
      example(input: OtherInput!): String!
    }

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @oneOf on INPUT_OBJECT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    ");

    let query = r#"
        query {
            example(input: { str: "hello" })
        }
    "#;

    let root = Query;
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;

    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "example": "String: hello" }));
}

#[tokio::test]
async fn test_schema_with_type_name() {
    #[derive(OneOfInput)]
    #[graphql(get_type_name)]
    enum ExampleInput {
        Str(String),
        Int(i32),
    }

    impl TypeName for ExampleInput {
        fn get_type_name() -> Cow<'static, str> {
            "OtherInput".into()
        }
    }

    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        async fn example(&self, input: ExampleInput) -> String {
            match input {
                ExampleInput::Str(s) => {
                    format!("String: {}", s)
                }
                ExampleInput::Int(i) => {
                    format!("Int: {}", i)
                }
            }
        }
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r"
    input OtherInput @oneOf {
      str: String
      int: Int
    }

    type Query {
      example(input: OtherInput!): String!
    }

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @oneOf on INPUT_OBJECT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    ");

    let query = r#"
        query {
            example(input: { str: "hello" })
        }
    "#;

    let root = Query;
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;

    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "example": "String: hello" }));
}

#[tokio::test]
async fn test_schema_with_skip() {
    #[derive(OneOfInput)]
    enum ExampleInput {
        Str(String),
        Int(i32),
        #[graphql(skip)]
        Other(String),
    }

    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        async fn example(&self, input: ExampleInput) -> String {
            match input {
                ExampleInput::Str(s) => {
                    format!("String: {}", s)
                }
                ExampleInput::Int(i) => {
                    format!("Int: {}", i)
                }
                ExampleInput::Other(s) => {
                    format!("Other: {}", s)
                }
            }
        }
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r"
    input ExampleInput @oneOf {
      str: String
      int: Int
    }

    type Query {
      example(input: ExampleInput!): String!
    }

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @oneOf on INPUT_OBJECT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    ");

    let query = r#"
        query {
            example(input: { str: "hello" })
        }
    "#;

    let root = Query;
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;

    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "example": "String: hello" }));
}

#[tokio::test]
async fn test_schema_with_doc() {
    /// the example oneOf input
    #[derive(OneOfInput)]
    enum ExampleInput {
        /// a string
        Str(String),
        Int(i32),
    }

    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        async fn example(&self, input: ExampleInput) -> String {
            match input {
                ExampleInput::Str(s) => {
                    format!("String: {}", s)
                }
                ExampleInput::Int(i) => {
                    format!("Int: {}", i)
                }
            }
        }
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r#"
    "the example oneOf input"
    input ExampleInput @oneOf {
      "a string" str: String
      int: Int
    }

    type Query {
      example(input: ExampleInput!): String!
    }

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @oneOf on INPUT_OBJECT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    "#);

    let query = r#"
        query {
            example(input: { str: "hello" })
        }
    "#;

    let root = Query;
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;

    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "example": "String: hello" }));
}

#[tokio::test]
async fn test_rename_fields() {
    #[derive(OneOfInput)]
    #[graphql(rename_fields = "UPPERCASE")]
    enum ExampleInput {
        Str(String),
        Int(i32),
    }

    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        async fn example(&self, input: ExampleInput) -> String {
            match input {
                ExampleInput::Str(s) => {
                    format!("String: {}", s)
                }
                ExampleInput::Int(i) => {
                    format!("Int: {}", i)
                }
            }
        }
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r"
    input ExampleInput @oneOf {
      STR: String
      INT: Int
    }

    type Query {
      example(input: ExampleInput!): String!
    }

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @oneOf on INPUT_OBJECT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    ");

    let query = r#"
        query {
            example(input: { STR: "hello" })
        }
    "#;

    let root = Query;
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;

    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "example": "String: hello" }));
}

#[tokio::test]
async fn test_auto_register() {
    #[derive(SimpleObject)]
    struct Foo {
        pub string: String,
    }

    #[derive(InputObject)]
    struct FooInput {
        pub string: String,
    }

    #[derive(OneOfInput)]
    #[graphql(register(Foo))]
    enum ExampleInput {
        Foo(FooInput),
        Str(String),
    }

    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        async fn example(&self, input: ExampleInput) -> String {
            match input {
                ExampleInput::Foo(f) => f.string,
                ExampleInput::Str(s) => s,
            }
        }
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r"
    input ExampleInput @oneOf {
      foo: FooInput
      str: String
    }

    type Foo {
      string: String!
    }

    input FooInput {
      string: String!
    }

    type Query {
      example(input: ExampleInput!): String!
    }

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @oneOf on INPUT_OBJECT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    ");

    let query = r#"
        query {
            example(input: { foo: { string: "hello" } })
        }
    "#;

    let root = Query;
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;

    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "example": "hello" }));
}

mod in_mod {
    use dynamic_graphql::App;
    use dynamic_graphql::FieldValue;
    use dynamic_graphql::ResolvedObject;
    use dynamic_graphql::ResolvedObjectFields;
    use dynamic_graphql::dynamic::DynamicRequestExt;

    use crate::one_of_input::tests::in_mod::example::ExampleInput;
    use crate::schema_utils::normalize_schema;

    mod foo {
        use dynamic_graphql::InputObject;

        #[derive(InputObject)]
        pub struct FooInput {
            pub string: String,
        }
    }
    mod example {
        use dynamic_graphql_derive::OneOfInput;

        #[derive(OneOfInput)]
        pub enum ExampleInput {
            Foo(super::foo::FooInput),
            Str(String),
        }
    }
    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        async fn example(&self, input: example::ExampleInput) -> String {
            match input {
                ExampleInput::Foo(f) => f.string,
                ExampleInput::Str(s) => s,
            }
        }
    }

    #[derive(App)]
    struct App(Query, foo::FooInput, example::ExampleInput);

    #[tokio::test]
    async fn test_schema() {
        let schema = App::create_schema().finish().unwrap();

        let sdl = schema.sdl();

        insta::assert_snapshot!(normalize_schema(&sdl), @r"
        input ExampleInput @oneOf {
          foo: FooInput
          str: String
        }

        input FooInput {
          string: String!
        }

        type Query {
          example(input: ExampleInput!): String!
        }

        directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

        directive @oneOf on INPUT_OBJECT

        directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

        schema {
          query: Query
        }
        ");

        let query = r#"
            query {
                example(input: { foo: { string: "hello" } })
            }
        "#;
        let root = Query;
        let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));

        let res = schema.execute(req).await;
        let data = res.data.into_json().unwrap();

        assert_eq!(data, serde_json::json!({ "example": "hello" }));
    }
}
