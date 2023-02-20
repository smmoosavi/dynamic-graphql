use std::collections::HashSet;

pub use args::*;
pub use fields::*;
pub use generics::*;
pub use interfaces::*;
use proc_macro2::TokenStream;
use quote::quote;

use crate::args::common;
use crate::utils::common::CommonArg;
use crate::utils::common::CommonField;
use crate::utils::common::CommonObject;
use crate::utils::common::GetArgs;
use crate::utils::common::GetFields;
use crate::utils::crate_name::get_crate_name;
use crate::utils::error::IntoTokenStream;
use crate::utils::impl_block::BaseFnArg;
use crate::utils::rename_rule::calc_enum_item_name;
use crate::utils::rename_rule::calc_input_field_name;
use crate::utils::rename_rule::calc_type_name;
use crate::utils::type_utils::get_owned_type;

mod args;
mod fields;
mod generics;
mod interfaces;

pub trait ArgImplementor: CommonArg {
    fn get_self_arg_definition(&self) -> darling::Result<TokenStream>;
    fn get_typed_arg_definition(&self) -> darling::Result<TokenStream>;
    fn get_arg_definition(&self) -> darling::Result<TokenStream> {
        match &self.get_arg() {
            BaseFnArg::Receiver(_) => self.get_self_arg_definition(),
            BaseFnArg::Typed(_) => self.get_typed_arg_definition(),
        }
    }
    fn get_self_arg_usage(&self) -> darling::Result<TokenStream>;
    fn get_typed_arg_usage(&self) -> darling::Result<TokenStream>;
    fn get_arg_usage(&self) -> darling::Result<TokenStream> {
        match &self.get_arg() {
            BaseFnArg::Receiver(_) => self.get_self_arg_usage(),
            BaseFnArg::Typed(_) => self.get_typed_arg_usage(),
        }
    }
}

pub trait FieldImplementor: CommonField {
    fn define_field(&self) -> darling::Result<TokenStream>;
    fn get_execute_code(&self) -> darling::Result<TokenStream>;
    fn get_resolve_code(&self) -> darling::Result<TokenStream>;
    fn get_field_argument_definition(&self) -> darling::Result<TokenStream>;
    fn get_field_description_code(&self) -> darling::Result<TokenStream>;
    fn get_field_deprecation_code(&self) -> darling::Result<TokenStream>;
    fn get_field_usage_code(&self) -> darling::Result<TokenStream>;
}

impl ArgImplementor for () {
    fn get_self_arg_definition(&self) -> darling::Result<TokenStream> {
        unreachable!("Self arg can't be defined")
    }

    fn get_typed_arg_definition(&self) -> darling::Result<TokenStream> {
        unreachable!("Typed arg can't be defined")
    }

    fn get_self_arg_usage(&self) -> darling::Result<TokenStream> {
        unreachable!("Self arg can't be used")
    }

    fn get_typed_arg_usage(&self) -> darling::Result<TokenStream> {
        unreachable!("Typed arg can't be used")
    }
}

pub fn impl_object(obj: &impl CommonObject) -> darling::Result<TokenStream> {
    let object_ident = obj.get_ident();
    let name = get_type_name(obj)?;
    let crate_name = get_crate_name();
    let (impl_generics, ty_generics, where_clause) = obj.get_generics()?.split_for_impl();

    let type_name = obj.should_impl_type_name().then_some(quote! {
        impl #impl_generics #crate_name::internal::TypeName for #object_ident #ty_generics #where_clause {
            fn get_type_name() -> std::borrow::Cow<'static, str> {
                #name.into()
            }
        }
    });

    Ok(quote! {
        impl #impl_generics #crate_name::internal::ParentType for #object_ident #ty_generics #where_clause {
            type Type = #object_ident #ty_generics;
        }
        #type_name
        impl #impl_generics #crate_name::internal::OutputTypeName for #object_ident #ty_generics #where_clause {}
        impl #impl_generics #crate_name::internal::Object for #object_ident #ty_generics #where_clause {}
    })
}

