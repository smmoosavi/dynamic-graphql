use std::ops::Deref;

use darling::util::Ignored;
use darling::{FromAttributes, FromMeta, ToTokens};
use proc_macro2::Ident;
use quote::quote;
use syn::{Generics, ItemTrait, Meta, Path, Type};

use crate::args::interface::others::impl_others_register;
use crate::utils::attributes::Attributes;
use crate::utils::common::{CommonArg, CommonField, CommonObject, GetArgs, GetFields};
use crate::utils::crate_name::get_crate_name;
use crate::utils::deprecation::Deprecation;
use crate::utils::error::IntoTokenStream;
use crate::utils::impl_block::{BaseFnArg, BaseItemTrait, BaseMethod, FromItemTrait};
use crate::utils::macros::*;
use crate::utils::rename_rule::RenameRule;
use crate::utils::with_arg::{SetArg, WithArg};
use crate::utils::with_attributes::WithAttributes;
use crate::utils::with_context::{MakeContext, SetContext, WithContext};
use crate::utils::with_doc::WithDoc;
use crate::utils::with_index::WithIndex;

mod others;
mod root;

#[derive(Debug, Clone)]
pub struct InterfaceArg {
    pub ident: syn::Ident,
}

impl FromMeta for InterfaceArg {
    fn from_meta(item: &Meta) -> darling::Result<Self> {
        match item {
            Meta::Path(path) => {
                let ident = path
                    .get_ident()
                    .ok_or_else(|| darling::Error::custom("expected identifier").with_span(path))?
                    .clone();
                Ok(InterfaceArg { ident })
            }
            _ => Err(darling::Error::custom("expected identifier").with_span(item)),
        }
    }
}

#[derive(FromAttributes, Debug, Clone)]
#[darling(attributes(graphql))]
pub struct InterfaceMethodArgAttrs {
    #[darling(default)]
    pub name: Option<String>,

    #[darling(default)]
    pub ctx: bool,
}

impl Attributes for InterfaceMethodArgAttrs {
    const ATTRIBUTES: &'static [&'static str] = &["graphql"];
}

#[derive(Default, Debug, Clone)]
pub struct InterfaceMethodArgContext {
    pub rename_args: Option<RenameRule>,
}

from_fn_arg!(InterfaceMethodArg,
   WithAttributes<
        InterfaceMethodArgAttrs,
        WithIndex<WithContext<InterfaceMethodArgContext, BaseFnArg>>,
    >,
);

#[derive(FromAttributes, Debug, Clone)]
#[darling(attributes(graphql))]
pub struct InterfaceMethodAttrs {
    #[darling(default)]
    pub skip: bool,

    #[darling(default)]
    pub name: Option<String>,

    #[darling(default)]
    pub rename_args: Option<RenameRule>,

    #[darling(default)]
    pub deprecation: Deprecation,
}

impl Attributes for InterfaceMethodAttrs {
    const ATTRIBUTES: &'static [&'static str] = &["graphql"];
}

#[derive(Default, Debug, Clone)]
pub struct InterfaceMethodContext {
    pub rename_args: Option<RenameRule>,
    pub rename_fields: Option<RenameRule>,
}

from_trait_item_method!(
    InterfaceMethod,
    WithAttributes<
        WithDoc<InterfaceMethodAttrs>,
        WithIndex<WithContext<InterfaceMethodContext, BaseMethod<InterfaceMethodArg>>>,
    >,
    inner = args,
);

impl MakeContext<InterfaceMethodArgContext> for InterfaceMethod {
    fn make_context(&self) -> InterfaceMethodArgContext {
        InterfaceMethodArgContext {
            rename_args: self.attrs.rename_args.or(self.ctx.rename_args),
        }
    }
}

impl<A> MakeContext<InterfaceMethodContext> for Interface<A> {
    fn make_context(&self) -> InterfaceMethodContext {
        InterfaceMethodContext {
            rename_args: self.attrs.rename_args,
            rename_fields: self.attrs.rename_fields,
        }
    }
}

impl CommonArg for InterfaceMethodArg {
    fn get_name(&self) -> Option<&str> {
        self.attrs.name.as_deref()
    }

    fn get_index(&self) -> usize {
        self.index
    }

    fn get_arg(&self) -> &BaseFnArg {
        &self.inner
    }

    fn get_arg_rename_rule(&self) -> Option<&RenameRule> {
        self.ctx.rename_args.as_ref()
    }

    fn is_marked_as_ctx(&self) -> bool {
        self.attrs.ctx
    }
}

impl GetArgs<InterfaceMethodArg> for InterfaceMethod {
    fn get_args(&self) -> darling::Result<&Vec<InterfaceMethodArg>> {
        Ok(&self.args)
    }
}

#[derive(FromAttributes, Debug, Clone)]
#[darling(attributes(graphql))]
pub struct InterfaceAttrs {
    #[darling(default)]
    pub name: Option<String>,

