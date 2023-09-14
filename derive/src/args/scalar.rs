use darling::util::Ignored;
use darling::FromAttributes;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::Generics;
use syn::Path;

use crate::args::common;
use crate::args::common::add_new_lifetime_to_generics;
use crate::args::common::get_type_name;
use crate::utils::common::CommonObject;
use crate::utils::crate_name::get_crate_name;
use crate::utils::derive_types::BaseStruct;
use crate::utils::error::IntoTokenStream;
use crate::utils::macros::*;
use crate::utils::path_attr::PathAttr;
use crate::utils::register_attr::RegisterAttr;
use crate::utils::with_attributes::WithAttributes;
use crate::utils::with_doc::WithDoc;

#[derive(FromAttributes, Debug, Clone)]
#[darling(attributes(graphql))]
pub struct ScalarAttrs {
    #[darling(default)]
    pub name: Option<String>,

    #[darling(default)]
    #[darling(rename = "get_type_name")]
    pub type_name: bool,

    #[darling(default)]
    pub validator: Option<PathAttr>,

    #[darling(default)]
    specified_by_url: Option<String>,

    #[darling(default, multiple)]
    #[darling(rename = "register")]
    pub registers: Vec<RegisterAttr>,
}

from_derive_input!(
    Scalar,
    WithAttributes<WithDoc<ScalarAttrs>, BaseStruct<Ignored, Generics>>,
);

impl CommonObject for Scalar {
    fn get_name(&self) -> Option<&str> {
        self.attrs.name.as_deref()
    }

    fn should_impl_type_name(&self) -> bool {
        !self.attrs.type_name
    }

    fn get_ident(&self) -> &Ident {
        &self.ident
    }

    fn get_type(&self) -> darling::Result<Path> {
        Ok(self.ident.clone().into())
    }

    fn get_generics(&self) -> darling::Result<&Generics> {
        Ok(&self.generics)
    }

    fn get_doc(&self) -> darling::Result<Option<String>> {
        Ok(self.attrs.doc.clone())
    }
}

fn impl_scalar(scalar: &Scalar) -> darling::Result<TokenStream> {
    let object_ident = scalar.get_ident();
    let name = get_type_name(scalar)?;
    let crate_name = get_crate_name();
    let (impl_generics, ty_generics, where_clause) = scalar.get_generics()?.split_for_impl();

    let type_name = scalar.should_impl_type_name().then_some(quote! {
        impl #impl_generics #crate_name::internal::TypeName for #object_ident #ty_generics #where_clause {
            fn get_type_name() -> std::borrow::Cow<'static, str> {
                #name.into()
            }
        }
    });

    Ok(quote! {
        #type_name
        impl #impl_generics #crate_name::internal::OutputTypeName for #object_ident #ty_generics #where_clause {}
        impl #impl_generics #crate_name::internal::InputTypeName for #object_ident #ty_generics #where_clause {}
        impl #impl_generics #crate_name::internal::Scalar for #object_ident #ty_generics #where_clause {}
    })
}

fn impl_resolved_own(scalar: &Scalar) -> darling::Result<TokenStream> {
    let crate_name = get_crate_name();
    let object_ident = scalar.get_ident();
    let (_, ty_generics, where_clause) = scalar.get_generics()?.split_for_impl();
    let (generics_with_lifetime, lifetime) = add_new_lifetime_to_generics(scalar.get_generics()?);
    let (impl_generics, _, _) = generics_with_lifetime.split_for_impl();

    Ok(quote! {
        impl #impl_generics #crate_name::internal::ResolveOwned<#lifetime> for #object_ident #ty_generics #where_clause {
            fn resolve_owned(self, _ctx: &#crate_name::Context) -> #crate_name::Result<Option<#crate_name::FieldValue<#lifetime>>> {
                let value = #crate_name::ScalarValue::to_value(&self);
                Ok(Some(#crate_name::FieldValue::value(value)))
            }
        }
    })
}

pub fn impl_resolve_ref(scalar: &impl CommonObject) -> darling::Result<TokenStream> {
    let crate_name = get_crate_name();
    let object_ident = scalar.get_ident();
    let (_, ty_generics, where_clause) = scalar.get_generics()?.split_for_impl();
    let (generics_with_lifetime, lifetime) = add_new_lifetime_to_generics(scalar.get_generics()?);
    let (impl_generics, _, _) = generics_with_lifetime.split_for_impl();

    Ok(quote! {
        impl #impl_generics #crate_name::internal::ResolveRef<#lifetime> for #object_ident #ty_generics #where_clause {
            fn resolve_ref(&#lifetime self, _ctx: &#crate_name::Context) -> #crate_name::Result<Option<#crate_name::FieldValue<#lifetime>>> {
                let value = #crate_name::ScalarValue::to_value(self);
                Ok(Some(#crate_name::FieldValue::value(value)))
            }
        }
    })
}

fn impl_from_value(scalar: &impl CommonObject) -> darling::Result<TokenStream> {
    let crate_name = get_crate_name();
    let ident = scalar.get_ident();
    Ok(quote!(
        impl #crate_name::internal::FromValue for #ident {
            fn from_value(value: #crate_name::Result<#crate_name::dynamic::ValueAccessor>) -> #crate_name::internal::InputValueResult<Self> {
                let value = value?.as_value().clone();
                Ok(#crate_name::ScalarValue::from_value(value)?)
            }
        }
    ))
}

pub fn get_specified_by_url_code(scalar: &Scalar) -> darling::Result<TokenStream> {
    let specified_by_url = scalar.attrs.specified_by_url.as_deref();
    Ok(match specified_by_url {
        Some(url) => quote! {
            let object = object.specified_by_url(#url);
        },
        None => quote!(),
    })
}

fn get_validator_code(scalar: &Scalar) -> darling::Result<TokenStream> {
    let validator = &scalar.attrs.validator;
    Ok(match validator {
        Some(validator) => {
            let path = &validator.0;
            quote! {
                let object = object.validator(#path);
            }
        }
        None => quote!(),
    })
}

fn impl_register(scalar: &Scalar) -> darling::Result<TokenStream> {
    let crate_name = get_crate_name();

    let ident = &scalar.get_ident();
    let description = common::object_description(scalar.get_doc()?.as_deref())?;
    let specified_by_url = get_specified_by_url_code(scalar)?;
    let validator_code = get_validator_code(scalar)?;

    let (impl_generics, ty_generics, where_clause) = scalar.generics.split_for_impl();
    let register_attr = &scalar.attrs.registers;
    Ok(quote! {
        impl #impl_generics #crate_name::internal::Register for #ident #ty_generics #where_clause {
            fn register(registry: #crate_name::internal::Registry) -> #crate_name::internal::Registry {
                #( #register_attr )*
                let object = #crate_name::dynamic::Scalar::new(<Self as #crate_name::internal::Scalar>::get_scalar_type_name().as_ref());
                #validator_code
                #description
                #specified_by_url
                registry.register_type(object)
            }
        }
    })
}

impl ToTokens for Scalar {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let impl_scalar = impl_scalar(self).into_token_stream();
        let impl_resolved_own = impl_resolved_own(self).into_token_stream();
        let impl_resolve_ref = impl_resolve_ref(self).into_token_stream();
        let impl_from_value = impl_from_value(self).into_token_stream();
        let impl_register = impl_register(self).into_token_stream();
        tokens.extend(quote! {
            #impl_scalar
            #impl_resolved_own
            #impl_resolve_ref
            #impl_from_value
            #impl_register
        })
    }
}
