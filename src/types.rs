use crate::registry::Registry;
use async_graphql::dynamic::TypeRef;
use std::borrow::Cow;

mod common;

pub trait Register {
    #[inline]
    fn register(registry: Registry) -> Registry {
        registry
    }
}

pub trait RegisterFns {
    const REGISTER_FNS: &'static [fn(registry: Registry) -> Registry];
}

pub trait GraphqlType: Register {
    fn get_type_name() -> Cow<'static, str>;
}

pub trait OutputType: GraphqlType {
    fn get_output_type_name() -> Cow<'static, str> {
        <Self as GraphqlType>::get_type_name()
    }
}

pub trait InputType: GraphqlType {
    fn get_input_type_name() -> Cow<'static, str> {
        <Self as GraphqlType>::get_type_name()
    }
}

pub trait Object: OutputType + ParentType {
    fn get_object_type_name() -> Cow<'static, str> {
        <Self as OutputType>::get_output_type_name()
    }
}

pub trait Enum: OutputType {
    fn get_enum_type_name() -> Cow<'static, str> {
        <Self as OutputType>::get_output_type_name()
    }
}

pub trait Scalar: OutputType {
    fn get_scalar_type_name() -> Cow<'static, str> {
        <Self as OutputType>::get_output_type_name()
    }
}

pub trait Union: OutputType {
    fn get_union_type_name() -> Cow<'static, str> {
        <Self as OutputType>::get_output_type_name()
    }
}

pub trait Interface: OutputType {
    fn get_interface_type_name() -> Cow<'static, str> {
        <Self as OutputType>::get_output_type_name()
    }
}

pub trait ParentType {
    type Type: Object;
}

pub trait InterfaceMark<T: Interface + ?Sized> {}

pub trait InputObject: InputType {
    fn get_input_object_type_name() -> Cow<'static, str> {
        <Self as InputType>::get_input_type_name()
    }
}

pub trait Mutation: ExpandObject {}

pub trait ExpandObject: ParentType {
    fn get_expand_object_name() -> Cow<'static, str>;
}

pub trait GetOutputTypeRef {
    type Output: Into<TypeRef>;
    fn get_output_type_ref() -> Self::Output;
}

pub trait GetInputTypeRef {
    type Output: Into<TypeRef>;
    fn get_input_type_ref() -> Self::Output;
}
