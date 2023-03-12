# Dynamic graphql


[![Build Status](https://github.com/smmoosavi/dynamic-graphql/workflows/CI/badge.svg)](https://github.com/smmoosavi/dynamic-graphql/actions)
[![Latest Version](https://img.shields.io/crates/v/dynamic-graphql.svg)](https://crates.io/crates/dynamic-graphql)
[![Rust Documentation](https://docs.rs/dynamic-graphql/badge.svg)](https://docs.rs/dynamic-graphql)
![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)


extendable and dynamic graphql schema definition for [async-graphql]

## Usage

```rust
use dynamic_graphql::App;

mod foo {
    use dynamic_graphql::{App, ExpandObject, ExpandObjectFields, SimpleObject};

    use super::root::Query;

    #[derive(SimpleObject)]
    pub struct Foo {
        id: String,
        name: String,
    }

    #[derive(ExpandObject)]
    pub struct FooQuery<'a>(&'a Query);

    #[ExpandObjectFields]
    impl FooQuery<'_> {
        async fn foo(id: String) -> Foo {
            Foo {
                id,
                name: "test".to_string(),
            }
        }
    }

    #[derive(App)]
    pub struct FooApp(FooQuery<'static>);
}

mod root {
    use dynamic_graphql::SimpleObject;

    #[derive(SimpleObject)]
    #[graphql(root)]
    pub struct Query;
}

#[derive(App)]
struct App(root::Query, foo::FooApp);

#[tokio::test]
async fn test() {
    let schema = App::create_schema().finish().unwrap();

    let sdl = schema.sdl();

    assert_eq!(
        &sdl,
        r#"

type Foo {
	id: String!
	name: String!
}



type Query {
	foo(id: String!): Foo!
}


schema {
	query: Query
}
"#
    );

    let query = r#"
        query {
            foo(id: "1") {
                id
                name
            }
        }
    "#;

    let res = schema.execute(query).await;
    let data = res.data.into_json().unwrap();

    assert_eq!(
        data,
        serde_json::json!({ "foo": { "id": "1", "name": "test" } })
    );
}

```

[async-graphql]: https://crates.io/crates/async-graphql