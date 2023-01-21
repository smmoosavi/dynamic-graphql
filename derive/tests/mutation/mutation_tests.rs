use dynamic_graphql::{ExpandObject, Mutation, MutationRoot, Object, ParentType};

#[test]
fn test_mutation_root() {
    #[derive(MutationRoot)]
    struct MutationRoot;

    assert_eq!(<MutationRoot as Object>::NAME, "MutationRoot");
}

#[test]
fn test_mutation_root_with_rename() {
    #[derive(MutationRoot)]
    #[graphql(name = "Mutation")]
    struct MutationRoot;

    assert_eq!(<MutationRoot as Object>::NAME, "Mutation");
}

#[test]
fn test_mutation() {
    #[derive(MutationRoot)]
    struct MutationRoot;

    #[derive(Mutation)]
    struct MyMutation(MutationRoot);

    assert_eq!(<MyMutation as ExpandObject>::NAME, "MyMutation");
    assert_eq!(
        <<MyMutation as ParentType>::Type as Object>::NAME,
        "MutationRoot"
    );
}
