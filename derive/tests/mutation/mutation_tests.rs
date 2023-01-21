use dynamic_graphql::{MutationRoot, Object};

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
