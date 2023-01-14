use crate::args::common;
use crate::utils::common::{CommonField, CommonObject, GetFields};
use crate::utils::crate_name::get_create_name;
use crate::utils::deprecation::Deprecation;
use crate::utils::derive_types::{BaseStruct, NamedField};
use crate::utils::error::IntoTokenStream;
use crate::utils::macros::*;
use crate::utils::rename_rule::RenameRule;
use crate::utils::with_attributes::WithAttributes;
use crate::utils::with_context::{MakeContext, WithContext};
use crate::utils::with_doc::WithDoc;
use darling::{FromAttributes, ToTokens};
use proc_macro2::{Ident, TokenStream};
use quote::quote;

#[derive(FromAttributes, Debug, Clone)]
#[darling(attributes(graphql))]
pub struct SimpleObjectFieldAttrs {
    #[darling(default)]
    pub skip: bool,

    #[darling(default)]
    pub name: Option<String>,

    #[darling(default)]
    pub deprecation: Deprecation,
}

#[derive(Default, Debug, Clone)]
pub struct SimpleObjectFieldContext {
    pub rename_fields: Option<RenameRule>,
}

from_field!(
    SimpleObjectField,
    WithAttributes<
        WithDoc<SimpleObjectFieldAttrs>,
        WithContext<SimpleObjectFieldContext, NamedField>,
    >,
);

#[derive(FromAttributes, Debug, Clone)]
#[darling(attributes(graphql))]
pub struct SimpleObjectAttrs {
    #[darling(default)]
    pub name: Option<String>,

    #[darling(default)]
    pub rename_fields: Option<RenameRule>,
}

from_derive_input!(
    SimpleObject,
    WithAttributes<WithDoc<SimpleObjectAttrs>, BaseStruct<SimpleObjectField>>,
    ctx,
);

impl MakeContext<SimpleObjectFieldContext> for SimpleObject {
    fn make_context(&self) -> SimpleObjectFieldContext {
        SimpleObjectFieldContext {
            rename_fields: self.attrs.rename_fields,
        }
    }
}

impl CommonObject for SimpleObject {
    fn get_name(&self) -> Option<&str> {
        self.attrs.name.as_deref()
    }

    fn get_ident(&self) -> &syn::Ident {
        &self.ident
    }

    fn get_doc(&self) -> darling::Result<Option<String>> {
        Ok(self.attrs.doc.clone())
    }
    fn get_fields_rename_rule(&self) -> Option<&RenameRule> {
        self.attrs.rename_fields.as_ref()
    }
}

impl CommonField for SimpleObjectField {
    fn get_name(&self) -> Option<&str> {
        self.attrs.name.as_deref()
    }

    fn get_ident(&self) -> darling::Result<&Ident> {
        Ok(&self.ident)
    }

    fn get_type(&self) -> darling::Result<&syn::Type> {
        Ok(&self.ty)
    }

    fn get_skip(&self) -> bool {
        self.attrs.skip
    }

    fn get_doc(&self) -> darling::Result<Option<String>> {
        Ok(self.attrs.doc.clone())
    }
    fn get_deprecation(&self) -> darling::Result<Deprecation> {
        Ok(self.attrs.deprecation.clone())
    }
    fn get_field_rename_rule(&self) -> Option<&RenameRule> {
        self.ctx.rename_fields.as_ref()
    }
}

impl GetFields<SimpleObjectField> for SimpleObject {
    fn get_fields(&self) -> darling::Result<&Vec<SimpleObjectField>> {
        Ok(&self.data.fields)
    }
}

fn get_resolver_ident(field: &impl CommonField) -> darling::Result<Ident> {
    let field_ident = field.get_ident()?;
    let resolver_name = format!("__resolve_{}", field_ident);

    let resolver_ident = syn::Ident::new(&resolver_name, field_ident.span());
    Ok(resolver_ident)
}

fn impl_resolver(field: &impl CommonField) -> darling::Result<TokenStream> {
    let field_ident = field.get_ident()?;
    let resolver_ident = get_resolver_ident(field)?;
    let ty = field.get_type()?;
    Ok(quote! {
        fn #resolver_ident(&self) -> &#ty {
            &self.#field_ident
        }
    })
}

fn impl_resolvers<O, F>(object: &O) -> darling::Result<TokenStream>
where
    O: CommonObject + GetFields<F>,
    F: CommonField,
{
    let ident = object.get_ident();
    let fields = object
        .get_fields()?
        .iter()
        .filter(|field| !field.get_skip())
        .map(impl_resolver)
        .map(|r| r.into_token_stream())
        .collect::<Vec<TokenStream>>();
    Ok(quote! {
        impl #ident {
            #(#fields)*
        }
    })
}

fn impl_define_field<F>(field: &F) -> darling::Result<TokenStream>
where
    F: CommonField,
{
    let field_name = common::get_field_name(field)?;
    let ty = field.get_type()?;
    let resolver_ident = get_resolver_ident(field)?;
    let create_name = get_create_name();
    let description = common::field_description(field)?;
    let deprecation = common::field_deprecation_code(field)?;
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

fn get_define_fields<O, F>(object: &O) -> darling::Result<TokenStream>
where
    O: CommonObject + GetFields<F>,
    F: CommonField,
{
    Ok(object
        .get_fields()?
        .iter()
        .filter(|field| !field.get_skip())
        .map(|field| impl_define_field(field).into_token_stream())
        .collect())
}

fn impl_register(object: &SimpleObject) -> darling::Result<TokenStream> {
    let create_name = get_create_name();
    let ident = &object.ident;
    let define_object = common::impl_define_object();
    let description = common::object_description(object.get_doc()?.as_deref())?;
    let define_fields = get_define_fields(object)?;
    let register_object_code = common::register_object_code();

    Ok(quote! {
        impl #create_name::Register for #ident {
            fn register(registry: #create_name::Registry) -> #create_name::Registry {
                #define_object

                #description

                #define_fields

                #register_object_code
            }
        }
    })
}

impl ToTokens for SimpleObject {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let impl_object = common::impl_object(self).into_token_stream();
        let impl_resolve_owned = common::impl_resolve_owned(self).into_token_stream();
        let impl_resolve_ref = common::impl_resolve_ref(self).into_token_stream();
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
