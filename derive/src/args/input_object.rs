use crate::args::common;
use crate::args::common::{CommonField, CommonObject, WithParent};
use crate::utils::crate_name::get_create_name;
use crate::utils::docs_utils::Doc;
use crate::utils::error::{GeneratorResult, IntoTokenStream, WithSpan};
use crate::utils::rename_rule::RenameRule;
use darling::ast::Data;
use darling::util::Ignored;
use darling::{FromAttributes, ToTokens};
use darling::{FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::quote;

#[derive(FromField)]
#[darling(attributes(graphql), forward_attrs(doc))]
pub struct InputObjectField {
    pub ident: Option<syn::Ident>,
    pub ty: syn::Type,
    pub attrs: Vec<syn::Attribute>,

    #[darling(default)]
    pub skip: bool,

    #[darling(default)]
    pub name: Option<String>,
}

#[derive(FromDeriveInput)]
#[darling(attributes(graphql), forward_attrs(doc))]
pub struct InputObject {
    pub ident: syn::Ident,
    pub data: Data<Ignored, InputObjectField>,
    pub attrs: Vec<syn::Attribute>,

    #[darling(default)]
    pub name: Option<String>,

    #[darling(default)]
    pub rename_fields: Option<RenameRule>,
}

impl CommonObject for InputObject {
    fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    fn get_ident(&self) -> &syn::Ident {
        &self.ident
    }

    fn get_doc(&self) -> GeneratorResult<Option<String>> {
        Ok(Doc::from_attributes(&self.attrs)?.doc)
    }
    fn get_fields_rename_rule(&self) -> Option<&RenameRule> {
        self.rename_fields.as_ref()
    }
}

impl CommonField for InputObjectField {
    fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    fn get_ident(&self) -> GeneratorResult<&syn::Ident> {
        self.ident.as_ref().ok_or_else(|| {
            darling::Error::custom("derive InputObject can't applied to tuple struct").into()
        })
    }

    fn get_type(&self) -> GeneratorResult<&syn::Type> {
        Ok(&self.ty)
    }

    fn get_skip(&self) -> bool {
        self.skip
    }

    fn get_doc(&self) -> GeneratorResult<Option<String>> {
        Ok(Doc::from_attributes(&self.attrs)?.doc)
    }
}

impl InputObjectField {
    fn with_parent<'f, 'p>(
        &'f self,
        parent: &'p InputObject,
    ) -> WithParent<'f, 'p, Self, InputObject> {
        WithParent::new(self, parent)
    }
}

fn get_define_field(
    object: &InputObject,
    field: &InputObjectField,
) -> GeneratorResult<TokenStream> {
    let field = field.with_parent(object);
    let description = common::field_description(&field)?;
    let get_new_input_value_code = common::get_new_input_value_code(&field)?;
    Ok(quote! {
        #get_new_input_value_code
        #description
        let object = object.field(field);
    })
}

fn get_define_fields(object: &InputObject) -> TokenStream {
    match &object.data {
        Data::Enum(_) => {
            quote! {}
        }
        Data::Struct(data) => data
            .fields
            .iter()
            .filter(|field| !field.get_skip())
            .map(|field| get_define_field(object, field).into_token_stream())
            .collect(),
    }
}

fn impl_register(object: &InputObject) -> GeneratorResult<TokenStream> {
    let create_name = get_create_name();
    let ident = &object.ident;
    let define_object = common::impl_define_input_object();
    let define_fields = get_define_fields(object);
    let doc = Doc::from_attributes(&object.attrs)?;
    let description = common::object_description(doc.as_deref())?;
    let register_object_code = common::register_object_code();
    Ok(quote! {
        impl #create_name::Register for #ident {
            fn register(registry: #create_name::Registry) -> #create_name::Registry {
                #define_object

                #description

                #define_fields

                #register_object_code
            }
        }
    })
}

fn get_item_ident(index: usize, ident: &syn::Ident) -> syn::Ident {
    syn::Ident::new(&format!("field{}", index), ident.span())
}

fn get_field_value(
    index: usize,
    object: &InputObject,
    field: &InputObjectField,
) -> GeneratorResult<TokenStream> {
    let field = field.with_parent(object);
    let create_name = get_create_name();
    let field_ident = field.get_ident().with_span(&object.ident)?;
    let item = get_item_ident(index, field_ident);
    let field_name = common::get_input_field_name(&field)?;
    if field.get_skip() {
        return Ok(quote! {
            let #item = Default::default();
        });
    }
    Ok(quote! {
        let #item = #create_name::FromValue::from_value(__object.try_get(#field_name)?)?;
    })
}

fn get_fields_value(object: &InputObject) -> TokenStream {
    match &object.data {
        Data::Enum(_) => {
            quote! {}
        }
        Data::Struct(data) => data
            .fields
            .iter()
            .enumerate()
            .map(|(index, field)| get_field_value(index, object, field).into_token_stream())
            .collect(),
    }
}

fn get_field_usage(
    index: usize,
    object: &InputObject,
    field: &InputObjectField,
) -> GeneratorResult<TokenStream> {
    let field = field.with_parent(object);
    let field_ident = field.get_ident()?;
    let item = get_item_ident(index, field_ident);
    Ok(quote! {
        #field_ident: #item,
    })
}

fn get_fields_usage(object: &InputObject) -> TokenStream {
    match &object.data {
        Data::Enum(_) => {
            quote! {}
        }
        Data::Struct(data) => {
            let items: Vec<_> = data
                .fields
                .iter()
                .enumerate()
                .map(|(index, field)| get_field_usage(index, object, field).into_token_stream())
                .collect();

            quote! {
                Ok(Self {
                    #(#items)*
                })
            }
        }
    }
}

fn impl_from_value(object: &InputObject) -> TokenStream {
    let create_name = get_create_name();
    let ident = &object.ident;
    let fields_value = get_fields_value(object);
    let fields_usage = get_fields_usage(object);
    quote!(
        impl #create_name::FromValue for #ident {
            fn from_value(__value: #create_name::dynamic::ValueAccessor) -> #create_name::Result<Self> {
                let __object = __value.object()?;
                #fields_value
                #fields_usage
            }
        }
    )
}

impl ToTokens for InputObject {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let impl_object = common::impl_input_object(self).into_token_stream();
        let impl_register = impl_register(self).into_token_stream();
        let impl_from_value = impl_from_value(self);
        tokens.extend(quote! {
            #impl_object
            #impl_register
            #impl_from_value
        });
    }
}
