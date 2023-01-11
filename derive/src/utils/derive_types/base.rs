use darling::ast::Data;
use darling::{FromDeriveInput, FromField, FromGenerics, FromVariant};

#[derive(FromDeriveInput)]
pub struct Base<V: FromVariant, F: FromField, G: FromGenerics> {
    pub ident: syn::Ident,
    pub generics: G,
    pub data: Data<V, F>,
}

#[derive(FromField)]
pub struct BaseField {
    pub ident: Option<syn::Ident>,
    pub ty: syn::Type,
}

#[derive(FromVariant)]
pub struct BaseVariant<F: FromField> {
    pub ident: syn::Ident,
    pub fields: darling::ast::Fields<F>,
}
