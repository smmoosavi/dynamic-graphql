#[allow(dead_code)]
pub fn is_type_ref(ty: &syn::Type) -> bool {
    matches!(ty, syn::Type::Reference(_))
}

#[allow(dead_code)]
pub fn is_type_mut_ref(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Reference(ref r) => r.mutability.is_some(),
        _ => false,
    }
}

#[allow(dead_code)]
pub fn is_type_str(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Reference(ref r) => {
            if let syn::Type::Path(ref p) = *r.elem {
                p.path.segments.len() == 1 && p.path.segments[0].ident == "str"
            } else {
                false
            }
        }
        _ => false,
    }
}

pub fn get_owned_type(ty: &syn::Type) -> &syn::Type {
    if is_type_str(ty) {
        return ty;
    }
    match ty {
        syn::Type::Reference(ref r) => &r.elem,
        _ => ty,
    }
}
