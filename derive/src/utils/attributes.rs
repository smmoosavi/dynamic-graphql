pub trait Attributes {
    const ATTRIBUTES: &'static [&'static str];
}

pub trait CleanAttributes {
    fn clean_attributes(attrs: &mut Vec<syn::Attribute>);
}

impl<T: Attributes> CleanAttributes for T {
    fn clean_attributes(attrs: &mut Vec<syn::Attribute>) {
        attrs.retain(|attr: &syn::Attribute| {
            !T::ATTRIBUTES.iter().any(|name| attr.path().is_ident(name))
        });
    }
}
