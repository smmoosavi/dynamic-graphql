use crate::args::common::{
    field_deprecation, impl_define_object, impl_object, impl_resolve_owned, impl_resolve_ref,
};
use crate::utils::crate_name::get_create_name;
use crate::utils::deprecation::Deprecation;
use crate::utils::docs_utils::Doc;
use crate::utils::error::{GeneratorResult, IntoTokenStream, WithSpan};
use crate::utils::rename_rule::{calc_field_name, RenameRule};
use darling::ast::{Data, Fields};
use darling::util::Ignored;
use darling::{FromAttributes, ToTokens};
use darling::{FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::quote;

#[derive(FromField)]
#[darling(attributes(graphql), forward_attrs(doc))]
pub struct ObjectField {
    pub ident: Option<syn::Ident>,
    pub ty: syn::Type,
    pub attrs: Vec<syn::Attribute>,

    #[darling(default)]
    pub skip: bool,

    #[darling(default)]
    pub name: Option<String>,

    #[darling(default)]
    pub deprecation: Deprecation,
}

#[derive(FromDeriveInput)]
#[darling(attributes(graphql), forward_attrs(doc))]
pub struct SimpleObject {
    pub ident: syn::Ident,
    pub data: Data<Ignored, ObjectField>,
    pub attrs: Vec<syn::Attribute>,

    #[darling(default)]
    pub name: Option<String>,

    #[darling(default)]
    pub rename_fields: Option<RenameRule>,
}

fn get_fields(object: &SimpleObject) -> GeneratorResult<&Fields<ObjectField>> {
    match object.data {
        Data::Struct(ref data) => Ok(data),
        Data::Enum(_) => Err(
            darling::Error::custom("derive Object can't applied to enum")
                .with_span(&object.ident)
                .into(),
        ),
    }
}

fn get_field_ident(field: &ObjectField) -> GeneratorResult<&syn::Ident> {
    let ident = field
        .ident
        .as_ref()
        .ok_or_else(|| darling::Error::custom("derive Object can't applied to tuple struct"))?;
    Ok(ident)
}

fn get_resolver_name(name: &str) -> String {
    format!("__resolve_{}", name)
}

fn impl_resolver(field: &ObjectField) -> GeneratorResult<TokenStream> {
    // fn resolve_<field_name>(&self) -> &<field_type> { &self.<field_name> }
    let field_ident = get_field_ident(field)?;
    let name = field_ident.to_string();
    let resolver_name = get_resolver_name(&name);
    let resolver_ident = syn::Ident::new(&resolver_name, field_ident.span());
    let ty = &field.ty;
    Ok(quote! {
        fn #resolver_ident(&self) -> &#ty {
            &self.#field_ident
        }
    })
}

fn impl_resolvers(object: &SimpleObject) -> GeneratorResult<TokenStream> {
    let ident = &object.ident;
    let struct_data = get_fields(object)?;
    let fields = struct_data
        .fields
        .iter()
        .filter(|field| !field.skip)
        .map(impl_resolver)
        .map(|r| r.with_span(&object.ident).into_token_stream())
        .collect::<Vec<TokenStream>>();
    Ok(quote! {
        impl #ident {
            #(#fields)*
        }
    })
}

fn field_description(doc: &Option<String>) -> TokenStream {
    if let Some(doc) = doc {
        quote! {
            let field = field.description(#doc);
        }
    } else {
        quote! {}
    }
}

fn impl_define_field(object: &SimpleObject, field: &ObjectField) -> GeneratorResult<TokenStream> {
    let field_ident = get_field_ident(field).with_span(&object.ident)?;
    let name = field_ident.to_string();
    let field_name = calc_field_name(
        field.name.as_ref(),
        &field_ident.to_string(),
        &object.rename_fields,
    );
    let ty = &field.ty;
    let resolver_name = get_resolver_name(&name);
    let resolver_ident = syn::Ident::new(&resolver_name, field_ident.span());
    let create_name = get_create_name();
    let doc = Doc::from_attributes(&field.attrs)?;
    let description = field_description(&doc);
    let deprecation = field_deprecation(&field.deprecation);
    Ok(quote! {
        let field = #create_name::dynamic::Field::new(#field_name, <#ty as #create_name::GetOutputTypeRef>::get_output_type_ref(), |ctx| {
            #create_name::dynamic::FieldFuture::new(async move {
                let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                let value = Self::#resolver_ident(parent);
                #create_name::ResolveRef::resolve_ref(value, &ctx)
            })
        });
        #description
        #deprecation
        let object = object.field(field);
    })
}

fn object_description(doc: &Option<String>) -> GeneratorResult<TokenStream> {
    if let Some(doc) = doc {
        Ok(quote! {
            let object = object.description(#doc);
        })
    } else {
        Ok(quote! {})
    }
}

fn impl_register(object: &SimpleObject) -> GeneratorResult<TokenStream> {
    let create_name = get_create_name();
    let ident = &object.ident;
    let define_object = impl_define_object();
    let doc = Doc::from_attributes(&object.attrs)?;
    let description = object_description(&doc)?;
    let fields = get_fields(object)?;
    let define_fields = fields
        .fields
        .iter()
        .filter(|field| !field.skip)
        .map(|field| impl_define_field(object, field))
        .collect::<GeneratorResult<Vec<TokenStream>>>()?;
    Ok(quote! {
        impl #create_name::Register for #ident {
            fn register(registry: #create_name::Registry) -> #create_name::Registry {
                #define_object

                #description

                #(#define_fields)*

                registry.register_type(object)
            }
        }
    })
}

impl ToTokens for SimpleObject {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let impl_object = impl_object(&self.name, &self.ident);
        let impl_resolve_owned = impl_resolve_owned(&self.ident);
        let impl_resolve_ref = impl_resolve_ref(&self.ident);
        let impl_resolvers = impl_resolvers(self).into_token_stream();
        let impl_register = impl_register(self).into_token_stream();
        tokens.extend(quote! {
            #impl_object
            #impl_resolve_owned
            #impl_resolve_ref
            #impl_resolvers
            #impl_register
        })
    }
}
