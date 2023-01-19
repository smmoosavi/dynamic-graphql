use crate::registry::Registry;
use async_graphql::dynamic::TypeRef;

mod common;

pub trait Register {
    fn register(registry: Registry) -> Registry;
}

pub trait RegisterFns {
    const REGISTER_FNS: &'static [fn(registry: Registry) -> Registry];
}

pub trait GraphqlType {
    const NAME: &'static str;
}

pub trait GraphqlDoc {
    const DOC: Option<&'static str>;
}

pub trait OutputType: GraphqlType {
    const NAME: &'static str = <Self as GraphqlType>::NAME;
}

pub trait InputType: GraphqlType {
    const NAME: &'static str = <Self as GraphqlType>::NAME;
}

pub trait Object: OutputType + InterfaceTarget {
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

pub trait InterfaceTarget {
    const TARGET: &'static str;
}

pub trait InterfaceMark<const MARK: u64> {}

pub struct InterfaceRoot;

pub trait InputObject: InputType {
    const NAME: &'static str = <Self as InputType>::NAME;
}

pub trait Mutation: Object {}

pub trait ExpandObject: InterfaceTarget {
    const NAME: &'static str;
    type Target: Object;
}

pub trait GetOutputTypeRef {
    fn get_output_type_ref() -> TypeRef;
}

pub trait GetInputTypeRef {
    fn get_input_type_ref() -> TypeRef;
}
