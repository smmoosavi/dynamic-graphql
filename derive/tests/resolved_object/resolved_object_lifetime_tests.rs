use dynamic_graphql::dynamic::DynamicRequestExt;
use dynamic_graphql::App;
use dynamic_graphql::FieldValue;
use dynamic_graphql::ResolvedObject;
use dynamic_graphql::ResolvedObjectFields;

// use crate::schema_utils::normalize_schema;

struct Outer {
    string: String,
}

struct Inner<'a> {
    pub string_ref: &'a str,
}

impl Outer {
    fn get_inner(&self) -> Inner<'_> {
        Inner {
            string_ref: &self.string,
        }
    }
}

#[derive(ResolvedObject)]
#[graphql(root)]
struct Query {
    pub outer: Outer,
}

#[derive(ResolvedObject)]
struct Foo<'a> {
    pub inner: Inner<'a>,
}

#[ResolvedObjectFields]
impl Query {
    fn string(&self) -> String {
        self.outer.string.clone()
    }
    fn string_ref(&self) -> &str {
        &self.outer.string
    }
    fn foo(&self) -> Foo {
        Foo {
            inner: Inner {
                string_ref: &self.outer.string,
            },
        }
    }
}

#[ResolvedObjectFields]
impl<'a> Foo<'a> {
    fn string(&self) -> String {
        self.inner.string_ref.to_string()
    }
    fn string_ref(&self) -> &str {
        self.inner.string_ref
    }
}

//
// #[tokio::test]
// async fn test_lifetime() {
//
//     #[derive(App)]
//     struct App(Query);
//
//     let schema = App::create_schema().finish().unwrap();
//
//     let sdl = schema.sdl();
//     assert_eq!(
//         normalize_schema(&sdl),
//         normalize_schema(
//             r#"
//             type Query {
//               string: String!
//               stringRef: String!
//             }
//             schema {
//               query: Query
//             }
//             "#
//         ),
//     );
//
//     let query = r#"
//         query {
//           string
//           stringRef
//         }
//     "#;
//
//     let root = Query {
//         outer: Outer {
//             string: "Hello".to_string(),
//         },
//     };
//     let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
//     let res = schema.execute(req).await;
//     let data = res.data.into_json().unwrap();
//
//     assert_eq!(
//         data,
//         serde_json::json!({ "string": "Hello", "stringRef": "Hello" })
//     );
// }
