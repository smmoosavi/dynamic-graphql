use crate::types::GetInputTypeRef;
use crate::Value;
use async_graphql::dynamic::TypeRef;
use async_graphql::{ErrorExtensionValues, Pos, ServerError};
use std::fmt::Display;
use std::marker::PhantomData;

fn get_type_name<T: GetInputTypeRef>() -> String {
    let type_ref: TypeRef = <Option<T>>::get_input_type_ref().into();
    type_ref.to_string()
}

#[derive(Debug)]
pub struct InputValueError<T> {
    message: String,
    extensions: Option<ErrorExtensionValues>,
    phantom: PhantomData<T>,
}

impl<T> InputValueError<T> {
    pub fn new(message: String) -> Self {
        Self {
            message,
            extensions: None,
            phantom: PhantomData,
        }
    }
    /// The expected input type did not match the actual input type.
    #[must_use]
    pub fn expected_type(actual: Value) -> Self
    where
        T: GetInputTypeRef,
    {
        Self::new(format!(
            r#"Expected input type "{}", found {}."#,
            get_type_name::<T>(),
            actual
        ))
    }
    /// A custom error message.
    ///
    /// Any type that implements `Display` is automatically converted to this if
    /// you use the `?` operator.
    #[must_use]
    pub fn custom(msg: impl Display) -> Self
    where
        T: GetInputTypeRef,
    {
        Self::new(format!(
            r#"Failed to parse "{}": {}"#,
            get_type_name::<T>(),
            msg
        ))
    }

    /// Propagate the error message to a different type.
    pub fn propagate<U: GetInputTypeRef>(self) -> InputValueError<U>
    where
        T: GetInputTypeRef,
    {
        if get_type_name::<T>() != get_type_name::<U>() {
            InputValueError::new(format!(
                r#"{} (occurred while parsing "{}")"#,
                self.message,
                get_type_name::<U>()
            ))
        } else {
            InputValueError::new(self.message)
        }
    }
    pub fn with_extension(mut self, name: impl AsRef<str>, value: impl Into<Value>) -> Self {
        self.extensions
            .get_or_insert_with(ErrorExtensionValues::default)
            .set(name, value);
        self
    }

    /// Convert the error into a server error.
    pub fn into_server_error(self, pos: Pos) -> ServerError {
        let mut err = ServerError::new(self.message, Some(pos));
        err.extensions = self.extensions;
        err
    }

    pub fn into_arg_error(self, name: &str) -> crate::Error {
        let mut error = crate::Error::new(format!(
            "Invalid value for argument \"{}\": {}",
            name, self.message
        ));
        error.extensions = self.extensions;
        error
    }
    pub fn into_field_error(self, name: &str) -> crate::Error {
        let mut error = crate::Error::new(format!(
            "Invalid value for field \"{}\": {}",
            name, self.message
        ));
        error.extensions = self.extensions;
        error
    }
}

impl<T: GetInputTypeRef> From<async_graphql::Error> for InputValueError<T> {
    fn from(value: async_graphql::Error) -> Self {
        Self::custom(value.message)
    }
}

pub type InputValueResult<T> = Result<T, InputValueError<T>>;
