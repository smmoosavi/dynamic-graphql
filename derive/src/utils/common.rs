use crate::utils::deprecation::Deprecation;
use crate::utils::impl_block::BaseFnArg;
use crate::utils::interface_attr::{InterfaceImplAttr, InterfaceMarkAttr};
use crate::utils::rename_rule::RenameRule;

pub trait CommonObject {
    /// user defined name
    fn get_name(&self) -> Option<&str>;
    fn should_impl_type_name(&self) -> bool;
    fn get_ident(&self) -> &syn::Ident;
    fn get_type(&self) -> darling::Result<syn::Path>;
    fn get_generics(&self) -> darling::Result<&syn::Generics>;
    fn get_doc(&self) -> darling::Result<Option<String>>;
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
    fn get_ident(&self) -> darling::Result<&syn::Ident>;
    fn get_type(&self) -> darling::Result<&syn::Type>;
    fn get_skip(&self) -> bool;
    fn get_doc(&self) -> darling::Result<Option<String>>;
    fn get_deprecation(&self) -> darling::Result<Deprecation> {
        Ok(Deprecation::NoDeprecated)
    }
    fn get_field_rename_rule(&self) -> Option<&RenameRule> {
        None
    }
    fn get_args_rename_rule(&self) -> Option<&RenameRule> {
        None
    }
}

pub trait CommonMethod: CommonField {
    fn is_async(&self) -> bool;
}

pub trait CommonInterfaceAttrs: CommonObject {
    fn get_marks(&self) -> &Vec<InterfaceMarkAttr>;
    fn get_impls(&self) -> &Vec<InterfaceImplAttr>;
}

pub trait CommonArg {
    /// user defined name
    fn get_name(&self) -> Option<&str>;
    fn get_index(&self) -> usize;
    fn get_arg(&self) -> &BaseFnArg;
    fn get_arg_rename_rule(&self) -> Option<&RenameRule> {
        None
    }
    fn is_marked_as_ctx(&self) -> bool;
}

pub trait GetFields<F> {
    fn get_fields(&self) -> darling::Result<&Vec<F>>;
}

pub trait GetArgs<A> {
    fn get_args(&self) -> darling::Result<&Vec<A>>;
}

impl CommonArg for () {
    fn get_name(&self) -> Option<&str> {
        unreachable!("() doesn't have a name")
    }

    fn get_index(&self) -> usize {
        unreachable!("() doesn't have an index")
    }

    fn get_arg(&self) -> &BaseFnArg {
        unreachable!("() doesn't have an arg")
    }

    fn is_marked_as_ctx(&self) -> bool {
        unreachable!("() doesn't have an arg")
    }
}

pub static EMPTY_ARGS: Vec<()> = Vec::new();
