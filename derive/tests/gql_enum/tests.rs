use std::borrow::Cow;

use dynamic_graphql::dynamic::DynamicRequestExt;
use dynamic_graphql::internal::TypeName;
use dynamic_graphql::App;
use dynamic_graphql::Enum;
use dynamic_graphql::FieldValue;
use dynamic_graphql::ResolvedObject;
use dynamic_graphql::ResolvedObjectFields;
use dynamic_graphql::Variables;

use crate::schema_utils::normalize_schema;

#[test]
fn test_impl_object() {
    #[allow(dead_code)]
    #[derive(Enum)]
    enum Example {
        Foo,
        Bar,
    }
    assert_eq!(
        <Example as dynamic_graphql::internal::Enum>::get_enum_type_name(),
        "Example"
    );
}

#[test]
fn test_remote() {
    enum Org {
        Foo,
        Bar,
    }

    #[derive(Enum)]
    #[graphql(remote(Org))]
    enum Example {
        Foo,
        Bar,
    }

    let org: Org = Example::Foo.into();
    assert!(matches!(org, Org::Foo));

    let example: Example = Org::Bar.into();
    assert!(matches!(example, Example::Bar));
}

#[tokio::test]
async fn test_schema() {
    #[derive(Enum)]
    enum Example {
        Foo,
        Bar,
    }

    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query {
        example: Example,
    }

    #[ResolvedObjectFields]
    impl Query {
        fn example(&self) -> &Example {
            &self.example
        }
        fn by_example(&self, example: Example) -> Example {
            example
        }
    }

    #[derive(App)]
    struct App(Query, Example);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @"");

    let query = r#"
        query {
            example
            byExample(example: FOO)
        }
    "#;
    let root = Query {
        example: Example::Foo,
    };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;

    let data = res.data.into_json().unwrap();

    assert_eq!(
        data,
        serde_json::json!({ "example": "FOO", "byExample": "FOO" })
    );

    let query = r#"
        query($example: Example!) {
            byExample(example: $example)
        }
    "#;
    let root = Query {
        example: Example::Foo,
    };
    let variables = serde_json::json!({
        "example": "BAR"
    });
    let req = dynamic_graphql::Request::new(query)
        .variables(Variables::from_json(variables))
        .root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;

    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "byExample": "BAR" }));
}

#[tokio::test]
async fn test_rename() {
    #[derive(Enum)]
    #[graphql(rename_items = "lowercase")]
    #[graphql(name = "Other")]
    enum Example {
        Foo,
        #[graphql(name = "Other")]
        Bar,
    }

    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query {
        example: Example,
    }

    #[ResolvedObjectFields]
    impl Query {
        fn example(&self) -> &Example {
            &self.example
        }
        fn by_example(&self, example: Example) -> Example {
            example
        }
    }

    #[derive(App)]
    struct App(Query, Example);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @"");

    let query = r#"
        query {
            example
            byExample(example: foo)
        }
    "#;
    let root = Query {
        example: Example::Foo,
    };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;

    let data = res.data.into_json().unwrap();

    assert_eq!(
        data,
        serde_json::json!({ "example": "foo", "byExample": "foo" })
    );

    let query = r#"
        query($example: Other!) {
            byExample(example: $example)
        }
    "#;
    let root = Query {
        example: Example::Foo,
    };
    let variables = serde_json::json!({
        "example": "Other"
    });
    let req = dynamic_graphql::Request::new(query)
        .variables(Variables::from_json(variables))
        .root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;

    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "byExample": "Other" }));
}

