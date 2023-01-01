use dynamic_graphql_derive::Object;

#[test]
fn test_impl_object() {
    #[allow(dead_code)]
    #[derive(Object)]
    struct Example {
        pub string: String,
    }
    assert_eq!(<Example as dynamic_graphql::Object>::NAME, "Example");
}

#[test]
fn test_impl_object_with_name() {
    #[allow(dead_code)]
    #[derive(Object)]
    #[graphql(name = "Other")]
    struct Example {
        pub string: String,
    }
    assert_eq!(<Example as dynamic_graphql::Object>::NAME, "Other");
}

#[test]
fn test_impl_resolvers() {
    #[allow(dead_code)]
    #[derive(Object)]
    struct Example {
        pub string: String,
    }
    let example = Example {
        string: "Hello".to_string(),
    };
    let s = example.resolve_string();
    assert_eq!(s, &"Hello".to_string());
}
