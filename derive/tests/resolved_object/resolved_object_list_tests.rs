use dynamic_graphql::dynamic::DynamicRequestExt;
use dynamic_graphql::App;
use dynamic_graphql::FieldValue;
use dynamic_graphql::ResolvedObject;
use dynamic_graphql::ResolvedObjectFields;
use dynamic_graphql::SimpleObject;

use crate::schema_utils::normalize_schema;

#[tokio::test]
async fn test_list() {
    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query {
        pub strings: Vec<String>,
    }
    #[ResolvedObjectFields]
    impl Query {
        fn strings(&self) -> &Vec<String> {
            &self.strings
        }
        fn new_strings(&self) -> Vec<String> {
            self.strings.clone()
        }
        fn strings_ref(&self) -> &[String] {
            &self.strings
        }
        fn ref_items(&self) -> Vec<&String> {
            self.strings.iter().collect()
        }
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @"");

    let query = r#"
        query {
            strings
            newStrings
            stringsRef
            refItems
        }
    "#;

    let root = Query {
        strings: vec!["Hello".to_string()],
    };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(
        data,
        serde_json::json!({ "strings": [ "Hello" ], "newStrings": [ "Hello" ], "stringsRef": [ "Hello" ], "refItems": [ "Hello" ] })
    );

    let root = Query { strings: vec![] };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(
        data,
        serde_json::json!({ "strings": [], "newStrings": [], "stringsRef": [], "refItems": [] })
    );
}

#[tokio::test]
async fn test_list_object() {
    #[derive(SimpleObject)]
    struct Item {
        pub name: String,
    }

    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query {
        pub items: Vec<Item>,
    }
    #[ResolvedObjectFields]
    impl Query {
        fn items(&self) -> &Vec<Item> {
            &self.items
        }
        fn new_items(&self) -> Vec<Item> {
            if self.items.is_empty() {
                vec![]
            } else {
                vec![Item {
                    name: "Hello".to_string(),
                }]
            }
        }
        fn items_ref(&self) -> &[Item] {
            &self.items
        }
        fn ref_items(&self) -> Vec<&Item> {
            self.items.iter().collect()
        }
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @"");

    let query = r#"
        query {
            items { name }
            newItems { name }
            itemsRef { name }
            refItems { name }
        }
    "#;

    let root = Query {
        items: vec![Item {
            name: "Hello".to_string(),
        }],
    };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(
        data,
        serde_json::json!({
            "items": [ { "name": "Hello" } ],
            "newItems": [ { "name": "Hello" } ],
            "itemsRef": [ { "name": "Hello" } ],
            "refItems": [ { "name": "Hello" } ]
        })
    );

    let root = Query { items: vec![] };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(
        data,
        serde_json::json!({ "items": [], "newItems": [], "itemsRef": [], "refItems": [] })
    );
}

#[tokio::test]
async fn test_optional_list() {
    #[allow(dead_code)]
    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query {
        pub maybe_list_of_strings: Option<Vec<String>>,
    }

    #[ResolvedObjectFields]
    impl Query {
        fn maybe_list_of_strings(&self) -> &Option<Vec<String>> {
            &self.maybe_list_of_strings
        }
        fn new_maybe_list_of_strings(&self) -> Option<Vec<String>> {
            self.maybe_list_of_strings.clone()
        }
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @"");

    let query = r#"
        query {
            maybeListOfStrings
            newMaybeListOfStrings
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
        serde_json::json!({ "maybeListOfStrings": [ "Hello" ], "newMaybeListOfStrings": [ "Hello" ] })
    );

    let root = Query {
        maybe_list_of_strings: None,
    };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(
        data,
        serde_json::json!({ "maybeListOfStrings": null, "newMaybeListOfStrings": null })
    );
}

#[tokio::test]
async fn test_list_of_optional() {
    #[allow(dead_code)]
    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query {
        pub list_of_maybe_strings: Vec<Option<String>>,
    }

    #[ResolvedObjectFields]
    impl Query {
        fn list_of_maybe_strings(&self) -> &Vec<Option<String>> {
            &self.list_of_maybe_strings
        }
        fn new_list_of_maybe_strings(&self) -> Vec<Option<String>> {
            self.list_of_maybe_strings.clone()
        }
        fn list_of_maybe_strings_ref(&self) -> &[Option<String>] {
            &self.list_of_maybe_strings
        }
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @"");

    let query = r#"
        query {
            listOfMaybeStrings
            newListOfMaybeStrings
            listOfMaybeStringsRef
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
        serde_json::json!({
            "listOfMaybeStrings": [ "Hello", null ],
            "newListOfMaybeStrings": [ "Hello", null ],
            "listOfMaybeStringsRef": [ "Hello", null ]
        })
    );
}

#[tokio::test]
async fn test_optional_list_of_optional() {
    #[allow(dead_code)]
    #[derive(ResolvedObject)]
    #[graphql(root)]
    struct Query {
        pub maybe_list_of_maybe_strings: Option<Vec<Option<String>>>,
    }

    #[ResolvedObjectFields]
    impl Query {
        fn maybe_list_of_maybe_strings(&self) -> &Option<Vec<Option<String>>> {
            &self.maybe_list_of_maybe_strings
        }
        fn new_maybe_list_of_maybe_strings(&self) -> Option<Vec<Option<String>>> {
            self.maybe_list_of_maybe_strings.clone()
        }
    }

    #[derive(App)]
    struct App(Query);

    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();
    insta::assert_snapshot!(normalize_schema(&sdl), @"");

    let query = r#"
        query {
            maybeListOfMaybeStrings
            newMaybeListOfMaybeStrings
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
        serde_json::json!({ "maybeListOfMaybeStrings": [ "Hello", null ], "newMaybeListOfMaybeStrings": [ "Hello", null ] })
    );

    let root = Query {
        maybe_list_of_maybe_strings: None,
    };
    let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
    let res = schema.execute(req).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(
        data,
        serde_json::json!({ "maybeListOfMaybeStrings": null, "newMaybeListOfMaybeStrings": null })
    );
}
