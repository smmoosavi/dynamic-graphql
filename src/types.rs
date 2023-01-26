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

pub trait GraphqlType: Register {
    const NAME: &'static str;
}

pub trait OutputType: GraphqlType {
    const NAME: &'static str = <Self as GraphqlType>::NAME;
}

pub trait InputType: GraphqlType {
    const NAME: &'static str = <Self as GraphqlType>::NAME;
}

pub trait Object: OutputType + InterfaceTarget + ParentType {
    const NAME: &'static str = <Self as OutputType>::NAME;
}

pub trait Enum: OutputType {
    const NAME: &'static str = <Self as OutputType>::NAME;
}

pub trait Scalar: OutputType {
    const NAME: &'static str = <Self as OutputType>::NAME;
}

pub trait Union: OutputType {
    const NAME: &'static str = <Self as OutputType>::NAME;
}

pub trait Interface: OutputType {
    const NAME: &'static str = <Self as OutputType>::NAME;
    const MARK: u64;
}

pub trait ParentType {
    type Type: Object;
}

pub trait InterfaceTarget {
    const TARGET: &'static str;
}

pub trait InterfaceMark<const MARK: u64> {}

pub struct InterfaceRoot;

pub trait InputObject: InputType {
    const NAME: &'static str = <Self as InputType>::NAME;
}

pub trait Mutation: ExpandObject {}

pub trait ExpandObject: ParentType {
    const NAME: &'static str;
}

pub trait GetOutputTypeRef {
    type Output: Into<TypeRef>;
    fn get_output_type_ref() -> Self::Output;
}

pub trait GetInputTypeRef {
    type Output: Into<TypeRef>;
    fn get_input_type_ref() -> Self::Output;
}
