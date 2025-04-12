use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_quote;
use syn::Type;

use crate::args::common;
use crate::args::common::ArgImplementor;
use crate::args::common::FieldImplementor;
use crate::args::interface::InterfaceMethod;
use crate::args::interface::InterfaceMethodArg;
use crate::args::Interface;
use crate::utils::common::CommonArg;
use crate::utils::common::CommonField;
use crate::utils::common::CommonMethod;
use crate::utils::common::GetArgs;
use crate::utils::common::GetFields;
use crate::utils::crate_name::get_crate_name;
use crate::utils::deprecation::Deprecation;
use crate::utils::error::IntoTokenStream;
use crate::utils::rename_rule::RenameRule;

struct OthersMethod<'a>(&'a InterfaceMethod);

impl GetArgs<InterfaceMethodArg> for OthersMethod<'_> {
    fn get_args(&self) -> darling::Result<&Vec<InterfaceMethodArg>> {
        self.0.get_args()
    }
}

impl CommonMethod for OthersMethod<'_> {
    fn is_async(&self) -> bool {
        self.0.asyncness
    }
}

impl CommonField for OthersMethod<'_> {
    fn get_name(&self) -> Option<&str> {
        self.0.get_name()
    }

    fn get_ident(&self) -> darling::Result<&Ident> {
        self.0.get_ident()
    }

    fn get_type(&self) -> darling::Result<&Type> {
        self.0.get_type()
    }

    fn get_skip(&self) -> bool {
        self.0.get_skip()
    }

    fn get_doc(&self) -> darling::Result<Option<String>> {
        self.0.get_doc()
    }

    fn get_deprecation(&self) -> darling::Result<Deprecation> {
        self.0.get_deprecation()
    }

    fn get_field_rename_rule(&self) -> Option<&RenameRule> {
        self.0.get_field_rename_rule()
    }

    fn get_args_rename_rule(&self) -> Option<&RenameRule> {
        self.0.get_args_rename_rule()
    }
}

impl FieldImplementor for OthersMethod<'_> {
    fn define_field(&self) -> darling::Result<TokenStream> {
        common::define_field(self)
    }

    fn get_execute_code(&self) -> darling::Result<TokenStream> {
        // let type_ident = self.0.ctx.rename_args;
        execute_code(self)
    }

    fn get_resolve_code(&self) -> darling::Result<TokenStream> {
        common::resolve_value_code()
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
        let field_var_ident = get_field_var_ident(self.0.index, &self.0.ident);

        Ok(quote! {
            let #field_var_ident = field;
        })
    }
}

impl ArgImplementor for InterfaceMethodArg {
    fn get_self_arg_definition(&self) -> darling::Result<TokenStream> {
        let arg_ident = common::get_arg_ident(self);

        Ok(quote! {
            let parent = ctx.parent_value.try_downcast_ref::<T>()?;
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

fn execute_code<F, A>(method: &F) -> darling::Result<TokenStream>
where
    F: CommonMethod + GetArgs<A>,
    A: CommonArg + ArgImplementor,
{
    let field_ident = method.get_ident()?;

    let args = common::get_args_usage(method)?;

    if method.is_async() {
        Ok(quote! {
            let value = T::#field_ident(#args).await;
        })
    } else {
        Ok(quote! {
            let value = T::#field_ident(#args);
        })
    }
}

fn define_fields_code(input: &Interface) -> darling::Result<TokenStream> {
    Ok(input
        .get_fields()?
        .iter()
        .filter(|method| !method.get_skip())
        .map(|method| common::build_field(&OthersMethod(method)).into_token_stream())
        .collect())
}

fn get_field_var_ident(index: usize, ident: &syn::Ident) -> syn::Ident {
    syn::Ident::new(&format!("__field_{}", index), ident.span())
}

fn use_field_code(method: &InterfaceMethod) -> darling::Result<TokenStream> {
    let field_var_ident = get_field_var_ident(method.index, &method.ident);

    Ok(quote! {
        let object = object.field(#field_var_ident);
    })
}

fn use_fields_code(input: &Interface) -> darling::Result<TokenStream> {
    Ok(input
        .get_fields()?
        .iter()
        .filter(|method| !method.get_skip())
        .map(|method| use_field_code(method).into_token_stream())
        .collect())
}

pub fn impl_others_register(input: &Interface) -> darling::Result<TokenStream> {
    let crate_name = get_crate_name();
    let ident = &input.ident;

    let define_fields = define_fields_code(input).into_token_stream();

    let use_fields = use_fields_code(input).into_token_stream();

    let mut auto_registers = input.attrs.auto_registers.clone();
    auto_registers.iter_mut().for_each(|register| {
        // add <T> to last segment
        register.with_generic(parse_quote!(T));
    });

    Ok(quote! {

        impl <T> #crate_name::internal::RegisterInstance<dyn #ident, T> for dyn #ident
                                where
        T: #ident + #crate_name::internal::Object + 'static,
        T: Send + Sync,

        {
            fn register_instance(registry: #crate_name::internal::Registry) -> #crate_name::internal::Registry

            {
                #( #auto_registers )*
                #define_fields
                registry.update_object(
                    <T as #crate_name::internal::Object>::get_object_type_name().as_ref(),
                    <dyn #ident as #crate_name::internal::Interface>::get_interface_type_name().as_ref(),
                    |object| {
                        #use_fields
                        let object = object.implement(<dyn #ident as #crate_name::internal::Interface>::get_interface_type_name().as_ref());
                        object
                    },
                )
            }
        }

    })
}
