use crate::utils::error::GeneratorResult;

pub fn get_rustdoc(attrs: &[syn::Attribute]) -> GeneratorResult<Option<String>> {
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
