use crate::dynamic;
use crate::errors::InputValueError;
use crate::errors::InputValueResult;
use crate::types::GetInputTypeRef;
use crate::MaybeUndefined;
use crate::Result;

pub trait FromValue: Sized {
    fn from_value(value: Result<dynamic::ValueAccessor>) -> InputValueResult<Self>;
}

impl FromValue for String {
    fn from_value(value: Result<dynamic::ValueAccessor>) -> InputValueResult<Self> {
        Ok(value?.string().map(|s| s.to_string())?)
    }
}

impl FromValue for async_graphql::ID {
    fn from_value(value: Result<dynamic::ValueAccessor>) -> InputValueResult<Self> {
        Ok(value?.string().map(|s| async_graphql::ID(s.to_string()))?)
    }
}

impl FromValue for bool {
    fn from_value(value: Result<dynamic::ValueAccessor>) -> InputValueResult<Self> {
        Ok(value?.boolean()?)
    }
}

impl FromValue for f32 {
    fn from_value(value: Result<dynamic::ValueAccessor>) -> InputValueResult<Self> {
        Ok(value?.f32()?)
    }
}

impl FromValue for f64 {
    fn from_value(value: Result<dynamic::ValueAccessor>) -> InputValueResult<Self> {
        Ok(value?.f64()?)
    }
}

macro_rules! uint_from_value {
    ($($t:ty),*) => {
        $(
            impl FromValue for $t {
                fn from_value(value: Result<dynamic::ValueAccessor>) -> InputValueResult<Self> {
                    Self::try_from(value?.u64()?).map_err(|_| {
                        InputValueError::custom(format!(
                            "Only integers from {} to {} are accepted for {}.",
                            Self::MIN,
                            Self::MAX,
                            stringify!($t),
                        ))
                    })
                }
            }
        )*
    };
}
macro_rules! int_from_value {
    ($($t:ty),*) => {
        $(
            impl FromValue for $t {
                fn from_value(value: Result<dynamic::ValueAccessor>) -> InputValueResult<Self> {
                    Self::try_from(value?.u64()?).map_err(|_| {
                        InputValueError::custom(format!(
                            "Only integers from {} to {} are accepted for {}.",
                            Self::MIN,
                            Self::MAX,
                            stringify!($t),
                        ))
                    })
                }
            }
        )*
    };
}

uint_from_value!(u8, u16, u32, u64, usize);
int_from_value!(i8, i16, i32, i64, isize);

impl<T> FromValue for Option<T>
where
    T: FromValue + GetInputTypeRef,
{
    fn from_value(value: Result<dynamic::ValueAccessor>) -> InputValueResult<Self> {
        match value.ok() {
            None => Ok(None),
            Some(value) if value.is_null() => Ok(None),
            Some(value) => Ok(Some(
                T::from_value(Ok(value)).map_err(InputValueError::propagate)?,
            )),
        }
    }
}

impl<T> FromValue for MaybeUndefined<T>
where
    T: FromValue + GetInputTypeRef,
{
    fn from_value(value: Result<dynamic::ValueAccessor>) -> InputValueResult<Self> {
        match value.ok() {
            None => Ok(MaybeUndefined::Undefined),
            Some(value) if value.is_null() => Ok(MaybeUndefined::Null),
            Some(value) => Ok(MaybeUndefined::Value(
                T::from_value(Ok(value)).map_err(InputValueError::propagate)?,
            )),
        }
    }
}

impl<T> FromValue for Vec<T>
where
    T: FromValue + GetInputTypeRef,
{
    fn from_value(value: Result<dynamic::ValueAccessor>) -> InputValueResult<Self> {
        value?
            .list()?
            .iter()
            .map(|v| T::from_value(Ok(v)).map_err(InputValueError::propagate))
            .collect()
    }
}
