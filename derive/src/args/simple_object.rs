use darling::FromAttributes;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::Generics;
use syn::Path;

use crate::args::common;
use crate::args::common::get_register_interface_code;
use crate::args::common::FieldImplementor;
use crate::utils::common::CommonField;
use crate::utils::common::CommonInterfaceAttrs;
use crate::utils::common::CommonObject;
use crate::utils::common::GetArgs;
use crate::utils::common::GetFields;
use crate::utils::common::EMPTY_ARGS;
use crate::utils::crate_name::get_crate_name;
use crate::utils::deprecation::Deprecation;
use crate::utils::derive_types::BaseStruct;
use crate::utils::derive_types::NamedField;
use crate::utils::error::IntoTokenStream;
use crate::utils::interface_attr::InterfaceImplAttr;
use crate::utils::interface_attr::InterfaceMarkAttr;
use crate::utils::macros::*;
use crate::utils::register_attr::RegisterAttr;
use crate::utils::rename_rule::RenameRule;
use crate::utils::with_attributes::WithAttributes;
use crate::utils::with_context::MakeContext;
use crate::utils::with_context::WithContext;
use crate::utils::with_doc::WithDoc;

#[derive(FromAttributes, Debug, Clone)]
#[darling(attributes(graphql))]
pub struct SimpleObjectFieldAttrs {
    #[darling(default)]
    pub skip: bool,

    #[darling(default)]
    pub name: Option<String>,

    #[darling(default)]
    pub deprecation: Deprecation,
}

#[derive(Default, Debug, Clone)]
pub struct SimpleObjectFieldContext {
    pub rename_fields: Option<RenameRule>,
}

from_field!(
    SimpleObjectField,
    WithAttributes<
        WithDoc<SimpleObjectFieldAttrs>,
        WithContext<SimpleObjectFieldContext, NamedField>,
    >,
);

#[derive(FromAttributes, Debug, Clone)]
#[darling(attributes(graphql))]
pub struct SimpleObjectAttrs {
    #[darling(default)]
    pub root: bool,

    #[darling(skip)]
    pub mutation_root: bool,

    #[darling(default)]
    pub name: Option<String>,

    #[darling(default)]
    #[darling(rename = "get_type_name")]
    pub type_name: bool,

    #[darling(default)]
    pub rename_fields: Option<RenameRule>,

    #[darling(default, multiple)]
    #[darling(rename = "register")]
    pub registers: Vec<RegisterAttr>,

    #[darling(default, multiple)]
    #[darling(rename = "mark")]
    pub marks: Vec<InterfaceMarkAttr>,

    #[darling(default, multiple)]
    #[darling(rename = "implements")]
    pub impls: Vec<InterfaceImplAttr>,
}

from_derive_input!(
    SimpleObject,
    WithAttributes<WithDoc<SimpleObjectAttrs>, BaseStruct<SimpleObjectField, Generics>>,
    ctx,
);

impl MakeContext<SimpleObjectFieldContext> for SimpleObject {
    fn make_context(&self) -> SimpleObjectFieldContext {
        SimpleObjectFieldContext {
            rename_fields: self.attrs.rename_fields,
        }
    }
}

impl CommonInterfaceAttrs for SimpleObject {
    fn get_marks(&self) -> &Vec<InterfaceMarkAttr> {
        &self.attrs.marks
    }

    fn get_impls(&self) -> &Vec<InterfaceImplAttr> {
        &self.attrs.impls
    }
}

impl CommonObject for SimpleObject {
    fn get_name(&self) -> Option<&str> {
        self.attrs.name.as_deref()
    }

    fn should_impl_type_name(&self) -> bool {
        !self.attrs.type_name
    }

