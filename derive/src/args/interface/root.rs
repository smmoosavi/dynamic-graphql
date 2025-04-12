use proc_macro2::TokenStream;
use quote::quote;

use crate::args::Interface;
use crate::args::common;
use crate::args::common::FieldImplementor;
use crate::args::common::get_field_name;
use crate::args::common::get_field_type;
use crate::args::common::get_type_name;
use crate::args::interface::InterfaceMethod;
use crate::utils::common::CommonObject;
use crate::utils::common::GetArgs;
use crate::utils::crate_name::get_crate_name;
use crate::utils::error::IntoTokenStream;

impl FieldImplementor for InterfaceMethod {
    fn define_field(&self) -> darling::Result<TokenStream> {
        let crate_name = get_crate_name();
        let field_name = get_field_name(self)?;
        let field_type = get_field_type(self)?;
        Ok(quote! {
            let field = #crate_name::dynamic::InterfaceField::new(
                #field_name,
                <#field_type as #crate_name::internal::GetOutputTypeRef>::get_output_type_ref(),
            );
        })
    }

    fn get_execute_code(&self) -> darling::Result<TokenStream> {
        unreachable!("Interface method can't be executed")
    }

    fn get_resolve_code(&self) -> darling::Result<TokenStream> {
        unreachable!("Interface method can't be resolved")
    }

    fn get_field_argument_definition(&self) -> darling::Result<TokenStream> {
        common::get_argument_definitions(self.get_args()?)
    }

    fn get_field_description_code(&self) -> darling::Result<TokenStream> {
        common::field_description(self)
    }

    fn get_field_deprecation_code(&self) -> darling::Result<TokenStream> {
        common::field_deprecation_code(self)
    }

    fn get_field_usage_code(&self) -> darling::Result<TokenStream> {
        Ok(quote! {
            let object = object.field(field);
        })
    }
}

pub fn impl_interface(input: &Interface) -> darling::Result<TokenStream> {
    let crate_name = get_crate_name();
    let name = get_type_name(input)?;

    let ident = &input.ident;

    let type_name = input.should_impl_type_name().then_some(quote! {
        impl #crate_name::internal::TypeName for dyn #ident {
            fn get_type_name() -> std::borrow::Cow<'static, str> {
                #name.into()
            }
        }
    });

    Ok(quote! {
        #type_name
        impl #crate_name::internal::OutputTypeName for dyn #ident {}
        impl #crate_name::internal::Interface for dyn #ident {}
    })
}

pub fn impl_register(input: &Interface) -> darling::Result<TokenStream> {
    let crate_name = get_crate_name();
    let ident = &input.ident;
    let register_nested_types = common::get_nested_type_register_code(input).into_token_stream();

    let description = common::object_description(input.get_doc()?.as_deref())?;
    let define_fields = common::get_define_fields_code(input)?;
    let register_code = common::register_object_code();

    let register_attr = &input.attrs.registers;

    Ok(quote! {
        impl #crate_name::internal::Register for dyn #ident {
            fn register(registry: #crate_name::internal::Registry) -> #crate_name::internal::Registry {

                #( #register_attr )*

                #register_nested_types

                // todo rename to interface
                let object = #crate_name::dynamic::Interface::new(<Self as #crate_name::internal::Interface>::get_interface_type_name().as_ref());

                #description
                #define_fields
                #register_code
            }
        }
    })
}
