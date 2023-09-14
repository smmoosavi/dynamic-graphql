use std::ops::Deref;

use darling::FromAttributes;
use syn::{Expr, Lit, Meta};

pub fn get_rustdoc(attrs: &[syn::Attribute]) -> Result<Option<String>, darling::Error> {
    let mut full_docs = String::new();
    for attr in attrs {
        if let Meta::NameValue(name_value) = &attr.meta {
            if name_value.path.is_ident("doc") {
                if let Expr::Lit(lit) = &name_value.value {
                    if let Lit::Str(lit_str) = &lit.lit {
                        let doc = lit_str.value();
                        let doc = doc.trim();
                        if !full_docs.is_empty() {
                            full_docs += "\n";
                        }
                        full_docs += doc;
                    }
                }
            }
        }
    }

    Ok(if full_docs.is_empty() {
        None
    } else {
        Some(full_docs)
    })
}

#[derive(Debug, Clone)]
pub struct Doc {
    pub doc: Option<String>,
}

impl Deref for Doc {
    type Target = Option<String>;

    fn deref(&self) -> &Self::Target {
        &self.doc
    }
}

impl FromAttributes for Doc {
    fn from_attributes(items: &[syn::Attribute]) -> Result<Self, darling::Error> {
        let doc = get_rustdoc(items)?;
        Ok(Doc { doc })
    }
}
