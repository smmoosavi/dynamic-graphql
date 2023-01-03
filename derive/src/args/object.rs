use crate::utils::crate_name::get_create_name;
use crate::utils::error::GeneratorResult;
use darling::ast::{Data, Fields};
use darling::util::Ignored;
use darling::{FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::quote;

#[derive(FromField)]
#[darling(attributes(graphql))]
pub struct ObjectField {
    pub ident: Option<syn::Ident>,
    pub ty: syn::Type,

    #[darling(default)]
    pub skip: bool,

    #[darling(default)]
    pub name: Option<String>,
}

#[derive(FromDeriveInput)]
#[darling(attributes(graphql))]
pub struct Object {
    pub ident: syn::Ident,
    pub data: Data<Ignored, ObjectField>,

    #[darling(default)]
    pub name: Option<String>,
}

fn get_type_name(object: &Object) -> String {
    object
        .name
        .clone()
        .unwrap_or_else(|| object.ident.to_string())
}

fn impl_object(object: &Object) -> TokenStream {
    let ident = &object.ident;
    let name = get_type_name(object);
    let create_name = get_create_name();
    quote! {
        impl #create_name::GraphqlType for #ident {
            const NAME: &'static str = #name;
        }
        impl #create_name::OutputType for #ident {}
        impl #create_name::Object for #ident {}
    }
}

fn impl_resolve_owned(object: &Object) -> TokenStream {
    let ident = &object.ident;
    let create_name = get_create_name();
    quote! {
        impl<'a> #create_name::ResolveOwned<'a> for #ident {
            fn resolve_owned(self, _ctx: &#create_name::Context) -> #create_name::Result<Option<#create_name::FieldValue<'a>>> {
                Ok(Some(#create_name::FieldValue::owned_any(self)))
            }
        }
    }
}

fn impl_resolve_ref(object: &Object) -> TokenStream {
    let ident = &object.ident;
    let create_name = get_create_name();
    quote! {
        impl<'a> #create_name::ResolveRef<'a> for #ident {
            fn resolve_ref(&'a self, _ctx: &#create_name::Context) -> #create_name::Result<Option<#create_name::FieldValue<'a>>> {
                Ok(Some(#create_name::FieldValue::borrowed_any(self)))
            }
        }
    }
}

fn get_fields(object: &Object) -> GeneratorResult<&Fields<ObjectField>> {
    match object.data {
        Data::Struct(ref data) => Ok(data),
        Data::Enum(_) => {
            Err(syn::Error::new_spanned(&object.ident, "Object can't applied to enum").into())
        }
    }
}

fn get_field_ident(field: &ObjectField) -> GeneratorResult<&syn::Ident> {
    let ident = field.ident.as_ref().ok_or_else(|| {
        syn::Error::new_spanned(&field.ident, "Object can't applied to tuple fields")
    })?;
    Ok(ident)
}

fn impl_resolver(field: &ObjectField) -> GeneratorResult<TokenStream> {
    // fn resolve_<field_name>(&self) -> &<field_type> { &self.<field_name> }
    let field_ident = get_field_ident(field)?;
    let name = field_ident.to_string();
    let resolver_name = format!("resolve_{}", name);
    let resolver_ident = syn::Ident::new(&resolver_name, field_ident.span());
    let ty = &field.ty;
    Ok(quote! {
        fn #resolver_ident(&self) -> &#ty {
            &self.#field_ident
        }
    })
}

fn impl_resolvers(object: &Object) -> GeneratorResult<TokenStream> {
    let ident = &object.ident;
    let struct_data = get_fields(object)?;
    let fields = struct_data
        .fields
        .iter()
        .filter(|field| !field.skip)
        .map(impl_resolver)
        .collect::<GeneratorResult<Vec<TokenStream>>>()?;
    Ok(quote! {
        impl #ident {
            #(#fields)*
        }
    })
}

fn impl_define_object(_object: &Object) -> GeneratorResult<TokenStream> {
    let create_name = get_create_name();
    Ok(quote! {
        let object = #create_name::dynamic::Object::new(<Self as #create_name::Object>::NAME);
    })
}

fn impl_define_field(field: &ObjectField) -> GeneratorResult<TokenStream> {
    let field_ident = get_field_ident(field)?;
    let name = field_ident.to_string();
    let field_name = field
        .name
        .clone()
        .unwrap_or_else(|| field_ident.to_string());
    let ty = &field.ty;
    let resolver_name = format!("resolve_{}", name);
    let resolver_ident = syn::Ident::new(&resolver_name, field_ident.span());
    let create_name = get_create_name();
    Ok(quote! {
        let field = #create_name::dynamic::Field::new(#field_name, <#ty as #create_name::GetOutputTypeRef>::get_output_type_ref(), |ctx| {
            #create_name::dynamic::FieldFuture::new(async move {
                let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
                let value = Self::#resolver_ident(parent);
                #create_name::ResolveRef::resolve_ref(value, &ctx)
            })
        });
        let object = object.field(field);
    })
}

fn impl_register(object: &Object) -> GeneratorResult<TokenStream> {
    let create_name = get_create_name();
    let ident = &object.ident;
    let define_object = impl_define_object(object)?;
    let fields = get_fields(object)?;
    let define_fields = fields
        .fields
        .iter()
        .filter(|field| !field.skip)
        .map(impl_define_field)
        .collect::<GeneratorResult<Vec<TokenStream>>>()?;
    Ok(quote! {
        impl #create_name::Register for #ident {
            fn register(registry: #create_name::Registry) -> #create_name::Registry {
                #define_object

                #(#define_fields)*

                registry.register_type(object)
            }
        }
    })
}

impl Object {
    pub fn generate(&self) -> GeneratorResult<TokenStream> {
        let impl_object = impl_object(self);
        let impl_resolve_owned = impl_resolve_owned(self);
        let impl_resolve_ref = impl_resolve_ref(self);
        let impl_resolvers = impl_resolvers(self)?;
        let impl_register = impl_register(self)?;
        Ok(quote! {
            #impl_object
            #impl_resolve_owned
            #impl_resolve_ref
            #impl_resolvers
            #impl_register
        })
    }
}
