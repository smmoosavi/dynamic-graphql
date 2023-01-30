use crate::types::{GetInputTypeRef, GetOutputTypeRef, GraphqlType, InputType, OutputType};
use crate::{Register, Registry};
use async_graphql::{dynamic, MaybeUndefined};
use std::borrow::Cow;

impl<T> Register for &T
where
    T: Register + 'static,
{
    fn register(registry: Registry) -> Registry {
        registry.register::<T>()
    }
}

impl<'a, T> GraphqlType for &'a T
where
    T: GraphqlType + 'static,
{
    const NAME: &'static str = <T as GraphqlType>::NAME;
}

impl<T: OutputType + 'static> OutputType for &T {}

impl<T: Register + Clone + 'static> Register for Cow<'_, T> {
    fn register(registry: Registry) -> Registry {
        registry.register::<T>()
    }
}
impl<T: OutputType + Clone + 'static> GraphqlType for Cow<'_, T> {
    const NAME: &'static str = <T as GraphqlType>::NAME;
}

impl<T: OutputType + Clone + 'static> OutputType for Cow<'_, T> {}

impl Register for String {}
impl GraphqlType for String {
    const NAME: &'static str = dynamic::TypeRef::STRING;
}

impl InputType for String {}

impl OutputType for String {}

impl Register for &str {}
impl GraphqlType for &str {
    const NAME: &'static str = dynamic::TypeRef::STRING;
}

impl InputType for &str {}

impl OutputType for &str {}

impl GraphqlType for str {
    const NAME: &'static str = dynamic::TypeRef::STRING;
}

impl Register for str {}
impl InputType for str {}

impl OutputType for str {}

impl Register for async_graphql::ID {}
impl GraphqlType for async_graphql::ID {
    const NAME: &'static str = dynamic::TypeRef::ID;
}

impl InputType for async_graphql::ID {}

impl OutputType for async_graphql::ID {}

impl Register for bool {}
impl GraphqlType for bool {
    const NAME: &'static str = dynamic::TypeRef::BOOLEAN;
}

impl InputType for bool {}

impl OutputType for bool {}

impl Register for f32 {}
impl GraphqlType for f32 {
    const NAME: &'static str = dynamic::TypeRef::FLOAT;
}

impl InputType for f32 {}

impl OutputType for f32 {}

impl Register for f64 {}
impl GraphqlType for f64 {
    const NAME: &'static str = dynamic::TypeRef::FLOAT;
}

impl InputType for f64 {}

impl OutputType for f64 {}

macro_rules! int_output_value {
    ($($t:ty),*) => {
        $(
            impl Register for $t {}
            impl GraphqlType for $t {
                const NAME: &'static str = dynamic::TypeRef::INT;
            }
            impl InputType for $t {}
            impl OutputType for $t {}
        )*
    };
}

int_output_value!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);

impl<T> Register for Option<T>
where
    T: Register + 'static,
{
    fn register(registry: Registry) -> Registry {
        registry.register::<T>()
    }
}
impl<T> Register for MaybeUndefined<T>
where
    T: Register + 'static,
{
    fn register(registry: Registry) -> Registry {
        registry.register::<T>()
    }
}
impl<T, E> Register for Result<T, E>
where
    T: Register + 'static,
{
    fn register(registry: Registry) -> Registry {
        registry.register::<T>()
    }
}
impl<T> Register for Vec<T>
where
    T: Register + 'static,
{
    fn register(registry: Registry) -> Registry {
        registry.register::<T>()
    }
}
impl<T> Register for &[T]
where
    T: Register + 'static,
{
    fn register(registry: Registry) -> Registry {
        registry.register::<T>()
    }
}

pub trait TypeRefExt {
    fn optional(self) -> Self;
    fn list(self) -> Self;
}

