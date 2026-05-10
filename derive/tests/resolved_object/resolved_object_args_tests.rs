use dynamic_graphql::App;
use dynamic_graphql::Context;
use dynamic_graphql::FieldValue;
use dynamic_graphql::MaybeUndefined;
use dynamic_graphql::ResolvedObject;
use dynamic_graphql::ResolvedObjectFields;
use dynamic_graphql::Result;
use dynamic_graphql::Variables;
use dynamic_graphql::dynamic::DynamicRequestExt;

use crate::schema_utils::normalize_schema;

#[test]
fn test_schema() {
    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        fn without_self() -> String {
            "Hello".to_string()
        }
        fn with_self(&self) -> String {
            "Hello".to_string()
        }
        fn with_arg(&self, name: String) -> String {
            format!("Hello {}", name)
        }
        fn without_self_with_args(name: String) -> String {
            format!("Hello {}", name)
        }
        fn unused_arg(&self, _name: String) -> String {
            "Hello".to_string()
        }
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r#"
    type Query {
      withoutSelf: String!
      withSelf: String!
      withArg(name: String!): String!
      withoutSelfWithArgs(name: String!): String!
      unusedArg(name: String!): String!
    }

    "Directs the executor to include this field or fragment only when the `if` argument is true."
    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    "Directs the executor to skip this field or fragment when the `if` argument is true."
    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    "#);
}

#[test]
fn test_schema_with_ctx() {
    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        #[allow(unused_variables)]
        fn without_underline(ctx: &Context) -> String {
            "Hello".to_string()
        }
        fn without_self(_ctx: &Context) -> String {
            "Hello".to_string()
        }
        fn with_self(&self, _ctx: &Context) -> String {
            "Hello".to_string()
        }
        fn renamed(&self, #[graphql(ctx)] _context: &Context) -> String {
            "Hello".to_string()
        }
        fn with_arg(name: String, _ctx: &Context) -> String {
            format!("Hello {}", name)
        }
        fn with_ctx_arg(#[graphql(name = "ctx")] my_ctx: String) -> String {
            format!("Hello {}", my_ctx)
        }
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r#"
    type Query {
      withoutUnderline: String!
      withoutSelf: String!
      withSelf: String!
      renamed: String!
      withArg(name: String!): String!
      withCtxArg(ctx: String!): String!
    }

    "Directs the executor to include this field or fragment only when the `if` argument is true."
    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    "Directs the executor to skip this field or fragment when the `if` argument is true."
    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    "#);
}