    fn get_ident(&self) -> &syn::Ident {
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

impl CommonField for SimpleObjectField {
    fn get_name(&self) -> Option<&str> {
        self.attrs.name.as_deref()
    }

    fn get_ident(&self) -> darling::Result<&Ident> {
        Ok(&self.ident)
    }

    fn get_type(&self) -> darling::Result<&syn::Type> {
        Ok(&self.ty)
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
}

impl FieldImplementor for SimpleObjectField {
    fn define_field(&self) -> darling::Result<TokenStream> {
        common::define_field(self)
    }

    fn get_execute_code(&self) -> darling::Result<TokenStream> {
        let resolver_ident = get_resolver_ident(self)?;

        Ok(quote! {
            let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
            let value = Self::#resolver_ident(parent);
        })
    }

    fn get_resolve_code(&self) -> darling::Result<TokenStream> {
        common::resolve_value_code()
    }

    fn get_field_argument_definition(&self) -> darling::Result<TokenStream> {
        Ok(quote!())
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

impl GetFields<SimpleObjectField> for SimpleObject {
    fn get_fields(&self) -> darling::Result<&Vec<SimpleObjectField>> {
        Ok(&self.data.fields)
    }
}

impl GetArgs<()> for SimpleObjectField {
    fn get_args(&self) -> darling::Result<&Vec<()>> {
        Ok(&EMPTY_ARGS)
    }
}

fn get_resolver_ident(field: &impl CommonField) -> darling::Result<Ident> {
    let field_ident = field.get_ident()?;
    let resolver_name = format!("__resolve_{}", field_ident);

    let resolver_ident = syn::Ident::new(&resolver_name, field_ident.span());
    Ok(resolver_ident)
}

fn impl_resolver(field: &impl CommonField) -> darling::Result<TokenStream> {
    let field_ident = field.get_ident()?;
    let resolver_ident = get_resolver_ident(field)?;
    let ty = field.get_type()?;
    Ok(quote! {
        fn #resolver_ident(&self) -> &#ty {
            &self.#field_ident
        }
    })
}

fn impl_resolvers<O, F>(object: &O) -> darling::Result<TokenStream>
where
    O: CommonObject + GetFields<F>,
    F: CommonField,
{
    let ident = object.get_ident();
    let fields = object
        .get_fields()?
        .iter()
        .filter(|field| !field.get_skip())
        .map(impl_resolver)
        .map(|r| r.into_token_stream())
        .collect::<Vec<TokenStream>>();
    let (impl_generics, ty_generics, where_clause) = object.get_generics()?.split_for_impl();
    Ok(quote! {
        impl #impl_generics #ident #ty_generics #where_clause {
            #(#fields)*
        }
    })
}

fn root_register_code(object: &SimpleObject) -> TokenStream {
    let root = if object.attrs.root {
        let crate_name = get_crate_name();
        Some(quote! {
            let registry = registry.set_root(<Self as #crate_name::internal::Object>::get_object_type_name().as_ref());
        })
    } else {
        None
    };
    let mutation_root = if object.attrs.mutation_root {
        let crate_name = get_crate_name();
        Some(quote! {
            let registry = registry.set_mutation(<Self as #crate_name::internal::Object>::get_object_type_name().as_ref());
        })
    } else {
        None
    };
    quote!(#root #mutation_root)
}

fn impl_register(object: &SimpleObject) -> darling::Result<TokenStream> {
    let crate_name = get_crate_name();

    let register_nested_types = common::get_nested_type_register_code(object).into_token_stream();
    let root_register = root_register_code(object);

    let ident = &object.ident;
    let define_object = common::impl_define_object();
    let add_interfaces = common::get_interface_mark_code(object)?;
    let register_interface_code = get_register_interface_code(object)?;
    let implement = common::get_add_implement_code(object, object.get_impls())?;

    let description = common::object_description(object.get_doc()?.as_deref())?;
    let define_fields = common::get_define_fields_code(object)?;
    let register_object_code = common::register_object_code();

    let (impl_generics, ty_generics, where_clause) = object.generics.split_for_impl();
    let register_attr = &object.attrs.registers;

    Ok(quote! {
        impl #impl_generics #crate_name::internal::Register for #ident #ty_generics #where_clause {
            fn register(registry: #crate_name::internal::Registry) -> #crate_name::internal::Registry {

                #( #register_attr )*

                #register_interface_code

                #register_nested_types

                #root_register

                #define_object

                #implement

                #add_interfaces

                #description

                #define_fields

                #register_object_code
            }
        }
    })
}

impl ToTokens for SimpleObject {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let impl_object = common::impl_object(self).into_token_stream();
        let impl_resolve_owned = common::impl_resolve_owned(self).into_token_stream();
        let impl_resolve_ref = common::impl_resolve_ref(self).into_token_stream();
        let impl_resolvers = impl_resolvers(self).into_token_stream();
        let impl_register = impl_register(self).into_token_stream();
        let impl_interface_mark = common::impl_interface_mark(self).into_token_stream();

        tokens.extend(quote! {
            #impl_object
            #impl_interface_mark
            #impl_resolve_owned
            #impl_resolve_ref
            #impl_resolvers
            #impl_register
        })
    }
}
