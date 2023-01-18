use proc_macro2::TokenStream;
use quote::quote;

use crate::args::common::{
    get_field_name, get_field_type, get_type_name, ArgImplementor, FieldImplementor,
};
use crate::args::interface::{InterfaceMethod, InterfaceMethodArg};
use crate::args::{common, Interface};
use crate::utils::common::{CommonArg, CommonObject, GetArgs, GetFields};
use crate::utils::crate_name::get_create_name;
use crate::utils::error::IntoTokenStream;

impl FieldImplementor for InterfaceMethod {
    fn define_field(&self) -> darling::Result<TokenStream> {
        let create_name = get_create_name();
        let field_name = get_field_name(self)?;
        let field_type = get_field_type(self)?;
        Ok(quote! {
            let field = #create_name::dynamic::InterfaceField::new(
                #field_name,
                <#field_type as #create_name::GetOutputTypeRef>::get_output_type_ref(),
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

impl ArgImplementor for InterfaceMethodArg {
    fn get_self_arg_definition(&self) -> darling::Result<TokenStream> {
        let arg_ident = common::get_arg_ident(self);

        Ok(quote! {
            let parent = ctx.parent_value.try_downcast_ref::<I>()?;
            let #arg_ident = parent;
        })
    }

    fn get_typed_arg_definition(&self) -> darling::Result<TokenStream> {
        common::get_typed_arg_definition(self)
    }

    fn get_self_arg_usage(&self) -> darling::Result<TokenStream> {
        common::get_self_arg_usage(self)
    }

    fn get_typed_arg_usage(&self) -> darling::Result<TokenStream> {
        common::get_typed_arg_usage(self)
    }
}

pub fn define_interface_struct(input: &Interface) -> darling::Result<TokenStream> {
    let create_name = get_create_name();
    let name = get_type_name(input)?;

    let ident = &input.arg.ident;
    Ok(quote! {
        pub struct #ident<T=#create_name::InterfaceRoot>(::std::marker::PhantomData<T>);
        impl #create_name::GraphqlType for #ident {
            const NAME: &'static str = #name;
        }
        impl #create_name::OutputType for #ident {}
        impl #create_name::Interface for #ident {}
    })
}

pub fn get_define_fields_code<O, F, A>(object: &O) -> darling::Result<TokenStream>
where
    O: GetFields<F>,
    F: FieldImplementor + GetArgs<A>,
    A: CommonArg + ArgImplementor,
{
    Ok(object
        .get_fields()?
        .iter()
        .filter(|method| !method.get_skip())
        .map(|method| common::build_field(method).into_token_stream())
        .collect())
}

pub fn impl_register(input: &Interface) -> darling::Result<TokenStream> {
    let create_name = get_create_name();
    let ident = &input.arg.ident;

    let description = common::object_description(input.get_doc()?.as_deref())?;
    let define_fields = get_define_fields_code(input)?;
    let register_code = common::register_object_code();
    Ok(quote! {
        impl #create_name::Register for #ident {
            fn register(registry: #create_name::Registry) -> #create_name::Registry {
                // todo rename to interface
                let object = #create_name::dynamic::Interface::new(<Self as #create_name::Interface>::NAME);

                #description
                #define_fields
                #register_code
            }
        }
    })
}
