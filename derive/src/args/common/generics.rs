use syn::parse_quote;
use syn::GenericParam;
use syn::Generics;

pub fn add_new_lifetime_to_generics(generics: &Generics) -> (Generics, GenericParam) {
    let mut generics = generics.clone();
    let lifetime: GenericParam = parse_quote!('__dynamic_graphql_lifetime);
    generics.params.push(lifetime.clone());
    (generics, lifetime)
}

pub fn add_where_static_to_generic_lifetimes(generics: &Generics) -> Generics {
    let mut generics = generics.clone();

    for param in &mut generics.params {
        if let GenericParam::Lifetime(lifetime) = param {
            lifetime.bounds.push(parse_quote!('static));
        }
    }

    generics
}

pub fn replace_type_generics_with_static(ty: &syn::Type) -> syn::Type {
    let mut ty = ty.clone();

    match &mut ty {
        syn::Type::Path(type_path) => {
            for segment in &mut type_path.path.segments {
                if let syn::PathArguments::AngleBracketed(args) = &mut segment.arguments {
                    for arg in &mut args.args {
                        match arg {
                            syn::GenericArgument::Type(ty) => {
                                *ty = replace_type_generics_with_static(ty);
                            }
                            syn::GenericArgument::Lifetime(lifetime) => {
                                lifetime.ident = syn::Ident::new("static", lifetime.ident.span())
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        syn::Type::Reference(type_ref) => {
            if let Some(lifetime) = &mut type_ref.lifetime {
                lifetime.ident = syn::Ident::new("static", lifetime.ident.span())
            }
        }
        _ => {}
    }
    ty
}
