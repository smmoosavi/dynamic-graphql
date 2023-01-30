use dynamic_graphql::dynamic::DynamicRequestExt;
use dynamic_graphql::{App, Context, ExpandObject, ExpandObjectFields, FieldValue, SimpleObject};

use crate::schema_utils::normalize_schema;

#[test]
fn test_schema() {
    #[derive(ExpandObject)]
    struct ExampleQuery<'a>(&'a Query);

    #[ExpandObjectFields]
    impl ExampleQuery<'_> {
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
    struct ExampleApp(ExampleQuery<'static>);

    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        foo: String,
    }

    #[derive(App)]
    struct App(Query, ExampleApp);

    let schema = App::create_schema().finish().unwrap();
    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"
            type Query {
              foo: String!
              withoutSelf: String!
              withSelf: String!
              withArg(name: String!): String!
              withoutSelfWithArgs(name: String!): String!
              unusedArg(name: String!): String!
            }

            schema {
              query: Query
            }

            "#
        ),
    );
}

#[test]
fn test_schema_with_ctx() {
    #[derive(ExpandObject)]
    struct ExampleQuery<'a>(&'a Query);

    #[ExpandObjectFields]
    impl ExampleQuery<'_> {
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
    struct ExampleApp(ExampleQuery<'static>);

    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        foo: String,
    }

    #[derive(App)]
    struct App(Query, ExampleApp);

    let schema = App::create_schema().finish().unwrap();
    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"
            type Query {
              foo: String!
              withoutUnderline: String!
              withoutSelf: String!
              withSelf: String!
              renamed: String!
              withArg(name: String!): String!
              withCtxArg(ctx: String!): String!
            }

            schema {
              query: Query
            }

            "#
        ),
    );
}

#[test]
fn test_schema_rename_args() {
    #[derive(ExpandObject)]
    struct ExampleQuery<'a>(&'a Query);

    #[ExpandObjectFields]
    #[graphql(rename_args = "UPPERCASE")]
    impl ExampleQuery<'_> {
        fn with_arg(the_name: String, #[graphql(name = "foo")] _other: String) -> String {
            format!("Hello {}", the_name)
        }
        #[graphql(rename_args = "snake_case")]
        fn with_field_rename(the_name: String, #[graphql(name = "foo")] _other: String) -> String {
            format!("Hello {}", the_name)
        }
    }

    #[derive(App)]
    struct ExampleApp(ExampleQuery<'static>);

    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        foo: String,
    }

    #[derive(App)]
    struct App(Query, ExampleApp);

    let schema = App::create_schema().finish().unwrap();
    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"
            type Query {
              foo: String!
              withArg(THE_NAME: String!, foo: String!): String!
              withFieldRename(the_name: String!, foo: String!): String!
            }

            schema {
              query: Query
            }

            "#
        ),
    );
}

#[test]
fn test_schema_with_arg_ref() {
    #[derive(ExpandObject)]
    struct ExampleQuery<'a>(&'a Query);

    #[ExpandObjectFields]
    impl ExampleQuery<'_> {
        fn without_string_ref(name: &String) -> String {
            format!("Hello {}", name)
        }
        fn with_str(name: &str) -> String {
            format!("Hello {}", name)
        }
    }

    #[derive(App)]
    struct ExampleApp(ExampleQuery<'static>);

    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        foo: String,
    }

    #[derive(App)]
    struct App(Query, ExampleApp);

    let schema = App::create_schema().finish().unwrap();
    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"
            type Query {
              foo: String!
              withoutStringRef(name: String!): String!
              withStr(name: String!): String!
            }

            schema {
              query: Query
            }

            "#
        ),
    );
}

#[test]
fn test_schema_with_arg_option() {
    #[derive(ExpandObject)]
    struct ExampleQuery<'a>(&'a Query);

    #[ExpandObjectFields]
    impl ExampleQuery<'_> {
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
    struct ExampleApp(ExampleQuery<'static>);

    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        foo: String,
    }

    #[derive(App)]
    struct App(Query, ExampleApp);

    let schema = App::create_schema().finish().unwrap();
    let sdl = schema.sdl();
    assert_eq!(
        normalize_schema(&sdl),
        normalize_schema(
            r#"
            type Query {
              foo: String!
                withoutOption(name: String!): String!
                withOption(name: String): String!
                withOptionRef(name: String): String!
            }

            schema {
              query: Query
            }

            "#
        ),
    );
}

#[tokio::test]
async fn test_query() {
    #[derive(ExpandObject)]
    struct ExampleQuery<'a>(&'a Query);

    #[ExpandObjectFields]
    impl ExampleQuery<'_> {
        fn hello(name: String) -> String {
            format!("Hello {}", name)
        }
        fn self_hello(&self, name: String) -> String {
            format!("{} {}", self.0.greeting, name)
        }
        fn with_ctx(ctx: &Context, name: String) -> String {
            let greeter = ctx.data::<String>().unwrap();
            format!("{} {}", greeter, name)
        }
    }

    #[derive(App)]
    struct ExampleApp(ExampleQuery<'static>);

    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        greeting: String,
    }

    #[derive(App)]
    struct App(Query, ExampleApp);

    let schema = App::create_schema().finish().unwrap();

    let query = r#"{
        greeting
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
            "greeting": "Hello",
            "hello": "Hello world",
            "selfHello": "Hello world",
            "withCtx": "Hello world"
        }),
    );
}
