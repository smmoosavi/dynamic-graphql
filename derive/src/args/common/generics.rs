use darling::ast::GenericParamExt;
use syn::{parse_quote, GenericParam, Generics, WherePredicate};

pub fn add_static_to_types_in_where_clause(generics: &Generics) -> Generics {
    let mut generics = generics.clone();
    let predicates: Vec<WherePredicate> = generics
        .params
        .iter()
        .filter_map(|param| param.as_type_param())
        .map(|param| {
            let ty = &param.ident;
            parse_quote! {
                #ty: 'static
            }
        })
        .collect();

    generics.make_where_clause().predicates.extend(predicates);
    generics
}

pub fn replace_generic_lifetime_with_static(generics: &syn::Generics) -> syn::Generics {
    let mut generics = generics.clone();
    generics.params.iter_mut().for_each(|param| {
        if let syn::GenericParam::Lifetime(lifetime) = param {
            lifetime.lifetime.ident = syn::Ident::new("static", lifetime.lifetime.ident.span());
        }
    });
    generics
}

pub fn replace_path_lifetime_with_static(path: &syn::Path) -> syn::Path {
    let mut path = path.clone();
    path.segments.iter_mut().for_each(|segment| {
        if let syn::PathArguments::AngleBracketed(args) = &mut segment.arguments {
            args.args.iter_mut().for_each(|arg| {
                if let syn::GenericArgument::Lifetime(lifetime) = arg {
                    lifetime.ident = syn::Ident::new("static", lifetime.ident.span());
                }
            });
        }
    });
    path
}

pub fn add_new_lifetime_to_generics(generics: &Generics) -> (Generics, GenericParam) {
    let mut generics = generics.clone();
    let lifetime: GenericParam = parse_quote!('__dynamic_graphql_lifetime);
    generics.params.push(lifetime.clone());
    (generics, lifetime)
}