#[test]
fn test_schema_rename_args() {
    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    #[graphql(rename_args = "UPPERCASE")]
    impl Query {
        fn with_arg(the_name: String, #[graphql(name = "foo")] _other: String) -> String {
            format!("Hello {}", the_name)
        }
        #[graphql(rename_args = "snake_case")]
        fn with_field_rename(the_name: String, #[graphql(name = "foo")] _other: String) -> String {
            format!("Hello {}", the_name)
        }
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r#"
    type Query {
      withArg(THE_NAME: String!, foo: String!): String!
      withFieldRename(the_name: String!, foo: String!): String!
    }

    "Directs the executor to include this field or fragment only when the `if` argument is true."
    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    "Directs the executor to skip this field or fragment when the `if` argument is true."
    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    "#);
}

#[test]
fn test_schema_with_arg_ref() {
    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        fn without_string_ref(name: &String) -> String {
            format!("Hello {}", name)
        }
        fn with_str(name: &str) -> String {
            format!("Hello {}", name)
        }
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r#"
    type Query {
      withoutStringRef(name: String!): String!
      withStr(name: String!): String!
    }

    "Directs the executor to include this field or fragment only when the `if` argument is true."
    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    "Directs the executor to skip this field or fragment when the `if` argument is true."
    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    "#);
}

#[test]
fn test_schema_with_arg_option() {
    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        fn without_option(name: String) -> String {
            format!("Hello {}", name)
        }
        fn with_option(name: Option<String>) -> String {
            format!("Hello {}", name.unwrap_or_default())
        }
        fn with_option_ref(name: &Option<String>) -> String {
            format!("Hello {}", name.as_ref().unwrap_or(&"".to_string()))
        }
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r#"
    type Query {
      withoutOption(name: String!): String!
      withOption(name: String): String!
      withOptionRef(name: String): String!
    }

    "Directs the executor to include this field or fragment only when the `if` argument is true."
    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    "Directs the executor to skip this field or fragment when the `if` argument is true."
    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    "#);
}

#[tokio::test]
async fn test_query() {
    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query {
        greeting: String,
    }

    #[ResolvedObjectFields]
    impl Query {
        fn hello(name: String) -> String {
            format!("Hello {}", name)
        }
        fn self_hello(&self, name: String) -> String {
            format!("{} {}", self.greeting, name)
        }
        fn with_ctx(ctx: &Context, name: String) -> String {
            let greeter = ctx.data::<String>().unwrap();
            format!("{} {}", greeter, name)
        }
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();

    let query = r#"{
        hello(name: "world")
        selfHello(name: "world")
        withCtx(name: "world")
     }"#;
    let root = Query {
        greeting: "Hello".to_string(),
    };
    let req = dynamic_graphql::Request::new(query)
        .data("Hello".to_string())
        .root_value(FieldValue::owned_any(root));

    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();
    assert_eq!(
        data,
        serde_json::json!({
            "hello": "Hello world",
            "selfHello": "Hello world",
            "withCtx": "Hello world"
        }),
    );
}

#[tokio::test]
async fn test_query_with_option() {
    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        fn with_option(name: Option<String>) -> String {
            match name {
                Some(name) => format!("Some({})", name),
                None => "None".to_string(),
            }
        }
        fn with_option_ref(name: &Option<String>) -> String {
            match name {
                Some(name) => format!("Some({})", name),
                None => "None".to_string(),
            }
        }
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();

    let query = r#"
    query {
        withOption
        withOptionRef
    }"#;

    let req = dynamic_graphql::Request::new(query);

    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();
    assert_eq!(
        data,
        serde_json::json!({
            "withOption": "None",
            "withOptionRef": "None"
        }),
    );

    let query = r#"
    query ($name: String) {
        withOption(name: $name)
        withOptionRef(name: $name)
    }"#;

    let variables = serde_json::json!({ "name": "world" });

    let req = dynamic_graphql::Request::new(query).variables(Variables::from_json(variables));

    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(
        data,
        serde_json::json!({
            "withOption": "Some(world)",
            "withOptionRef": "Some(world)"
        }),
    );

    let variables = serde_json::json!({ "name": null });

    let req = dynamic_graphql::Request::new(query).variables(Variables::from_json(variables));

    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();
    assert_eq!(
        data,
        serde_json::json!({
            "withOption": "None",
            "withOptionRef": "None"
        }),
    );

    let variables = serde_json::json!({});

    let req = dynamic_graphql::Request::new(query).variables(Variables::from_json(variables));

    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();
    assert_eq!(
        data,
        serde_json::json!({
            "withOption": "None",
            "withOptionRef": "None"
        }),
    );
}

#[tokio::test]
async fn test_query_with_result() {
    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        fn with_result(value: Result<u8>) -> String {
            match value {
                Ok(name) => format!("Ok({})", name),
                Err(err) => format!("Err({})", err.message),
            }
        }

        fn with_result_of_option(value: Result<Option<u8>>) -> String {
            match value {
                Ok(Some(name)) => format!("Ok(Some({}))", name),
                Ok(None) => "Ok(None)".to_string(),
                Err(err) => format!("Err({})", err.message),
            }
        }
        fn with_option_of_result(value: Option<Result<u8>>) -> String {
            match value {
                Some(Ok(name)) => format!("Some(Ok({}))", name),
                Some(Err(err)) => format!("Some(Err({}))", err.message),
                None => "None".to_string(),
            }
        }
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();

    let query = r#"
    query {
        withResult
    }"#;

    let req = dynamic_graphql::Request::new(query);

    let res = schema.execute(req).await;
    assert_eq!(res.errors.len(), 1);
    assert_eq!(
        res.errors[0].message,
        r#"Invalid value for argument "value": Failed to parse "Int": internal: key "value" not found"#
    );

    let query = r#"
    query {
        withResultOfOption
    }"#;

    let req = dynamic_graphql::Request::new(query);

    let res = schema.execute(req).await;
    assert_eq!(res.errors.len(), 1);
    assert_eq!(
        res.errors[0].message,
        r#"Invalid value for argument "value": Failed to parse "Int": internal: key "value" not found"#
    );

    let query = r#"
    query {
        withOptionOfResult
    }"#;

    let req = dynamic_graphql::Request::new(query);

    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();
    assert_eq!(
        data,
        serde_json::json!({
            "withOptionOfResult":"None",
        }),
    );

    let query = r#"
    query ($value: Int) {
        withResult(value: $value)
        withResultOfOption(value: $value)
        withOptionOfResult(value: $value)
    }"#;

    let variables = serde_json::json!({ "value": 255 }); // max u8

    let req = dynamic_graphql::Request::new(query).variables(Variables::from_json(variables));

    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(
        data,
        serde_json::json!({
            "withResult": "Ok(255)",
            "withResultOfOption": "Ok(Some(255))",
            "withOptionOfResult": "Some(Ok(255))",
        }),
    );

    let variables = serde_json::json!({ "value": 2565 }); // max u8 + 1

    let req = dynamic_graphql::Request::new(query).variables(Variables::from_json(variables));

    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(
        data,
        serde_json::json!({
            "withResult":"Err(Failed to parse \"Int\": Only integers from 0 to 255 are accepted for u8.)",
            "withResultOfOption":"Err(Failed to parse \"Int\": Only integers from 0 to 255 are accepted for u8.)",
            "withOptionOfResult":"Some(Err(Failed to parse \"Int\": Only integers from 0 to 255 are accepted for u8.))",
        }),
    );

    let variables = serde_json::json!({ "value": null });

    let req = dynamic_graphql::Request::new(query).variables(Variables::from_json(variables));

    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(
        data,
        serde_json::json!({
            "withResult": "Err(Failed to parse \"Int\": internal: not an unsigned integer)",
            "withResultOfOption": "Ok(None)",
            "withOptionOfResult": "None",
        }),
    );

    let variables = serde_json::json!({});

    let req = dynamic_graphql::Request::new(query).variables(Variables::from_json(variables));

    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(
        data,
        serde_json::json!({
            "withResult": "Err(Failed to parse \"Int\": internal: not an unsigned integer)",
            "withResultOfOption": "Ok(None)",
            "withOptionOfResult": "None",
        }),
    );
}

#[tokio::test]
async fn test_query_with_maybe_undefined() {
    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        fn with_option(name: MaybeUndefined<String>) -> String {
            match name {
                MaybeUndefined::Value(name) => format!("Some({})", name),
                MaybeUndefined::Undefined => "Undefined".to_string(),
                MaybeUndefined::Null => "Null".to_string(),
            }
        }
        fn with_option_ref(name: &MaybeUndefined<String>) -> String {
            match name {
                MaybeUndefined::Value(name) => format!("Some({})", name),
                MaybeUndefined::Undefined => "Undefined".to_string(),
                MaybeUndefined::Null => "Null".to_string(),
            }
        }
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();

    let query = r#"
    query {
        withOption
        withOptionRef
    }"#;

    let req = dynamic_graphql::Request::new(query);

    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();
    assert_eq!(
        data,
        serde_json::json!({
            "withOption": "Undefined",
            "withOptionRef": "Undefined"
        }),
    );

    let query = r#"
    query ($name: String) {
        withOption(name: $name)
        withOptionRef(name: $name)
    }"#;

    let variables = serde_json::json!({ "name": "world" });

    let req = dynamic_graphql::Request::new(query).variables(Variables::from_json(variables));

    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(
        data,
        serde_json::json!({
            "withOption": "Some(world)",
            "withOptionRef": "Some(world)"
        }),
    );

    let variables = serde_json::json!({ "name": null });

    let req = dynamic_graphql::Request::new(query).variables(Variables::from_json(variables));

    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();
    assert_eq!(
        data,
        serde_json::json!({
            "withOption": "Null",
            "withOptionRef": "Null"
        }),
    );

    let variables = serde_json::json!({});

    let req = dynamic_graphql::Request::new(query).variables(Variables::from_json(variables));

    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();
    assert_eq!(
        data,
        serde_json::json!({
            "withOption": "Null",
            "withOptionRef": "Null"
        }),
    );
}

#[tokio::test]
async fn test_query_numbers() {
    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query {}

    #[ResolvedObjectFields]
    impl Query {
        fn by_u8(&self, name: u8) -> String {
            format!("u8: {}", name)
        }
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();

    insta::assert_snapshot!(normalize_schema(&sdl), @r#"
    type Query {
      byU8(name: Int!): String!
    }

    "Directs the executor to include this field or fragment only when the `if` argument is true."
    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    "Directs the executor to skip this field or fragment when the `if` argument is true."
    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    "#);

    let query = r#"{
        byU8(name: 1)
     }"#;
    let root = Query {};
    let req = dynamic_graphql::Request::new(query)
        .data("Hello".to_string())
        .root_value(FieldValue::owned_any(root));

    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();
    assert_eq!(
        data,
        serde_json::json!({
            "byU8": "u8: 1",
        }),
    );

    // error on overflow
    let query = r#"{
        byU8(name: 300)
     }"#;
    let root = Query {};
    let req = dynamic_graphql::Request::new(query)
        .data("Hello".to_string())
        .root_value(FieldValue::owned_any(root));

    let res = schema.execute(req).await;
    println!("{:?}", res.errors);
    assert_eq!(
        res.errors[0].message,
        r#"Invalid value for argument "name": Failed to parse "Int": Only integers from 0 to 255 are accepted for u8."#,
    );
}

#[test]
fn test_arg_descriptions_via_attr_and_doc_comment() {
    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query;

    #[ResolvedObjectFields]
    impl Query {
        /// Greeter.
        ///
        /// * `name` — the name of the person to greet.
        /// * `times` — how many times to repeat.
        fn greet_via_doc(&self, name: String, times: i32) -> String {
            name.repeat(times as usize)
        }

        /// `desc` attribute wins over a generic doc comment when both are
        /// present (only the `desc` attribute is set here so we can verify
        /// it lands in the schema).
        fn greet_via_attr(
            &self,
            #[graphql(desc = "the salutation, e.g. 'hi' or 'hello'")] greeting: String,
        ) -> String {
            greeting
        }
    }

    #[derive(App)]
    struct App(Query);

    let sdl = App::create_schema().finish().unwrap().sdl();

    insta::assert_snapshot!(normalize_schema(&sdl), @r#"
    type Query {
      """
        Greeter.

        * `name` — the name of the person to greet.
        * `times` — how many times to repeat.
      """
      greetViaDoc(name: String!, times: Int!): String!
      """
        `desc` attribute wins over a generic doc comment when both are
        present (only the `desc` attribute is set here so we can verify
        it lands in the schema).
      """
      greetViaAttr("the salutation, e.g. 'hi' or 'hello'" greeting: String!): String!
    }

    "Directs the executor to include this field or fragment only when the `if` argument is true."
    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    "Directs the executor to skip this field or fragment when the `if` argument is true."
    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    "#);

    // Each arg description appears as a `"""…"""` block immediately above
    // the arg in the SDL, separate from the field's own description.
    fn has_arg_desc(sdl: &str, arg_name: &str, expected_desc: &str) -> bool {
        sdl.lines().collect::<Vec<_>>().windows(4).any(|w| {
            w[0].trim() == "\"\"\""
                && w[1].trim() == expected_desc
                && w[2].trim() == "\"\"\""
                && w[3].trim().starts_with(arg_name)
        })
    }

    assert!(
        has_arg_desc(&sdl, "greeting", "the salutation, e.g. 'hi' or 'hello'"),
        "expected greeting arg description in SDL:\n{sdl}"
    );
}
