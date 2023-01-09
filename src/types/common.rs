use crate::types::{GetInputTypeRef, GetOutputTypeRef, GraphqlType, InputType, OutputType};
use async_graphql::dynamic;
use async_graphql::dynamic::TypeRef;

impl GraphqlType for String {
    const NAME: &'static str = dynamic::TypeRef::STRING;
}

impl InputType for String {}

impl OutputType for String {}

impl GraphqlType for &str {
    const NAME: &'static str = dynamic::TypeRef::STRING;
}

impl InputType for &str {}

impl OutputType for &str {}

impl GraphqlType for str {
    const NAME: &'static str = dynamic::TypeRef::STRING;
}

impl InputType for str {}

impl OutputType for str {}

impl GraphqlType for async_graphql::ID {
    const NAME: &'static str = dynamic::TypeRef::ID;
}

impl InputType for async_graphql::ID {}

impl OutputType for async_graphql::ID {}

impl GraphqlType for bool {
    const NAME: &'static str = dynamic::TypeRef::BOOLEAN;
}

impl InputType for bool {}

impl OutputType for bool {}

impl GraphqlType for f32 {
    const NAME: &'static str = dynamic::TypeRef::FLOAT;
}

impl InputType for f32 {}

impl OutputType for f32 {}

impl GraphqlType for f64 {
    const NAME: &'static str = dynamic::TypeRef::FLOAT;
}

impl InputType for f64 {}

impl OutputType for f64 {}

macro_rules! int_output_value {
    ($($t:ty),*) => {
        $(
            impl GraphqlType for $t {
                const NAME: &'static str = dynamic::TypeRef::INT;
            }
            impl InputType for $t {}
            impl OutputType for $t {}
        )*
    };
}

int_output_value!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);

impl<T: OutputType> GetOutputTypeRef for T {
    #[inline]
    fn get_output_type_ref() -> TypeRef {
        TypeRef::named_nn(<T as OutputType>::NAME)
    }
}

impl<T: OutputType> GetOutputTypeRef for Option<T> {
    #[inline]
    fn get_output_type_ref() -> TypeRef {
        TypeRef::named(<T as OutputType>::NAME)
    }
}

impl<T: OutputType> GetOutputTypeRef for Vec<T> {
    #[inline]
    fn get_output_type_ref() -> TypeRef {
        TypeRef::named_nn_list_nn(<T as OutputType>::NAME)
    }
}

impl<T: OutputType> GetOutputTypeRef for Option<Vec<T>> {
    #[inline]
    fn get_output_type_ref() -> TypeRef {
        TypeRef::named_nn_list(<T as OutputType>::NAME)
    }
}

impl<T: OutputType> GetOutputTypeRef for Vec<Option<T>> {
    #[inline]
    fn get_output_type_ref() -> TypeRef {
        TypeRef::named_list_nn(<T as OutputType>::NAME)
    }
}

impl<T: OutputType> GetOutputTypeRef for Option<Vec<Option<T>>> {
    #[inline]
    fn get_output_type_ref() -> TypeRef {
        TypeRef::named_list(<T as OutputType>::NAME)
    }
}

impl<T: OutputType> GetOutputTypeRef for &[T] {
    #[inline]
    fn get_output_type_ref() -> TypeRef {
        TypeRef::named_nn_list_nn(<T as OutputType>::NAME)
    }
}

impl<T: OutputType> GetOutputTypeRef for Option<&[T]> {
    #[inline]
    fn get_output_type_ref() -> TypeRef {
        TypeRef::named_nn_list(<T as OutputType>::NAME)
    }
}

impl<T: OutputType> GetOutputTypeRef for &[Option<T>] {
    #[inline]
    fn get_output_type_ref() -> TypeRef {
        TypeRef::named_list_nn(<T as OutputType>::NAME)
    }
}

impl<T: OutputType> GetOutputTypeRef for Option<&[Option<T>]> {
    #[inline]
    fn get_output_type_ref() -> TypeRef {
        TypeRef::named_list(<T as OutputType>::NAME)
    }
}

impl<T: InputType> GetInputTypeRef for T {
    #[inline]
    fn get_input_type_ref() -> TypeRef {
        TypeRef::named_nn(<T as InputType>::NAME)
    }
}

impl<T: InputType> GetInputTypeRef for Option<T> {
    #[inline]
    fn get_input_type_ref() -> TypeRef {
        TypeRef::named(<T as InputType>::NAME)
    }
}

impl<T: InputType> GetInputTypeRef for Vec<T> {
    #[inline]
    fn get_input_type_ref() -> TypeRef {
        TypeRef::named_nn_list_nn(<T as InputType>::NAME)
    }
}

