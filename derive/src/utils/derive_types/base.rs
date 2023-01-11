use darling::ast::Data;
use darling::{FromDeriveInput, FromField, FromVariant};

#[derive(FromDeriveInput)]
pub struct Base<V: FromVariant, F: FromField> {
    pub ident: syn::Ident,
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
