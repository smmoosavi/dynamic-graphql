use proc_macro2::TokenStream;
use quote::quote;

pub use args::*;
pub use fields::*;
pub use generics::*;
pub use interfaces::*;

use crate::utils::common::{CommonArg, CommonField, CommonObject};
use crate::utils::crate_name::get_create_name;
use crate::utils::impl_block::BaseFnArg;
use crate::utils::rename_rule::{calc_enum_item_name, calc_input_field_name, calc_type_name};
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

pub fn impl_object(obj: &impl CommonObject) -> darling::Result<TokenStream> {
    let object_ident = obj.get_ident();
    let name = get_type_name(obj)?;
    let create_name = get_create_name();
    Ok(quote! {
        impl #create_name::ParentType for #object_ident {
            type Type = #object_ident;
        }
        impl #create_name::FromParent<#object_ident> for #object_ident {
            type Output<'a> = &'a #object_ident;
            fn from_parent<'a>(parent: &'a #object_ident) -> Self::Output<'a> {
                parent
            }
        }
        impl #create_name::GraphqlType for #object_ident {
            const NAME: &'static str = #name;
        }
        impl #create_name::OutputType for #object_ident {}
        impl #create_name::Object for #object_ident {}
        impl #create_name::InterfaceTarget for #object_ident {
            const TARGET: &'static str = #name;
        }
    })
}

pub fn impl_input_object(obj: &impl CommonObject) -> darling::Result<TokenStream> {
    let object_ident = obj.get_ident();
    let name = get_type_name(obj)?;
    let create_name = get_create_name();
    Ok(quote! {
        impl #create_name::GraphqlType for #object_ident {
            const NAME: &'static str = #name;
        }
        impl #create_name::InputType for #object_ident {}
        impl #create_name::InputObject for #object_ident {}
    })
}

pub fn impl_graphql_doc(obj: &impl CommonObject) -> darling::Result<TokenStream> {
    let doc = obj.get_doc()?;
    let object_ident = obj.get_ident();
    let create_name = get_create_name();
    let doc = match doc {
        None => quote!(None),
        Some(ref doc) => quote!(Some(#doc)),
    };

    Ok(quote! {
        impl #create_name::GraphqlDoc for #object_ident {
            const DOC: Option<&'static str> = #doc;
        }
    })
}

pub fn impl_resolve_owned(obj: &impl CommonObject) -> darling::Result<TokenStream> {
    let create_name = get_create_name();
    let object_ident = obj.get_ident();

    Ok(quote! {
        impl<'a> #create_name::ResolveOwned<'a> for #object_ident {
            fn resolve_owned(self, _ctx: &#create_name::Context) -> #create_name::Result<Option<#create_name::FieldValue<'a>>> {
                Ok(Some(#create_name::FieldValue::owned_any(self)))
            }
        }
    })
}

pub fn impl_resolve_ref(obj: &impl CommonObject) -> darling::Result<TokenStream> {
    let create_name = get_create_name();
    let object_ident = obj.get_ident();
    Ok(quote! {
        impl<'a> #create_name::ResolveRef<'a> for #object_ident {
            fn resolve_ref(&'a self, _ctx: &#create_name::Context) -> #create_name::Result<Option<#create_name::FieldValue<'a>>> {
                Ok(Some(#create_name::FieldValue::borrowed_any(self)))
            }
        }
    })
}

pub fn impl_resolve_owned_by_value(obj: &impl CommonObject) -> darling::Result<TokenStream> {
    let create_name = get_create_name();
    let object_ident = obj.get_ident();

    Ok(quote! {
        impl<'a> #create_name::ResolveOwned<'a> for #object_ident {
            fn resolve_owned(self, _ctx: &#create_name::Context) -> #create_name::Result<Option<#create_name::FieldValue<'a>>> {
                Ok(Some(#create_name::FieldValue::value(&self)))
            }
        }
    })
}

pub fn impl_resolve_ref_by_value(obj: &impl CommonObject) -> darling::Result<TokenStream> {
    let create_name = get_create_name();
    let object_ident = obj.get_ident();
    Ok(quote! {
        impl<'a> #create_name::ResolveRef<'a> for #object_ident {
            fn resolve_ref(&'a self, _ctx: &#create_name::Context) -> #create_name::Result<Option<#create_name::FieldValue<'a>>> {
                Ok(Some(#create_name::FieldValue::value(self)))
            }
        }
    })
}

pub fn impl_define_object() -> TokenStream {
    // todo get "object" from input
    let create_name = get_create_name();
    quote! {
        let object = #create_name::dynamic::Object::new(<Self as #create_name::Object>::NAME);
    }
}

pub fn impl_define_input_object() -> TokenStream {
    // todo get "object" from input
    let create_name = get_create_name();
    quote! {
        let object = #create_name::dynamic::InputObject::new(<Self as #create_name::InputObject>::NAME);
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
    let create_name = get_create_name();
    let field_type = get_owned_type(field.get_type()?);
    Ok(quote! {
        <#field_type as #create_name::GetInputTypeRef>::get_input_type_ref()
    })
}

pub fn get_new_input_value_code(field: &impl CommonField) -> darling::Result<TokenStream> {
    // todo get "field" from input
    let create_name = get_create_name();
    let field_name = get_input_field_name(field)?;
    let get_input_type_ref_code = get_input_type_ref_code(field)?;

    Ok(quote! {
        let field = #create_name::dynamic::InputValue::new(#field_name, #get_input_type_ref_code);
    })
}

pub fn call_register_fns() -> TokenStream {
    let create_name = get_create_name();
    quote!(
        let registry = <Self as #create_name::RegisterFns>::REGISTER_FNS.iter().fold(registry, |registry, f| f(registry));
    )
}
