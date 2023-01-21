use syn::{parse_quote, GenericParam, Generics};

pub fn add_new_lifetime_to_generics(generics: &Generics) -> (Generics, GenericParam) {
    let mut generics = generics.clone();
    let lifetime: GenericParam = parse_quote!('__dynamic_graphql_lifetime);
    generics.params.push(lifetime.clone());
    (generics, lifetime)
}
