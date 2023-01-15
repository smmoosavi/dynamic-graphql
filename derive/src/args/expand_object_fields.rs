use crate::args::common;
use crate::utils::attributes::Attributes;
use crate::utils::common::{CommonArg, CommonField, CommonMethod, GetArgs, GetFields, GetGenerics};
use crate::utils::crate_name::get_create_name;
use crate::utils::deprecation::Deprecation;
use crate::utils::error::IntoTokenStream;
use crate::utils::impl_block::{BaseFnArg, BaseItemImpl, BaseMethod, TypedArg};
use crate::utils::macros::*;
use crate::utils::rename_rule::RenameRule;
use crate::utils::rename_rule::{calc_arg_name, calc_field_name};
use crate::utils::type_utils::{
    get_owned_type, get_type_ident, get_value_type, is_type_ref, is_type_slice, is_type_str,
};
use crate::utils::with_attributes::WithAttributes;
use crate::utils::with_context::{MakeContext, WithContext};
use crate::utils::with_doc::WithDoc;
use crate::utils::with_index::WithIndex;
use darling::FromAttributes;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::spanned::Spanned;
use syn::Generics;

#[derive(FromAttributes, Debug, Clone)]
#[darling(attributes(graphql))]
pub struct ExpandObjectFieldsArgAttrs {
    #[darling(default)]
    pub name: Option<String>,

    #[darling(default)]
    pub ctx: bool,
}

impl Attributes for ExpandObjectFieldsArgAttrs {
    const ATTRIBUTES: &'static [&'static str] = &["graphql"];
}

#[derive(Default, Debug, Clone)]
pub struct ExpandObjectFieldsArgContext {
    pub rename_args: Option<RenameRule>,
}

from_fn_arg!(ExpandObjectFieldsArg,
    WithAttributes<
        ExpandObjectFieldsArgAttrs,
        WithIndex<WithContext<ExpandObjectFieldsArgContext, BaseFnArg>>,
    >,
);

#[derive(FromAttributes, Debug, Clone)]
#[darling(attributes(graphql))]
pub struct ExpandObjectFieldsMethodAttrs {
    #[darling(default)]
    pub skip: bool,

    #[darling(default)]
    pub name: Option<String>,

    #[darling(default)]
    pub rename_args: Option<RenameRule>,

    #[darling(default)]
    pub deprecation: Deprecation,
}

impl Attributes for ExpandObjectFieldsMethodAttrs {
    const ATTRIBUTES: &'static [&'static str] = &["graphql"];
}

#[derive(Default, Debug, Clone)]
pub struct ExpandObjectFieldsMethodContext {
    pub rename_args: Option<RenameRule>,
    pub rename_fields: Option<RenameRule>,
}

from_method!(
    ExpandObjectFieldsMethod,
    WithAttributes<
        WithDoc<ExpandObjectFieldsMethodAttrs>,
        WithContext<ExpandObjectFieldsMethodContext, BaseMethod<ExpandObjectFieldsArg>>,
    >,
    inner = args,
);

impl MakeContext<ExpandObjectFieldsArgContext> for ExpandObjectFieldsMethod {
    fn make_context(&self) -> ExpandObjectFieldsArgContext {
        ExpandObjectFieldsArgContext {
            rename_args: self.attrs.rename_args.or(self.ctx.rename_args),
        }
    }
}

#[derive(FromAttributes, Debug, Clone)]
#[darling(attributes(graphql))]
pub struct ExpandObjectFieldsAttrs {
    #[darling(default)]
    pub rename_fields: Option<RenameRule>,

    #[darling(default)]
    pub rename_args: Option<RenameRule>,
}

impl Attributes for ExpandObjectFieldsAttrs {
    const ATTRIBUTES: &'static [&'static str] = &["graphql"];
}

from_item_impl!(
    ExpandObjectFields,
    WithAttributes<
        WithDoc<ExpandObjectFieldsAttrs>,
        BaseItemImpl<ExpandObjectFieldsMethod, Generics>,
    >,
    ctx,
);

impl MakeContext<ExpandObjectFieldsMethodContext> for ExpandObjectFields {
    fn make_context(&self) -> ExpandObjectFieldsMethodContext {
        ExpandObjectFieldsMethodContext {
            rename_args: self.attrs.rename_args,
            rename_fields: self.attrs.rename_fields,
        }
    }
}

impl CommonField for ExpandObjectFieldsMethod {
    fn get_name(&self) -> Option<&str> {
        self.attrs.name.as_deref()
    }

    fn get_ident(&self) -> darling::Result<&Ident> {
        Ok(&self.ident)
    }

