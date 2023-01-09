mod with_parent;

pub use self::with_parent::WithParent;
use crate::utils::crate_name::get_create_name;
use crate::utils::deprecation::Deprecation;
use crate::utils::error::GeneratorResult;
use crate::utils::rename_rule::{
    calc_field_name, calc_input_field_name, calc_type_name, RenameRule,
};
use crate::utils::type_utils::get_owned_type;
use proc_macro2::TokenStream;
use quote::quote;

pub trait CommonObject {
    /// user defined name
    fn get_name(&self) -> Option<&str>;
    fn get_ident(&self) -> &syn::Ident;
    fn get_doc(&self) -> GeneratorResult<Option<String>>;
    fn get_fields_rename_rule(&self) -> Option<&RenameRule> {
        None
    }
    fn get_args_rename_rule(&self) -> Option<&RenameRule> {
        None
    }
}

pub trait CommonField {
    /// user defined name
    fn get_name(&self) -> Option<&str>;
    fn get_ident(&self) -> GeneratorResult<&syn::Ident>;
    fn get_type(&self) -> GeneratorResult<&syn::Type>;
    fn get_skip(&self) -> bool;
    fn get_doc(&self) -> GeneratorResult<Option<String>>;
    fn get_deprecation(&self) -> GeneratorResult<Deprecation> {
        Ok(Deprecation::NoDeprecated)
    }
    fn get_field_rename_rule(&self) -> Option<&RenameRule> {
        None
    }
    fn get_args_rename_rule(&self) -> Option<&RenameRule> {
        None
    }
}

pub trait CommonArg {
    /// user defined name
    fn get_name(&self) -> Option<&str>;
    fn get_arg_rename_rule(&self) -> Option<&RenameRule> {
        None
    }
}

pub fn impl_object(obj: &impl CommonObject) -> GeneratorResult<TokenStream> {
    let name = obj.get_name();
    let object_ident = obj.get_ident();
    let name = calc_type_name(name, &object_ident.to_string());
    let create_name = get_create_name();
    Ok(quote! {
        impl #create_name::GraphqlType for #object_ident {
            const NAME: &'static str = #name;
        }
        impl #create_name::OutputType for #object_ident {}
        impl #create_name::Object for #object_ident {}
    })
}

pub fn impl_input_object(obj: &impl CommonObject) -> GeneratorResult<TokenStream> {
    let name = obj.get_name();
    let object_ident = obj.get_ident();
    let name = calc_type_name(name, &object_ident.to_string());
    let create_name = get_create_name();
    Ok(quote! {
        impl #create_name::GraphqlType for #object_ident {
            const NAME: &'static str = #name;
        }
        impl #create_name::InputType for #object_ident {}
        impl #create_name::InputObject for #object_ident {}
    })
}

pub fn impl_graphql_doc(obj: &impl CommonObject) -> GeneratorResult<TokenStream> {
    let doc = obj.get_doc()?;
    let object_ident = obj.get_ident();
    let create_name = get_create_name();
    let doc = match doc {
        None => quote!(None),
        Some(ref doc) => quote!(Some(#doc)),
    };

    Ok(quote! {
        impl #create_name::GraphqlDoc for #object_ident {
            const DOC: Option<&'static str> = #doc;
        }
    })
}

pub fn impl_resolve_owned(obj: &impl CommonObject) -> GeneratorResult<TokenStream> {
    let create_name = get_create_name();
    let object_ident = obj.get_ident();

    Ok(quote! {
        impl<'a> #create_name::ResolveOwned<'a> for #object_ident {
            fn resolve_owned(self, _ctx: &#create_name::Context) -> #create_name::Result<Option<#create_name::FieldValue<'a>>> {
                Ok(Some(#create_name::FieldValue::owned_any(self)))
            }
        }
    })
}

pub fn impl_resolve_ref(obj: &impl CommonObject) -> GeneratorResult<TokenStream> {
    let create_name = get_create_name();
    let object_ident = obj.get_ident();
    Ok(quote! {
        impl<'a> #create_name::ResolveRef<'a> for #object_ident {
            fn resolve_ref(&'a self, _ctx: &#create_name::Context) -> #create_name::Result<Option<#create_name::FieldValue<'a>>> {
                Ok(Some(#create_name::FieldValue::borrowed_any(self)))
            }
        }
    })
}

pub fn impl_define_object() -> TokenStream {
    // todo get "object" from input
    let create_name = get_create_name();
    quote! {
        let object = #create_name::dynamic::Object::new(<Self as #create_name::Object>::NAME);
    }
}

pub fn impl_define_input_object() -> TokenStream {
    // todo get "object" from input
    let create_name = get_create_name();
    quote! {
        let object = #create_name::dynamic::InputObject::new(<Self as #create_name::InputObject>::NAME);
    }
}

pub fn register_object_code() -> TokenStream {
    quote!(registry.register_type(object))
}

pub fn field_deprecation_code(deprecation: &impl CommonField) -> GeneratorResult<TokenStream> {
    let deprecation = deprecation.get_deprecation()?;
    // todo get "field" from input
    match deprecation {
        Deprecation::NoDeprecated => Ok(quote! {}),
        Deprecation::Deprecated { reason: None } => Ok(quote! {
            let field = field.deprecation(None);
        }),
        Deprecation::Deprecated {
            reason: Some(ref reason),
        } => Ok(quote! {
            let field = field.deprecation(Some(#reason));
        }),
    }
}

pub fn field_description(field: &impl CommonField) -> GeneratorResult<TokenStream> {
    let doc = field.get_doc()?;
    // todo get "field" from input
    if let Some(doc) = doc {
        Ok(quote! {
            let field = field.description(#doc);
        })
    } else {
        Ok(quote! {})
    }
}

pub fn object_description(doc: Option<&str>) -> GeneratorResult<TokenStream> {
    // todo get "object" from input
    if let Some(doc) = doc {
        Ok(quote! {
            let object = object.description(#doc);
        })
    } else {
        Ok(quote! {})
    }
}

pub fn get_input_field_name(field: &impl CommonField) -> GeneratorResult<String> {
    Ok(calc_input_field_name(
        field.get_name(),
        &field.get_ident()?.to_string(),
        field.get_field_rename_rule(),
    ))
}

pub fn get_input_type_ref_code(field: &impl CommonField) -> GeneratorResult<TokenStream> {
    let create_name = get_create_name();
    let field_type = get_owned_type(field.get_type()?);
    Ok(quote! {
        <#field_type as #create_name::GetInputTypeRef>::get_input_type_ref()
    })
}

pub fn get_new_input_value_code(field: &impl CommonField) -> GeneratorResult<TokenStream> {
    // todo get "field" from input
    let create_name = get_create_name();
    let field_name = get_input_field_name(field)?;
    let get_input_type_ref_code = get_input_type_ref_code(field)?;

    Ok(quote! {
        let field = #create_name::dynamic::InputValue::new(#field_name, #get_input_type_ref_code);
    })
}

pub fn get_field_name(field: &impl CommonField) -> GeneratorResult<String> {
    Ok(calc_field_name(
        field.get_name(),
        &field.get_ident()?.to_string(),
        field.get_field_rename_rule(),
    ))
}
