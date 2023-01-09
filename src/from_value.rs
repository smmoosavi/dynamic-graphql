use crate::{dynamic, Result};

pub trait FromValue: Sized {
    fn from_value(value: dynamic::ValueAccessor) -> Result<Self>;
}

impl FromValue for String {
    fn from_value(value: dynamic::ValueAccessor) -> Result<Self> {
        value.string().map(|s| s.to_string())
    }
}

impl FromValue for async_graphql::ID {
    fn from_value(value: dynamic::ValueAccessor) -> Result<Self> {
        value.string().map(|s| async_graphql::ID(s.to_string()))
    }
}

impl FromValue for bool {
    fn from_value(value: dynamic::ValueAccessor) -> Result<Self> {
        value.boolean()
    }
}

impl FromValue for f32 {
    fn from_value(value: dynamic::ValueAccessor) -> Result<Self> {
        value.f32()
    }
}

impl FromValue for f64 {
    fn from_value(value: dynamic::ValueAccessor) -> Result<Self> {
        value.f64()
    }
}

macro_rules! uint_from_value {
    ($($t:ty),*) => {
        $(
            impl FromValue for $t {
                fn from_value(value: dynamic::ValueAccessor) -> Result<Self> {
                    value.u64().map(|v| v as $t)
                }
            }
        )*
    };
}
macro_rules! int_from_value {
    ($($t:ty),*) => {
        $(
            impl FromValue for $t {
                fn from_value(value: dynamic::ValueAccessor) -> Result<Self> {
                    value.i64().map(|v| v as $t)
                }
            }
        )*
    };
}

uint_from_value!(u8, u16, u32, u64, usize);
int_from_value!(i8, i16, i32, i64, isize);

impl<T> FromValue for Option<T>
where
    T: FromValue,
{
    fn from_value(value: dynamic::ValueAccessor) -> Result<Self> {
        if value.is_null() {
            Ok(None)
        } else {
            Ok(Some(T::from_value(value)?))
        }
    }
}

impl<T> FromValue for Vec<T>
where
    T: FromValue,
{
    fn from_value(value: dynamic::ValueAccessor) -> Result<Self> {
        value.list()?.iter().map(|v| T::from_value(v)).collect()
    }
}