pub fn impl_input_object(obj: &impl CommonObject) -> darling::Result<TokenStream> {
    let object_ident = obj.get_ident();
    let name = get_type_name(obj)?;
    let crate_name = get_crate_name();
    let type_name = obj.should_impl_type_name().then_some(quote! {
        impl #crate_name::internal::TypeName for #object_ident {
            fn get_type_name() -> std::borrow::Cow<'static, str> {
                #name.into()
            }
        }
    });
    Ok(quote! {
        #type_name
        impl #crate_name::internal::InputTypeName for #object_ident {}
        impl #crate_name::internal::InputObject for #object_ident {}
    })
}

pub fn impl_resolve_owned(obj: &impl CommonObject) -> darling::Result<TokenStream> {
    let crate_name = get_crate_name();
    let object_ident = obj.get_ident();
    let (_, ty_generics, where_clause) = obj.get_generics()?.split_for_impl();
    let (generics_with_lifetime, lifetime) = add_new_lifetime_to_generics(obj.get_generics()?);
    let (impl_generics, _, _) = generics_with_lifetime.split_for_impl();

    Ok(quote! {
        impl #impl_generics #crate_name::internal::ResolveOwned<#lifetime> for #object_ident #ty_generics #where_clause {
            fn resolve_owned(self, _ctx: &#crate_name::Context) -> #crate_name::Result<Option<#crate_name::FieldValue<#lifetime>>> {
                Ok(Some(#crate_name::FieldValue::owned_any(self)))
            }
        }
    })
}

pub fn impl_resolve_ref(obj: &impl CommonObject) -> darling::Result<TokenStream> {
    let crate_name = get_crate_name();
    let object_ident = obj.get_ident();
    let (_, ty_generics, where_clause) = obj.get_generics()?.split_for_impl();
    let (generics_with_lifetime, lifetime) = add_new_lifetime_to_generics(obj.get_generics()?);
    let (impl_generics, _, _) = generics_with_lifetime.split_for_impl();

    Ok(quote! {
        impl #impl_generics #crate_name::internal::ResolveRef<#lifetime> for #object_ident #ty_generics #where_clause {
            fn resolve_ref(&#lifetime self, _ctx: &#crate_name::Context) -> #crate_name::Result<Option<#crate_name::FieldValue<#lifetime>>> {
                Ok(Some(#crate_name::FieldValue::borrowed_any(self)))
            }
        }
    })
}

pub fn impl_resolve_owned_by_value(obj: &impl CommonObject) -> darling::Result<TokenStream> {
    let crate_name = get_crate_name();
    let object_ident = obj.get_ident();

    Ok(quote! {
        impl<'__dynamic_graphql_lifetime> #crate_name::internal::ResolveOwned<'__dynamic_graphql_lifetime> for #object_ident {
            fn resolve_owned(self, _ctx: &#crate_name::Context) -> #crate_name::Result<Option<#crate_name::FieldValue<'__dynamic_graphql_lifetime>>> {
                Ok(Some(#crate_name::FieldValue::value(&self)))
            }
        }
    })
}

pub fn impl_resolve_ref_by_value(obj: &impl CommonObject) -> darling::Result<TokenStream> {
    let crate_name = get_crate_name();
    let object_ident = obj.get_ident();
    Ok(quote! {
        impl<'__dynamic_graphql_lifetime> #crate_name::internal::ResolveRef<'__dynamic_graphql_lifetime> for #object_ident {
            fn resolve_ref(&'__dynamic_graphql_lifetime self, _ctx: &#crate_name::Context) -> #crate_name::Result<Option<#crate_name::FieldValue<'__dynamic_graphql_lifetime>>> {
                Ok(Some(#crate_name::FieldValue::value(self)))
            }
        }
    })
}

pub fn impl_define_object() -> TokenStream {
    // todo get "object" from input
    let crate_name = get_crate_name();
    quote! {
        let object = #crate_name::dynamic::Object::new(<Self as #crate_name::internal::Object>::get_object_type_name().as_ref());
    }
}

pub fn impl_define_input_object() -> TokenStream {
    // todo get "object" from input
    let crate_name = get_crate_name();
    quote! {
        let object = #crate_name::dynamic::InputObject::new(<Self as #crate_name::internal::InputObject>::get_input_object_type_name().as_ref());
    }
}

pub fn register_object_code() -> TokenStream {
    quote!(registry.register_type(object))
}

pub fn object_description(doc: Option<&str>) -> darling::Result<TokenStream> {
    // todo get "object" from input
    if let Some(doc) = doc {
        Ok(quote! {
            let object = object.description(#doc);
        })
    } else {
        Ok(quote! {})
    }
}

pub fn get_type_name(obj: &impl CommonObject) -> darling::Result<String> {
    let name = obj.get_name();
    let object_ident = obj.get_ident();
    let name = calc_type_name(name, &object_ident.to_string());
    Ok(name)
}

pub fn get_enum_item_name(item: &impl CommonField) -> darling::Result<String> {
    let name = item.get_name();
    let item_ident = item.get_ident()?;
    let name = calc_enum_item_name(name, &item_ident.to_string(), item.get_field_rename_rule());
    Ok(name)
}

pub fn get_input_field_name(field: &impl CommonField) -> darling::Result<String> {
    Ok(calc_input_field_name(
        field.get_name(),
        &field.get_ident()?.to_string(),
        field.get_field_rename_rule(),
    ))
}

pub fn get_input_type_ref_code(field: &impl CommonField) -> darling::Result<TokenStream> {
    let crate_name = get_crate_name();
    let field_type = get_owned_type(field.get_type()?);
    Ok(quote! {
        <#field_type as #crate_name::internal::GetInputTypeRef>::get_input_type_ref()
    })
}

pub fn get_new_input_value_code(field: &impl CommonField) -> darling::Result<TokenStream> {
    // todo get "field" from input
    let crate_name = get_crate_name();
    let field_name = get_input_field_name(field)?;
    let get_input_type_ref_code = get_input_type_ref_code(field)?;

    Ok(quote! {
        let field = #crate_name::dynamic::InputValue::new(#field_name, #get_input_type_ref_code);
    })
}

pub fn call_register_fns() -> TokenStream {
    let crate_name = get_crate_name();
    quote!(
        let registry = <Self as #crate_name::internal::RegisterFns>::REGISTER_FNS.iter().fold(registry, |registry, f| f(registry));
    )
}

pub fn get_define_fields_code<O, F, A>(object: &O) -> darling::Result<TokenStream>
where
    O: GetFields<F>,
    F: FieldImplementor + GetArgs<A>,
    A: ArgImplementor,
{
    Ok(object
        .get_fields()?
        .iter()
        .filter(|field| !field.get_skip())
        .map(|field| common::build_field(field).into_token_stream())
        .collect())
}

pub fn get_nested_type_register_code<O, F, A>(object: &O) -> darling::Result<TokenStream>
where
    O: GetFields<F>,
    F: CommonField + GetArgs<A>,
    A: CommonArg,
{
    let mut errors = Vec::new();
    let mut types = HashSet::new();

    let fields = object.get_fields()?;
    fields
        .iter()
        .filter(|field| !field.get_skip())
        .for_each(|field| {
            let args = match field.get_args() {
                Ok(args) => args,
                Err(err) => {
                    errors.push(err);
                    return;
                }
            };
            args.iter().for_each(|arg| {
                if let BaseFnArg::Typed(ty) = arg.get_arg() {
                    if is_arg_ctx(arg) {
                        return;
                    }
                    types.insert(&ty.ty);
                }
            });
            let ty = field.get_type();
            match ty {
                Ok(ty) => {
                    types.insert(ty);
                }
                Err(err) => errors.push(err),
            };
        });

    let errors = errors
        .into_iter()
        .map(|err| err.write_errors())
        .collect::<Vec<_>>();
    let codes = types
        .into_iter()
        .map(|ty| {
            let ty = replace_type_generics_with_static(ty);
            quote! {
                let registry = registry.register::<#ty>();
            }
        })
        .collect::<Vec<_>>();

    Ok(quote! {
        #(#errors)*
        #(#codes)*
    })
}
