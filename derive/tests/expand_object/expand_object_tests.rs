use dynamic_graphql::{ExpandObject, Object, SimpleObject};

#[test]
fn test_impl_expand_object() {
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

    let example = Example {
        field: "field".to_string(),
    };
    let expand_example = ExpandExample(&example);
    assert_eq!(expand_example.parent().field, "field");
}

#[test]
fn test_impl_expand_object_with_generic() {
    trait GetName {
        fn get_name(&self) -> String;
    }

    #[derive(SimpleObject)]
    struct Example {
        field: String,
    }
    impl GetName for Example {
        fn get_name(&self) -> String {
            "foo".to_string()
        }
    }

    #[derive(ExpandObject)]
    struct ExpandExample<'a, T: GetName + Object>(&'a T);

    assert_eq!(
        <<ExpandExample<Example> as ExpandObject>::Target as Object>::NAME,
        "Example"
    );
    let example = Example {
        field: "field".to_string(),
    };
    let expand_example = ExpandExample(&example);
    assert_eq!(expand_example.parent().field, "field");
    assert_eq!(expand_example.parent().get_name(), "foo");
}

#[test]
fn test_impl_expand_object_with_where() {
    trait GetName {
        fn get_name(&self) -> String;
    }

    #[derive(SimpleObject)]
    struct Example {
        field: String,
    }
    impl GetName for Example {
        fn get_name(&self) -> String {
            "foo".to_string()
        }
    }

    #[derive(ExpandObject)]
    struct ExpandExample<'a, T>(&'a T)
    where
        T: GetName + Object;

    assert_eq!(
        <<ExpandExample<Example> as ExpandObject>::Target as Object>::NAME,
        "Example"
    );
    let example = Example {
        field: "field".to_string(),
    };
    let expand_example = ExpandExample(&example);
    assert_eq!(expand_example.parent().field, "field");
    assert_eq!(expand_example.parent().get_name(), "foo");
}
