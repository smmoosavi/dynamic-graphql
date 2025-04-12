use std::borrow::Cow;

use dynamic_graphql::internal::Interface;
use dynamic_graphql::internal::Object;
use dynamic_graphql::internal::TypeName;
use dynamic_graphql::App;
use dynamic_graphql::ExpandObject;
use dynamic_graphql::ExpandObjectFields;
use dynamic_graphql::Interface;
use dynamic_graphql::SimpleObject;

use crate::schema_utils::normalize_schema;

#[test]
fn test_impl_interface() {
    #[Interface]
    trait Node {
        fn id(&self) -> String;
    }

    assert_eq!(<dyn Node as Interface>::get_interface_type_name(), "Node");
}

#[test]
fn test_impl_interface_with_name() {
    #[Interface]
    #[graphql(name = "Other")]
    trait Node {
        fn id(&self) -> String;
    }

    assert_eq!(<dyn Node as Interface>::get_interface_type_name(), "Other");
}

#[test]
fn test_schema() {
    #[Interface]
    trait Node {
        fn the_id(&self) -> String;
    }

    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        foo: String,
    }

    #[derive(App)]
    struct App(Query, dyn Node);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r###"
    interface Node {
      theId: String!
    }

    type Query {
      foo: String!
    }

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    "###);
}

#[test]
fn test_schema_with_name() {
    #[Interface]
    #[graphql(name = "Other")]
    trait Node {
        #[graphql(name = "id")]
        fn get_id(&self) -> String;
    }

    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        foo: String,
    }

    #[derive(App)]
    struct App(Query, dyn Node);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r###"
    interface Other {
      id: String!
    }

    type Query {
      foo: String!
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
    #[Interface]
    #[graphql(get_type_name)]
    trait Node {
        fn the_id(&self) -> String;
    }

    impl TypeName for dyn Node {
        fn get_type_name() -> Cow<'static, str> {
            "Other".into()
        }
    }

    #[derive(SimpleObject)]
    #[graphql(mark(Node))]
    struct FooNode {
        the_id: String,
    }

    #[derive(SimpleObject)]
    #[graphql(implements(Node))]
    struct BarNode {
        #[graphql(skip)]
        the_id: String,
    }

    impl Node for BarNode {
        fn the_id(&self) -> String {
            self.the_id.clone()
        }
    }

    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        foo: String,
    }

    #[derive(App)]
    struct App(Query, FooNode, BarNode, dyn Node);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r###"
    type BarNode implements Other {
      theId: String!
    }

    type FooNode implements Other {
      theId: String!
    }

    interface Other {
      theId: String!
    }

    type Query {
      foo: String!
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
    #[Interface]
    #[graphql(rename_fields = "snake_case")]
    trait Node {
        #[graphql(name = "id")]
        fn get_id(&self) -> String;

        fn the_id(&self) -> String;
    }

    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        foo: String,
    }

    #[derive(App)]
    struct App(Query, dyn Node);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r###"
    interface Node {
      id: String!
      the_id: String!
    }

    type Query {
      foo: String!
    }

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    "###);
}

#[test]
fn test_schema_description() {
    /// the interface
    #[Interface]
    trait Node {
        /// the id
        fn the_id(&self) -> String;
    }

    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        foo: String,
    }

    #[derive(App)]
    struct App(Query, dyn Node);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r#"
    "the interface"
    interface Node {
      "the id"
      theId: String!
    }

    type Query {
      foo: String!
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
    #[Interface]
    trait Node {
        #[graphql(deprecation)]
        fn the_id(&self) -> String;

        #[graphql(deprecation = "deprecated")]
        fn old(&self) -> String;
    }

    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        foo: String,
    }

    #[derive(App)]
    struct App(Query, dyn Node);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r###"
    interface Node {
      theId: String! @deprecated
      old: String! @deprecated(reason: "deprecated")
    }

    type Query {
      foo: String!
    }

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    "###);
}

#[test]
fn test_schema_with_skip() {
    #[Interface]
    trait Node {
        fn the_id(&self) -> String;
        #[allow(dead_code)]
        #[graphql(skip)]
        fn old(&self) -> String;
    }

    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        foo: String,
    }

    #[derive(App)]
    struct App(Query, dyn Node);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r###"
    interface Node {
      theId: String!
    }

    type Query {
      foo: String!
    }

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    "###);
}

