use crate::types::{GraphqlType, InputType, OutputType};
use async_graphql::dynamic;

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