    fn get_type(&self) -> darling::Result<&syn::Type> {
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

impl CommonArg for ExpandObjectFieldsArg {
    fn get_name(&self) -> Option<&str> {
        self.attrs.name.as_deref()
    }

    fn get_index(&self) -> usize {
        self.index
    }

    fn get_arg(&self) -> &BaseFnArg {
        self
    }

    fn get_arg_rename_rule(&self) -> Option<&RenameRule> {
        self.ctx.rename_args.as_ref()
    }

    fn is_marked_as_ctx(&self) -> bool {
        self.attrs.ctx
    }
}

impl CommonMethod for ExpandObjectFieldsMethod {
    fn is_async(&self) -> bool {
        self.asyncness
    }
}

impl GetArgs<ExpandObjectFieldsArg> for ExpandObjectFieldsMethod {
    fn get_args(&self) -> darling::Result<&Vec<ExpandObjectFieldsArg>> {
        Ok(&self.args)
    }
}

impl GetFields<ExpandObjectFieldsMethod> for ExpandObjectFields {
    fn get_fields(&self) -> darling::Result<&Vec<ExpandObjectFieldsMethod>> {
        Ok(&self.methods)
    }
}

impl GetGenerics for ExpandObjectFields {
    fn get_generics(&self) -> &syn::Generics {
        &self.generics
    }
}

fn get_arg_ident(arg: &impl CommonArg) -> syn::Ident {
    syn::Ident::new(&format!("arg{}", arg.get_index()), arg.get_arg().span())
}

fn get_field_var_ident(index: usize, ident: &syn::Ident) -> Ident {
    Ident::new(&format!("__field_{}", index), ident.span())
}

fn is_arg_ctx(arg: &impl CommonArg) -> bool {
    arg.is_marked_as_ctx()
        || matches!(arg.get_arg(), BaseFnArg::Typed(TypedArg{ref ident, ..}) if ident == "ctx" || ident == "_ctx")
}

fn get_arg_definition(arg: &impl CommonArg) -> darling::Result<TokenStream> {
    let create_name = get_create_name();
    let arg_ident = get_arg_ident(arg);

    match &arg.get_arg() {
        BaseFnArg::Receiver(_) => Ok(quote! {
            let parent = ctx.parent_value.try_downcast_ref::<<Self as #create_name::ExpandObject>::Target>()?.into();
            let #arg_ident = &parent;
        }),
        BaseFnArg::Typed(typed) => {
            if is_arg_ctx(arg) {
                Ok(quote! {
                    let #arg_ident = &ctx;
                })
            } else {
                let arg_name = calc_arg_name(
                    arg.get_name(),
                    &typed.ident.to_string(),
                    arg.get_arg_rename_rule(),
                );
                let value_type = get_value_type(&typed.ty);
                match value_type {
                    None => Ok(quote! {
                        let #arg_ident = #create_name::FromValue::from_value(ctx.args.try_get(#arg_name)?)?;
                    }),
                    Some(ty) => Ok(quote! {
                        let #arg_ident: #ty = #create_name::FromValue::from_value(ctx.args.try_get(#arg_name)?)?;
                    }),
                }
            }
        }
    }
}

fn get_args_definition(args: &[impl CommonArg]) -> TokenStream {
    args.iter()
        .map(|arg| get_arg_definition(arg).into_token_stream())
        .collect()
}

fn get_arg_usage(arg: &impl CommonArg) -> TokenStream {
    let arg_ident = get_arg_ident(arg);
    match arg.get_arg() {
        BaseFnArg::Receiver(_) => {
            quote!(#arg_ident,)
        }
        BaseFnArg::Typed(ref typed) => {
            let is_ctx = is_arg_ctx(arg);
            let is_owned = !is_type_ref(&typed.ty);
            if is_ctx || is_owned {
                quote!(#arg_ident,)
            } else {
                quote!(&#arg_ident,)
            }
        }
    }
}

fn get_args_usage(args: &[impl CommonArg]) -> TokenStream {
    args.iter().map(get_arg_usage).collect()
}

fn execute_code<F, A>(type_ident: &syn::Ident, method: &F) -> darling::Result<TokenStream>
where
    F: CommonMethod + GetArgs<A>,
    A: CommonArg,
{
    let field_ident = method.get_ident()?;

    let args = get_args_usage(method.get_args()?);

    if method.is_async() {
        Ok(quote! {
            let value = #type_ident::#field_ident(#args).await;
        })
    } else {
        Ok(quote! {
            let value = #type_ident::#field_ident(#args);
        })
    }
}

fn resolve_value_code(ty: &syn::Type) -> TokenStream {
    let create_name = get_create_name();

    let is_str = is_type_str(ty);
    let is_slice = is_type_slice(ty);
    let is_ref = is_type_ref(ty);
    let is_owned = !is_ref;

    let resolve_ref = quote! {
        #create_name::ResolveRef::resolve_ref(value, &ctx)
    };
    let resolve_owned = quote! {
        #create_name::ResolveOwned::resolve_owned(value, &ctx)
    };
    if is_owned || is_str || is_slice {
        resolve_owned
    } else {
        resolve_ref
    }
}

fn get_argument_definition(arg: &impl CommonArg) -> TokenStream {
    if is_arg_ctx(arg) {
        return quote!();
    }
    let BaseFnArg::Typed(typed) = arg.get_arg() else {
        return quote!();
    };
    let create_name = get_create_name();
    let arg_name = calc_arg_name(
        arg.get_name(),
        &typed.ident.to_string(),
        arg.get_arg_rename_rule(),
    );
    let arg_type = get_owned_type(&typed.ty);

    quote! {
        let arg = #create_name::dynamic::InputValue::new(#arg_name, <#arg_type as #create_name::GetInputTypeRef>::get_input_type_ref());
        let field = field.argument(arg);
    }
}

fn get_argument_definitions(args: &[impl CommonArg]) -> TokenStream {
    args.iter().map(get_argument_definition).collect()
}

fn define_field_code(
    expand: &ExpandObjectFields,
    index: usize,
    method: &ExpandObjectFieldsMethod,
) -> darling::Result<TokenStream> {
    let create_name = get_create_name();
    let field_ident = method.get_ident()?;
    let field_var_ident = get_field_var_ident(index, &method.ident);
    let field_name = calc_field_name(
        method.get_name(),
        &field_ident.to_string(),
        method.get_field_rename_rule(),
    );

    let ty = method.get_type()?;
    let owned_type = get_owned_type(ty);

    let args_definition = get_args_definition(method.get_args()?);
    let type_ident = get_type_ident(&expand.ty).unwrap();
    let execute = execute_code(type_ident, method)?;
    let resolve = resolve_value_code(ty);

    let args = method.get_args()?;

    let argument_definitions = get_argument_definitions(args);

    let description = common::field_description(method)?;

    let deprecation = common::field_deprecation_code(method)?;

    Ok(quote! {
        let field = #create_name::dynamic::Field::new(#field_name, <#owned_type as #create_name::GetOutputTypeRef>::get_output_type_ref(), |ctx| {
            #create_name::dynamic::FieldFuture::new(async move {
                #args_definition
                #execute
                #resolve
            })
        });
        #argument_definitions
        #description
        #deprecation
        let #field_var_ident = field;
    })
}

fn define_fields_code(expand: &ExpandObjectFields) -> darling::Result<TokenStream> {
    Ok(expand
        .get_fields()?
        .iter()
        .filter(|method| !method.get_skip())
        .enumerate()
        .map(|(index, method)| define_field_code(expand, index, method).into_token_stream())
        .collect())
}

fn use_field_code(index: usize, method: &ExpandObjectFieldsMethod) -> darling::Result<TokenStream> {
    let field_var_ident = get_field_var_ident(index, &method.ident);

    Ok(quote! {
        let object = object.field(#field_var_ident);
    })
}

fn use_fields_code(expand: &ExpandObjectFields) -> darling::Result<TokenStream> {
    Ok(expand
        .get_fields()?
        .iter()
        .filter(|method| !method.get_skip())
        .enumerate()
        .map(|(index, method)| use_field_code(index, method).into_token_stream())
        .collect())
}

fn impl_register(expand: &ExpandObjectFields) -> darling::Result<TokenStream> {
    let create_name = get_create_name();
    let (impl_generics, _, where_clause) = expand.generics.split_for_impl();
    let ty = &expand.ty;

    let define_fields = define_fields_code(expand).into_token_stream();
    let use_fields = use_fields_code(expand).into_token_stream();

    Ok(quote! {
        impl #impl_generics #create_name::Register for #ty #where_clause {
            fn register(registry: #create_name::Registry) -> #create_name::Registry {
                #define_fields
                registry.update_object(
                    <<Self as #create_name::ExpandObject>::Target as #create_name::Object>::NAME,
                    <Self as #create_name::ExpandObject>::NAME,
                    |object| {
                        #use_fields
                        object
                    },
                )
            }
        }
    })
}

impl ToTokens for ExpandObjectFields {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let impl_register = impl_register(self).into_token_stream();
        tokens.extend(quote! {
            #impl_register
        });
    }
}
