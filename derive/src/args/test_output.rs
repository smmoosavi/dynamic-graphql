use super::*;
use crate::utils::impl_block::FromItemImpl;
use crate::FromItemTrait;
use darling::FromDeriveInput;
use prettier_please::unparse;
use unindent::Unindent;

fn pretty_expand(tokens: impl quote::ToTokens) -> String {
    let tokens = tokens.into_token_stream();
    let file: syn::File = syn::parse2(tokens).unwrap();
    unparse(&file)
}

fn derive<D: FromDeriveInput>(input: impl AsRef<str>) -> D {
    let input = input.as_ref();
    let file: syn::DeriveInput = syn::parse_str(input).unwrap();
    D::from_derive_input(&file).unwrap()
}

fn pretty_derive<D: FromDeriveInput + quote::ToTokens>(input: impl AsRef<str>) -> String {
    let input = input.as_ref().unindent();
    let derived = derive::<D>(&input);
    let pretty = pretty_expand(&derived);
    format!("{}\n{}", input, pretty)
}

fn expand_item_impl<D: FromItemImpl>(input: impl AsRef<str>) -> D {
    let input = input.as_ref();
    let mut file: syn::ItemImpl = syn::parse_str(input).unwrap();
    D::from_item_impl(&mut file).unwrap()
}

fn pretty_expand_item_impl<D: FromItemImpl + quote::ToTokens>(input: impl AsRef<str>) -> String {
    let expanded = expand_item_impl::<D>(&input);
    let pretty = pretty_expand(&expanded);
    format!("{}\n{}", input.as_ref().unindent(), pretty)
}

fn expand_item_trait<D: FromItemTrait>(input: impl AsRef<str>) -> D {
    let input = input.as_ref();
    let mut file: syn::ItemTrait = syn::parse_str(input).unwrap();
    D::from_item_trait(&mut file).unwrap()
}

fn pretty_expand_item_trait<D: FromItemTrait + quote::ToTokens>(input: impl AsRef<str>) -> String {
    let expanded = expand_item_trait::<D>(&input);
    let pretty = pretty_expand(&expanded);
    format!("{}\n{}", input.as_ref().unindent(), pretty)
}

fn md(outputs: &[&str]) -> String {
    format!("```rust\n{}\n```", outputs.join("\n\n"))
}

#[test]
fn test_app() {
    let input = r#"
        struct App(QueryRoot, PostApp);
    "#;

    let pretty = pretty_derive::<App>(input);
    let output = md(&[&pretty]);
    insta::assert_snapshot!(output);
}

#[test]
fn test_expand_object() {
    let input1 = r#"
        struct ExampleQuery<'a>(&'a Query);
    "#;

    let input2 = r#"
    impl ExampleQuery<'_> {
        fn the_example(&self) -> Example {
            Example {
                field: "field".to_string(),
            }
        }
    }
    "#;

    let pretty1 = pretty_derive::<ExpandObject>(input1);
    let pretty2 = pretty_expand_item_impl::<ExpandObjectFields>(input2);
    let output = md(&[&pretty1, &pretty2]);
    insta::assert_snapshot!(output);
}

#[test]
fn test_enum() {
    let input = r#"
        enum Example {
            Foo,
            Bar,
        }
    "#;
    let pretty = pretty_derive::<Enum>(input);
    let output = md(&[&pretty]);
    insta::assert_snapshot!(output);
}

#[test]
fn test_input_object() {
    let input = r#"
        struct ExampleInput {
            pub string: String,
        }
    "#;

    let pretty = pretty_derive::<InputObject>(input);
    let output = md(&[&pretty]);
    insta::assert_snapshot!(output);
}

#[test]
fn test_interface() {
    let input = r#"
        trait Node {
            fn id(&self) -> String;
        }

    "#;

    let pretty = pretty_expand_item_trait::<Interface>(input);
    let output = md(&[&pretty]);
    insta::assert_snapshot!(output);
}

#[test]
fn test_mutation() {
    let input1 = r#"
        struct MutationRoot;
    "#;

    let input2 = r#"
        struct MyMutation(MutationRoot);
    "#;
    let input3 = r#"
        impl MyMutation {
            fn the_example() -> String {
                "field".to_string()
            }
        }
    "#;

    let pretty1 = pretty_derive::<MutationRoot>(input1);
    let pretty2 = pretty_derive::<Mutation>(input2);
    let pretty3 = pretty_expand_item_impl::<MutationFields>(input3);
    let output = md(&[&pretty1, &pretty2, &pretty3]);
    insta::assert_snapshot!(output);
}

#[test]
fn test_resoled_object() {
    let input1 = r#"
        struct Example {
            pub field: String,
        }
    "#;

    let input2 = r#"
        impl Example {
            fn field(&self) -> &str {
                &self.field
            }
        }
    "#;

    let pretty1 = pretty_derive::<ResolvedObject>(input1);
    let pretty2 = pretty_expand_item_impl::<ResolvedObjectFields>(input2);
    let output = md(&[&pretty1, &pretty2]);
    insta::assert_snapshot!(output);
}

#[test]
fn test_scalar() {
    let input = r#"
        struct Example;
    "#;

    let pretty = pretty_derive::<Scalar>(input);
    let output = md(&[&pretty]);
    insta::assert_snapshot!(output);
}

#[test]
fn test_simple_object() {
    let input = r#"
        struct Example {
            pub field: String,
        }
    "#;

    let pretty = pretty_derive::<SimpleObject>(input);
    let output = md(&[&pretty]);
    insta::assert_snapshot!(output);
}

#[test]
fn test_union() {
    let input = r#"
        enum Animal {
            Dog(Dog),
            Cat(Cat),
        }
    "#;

    let pretty = pretty_derive::<Union>(input);
    let output = md(&[&pretty]);
    insta::assert_snapshot!(output);
}
