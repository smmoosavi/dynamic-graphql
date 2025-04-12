use proc_macro2::TokenStream;
use quote::quote;

#[allow(dead_code)]
pub fn is_type_ref(ty: &syn::Type) -> bool {
    matches!(ty, syn::Type::Reference(_))
}

#[allow(dead_code)]
pub fn is_type_mut_ref(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Reference(r) => r.mutability.is_some(),
        _ => false,
    }
}

#[allow(dead_code)]
pub fn is_type_str(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Reference(r) => {
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
        syn::Type::Reference(r) => {
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
        syn::Type::Reference(r) => &r.elem,
        _ => ty,
    }
}

pub fn get_type_path(ty: &syn::Type) -> darling::Result<&syn::Path> {
    match ty {
        syn::Type::Reference(r) => get_type_path(&r.elem),
        syn::Type::Path(p) => Ok(&p.path),
        _ => Err(darling::Error::custom("Unsupported type").with_span(ty)),
    }
}

pub fn remove_path_generics(path: &syn::Path) -> syn::Path {
    syn::Path {
        leading_colon: path.leading_colon,
        segments: path
            .segments
            .iter()
            .map(|s| syn::PathSegment {
                ident: s.ident.clone(),
                arguments: syn::PathArguments::None,
            })
            .collect(),
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
