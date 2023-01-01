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
