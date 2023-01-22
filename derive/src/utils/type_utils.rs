use proc_macro2::TokenStream;
use quote::quote;

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

/// check if the type is a `&[SomeType]`
pub fn is_type_slice(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Reference(ref r) => {
            matches!(*r.elem, syn::Type::Slice(_))
        }
        _ => false,
    }
}

pub fn get_owned_type(ty: &syn::Type) -> &syn::Type {
    if is_type_slice(ty) {
        return ty;
    }
    if is_type_str(ty) {
        return ty;
    }
    match ty {
        syn::Type::Reference(ref r) => &r.elem,
        _ => ty,
    }
}

pub fn get_type_ident(ty: &syn::Type) -> darling::Result<&syn::Ident> {
    match ty {
        syn::Type::Reference(ref r) => get_type_ident(&r.elem),
        syn::Type::Path(ref p) => Ok(&p.path.segments[0].ident),
        _ => Err(darling::Error::custom("Unsupported type").with_span(ty)),
    }
}

pub fn get_type_path(ty: &syn::Type) -> darling::Result<&syn::Path> {
    match ty {
        syn::Type::Reference(ref r) => get_type_path(&r.elem),
        syn::Type::Path(ref p) => Ok(&p.path),
        _ => Err(darling::Error::custom("Unsupported type").with_span(ty)),
    }
}

pub fn get_value_type(ty: &syn::Type) -> Option<TokenStream> {
    if is_type_slice(ty) {
        return Some(quote!(Vec<_>));
    }
    if is_type_str(ty) {
        return Some(quote!(String));
    }
    None
}

pub fn get_ref_type_lifetime(ty: &syn::Type) -> Option<&syn::Lifetime> {
    match ty {
        syn::Type::Reference(ty) => ty.lifetime.as_ref(),
        _ => None,
    }
}