impl<T: InputType> GetInputTypeRef for Option<Vec<T>> {
    #[inline]
    fn get_input_type_ref() -> TypeRef {
        TypeRef::named_nn_list(<T as InputType>::NAME)
    }
}

impl<T: InputType> GetInputTypeRef for Vec<Option<T>> {
    #[inline]
    fn get_input_type_ref() -> TypeRef {
        TypeRef::named_list_nn(<T as InputType>::NAME)
    }
}

impl<T: InputType> GetInputTypeRef for Option<Vec<Option<T>>> {
    #[inline]
    fn get_input_type_ref() -> TypeRef {
        TypeRef::named_list(<T as InputType>::NAME)
    }
}

impl<T: InputType> GetInputTypeRef for &[T] {
    #[inline]
    fn get_input_type_ref() -> TypeRef {
        TypeRef::named_nn_list_nn(<T as InputType>::NAME)
    }
}

impl<T: InputType> GetInputTypeRef for Option<&[T]> {
    #[inline]
    fn get_input_type_ref() -> TypeRef {
        TypeRef::named_nn_list(<T as InputType>::NAME)
    }
}

impl<T: InputType> GetInputTypeRef for &[Option<T>] {
    #[inline]
    fn get_input_type_ref() -> TypeRef {
        TypeRef::named_list_nn(<T as InputType>::NAME)
    }
}

impl<T: InputType> GetInputTypeRef for Option<&[Option<T>]> {
    #[inline]
    fn get_input_type_ref() -> TypeRef {
        TypeRef::named_list(<T as InputType>::NAME)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_output_type_ref() {
        assert_eq!(
            <String as GetOutputTypeRef>::get_output_type_ref().to_string(),
            "String!"
        );
        assert_eq!(
            <Option<String> as GetOutputTypeRef>::get_output_type_ref().to_string(),
            "String"
        );
        assert_eq!(
            <Vec<String> as GetOutputTypeRef>::get_output_type_ref().to_string(),
            "[String!]!"
        );
        assert_eq!(
            <Option<Vec<String>> as GetOutputTypeRef>::get_output_type_ref().to_string(),
            "[String!]"
        );
        assert_eq!(
            <Vec<Option<String>> as GetOutputTypeRef>::get_output_type_ref().to_string(),
            "[String]!"
        );
        assert_eq!(
            <Option<Vec<Option<String>>> as GetOutputTypeRef>::get_output_type_ref().to_string(),
            "[String]"
        );
        assert_eq!(
            <&[String] as GetOutputTypeRef>::get_output_type_ref().to_string(),
            "[String!]!"
        );
        assert_eq!(
            <Option<&[String]> as GetOutputTypeRef>::get_output_type_ref().to_string(),
            "[String!]"
        );
        assert_eq!(
            <&[Option<String>] as GetOutputTypeRef>::get_output_type_ref().to_string(),
            "[String]!"
        );
        assert_eq!(
            <Option<&[Option<String>]> as GetOutputTypeRef>::get_output_type_ref().to_string(),
            "[String]"
        );
    }

    #[test]
    fn test_get_input_type_ref() {
        assert_eq!(
            <String as GetInputTypeRef>::get_input_type_ref().to_string(),
            "String!"
        );
        assert_eq!(
            <Option<String> as GetInputTypeRef>::get_input_type_ref().to_string(),
            "String"
        );
        assert_eq!(
            <Vec<String> as GetInputTypeRef>::get_input_type_ref().to_string(),
            "[String!]!"
        );
        assert_eq!(
            <Option<Vec<String>> as GetInputTypeRef>::get_input_type_ref().to_string(),
            "[String!]"
        );
        assert_eq!(
            <Vec<Option<String>> as GetInputTypeRef>::get_input_type_ref().to_string(),
            "[String]!"
        );
        assert_eq!(
            <Option<Vec<Option<String>>> as GetInputTypeRef>::get_input_type_ref().to_string(),
            "[String]"
        );
        assert_eq!(
            <&[String] as GetInputTypeRef>::get_input_type_ref().to_string(),
            "[String!]!"
        );
        assert_eq!(
            <Option<&[String]> as GetInputTypeRef>::get_input_type_ref().to_string(),
            "[String!]"
        );
        assert_eq!(
            <&[Option<String>] as GetInputTypeRef>::get_input_type_ref().to_string(),
            "[String]!"
        );
        assert_eq!(
            <Option<&[Option<String>]> as GetInputTypeRef>::get_input_type_ref().to_string(),
            "[String]"
        );
    }
}
