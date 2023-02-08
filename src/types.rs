use crate::registry::Registry;
use async_graphql::dynamic::TypeRef;

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

pub trait TypeName: Register {
    fn get_type_name() -> String;
}

pub trait OutputTypeName: TypeName {
    fn get_output_type_name() -> String {
        <Self as TypeName>::get_type_name()
    }
}

pub trait InputTypeName: TypeName {
    fn get_input_type_name() -> String {
        <Self as TypeName>::get_type_name()
    }
}

pub trait Object: OutputTypeName + ParentType {
    fn get_object_type_name() -> String {
        <Self as OutputTypeName>::get_output_type_name()
    }
}

pub trait Enum: OutputTypeName {
    fn get_enum_type_name() -> String {
        <Self as OutputTypeName>::get_output_type_name()
    }
}

pub trait Scalar: OutputTypeName {
    fn get_scalar_type_name() -> String {
        <Self as OutputTypeName>::get_output_type_name()
    }
}

pub trait Union: OutputTypeName {
    fn get_union_type_name() -> String {
        <Self as OutputTypeName>::get_output_type_name()
    }
}

pub trait Interface: OutputTypeName {
    fn get_interface_type_name() -> String {
        <Self as OutputTypeName>::get_output_type_name()
    }
}

pub trait ParentType {
    type Type: Object;
}

pub trait InterfaceMark<T: Interface + ?Sized> {}

pub trait InputObject: InputTypeName {
    fn get_input_object_type_name() -> String {
        <Self as InputTypeName>::get_input_type_name()
    }
}

pub trait Mutation: ExpandObject {}

pub trait ExpandObject: ParentType {
    fn get_expand_object_name() -> String;
}

pub trait GetOutputTypeRef {
    type Output: Into<TypeRef>;
    fn get_output_type_ref() -> Self::Output;
}

pub trait GetInputTypeRef {
    type Output: Into<TypeRef>;
    fn get_input_type_ref() -> Self::Output;
}
