use rscodeshift_core::{transform_files, Transform, TransformError};
use syn::File;

struct AssertTransform;

impl Transform for AssertTransform {
    type Error = TransformError;

    fn transform(&self, _path: &str, mut syn: File) -> Result<Option<File>, Self::Error> {
        let mut changed = false;
        syn.items.iter_mut().for_each(|item| {
            if let syn::Item::Fn(f) = item {
                f.block.stmts.iter_mut().for_each(|stmt| {
                    if let syn::Stmt::Macro(mac) = stmt {
                        if !mac.mac.path.is_ident("assert_eq") {
                            return;
                        }
                        if mac.mac.tokens.clone().into_iter().next().unwrap().to_string() != "normalize_schema" {
                            return
                        }
                        changed = true;
                        mac.mac.path = syn::parse_str("\ninsta::assert_snapshot").unwrap();
                        let left_end_index = mac
                            .mac
                            .clone()
                            .tokens
                            .into_iter()
                            .position(
                                |t| matches!(t, proc_macro2::TokenTree::Punct(p) if p.as_char() == ','),
                            )
                            .unwrap();
                        let left = mac
                            .mac
                            .tokens
                            .clone()
                            .into_iter()
                            .take(left_end_index)
                            .collect::<proc_macro2::TokenStream>();
                        let right = quote::quote!(@r"");
                        mac.mac.tokens = quote::quote!(#left, #right);
                    }
                })
            }
        });

        if changed {
            Ok(Some(syn))
        } else {
            Ok(None)
        }
    }
}

fn main() {
    transform_files("derive/tests/**/*.rs", &AssertTransform).unwrap();
}