#[tokio::test]
async fn test_type_name() {
    #[derive(Enum)]
    #[graphql(get_type_name)]
    enum Example {
        Foo,
        Bar,
    }

    impl TypeName for Example {
        fn get_type_name() -> Cow<'static, str> {
            "Other".into()
        }
    }

    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query {
        example: Example,
    }

    #[ResolvedObjectFields]
    impl Query {
        fn example(&self) -> &Example {
            &self.example
        }
        fn by_example(&self, example: Example) -> Example {
            example
        }
    }

    #[derive(App)]
    struct App(Query, Example);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @"");

    let query = r#"
        query {
            example
            byExample(example: FOO)
        }
    "#;
    let root = Query {
        example: Example::Foo,
    };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;

    let data = res.data.into_json().unwrap();

    assert_eq!(
        data,
        serde_json::json!({ "example": "FOO", "byExample": "FOO" })
    );

    let query = r#"
        query($example: Other!) {
            byExample(example: $example)
        }
    "#;
    let root = Query {
        example: Example::Foo,
    };
    let variables = serde_json::json!({
        "example": "FOO"
    });
    let req = dynamic_graphql::Request::new(query)
        .variables(Variables::from_json(variables))
        .root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;

    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "byExample": "FOO" }));
}

#[tokio::test]
async fn test_deprecation() {
    #[derive(Enum)]
    enum Example {
        #[graphql(deprecation)]
        Foo,
        #[graphql(deprecation = "This is old")]
        Bar,
    }

    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query {
        example: Example,
    }

    #[ResolvedObjectFields]
    impl Query {
        fn example(&self) -> &Example {
            &self.example
        }
        fn by_example(&self, example: Example) -> Example {
            example
        }
    }

    #[derive(App)]
    struct App(Query, Example);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @"");
}

#[tokio::test]
async fn test_doc() {
    /// the example enum
    #[derive(Enum)]
    enum Example {
        /// the foo item
        Foo,
        Bar,
    }

    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query {
        example: Example,
    }

    #[ResolvedObjectFields]
    impl Query {
        fn example(&self) -> &Example {
            &self.example
        }
        fn by_example(&self, example: Example) -> Example {
            example
        }
    }

    #[derive(App)]
    struct App(Query, Example);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @"");
}

mod in_mod {
    use dynamic_graphql::dynamic::DynamicRequestExt;
    use dynamic_graphql::App;
    use dynamic_graphql::Enum;
    use dynamic_graphql::FieldValue;
    use dynamic_graphql::ResolvedObject;
    use dynamic_graphql::ResolvedObjectFields;
    use dynamic_graphql::Variables;

    use crate::schema_utils::normalize_schema;

    mod remote {
        pub enum Example {
            Foo,
            Bar,
        }
    }

    #[derive(Enum)]
    #[graphql(remote(remote::Example))]
    enum Example {
        Foo,
        Bar,
    }

    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query {
        example: Example,
    }

    #[ResolvedObjectFields]
    impl Query {
        fn example(&self) -> &Example {
            &self.example
        }
        fn by_example(&self, example: Example) -> Example {
            example
        }
    }

    #[derive(App)]
    struct App(Query, Example);

    #[tokio::test]
    async fn test_schema() {
        let org: remote::Example = Example::Foo.into();
        assert!(matches!(org, remote::Example::Foo));

        let example: Example = remote::Example::Bar.into();
        assert!(matches!(example, Example::Bar));

        let schema = App::create_schema().finish().unwrap();

        let sdl = schema.sdl();
        insta::assert_snapshot!(normalize_schema(&sdl), @"");

        let query = r#"
        query {
            example
            byExample(example: FOO)
        }
    "#;
        let root = Query {
            example: Example::Foo,
        };
        let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
        let res = schema.execute(req).await;

        let data = res.data.into_json().unwrap();

        assert_eq!(
            data,
            serde_json::json!({ "example": "FOO", "byExample": "FOO" })
        );

        let query = r#"
        query($example: Example!) {
            byExample(example: $example)
        }
    "#;
        let root = Query {
            example: Example::Foo,
        };
        let variables = serde_json::json!({
            "example": "BAR"
        });
        let req = dynamic_graphql::Request::new(query)
            .variables(Variables::from_json(variables))
            .root_value(FieldValue::owned_any(root));
        let res = schema.execute(req).await;

        let data = res.data.into_json().unwrap();

        assert_eq!(data, serde_json::json!({ "byExample": "BAR" }));
    }
}
