use dynamic_graphql::dynamic::DynamicRequestExt;
use dynamic_graphql::App;
use dynamic_graphql::FieldValue;
use dynamic_graphql::SimpleObject;

use crate::schema_utils::normalize_schema;

#[tokio::test]
async fn test_list() {
    #[allow(dead_code)]
    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        pub strings: Vec<String>,
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r"
    type Query {
      strings: [String!]!
    }

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    ");

    let query = r#"
        query {
            strings
        }
    "#;

    let root = Query {
        strings: vec!["Hello".to_string()],
    };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "strings": [ "Hello" ] }));

    let root = Query { strings: vec![] };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "strings": [] }));
}

#[tokio::test]
async fn test_optional_list() {
    #[allow(dead_code)]
    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        pub maybe_list_of_strings: Option<Vec<String>>,
    }
    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r"
    type Query {
      maybeListOfStrings: [String!]
    }

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    ");

    let query = r#"
        query {
            maybeListOfStrings
        }
    "#;

    let root = Query {
        maybe_list_of_strings: Some(vec!["Hello".to_string()]),
    };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(
        data,
        serde_json::json!({ "maybeListOfStrings": [ "Hello" ] })
    );

    let root = Query {
        maybe_list_of_strings: None,
    };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "maybeListOfStrings": null }));
}

#[tokio::test]
async fn test_list_of_optional() {
    #[allow(dead_code)]
    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        pub list_of_maybe_strings: Vec<Option<String>>,
    }
    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r"
    type Query {
      listOfMaybeStrings: [String]!
    }

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    ");

    let query = r#"
        query {
            listOfMaybeStrings
        }
    "#;

    let root = Query {
        list_of_maybe_strings: vec![Some("Hello".to_string()), None],
    };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(
        data,
        serde_json::json!({ "listOfMaybeStrings": [ "Hello", null ] })
    );
}

#[tokio::test]
async fn test_optional_list_of_optional() {
    #[allow(dead_code)]
    #[derive(SimpleObject)]
    #[graphql(root)]
    struct Query {
        pub maybe_list_of_maybe_strings: Option<Vec<Option<String>>>,
    }
    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @r"
    type Query {
      maybeListOfMaybeStrings: [String]
    }

    directive @include(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    directive @skip(if: Boolean!) on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT

    schema {
      query: Query
    }
    ");

    let query = r#"
        query {
            maybeListOfMaybeStrings
        }
    "#;

    let root = Query {
        maybe_list_of_maybe_strings: Some(vec![Some("Hello".to_string()), None]),
    };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(
        data,
        serde_json::json!({ "maybeListOfMaybeStrings": [ "Hello", null ] })
    );

    let root = Query {
        maybe_list_of_maybe_strings: None,
    };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(data, serde_json::json!({ "maybeListOfMaybeStrings": null }));
}
