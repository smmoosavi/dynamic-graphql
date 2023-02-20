use std::borrow::Cow;

use async_graphql::dynamic;
use async_graphql::MaybeUndefined;

use crate::registry::Registry;
use crate::type_ref_builder::TypeRefBuilder;
use crate::types::GetInputTypeRef;
use crate::types::GetOutputTypeRef;
use crate::types::InputTypeName;
use crate::types::OutputTypeName;
use crate::types::Register;
use crate::types::TypeName;

impl<T> Register for &T
where
    T: Register + 'static,
{
    fn register(registry: Registry) -> Registry {
        registry.register::<T>()
    }
}

impl<'a, T> TypeName for &'a T
where
    T: TypeName + 'static,
{
    fn get_type_name() -> Cow<'static, str> {
        <T as TypeName>::get_type_name()
    }
}

impl<T: OutputTypeName + 'static> OutputTypeName for &T {}

impl<T: Register + Clone + 'static> Register for Cow<'_, T> {
    fn register(registry: Registry) -> Registry {
        registry.register::<T>()
    }
}
impl<T: OutputTypeName + Clone + 'static> TypeName for Cow<'_, T> {
    fn get_type_name() -> Cow<'static, str> {
        <T as TypeName>::get_type_name()
    }
}

impl<T: OutputTypeName + Clone + 'static> OutputTypeName for Cow<'_, T> {}

impl Register for String {}
impl TypeName for String {
    fn get_type_name() -> Cow<'static, str> {
        dynamic::TypeRef::STRING.into()
    }
}

impl InputTypeName for String {}

impl OutputTypeName for String {}

impl Register for &str {}
impl TypeName for &str {
    fn get_type_name() -> Cow<'static, str> {
        dynamic::TypeRef::STRING.into()
    }
}

impl InputTypeName for &str {}

impl OutputTypeName for &str {}

impl TypeName for str {
    fn get_type_name() -> Cow<'static, str> {
        dynamic::TypeRef::STRING.into()
    }
}

impl Register for str {}
impl InputTypeName for str {}

impl OutputTypeName for str {}

impl Register for async_graphql::ID {}
impl TypeName for async_graphql::ID {
    fn get_type_name() -> Cow<'static, str> {
        dynamic::TypeRef::ID.into()
    }
}

impl InputTypeName for async_graphql::ID {}

impl OutputTypeName for async_graphql::ID {}

impl Register for bool {}
impl TypeName for bool {
    fn get_type_name() -> Cow<'static, str> {
        dynamic::TypeRef::BOOLEAN.into()
    }
}

impl InputTypeName for bool {}

impl OutputTypeName for bool {}

impl Register for f32 {}
impl TypeName for f32 {
    fn get_type_name() -> Cow<'static, str> {
        dynamic::TypeRef::FLOAT.into()
    }
}

impl InputTypeName for f32 {}

impl OutputTypeName for f32 {}

impl Register for f64 {}
impl TypeName for f64 {
    fn get_type_name() -> Cow<'static, str> {
        dynamic::TypeRef::FLOAT.into()
    }
}

impl InputTypeName for f64 {}

impl OutputTypeName for f64 {}

macro_rules! int_output_value {
    ($($t:ty),*) => {
        $(
            impl Register for $t {}
            impl TypeName for $t {
                fn get_type_name() -> Cow<'static, str> {
                    dynamic::TypeRef::INT.into()
                }
            }
            impl InputTypeName for $t {}
            impl OutputTypeName for $t {}
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

impl<T, E> GetOutputTypeRef for Result<T, E>
where
    T: GetOutputTypeRef,
{
    #[inline]
    fn get_output_type_ref() -> TypeRefBuilder {
        T::get_output_type_ref()
    }
}

impl<T: OutputTypeName> GetOutputTypeRef for T {
    #[inline]
    fn get_output_type_ref() -> TypeRefBuilder {
        TypeRefBuilder::NamedNN(T::get_output_type_name().to_string())
    }
}

impl<T: GetOutputTypeRef> GetOutputTypeRef for Option<T> {
    #[inline]
    fn get_output_type_ref() -> TypeRefBuilder {
        let t = T::get_output_type_ref();
        t.optional()
    }
}

impl<T: GetOutputTypeRef> GetOutputTypeRef for Vec<T> {
    #[inline]
    fn get_output_type_ref() -> TypeRefBuilder {
        T::get_output_type_ref().list()
    }
}

impl<T: GetOutputTypeRef> GetOutputTypeRef for &[T] {
    #[inline]
    fn get_output_type_ref() -> TypeRefBuilder {
        T::get_output_type_ref().list()
    }
}

impl<T: InputTypeName> GetInputTypeRef for T {
    #[inline]
    fn get_input_type_ref() -> TypeRefBuilder {
        TypeRefBuilder::NamedNN(T::get_input_type_name().to_string())
    }
}

impl<T: GetInputTypeRef> GetInputTypeRef for Option<T> {
    #[inline]
    fn get_input_type_ref() -> TypeRefBuilder {
        T::get_input_type_ref().optional()
    }
}

impl<T: GetInputTypeRef> GetInputTypeRef for MaybeUndefined<T> {
    #[inline]
    fn get_input_type_ref() -> TypeRefBuilder {
        T::get_input_type_ref().optional()
    }
}

impl<T: GetInputTypeRef> GetInputTypeRef for Vec<T> {
    #[inline]
    fn get_input_type_ref() -> TypeRefBuilder {
        T::get_input_type_ref().list()
    }
}
impl<T: GetInputTypeRef> GetInputTypeRef for &[T] {
    #[inline]
    fn get_input_type_ref() -> TypeRefBuilder {
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
