use crate::args::common::{
    field_deprecation, impl_define_object, impl_graphql_doc, impl_object, impl_resolve_owned,
    impl_resolve_ref,
};
use crate::utils::attributes::{Attributes, CleanAttributes};
use crate::utils::crate_name::get_create_name;
use crate::utils::deprecation::Deprecation;
use crate::utils::docs_utils::Doc;
use crate::utils::error::{GeneratorResult, IntoTokenStream};
use crate::utils::impl_block::{
    BaseFnArg, BaseItemImpl, BaseMethod, FromFnArg, FromItemImpl, FromMethod,
};
use crate::utils::rename_rule::{calc_field_name, RenameRule};
use crate::utils::type_utils::{get_owned_type, is_type_ref, is_type_str};
use darling::{FromAttributes, FromDeriveInput};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::FnArg;

#[derive(FromDeriveInput)]
#[darling(attributes(graphql), forward_attrs(doc))]
pub struct ResolvedObject {
    pub ident: syn::Ident,
    pub attrs: Vec<syn::Attribute>,

    #[darling(default)]
    pub name: Option<String>,
}

#[derive(FromAttributes, Debug, Clone)]
#[darling(attributes(graphql))]
pub struct ResolvedObjectFieldsArgAttrs {
    #[darling(default)]
    pub name: Option<String>,
}

impl Attributes for ResolvedObjectFieldsArgAttrs {
    const ATTRIBUTES: &'static [&'static str] = &["graphql"];
}

#[derive(Debug, Clone)]
pub struct ResolvedObjectFieldsArg {
    pub base: BaseFnArg,
    pub attrs: ResolvedObjectFieldsArgAttrs,
}

impl FromFnArg for ResolvedObjectFieldsArg {
    fn from_fn_arg(arg: &mut FnArg) -> GeneratorResult<Self> {
        let base = BaseFnArg::from_fn_arg(arg)?;
        let base_attrs = BaseFnArg::get_attrs_mut(arg);
        let attrs = ResolvedObjectFieldsArgAttrs::from_attributes(base_attrs)?;
        ResolvedObjectFieldsArgAttrs::clean_attributes(base_attrs);

        Ok(ResolvedObjectFieldsArg { base, attrs })
    }
}

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

#[derive(Debug, Clone)]
pub struct ResolvedObjectFieldsMethod {
    pub doc: Doc,
    pub base: BaseMethod<ResolvedObjectFieldsArg>,
    pub attrs: ResolvedObjectFieldsMethodAttrs,
}

impl FromMethod for ResolvedObjectFieldsMethod {
    fn from_method(method: &mut syn::ImplItemMethod) -> GeneratorResult<Self> {
        let doc = Doc::from_attributes(&method.attrs)?;
        let base = BaseMethod::from_method(method)?;
        let attrs = ResolvedObjectFieldsMethodAttrs::from_attributes(&method.attrs)?;
        ResolvedObjectFieldsMethodAttrs::clean_attributes(&mut method.attrs);
        Ok(Self { doc, base, attrs })
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

#[derive(Debug, Clone)]
pub struct ResolvedObjectFields {
    pub doc: Doc,
    pub base: BaseItemImpl<ResolvedObjectFieldsMethod>,
    pub attrs: ResolvedObjectFieldsAttrs,
}

impl FromItemImpl for ResolvedObjectFields {
    fn from_item_impl(item_impl: &mut syn::ItemImpl) -> GeneratorResult<Self> {
        let doc = Doc::from_attributes(&item_impl.attrs)?;
        let base = BaseItemImpl::from_item_impl(item_impl)?;
        let attrs = ResolvedObjectFieldsAttrs::from_attributes(&item_impl.attrs)?;
        ResolvedObjectFieldsAttrs::clean_attributes(&mut item_impl.attrs);

        Ok(Self { doc, base, attrs })
    }
}

impl ToTokens for ResolvedObject {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let impl_object = impl_object(&self.name, &self.ident).into_token_stream();
        let impl_resolve_owned = impl_resolve_owned(&self.ident);
        let impl_resolve_ref = impl_resolve_ref(&self.ident);
        let impl_graphql_doc = Doc::from_attributes(&self.attrs)
            .map(|doc| impl_graphql_doc(&self.ident, &doc))
            .into_token_stream();

        tokens.extend(quote! {
            #impl_object
            #impl_resolve_owned
            #impl_resolve_ref
            #impl_graphql_doc
        });
    }
}

fn field_description(doc: &Doc) -> GeneratorResult<TokenStream> {
    if let Some(doc) = &doc.doc {
        Ok(quote! {
            let field = field.description(#doc);
        })
    } else {
        Ok(quote! {})
    }
}
fn define_fields(
    object: &ResolvedObjectFields,
    method: &ResolvedObjectFieldsMethod,
) -> GeneratorResult<TokenStream> {
    let field_ident = &method.base.ident;
    let field_name = calc_field_name(
        method.attrs.name.as_ref(),
        &field_ident.to_string(),
        &object.attrs.rename_fields,
    );
    let ty = method.base.output_type.as_ref().ok_or_else(|| {
        darling::Error::custom("Field must have return type").with_span(&method.base.ident)
    })?;
    let create_name = get_create_name();

    let is_str = is_type_str(ty);
    let is_ref = is_type_ref(ty);
    let is_owned = !is_ref;
    let owned_type = get_owned_type(ty);

    let resolve_ref = quote! {
        #create_name::ResolveRef::resolve_ref(value, &ctx)
    };
    let resolve_owned = quote! {
        #create_name::ResolveOwned::resolve_owned(value, &ctx)
    };
    let resolve = if is_owned || is_str {
        resolve_owned
    } else {
        resolve_ref
    };

    let execute = if method.base.asyncness {
        quote! {
            let value = Self::#field_ident(parent).await;
        }
    } else {
        quote! {
            let value = Self::#field_ident(parent);
        }
    };

    let description = field_description(&method.doc)?;
    let deprecation = field_deprecation(&method.attrs.deprecation);

    // todo args
    Ok(quote! {
        let field = #create_name::dynamic::Field::new(#field_name, <#owned_type as #create_name::GetOutputTypeRef>::get_output_type_ref(), |ctx| {
            #create_name::dynamic::FieldFuture::new(async move {
                let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                #execute
                #resolve
            })
        });
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

fn impl_register(object: &ResolvedObjectFields) -> GeneratorResult<TokenStream> {
    let create_name = get_create_name();
    let ty = &object.base.ty;
    let define_object = impl_define_object();
    let description = impl_object_description();
    let define_fields = object
        .base
        .methods
        .iter()
        .filter(|method| !method.attrs.skip)
        .map(|method| define_fields(object, method).into_token_stream())
        .collect::<Vec<_>>();

    Ok(quote! {
        impl #create_name::Register for #ty {
            fn register(registry: #create_name::Registry) -> #create_name::Registry {
                #define_object

                #(#define_fields)*

                #description

                registry.register_type(object)
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
