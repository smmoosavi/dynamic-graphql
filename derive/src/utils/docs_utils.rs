use darling::FromAttributes;
use std::ops::Deref;

pub fn get_rustdoc(attrs: &[syn::Attribute]) -> Result<Option<String>, darling::Error> {
    let mut full_docs = String::new();
    for attr in attrs {
        match attr.parse_meta()? {
            syn::Meta::NameValue(nv) if nv.path.is_ident("doc") => {
                if let syn::Lit::Str(doc) = nv.lit {
                    let doc = doc.value();
                    let doc = doc.trim();
                    if !full_docs.is_empty() {
                        full_docs += "\n";
                    }
                    full_docs += doc;
                }
            }
            _ => {}
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
