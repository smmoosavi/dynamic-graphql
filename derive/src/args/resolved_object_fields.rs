use crate::args::common;
use crate::utils::attributes::Attributes;
use crate::utils::common::{CommonArg, CommonField};
use crate::utils::crate_name::get_create_name;
use crate::utils::deprecation::Deprecation;
use crate::utils::error::IntoTokenStream;
use crate::utils::impl_block::{BaseFnArg, BaseItemImpl, BaseMethod, TypedArg};
use crate::utils::macros::*;
use crate::utils::rename_rule::{calc_arg_name, calc_field_name, RenameRule};
use crate::utils::type_utils::{
    get_owned_type, get_value_type, is_type_ref, is_type_slice, is_type_str,
};
use crate::utils::with_attributes::WithAttributes;
use crate::utils::with_context::{MakeContext, WithContext};
use crate::utils::with_doc::WithDoc;
use crate::utils::with_index::WithIndex;
use darling::FromAttributes;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::spanned::Spanned;

#[derive(FromAttributes, Debug, Clone)]
#[darling(attributes(graphql))]
pub struct ResolvedObjectFieldsArgAttrs {
    #[darling(default)]
    pub name: Option<String>,

    #[darling(default)]
    pub ctx: bool,
}

impl Attributes for ResolvedObjectFieldsArgAttrs {
    const ATTRIBUTES: &'static [&'static str] = &["graphql"];
}

#[derive(Default, Debug, Clone)]
pub struct ResolvedObjectFieldsArgContext {
    pub rename_args: Option<RenameRule>,
}

from_fn_arg!(ResolvedObjectFieldsArg,
    WithAttributes<
        ResolvedObjectFieldsArgAttrs,
        WithIndex<WithContext<ResolvedObjectFieldsArgContext, BaseFnArg>>,
    >,
);

#[derive(FromAttributes, Debug, Clone)]
#[darling(attributes(graphql))]
pub struct ResolvedObjectFieldsMethodAttrs {
    #[darling(default)]
    pub skip: bool,

    #[darling(default)]
    pub name: Option<String>,

    #[darling(default)]
    pub rename_args: Option<RenameRule>,

    #[darling(default)]
    pub deprecation: Deprecation,
}

impl Attributes for ResolvedObjectFieldsMethodAttrs {
    const ATTRIBUTES: &'static [&'static str] = &["graphql"];
}

#[derive(Default, Debug, Clone)]
pub struct ResolvedObjectFieldsMethodContext {
    pub rename_args: Option<RenameRule>,
    pub rename_fields: Option<RenameRule>,
}

from_method!(
    ResolvedObjectFieldsMethod,
    WithAttributes<
        WithDoc<ResolvedObjectFieldsMethodAttrs>,
        WithContext<ResolvedObjectFieldsMethodContext, BaseMethod<ResolvedObjectFieldsArg>>,
    >,
    inner = args,
);

impl MakeContext<ResolvedObjectFieldsArgContext> for ResolvedObjectFieldsMethod {
    fn make_context(&self) -> ResolvedObjectFieldsArgContext {
        ResolvedObjectFieldsArgContext {
            rename_args: self.attrs.rename_args.or(self.ctx.rename_args),
        }
    }
}

#[derive(FromAttributes, Debug, Clone)]
#[darling(attributes(graphql))]
pub struct ResolvedObjectFieldsAttrs {
    #[darling(default)]
    pub rename_fields: Option<RenameRule>,

    #[darling(default)]
    pub rename_args: Option<RenameRule>,
}

impl Attributes for ResolvedObjectFieldsAttrs {
    const ATTRIBUTES: &'static [&'static str] = &["graphql"];
}

from_item_impl!(
    ResolvedObjectFields,
    WithAttributes<WithDoc<ResolvedObjectFieldsAttrs>, BaseItemImpl<ResolvedObjectFieldsMethod>>,
    ctx,
);

impl MakeContext<ResolvedObjectFieldsMethodContext> for ResolvedObjectFields {
    fn make_context(&self) -> ResolvedObjectFieldsMethodContext {
        ResolvedObjectFieldsMethodContext {
            rename_args: self.attrs.rename_args,
            rename_fields: self.attrs.rename_fields,
        }
    }
}

impl CommonField for ResolvedObjectFieldsMethod {
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

impl CommonArg for ResolvedObjectFieldsArg {
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

fn get_arg_ident(arg: &impl CommonArg) -> syn::Ident {
    syn::Ident::new(&format!("arg{}", arg.get_index()), arg.get_arg().span())
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
            let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
            let #arg_ident = parent;
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

fn execute_code(method: &ResolvedObjectFieldsMethod) -> darling::Result<TokenStream> {
    let field_ident = method.get_ident()?;

    let args = get_args_usage(&method.args);

    if method.asyncness {
        Ok(quote! {
            let value = Self::#field_ident(#args).await;
        })
    } else {
        Ok(quote! {
            let value = Self::#field_ident(#args);
        })
    }
}

fn define_fields(method: &ResolvedObjectFieldsMethod) -> darling::Result<TokenStream> {
    let field_ident = method.get_ident()?;
    let field_name = calc_field_name(
        method.get_name(),
        &field_ident.to_string(),
        method.get_field_rename_rule(),
    );
    let ty = method.get_type()?;
    let create_name = get_create_name();

    let owned_type = get_owned_type(ty);
    let resolve = resolve_value_code(ty);

    let args_definition = get_args_definition(&method.args);

    let execute = execute_code(method)?;
    let argument_definitions = get_argument_definitions(&method.args);

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
        let object = object.field(field);
    })
}

fn impl_object_description() -> TokenStream {
    let create_name = get_create_name();

    quote! {
        let object = <Self as #create_name::GraphqlDoc>::DOC.iter().fold(object, |object, doc| {
            object.description(doc.to_owned())
        });
    }
}

fn get_define_fields_code(object: &ResolvedObjectFields) -> TokenStream {
    object
        .methods
        .iter()
        .filter(|method| !method.get_skip())
        .map(|method| define_fields(method).into_token_stream())
        .collect()
}

fn impl_register(object: &ResolvedObjectFields) -> darling::Result<TokenStream> {
    let create_name = get_create_name();
    let ty = &object.ty;
    let define_object = common::impl_define_object();
    let description = impl_object_description();
    let define_fields = get_define_fields_code(object);
    let register_object_code = common::register_object_code();

    Ok(quote! {
        impl #create_name::Register for #ty {
            fn register(registry: #create_name::Registry) -> #create_name::Registry {
                #define_object

                #define_fields

                #description

                #register_object_code
            }
        }
    })
}

impl ToTokens for ResolvedObjectFields {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let impl_register = impl_register(self).into_token_stream();
        tokens.extend(quote! {
            #impl_register
        });
    }
}
