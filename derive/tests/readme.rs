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
    pub struct FooApp(Foo, FooQuery<'static>);
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
