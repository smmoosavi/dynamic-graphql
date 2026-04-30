use syn::Expr;
use syn::Lit;
use syn::Meta;

pub fn get_rustdoc(attrs: &[syn::Attribute]) -> Result<Option<String>, darling::Error> {
    let mut full_docs = String::new();
    for attr in attrs {
        if let Meta::NameValue(name_value) = &attr.meta
            && name_value.path.is_ident("doc")
            && let Expr::Lit(lit) = &name_value.value
            && let Lit::Str(lit_str) = &lit.lit
        {
            let doc = lit_str.value();
            let doc = doc.trim();
            if !full_docs.is_empty() {
                full_docs += "\n";
            }
            full_docs += doc;
        }
    }

    Ok(if full_docs.is_empty() {
        None
    } else {
        Some(full_docs)
    })
}
