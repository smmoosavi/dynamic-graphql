use crate::utils::crate_name::get_create_name;
use crate::utils::deprecation::Deprecation;
use crate::utils::rename_rule::calc_type_name;
use proc_macro2::TokenStream;
use quote::quote;

pub fn impl_object(name: &Option<String>, object_ident: &syn::Ident) -> TokenStream {
    let name = calc_type_name(name, &object_ident.to_string());
    let create_name = get_create_name();
    quote! {
        impl #create_name::GraphqlType for #object_ident {
            const NAME: &'static str = #name;
        }
        impl #create_name::OutputType for #object_ident {}
        impl #create_name::Object for #object_ident {}
    }
}

pub fn impl_resolve_owned(object_ident: &syn::Ident) -> TokenStream {
    let create_name = get_create_name();
    quote! {
        impl<'a> #create_name::ResolveOwned<'a> for #object_ident {
            fn resolve_owned(self, _ctx: &#create_name::Context) -> #create_name::Result<Option<#create_name::FieldValue<'a>>> {
                Ok(Some(#create_name::FieldValue::owned_any(self)))
            }
        }
    }
}

pub fn impl_resolve_ref(object_ident: &syn::Ident) -> TokenStream {
    let create_name = get_create_name();
    quote! {
        impl<'a> #create_name::ResolveRef<'a> for #object_ident {
            fn resolve_ref(&'a self, _ctx: &#create_name::Context) -> #create_name::Result<Option<#create_name::FieldValue<'a>>> {
                Ok(Some(#create_name::FieldValue::borrowed_any(self)))
            }
        }
    }
}

pub fn impl_define_object() -> TokenStream {
    let create_name = get_create_name();
    quote! {
        let object = #create_name::dynamic::Object::new(<Self as #create_name::Object>::NAME);
    }
}

pub fn field_deprecation(deprecation: &Deprecation) -> TokenStream {
    match deprecation {
        Deprecation::NoDeprecated => quote! {},
        Deprecation::Deprecated { reason: None } => quote! {
            let field = field.deprecation(None);
        },
        Deprecation::Deprecated {
            reason: Some(ref reason),
        } => quote! {
            let field = field.deprecation(Some(#reason));
        },
    }
}
