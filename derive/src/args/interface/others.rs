use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::Type;

use crate::args::common::{ArgImplementor, FieldImplementor};
use crate::args::interface::{InterfaceMethod, InterfaceMethodArg};
use crate::args::{common, Interface};
use crate::utils::common::{CommonArg, CommonField, CommonMethod, GetArgs, GetFields};
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

impl<'a> CommonMethod for OthersMethod<'a> {
    fn is_async(&self) -> bool {
        self.0.asyncness
    }
}

impl<'a> CommonField for OthersMethod<'a> {
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

impl<'a> FieldImplementor for OthersMethod<'a> {
    fn define_field(&self) -> darling::Result<TokenStream> {
        common::define_field(self)
    }

    fn get_execute_code(&self) -> darling::Result<TokenStream> {
        // let type_ident = self.0.ctx.rename_args;
        execute_code(self)
    }

    fn get_resolve_code(&self) -> darling::Result<TokenStream> {
        let ty = self.get_type()?;
        common::resolve_value_code(ty)
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

fn execute_code<F, A>(method: &F) -> darling::Result<TokenStream>
where
    F: CommonMethod + GetArgs<A>,
    A: CommonArg + ArgImplementor,
{
    let field_ident = method.get_ident()?;

    let args = common::get_args_usage(method)?;

    if method.is_async() {
        Ok(quote! {
            let value = I::#field_ident(#args).await;
        })
    } else {
        Ok(quote! {
            let value = I::#field_ident(#args);
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
    let ident = &input.arg.ident;
    let trait_ident = &input.ident;

    let define_fields = define_fields_code(input).into_token_stream();

    let use_fields = use_fields_code(input).into_token_stream();

    Ok(quote! {

        impl <I> #crate_name::Register for #ident<'static, I> where I: #trait_ident + #crate_name::InterfaceTarget + 'static {
            fn register(registry: #crate_name::Registry) -> #crate_name::Registry {
                #define_fields
                registry.update_object(
                    <I as #crate_name::InterfaceTarget>::TARGET,
                    <#ident as #crate_name::Interface>::NAME,
                    |object| {
                        #use_fields
                        let object = object.implement(<#ident as #crate_name::Interface>::NAME);
                        object
                    },
                )
            }
        }

    })
}