pub enum TypeRefInner {
    Named(&'static str),
    NamedNN(&'static str),
    List(&'static str),
    ListNN(&'static str),
    NNList(&'static str),
    NNListNN(&'static str),
}

impl From<TypeRefInner> for dynamic::TypeRef {
    fn from(value: TypeRefInner) -> Self {
        match value {
            TypeRefInner::Named(name) => dynamic::TypeRef::named(name),
            TypeRefInner::NamedNN(name) => dynamic::TypeRef::named_nn(name),
            TypeRefInner::List(name) => dynamic::TypeRef::named_list(name),
            TypeRefInner::ListNN(name) => dynamic::TypeRef::named_list_nn(name),
            TypeRefInner::NNList(name) => dynamic::TypeRef::named_nn_list(name),
            TypeRefInner::NNListNN(name) => dynamic::TypeRef::named_nn_list_nn(name),
        }
    }
}

impl TypeRefExt for TypeRefInner {
    fn optional(self) -> Self {
        match self {
            TypeRefInner::Named(name) => TypeRefInner::Named(name),
            TypeRefInner::NamedNN(name) => TypeRefInner::Named(name),
            TypeRefInner::List(name) => TypeRefInner::List(name),
            TypeRefInner::ListNN(name) => TypeRefInner::List(name),
            TypeRefInner::NNList(name) => TypeRefInner::NNList(name),
            TypeRefInner::NNListNN(name) => TypeRefInner::NNList(name),
        }
    }

    fn list(self) -> Self {
        match self {
            TypeRefInner::Named(name) => TypeRefInner::ListNN(name),
            TypeRefInner::NamedNN(name) => TypeRefInner::NNListNN(name),
            TypeRefInner::List(name) => TypeRefInner::List(name),
            TypeRefInner::ListNN(name) => TypeRefInner::ListNN(name),
            TypeRefInner::NNList(name) => TypeRefInner::NNList(name),
            TypeRefInner::NNListNN(name) => TypeRefInner::NNListNN(name),
        }
    }
}

impl<T, E> GetOutputTypeRef for Result<T, E>
where
    T: GetOutputTypeRef,
{
    type Output = T::Output;
    #[inline]
    fn get_output_type_ref() -> Self::Output {
        T::get_output_type_ref()
    }
}

impl<T: OutputType> GetOutputTypeRef for T {
    type Output = TypeRefInner;
    #[inline]
    fn get_output_type_ref() -> Self::Output {
        TypeRefInner::NamedNN(<T as OutputType>::NAME)
    }
}

impl<T: GetOutputTypeRef<Output = TypeRefInner>> GetOutputTypeRef for Option<T> {
    type Output = TypeRefInner;
    #[inline]
    fn get_output_type_ref() -> Self::Output {
        let t = T::get_output_type_ref();
        t.optional()
    }
}

impl<T: GetOutputTypeRef<Output = TypeRefInner>> GetOutputTypeRef for Vec<T> {
    type Output = TypeRefInner;
    #[inline]
    fn get_output_type_ref() -> Self::Output {
        T::get_output_type_ref().list()
    }
}

impl<T: GetOutputTypeRef<Output = TypeRefInner>> GetOutputTypeRef for &[T] {
    type Output = TypeRefInner;
    #[inline]
    fn get_output_type_ref() -> Self::Output {
        T::get_output_type_ref().list()
    }
}

impl<T: InputType> GetInputTypeRef for T {
    type Output = TypeRefInner;
    #[inline]
    fn get_input_type_ref() -> Self::Output {
        TypeRefInner::NamedNN(<T as InputType>::NAME)
    }
}

impl<T: GetInputTypeRef<Output = TypeRefInner>> GetInputTypeRef for Option<T> {
    type Output = TypeRefInner;
    #[inline]
    fn get_input_type_ref() -> Self::Output {
        T::get_input_type_ref().optional()
    }
}

impl<T: GetInputTypeRef<Output = TypeRefInner>> GetInputTypeRef for MaybeUndefined<T> {
    type Output = TypeRefInner;
    #[inline]
    fn get_input_type_ref() -> Self::Output {
        T::get_input_type_ref().optional()
    }
}

impl<T: GetInputTypeRef<Output = TypeRefInner>> GetInputTypeRef for Vec<T> {
    type Output = TypeRefInner;
    #[inline]
    fn get_input_type_ref() -> Self::Output {
        T::get_input_type_ref().list()
    }
}
impl<T: GetInputTypeRef<Output = TypeRefInner>> GetInputTypeRef for &[T] {
    type Output = TypeRefInner;
    #[inline]
    fn get_input_type_ref() -> Self::Output {
        T::get_input_type_ref().list()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_output_type_ref() {
        let type_ref: dynamic::TypeRef = <String as GetOutputTypeRef>::get_output_type_ref().into();
        assert_eq!(type_ref.to_string(), "String!");
        let type_ref: dynamic::TypeRef =
            <Option<String> as GetOutputTypeRef>::get_output_type_ref().into();
        assert_eq!(type_ref.to_string(), "String");
        let type_ref: dynamic::TypeRef =
            <Vec<String> as GetOutputTypeRef>::get_output_type_ref().into();
        assert_eq!(type_ref.to_string(), "[String!]!");
        let type_ref: dynamic::TypeRef =
            <Option<Vec<String>> as GetOutputTypeRef>::get_output_type_ref().into();
        assert_eq!(type_ref.to_string(), "[String!]");
        let type_ref: dynamic::TypeRef =
            <Vec<Option<String>> as GetOutputTypeRef>::get_output_type_ref().into();
        assert_eq!(type_ref.to_string(), "[String]!");
        let type_ref: dynamic::TypeRef =
            <Option<Vec<Option<String>>> as GetOutputTypeRef>::get_output_type_ref().into();
        assert_eq!(type_ref.to_string(), "[String]");
        let type_ref: dynamic::TypeRef =
            <&[String] as GetOutputTypeRef>::get_output_type_ref().into();
        assert_eq!(type_ref.to_string(), "[String!]!");
        let type_ref: dynamic::TypeRef =
            <Option<&[String]> as GetOutputTypeRef>::get_output_type_ref().into();
        assert_eq!(type_ref.to_string(), "[String!]");
        let type_ref: dynamic::TypeRef =
            <&[Option<String>] as GetOutputTypeRef>::get_output_type_ref().into();
        assert_eq!(type_ref.to_string(), "[String]!");
        let type_ref: dynamic::TypeRef =
            <Option<&[Option<String>]> as GetOutputTypeRef>::get_output_type_ref().into();
        assert_eq!(type_ref.to_string(), "[String]");
    }

    #[test]
    fn test_get_input_type_ref() {
        let type_ref: dynamic::TypeRef = <String as GetInputTypeRef>::get_input_type_ref().into();
        assert_eq!(type_ref.to_string(), "String!");
        let type_ref: dynamic::TypeRef =
            <Option<String> as GetInputTypeRef>::get_input_type_ref().into();
        assert_eq!(type_ref.to_string(), "String");
        let type_ref: dynamic::TypeRef =
            <Vec<String> as GetInputTypeRef>::get_input_type_ref().into();
        assert_eq!(type_ref.to_string(), "[String!]!");
        let type_ref: dynamic::TypeRef =
            <Option<Vec<String>> as GetInputTypeRef>::get_input_type_ref().into();
        assert_eq!(type_ref.to_string(), "[String!]");
        let type_ref: dynamic::TypeRef =
            <Vec<Option<String>> as GetInputTypeRef>::get_input_type_ref().into();
        assert_eq!(type_ref.to_string(), "[String]!");
        let type_ref: dynamic::TypeRef =
            <Option<Vec<Option<String>>> as GetInputTypeRef>::get_input_type_ref().into();
        assert_eq!(type_ref.to_string(), "[String]");
        let type_ref: dynamic::TypeRef =
            <&[String] as GetInputTypeRef>::get_input_type_ref().into();
        assert_eq!(type_ref.to_string(), "[String!]!");
        let type_ref: dynamic::TypeRef =
            <Option<&[String]> as GetInputTypeRef>::get_input_type_ref().into();
        assert_eq!(type_ref.to_string(), "[String!]");
        let type_ref: dynamic::TypeRef =
            <&[Option<String>] as GetInputTypeRef>::get_input_type_ref().into();
        assert_eq!(type_ref.to_string(), "[String]!");
        let type_ref: dynamic::TypeRef =
            <Option<&[Option<String>]> as GetInputTypeRef>::get_input_type_ref().into();
        assert_eq!(type_ref.to_string(), "[String]");
    }
}
