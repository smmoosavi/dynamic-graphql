use dynamic_graphql::{ExpandObject, Object, SimpleObject};

#[test]
fn test_impl_resolved_object() {
    #[derive(SimpleObject)]
    struct Example {
        field: String,
    }

    #[derive(ExpandObject)]
    struct ExpandExample<'a>(&'a Example);

    assert_eq!(
        <<ExpandExample as ExpandObject>::Target as Object>::NAME,
        "Example"
    );
}

#[test]
fn test_impl_prent() {
    #[derive(SimpleObject, Eq, PartialEq, Debug)]
    struct Example {
        field: String,
    }

    #[derive(ExpandObject)]
    struct ExpandExample<'a>(&'a Example);

    let example = Example {
        field: "field".to_string(),
    };
    let expand_example = ExpandExample(&example);
    assert_eq!(expand_example.parent(), &example);
}