    #[darling(default)]
    pub rename_fields: Option<RenameRule>,

    #[darling(default)]
    pub rename_args: Option<RenameRule>,
}

impl Attributes for InterfaceAttrs {
    const ATTRIBUTES: &'static [&'static str] = &["graphql"];
}

pub struct Interface<Arg = InterfaceArg>(
    WithArg<Arg, WithAttributes<WithDoc<InterfaceAttrs>, BaseItemTrait<InterfaceMethod, Generics>>>,
);

impl<Arg> Deref for Interface<Arg> {
    type Target = WithArg<
        Arg,
        WithAttributes<WithDoc<InterfaceAttrs>, BaseItemTrait<InterfaceMethod, Generics>>,
    >;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Arg> SetArg<Arg> for Interface<Ignored> {
    type Output = Interface<Arg>;

    fn with_arg(self, arg: Arg) -> Self::Output {
        Interface(self.0.with_arg(arg))
    }
}

impl FromItemTrait for Interface<Ignored> {
    fn from_item_trait(item_trait: &mut ItemTrait) -> darling::Result<Self>
    where
        Self: Sized,
    {
        let mut value = Self(FromItemTrait::from_item_trait(item_trait)?);
        let ctx = MakeContext::make_context(&value);
        SetContext::set_context(&mut value.0, ctx);
        Ok(value)
    }
}

impl<A> CommonObject for Interface<A> {
    fn get_name(&self) -> Option<&str> {
        self.attrs.name.as_deref()
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

impl<A> GetFields<InterfaceMethod> for Interface<A> {
    fn get_fields(&self) -> darling::Result<&Vec<InterfaceMethod>> {
        Ok(&self.methods)
    }
}

impl CommonField for InterfaceMethod {
    fn get_name(&self) -> Option<&str> {
        self.attrs.name.as_deref()
    }

    fn get_ident(&self) -> darling::Result<&Ident> {
        Ok(&self.ident)
    }

    fn get_type(&self) -> darling::Result<&Type> {
        self.output_type.as_ref().ok_or_else(|| {
            darling::Error::custom("Field must have return type").with_span(&self.ident)
        })
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

    fn get_args_rename_rule(&self) -> Option<&RenameRule> {
        self.attrs
            .rename_args
            .as_ref()
            .or(self.ctx.rename_args.as_ref())
    }
}

fn define_new_fns_for_interface(input: &Interface) -> darling::Result<proc_macro2::TokenStream> {
    let crate_name = get_crate_name();
    let ident = &input.arg.ident;

    let mark = quote!(<#ident <'static> as #crate_name::Interface>::MARK);
    Ok(quote! {

        impl #ident<'_> {
            pub fn new_owned<'__dynamic_graphql_lifetime, T>(value: T) -> #ident<'__dynamic_graphql_lifetime>
            where
                T: #crate_name::InterfaceMark<{#mark}> + #crate_name::Object + #crate_name::ResolveOwned<'__dynamic_graphql_lifetime> + Send + Sync + 'static,
            {
                #ident(Default::default() ,#crate_name::AnyBox::new_owned(value, <T as #crate_name::Object>::NAME.to_string()))
            }

            pub fn new_borrowed<'__dynamic_graphql_lifetime, T>(value: &'__dynamic_graphql_lifetime T) -> #ident<'__dynamic_graphql_lifetime>
            where
                T: #crate_name::InterfaceMark<{#mark}> + #crate_name::Object + #crate_name::ResolveRef<'__dynamic_graphql_lifetime> + Send + Sync + 'static,
            {
                #ident(Default::default() ,#crate_name::AnyBox::new_borrowed(value, <T as #crate_name::Object>::NAME.to_string()))
            }
        }

    })
}

fn define_resolve_owned_for_interface(
    input: &Interface,
) -> darling::Result<proc_macro2::TokenStream> {
    let crate_name = get_crate_name();
    let ident = &input.arg.ident;

    Ok(quote! {
        impl<'__dynamic_graphql_lifetime> #crate_name::ResolveOwned<'__dynamic_graphql_lifetime> for #ident<'__dynamic_graphql_lifetime> {
            fn resolve_owned(self, ctx: &#crate_name::Context) -> #crate_name::Result<Option<#crate_name::FieldValue<'__dynamic_graphql_lifetime>>> {
                self.1.resolve_owned(ctx)
            }
        }
    })
}

impl ToTokens for Interface {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let interface_struct = root::define_interface_struct(self).into_token_stream();
        let resolve = define_resolve_owned_for_interface(self).into_token_stream();
        let new_fns = define_new_fns_for_interface(self).into_token_stream();
        let register = root::impl_register(self).into_token_stream();
        let register_other = impl_others_register(self).into_token_stream();

        tokens.extend(quote! {
            #interface_struct
            #resolve
            #new_fns
            #register
            #register_other
        });
    }
}