#[tokio::test]
async fn test_auto_register() {
    #[derive(SimpleObject)]
    struct Bar {
        id: String,
    }

    #[derive(SimpleObject)]
    struct Foo {
        id: String,
    }
    #[Interface]
    #[graphql(register(Bar))]
    trait GetFoo {
        fn get_foo(&self) -> Foo;
    }

    #[derive(SimpleObject)]
    #[graphql(implements(GetFoo))]
    #[graphql(root)]
    struct Query;

    impl GetFoo for Query {
        fn get_foo(&self) -> Foo {
            Foo {
                id: "foo".to_string(),
            }
        }
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();
    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r###"
    type Bar {
      id: String!
    }

    type Foo {
      id: String!
    }

    interface GetFoo {
      getFoo: Foo!
    }

    type Query implements GetFoo {
      getFoo: Foo!
    }

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    "###);
}

#[tokio::test]
async fn test_auto_register_instance() {
    #[derive(ExpandObject)]
    struct WithName<'a, T>(&'a T)
    where
        T: GetFoo + Object + 'static;
    #[ExpandObjectFields]
    impl<T> WithName<'_, T>
    where
        T: GetFoo + Object + 'static,
    {
        fn name(&self) -> String {
            self.0.get_name()
        }
    }

    #[derive(SimpleObject)]
    struct Bar {
        id: String,
    }

    #[derive(SimpleObject)]
    struct Foo {
        id: String,
    }
    #[Interface]
    #[graphql(auto_register(WithName))]
    trait GetFoo {
        fn get_foo(&self) -> Foo;
        #[graphql(skip)]
        fn get_name(&self) -> String;
    }

    #[derive(SimpleObject)]
    #[graphql(implements(GetFoo))]
    #[graphql(root)]
    struct Query;

    impl GetFoo for Query {
        fn get_foo(&self) -> Foo {
            Foo {
                id: "foo".to_string(),
            }
        }
        fn get_name(&self) -> String {
            "name".to_string()
        }
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();
    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r###"
    type Foo {
      id: String!
    }

    interface GetFoo {
      getFoo: Foo!
    }

    type Query implements GetFoo {
      name: String!
      getFoo: Foo!
    }

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    "###);
}

mod in_mod {
    mod node {
        use dynamic_graphql::Interface;

        #[Interface]
        pub trait Node {
            fn id(&self) -> String;
        }
    }

    mod foo {
        use dynamic_graphql::dynamic::DynamicRequestExt;
        use dynamic_graphql::FieldValue;
        use dynamic_graphql::Instance;
        use dynamic_graphql::ResolvedObject;
        use dynamic_graphql::ResolvedObjectFields;
        use dynamic_graphql::SimpleObject;

        use crate::schema_utils::normalize_schema;

        #[derive(SimpleObject)]
        #[graphql(mark(super::node::Node))]
        struct Bar {
            id: String,
            other: String,
        }

        #[derive(SimpleObject)]
        #[graphql(implements(super::node::Node))]
        struct Foo {
            other: String,
        }

        impl super::node::Node for Foo {
            fn id(&self) -> String {
                "foo".to_string()
            }
        }

        #[derive(ResolvedObject)]
        #[graphql(root)]
        pub struct Query;

        #[ResolvedObjectFields]
        impl Query {
            async fn foo(&self) -> Instance<dyn super::node::Node> {
                Instance::new_owned(Foo {
                    other: "foo".to_string(),
                })
            }
            async fn bar(&self) -> Instance<dyn super::node::Node> {
                Instance::new_owned(Bar {
                    id: "bar".to_string(),
                    other: "bar".to_string(),
                })
            }
        }

        #[derive(dynamic_graphql::App)]
        pub struct App(Query, Bar, Foo);

        #[tokio::test]
        async fn test_in_mode() {
            let schema = App::create_schema().finish().unwrap();

            let sdl = schema.sdl();
            insta::assert_snapshot!(normalize_schema(&sdl), @r###"
            type Bar implements Node {
              id: String!
              other: String!
            }

            type Foo implements Node {
              other: String!
              id: String!
            }

            interface Node {
              id: String!
            }

            type Query {
              foo: Node!
              bar: Node!
            }

            directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

            directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

            schema {
              query: Query
            }
            "###);

            let query = r#"
                query {
                    foo {
                        id
                        ... on Foo {
                            other
                        }
                    }
                    bar {
                        id
                        ... on Bar {
                            other
                        }
                    }
                }
            "#;

            let root = Query;
            let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));

            let res = schema.execute(req).await;
            let data = res.data.into_json().unwrap();

            assert_eq!(
                data,
                serde_json::json!({
                    "foo": { "id": "foo", "other": "foo" },
                    "bar": { "id": "bar", "other": "bar" },
                })
            );
        }
    }
}
