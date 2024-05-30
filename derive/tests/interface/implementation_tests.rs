use dynamic_graphql::App;
use dynamic_graphql::Interface;
use dynamic_graphql::ResolvedObject;
use dynamic_graphql::ResolvedObjectFields;
use dynamic_graphql::SimpleObject;

use crate::schema_utils::normalize_schema;

#[test]
fn test_schema_simple_object_mark_with() {
    #[Interface]
    trait Node {
        fn the_id(&self) -> String;
    }

    #[derive(SimpleObject)]
    #[graphql(mark(Node))]
    struct FooNode {
        the_id: String,
    }

    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        foo: FooNode,
    }

    #[derive(App)]
    struct App(Query, FooNode);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r###"
    type FooNode implements Node {
      theId: String!
    }

    interface Node {
      theId: String!
    }

    type Query {
      foo: FooNode!
    }

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    "###);
}

#[test]
fn test_schema_simple_object_with_implement() {
    #[Interface]
    trait Node {
        fn the_id(&self) -> &String;
    }

    #[derive(SimpleObject)]
    #[graphql(implements(Node))]
    struct FooNode {
        some_field: String,
        #[graphql(skip)]
        id: String,
    }

    impl Node for FooNode {
        fn the_id(&self) -> &String {
            &self.id
        }
    }

    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        foo: FooNode,
    }

    #[derive(App)]
    struct App(Query, FooNode);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r###"
    type FooNode implements Node {
      someField: String!
      theId: String!
    }

    interface Node {
      theId: String!
    }

    type Query {
      foo: FooNode!
    }

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    "###);
}

#[test]
fn test_schema_simple_object_with_error() {
    #[Interface]
    trait Node {
        fn the_id(&self) -> String;
    }

    #[derive(SimpleObject)]
    #[graphql(mark(Node))]
    struct FooNode {
        other_field: String,
    }

    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        foo: FooNode,
    }

    #[derive(App)]
    struct App(Query, FooNode);

    let schema = App::create_schema().finish();

    assert!(schema.is_err());
    assert_eq!(
        schema.err().unwrap().to_string(),
        r#"Object "FooNode" requires field "theId" defined by interface "Node""#
    );
}

#[test]
fn test_schema_resolved_object_mark_with() {
    #[Interface]
    trait Node {
        fn the_id(&self) -> String;
    }

    #[derive(ResolvedObject)]
    #[graphql(mark(Node))]
    struct FooNode;

    #[ResolvedObjectFields]
    impl FooNode {
        fn the_id(&self) -> String {
            "the_id".to_string()
        }
    }
    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        foo: FooNode,
    }

    #[derive(App)]
    struct App(Query, FooNode);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r###"
    type FooNode implements Node {
      theId: String!
    }

    interface Node {
      theId: String!
    }

    type Query {
      foo: FooNode!
    }

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    "###);
}

#[test]
fn test_schema_resolved_object_with_implement() {
    #[Interface]
    trait Node {
        fn the_id(&self) -> String;
    }

    #[derive(ResolvedObject)]
    #[graphql(implements(Node))]
    struct FooNode;

    #[ResolvedObjectFields]
    impl FooNode {
        fn other_fields(&self) -> String {
            "other".to_string()
        }
    }

    impl Node for FooNode {
        fn the_id(&self) -> String {
            "the_id".to_string()
        }
    }

    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        foo: FooNode,
    }

    #[derive(App)]
    struct App(Query, FooNode);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r###"
    type FooNode implements Node {
      otherFields: String!
      theId: String!
    }

    interface Node {
      theId: String!
    }

    type Query {
      foo: FooNode!
    }

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    "###);
}

#[test]
fn test_schema_resolved_object_with_error() {
    #[Interface]
    trait Node {
        fn the_id(&self) -> String;
    }

    #[derive(ResolvedObject)]
    #[graphql(mark(Node))]
    struct FooNode;

    #[ResolvedObjectFields]
    impl FooNode {
        fn other_field(&self) -> String {
            "the_id".to_string()
        }
    }

    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        foo: FooNode,
    }

    #[derive(App)]
    struct App(Query, FooNode);

    let schema = App::create_schema().finish();

    assert!(schema.is_err());
    assert_eq!(
        schema.err().unwrap().to_string(),
        r#"Object "FooNode" requires field "theId" defined by interface "Node""#
    );
}
