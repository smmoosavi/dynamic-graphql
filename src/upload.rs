use crate::errors::{InputValueError, InputValueResult};
use crate::from_value::FromValue;
use crate::registry::Registry;
use crate::types::{InputTypeName, Register, TypeName};
use async_graphql::dynamic::ValueAccessor;
use async_graphql::{dynamic, Context, UploadValue};
use std::borrow::Cow;

pub struct Upload(usize);

impl TypeName for Upload {
    fn get_type_name() -> Cow<'static, str> {
        "Upload".into()
    }
}
impl InputTypeName for Upload {}

impl Upload {
    /// Get the upload value.
    pub fn value(&self, ctx: &Context<'_>) -> std::io::Result<UploadValue> {
        ctx.query_env
            .uploads
            .get(self.0)
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "Upload not found"))?
            .try_clone()
    }
}

impl FromValue for Upload {
    fn from_value(value: async_graphql::Result<ValueAccessor>) -> InputValueResult<Self> {
        const PREFIX: &str = "#__graphql_file__:";
        let value = value?;
        let value = value.string()?;

        if let Some(filename) = value.strip_prefix(PREFIX) {
            let index = filename.parse::<usize>().map_err(|_| {
                async_graphql::Error::new(
                    "Invalid upload value, expected #__graphql_file__:index format",
                )
            })?;
            return Ok(Upload(index));
        }
        Err(InputValueError::custom(
            "Invalid upload value, expected #__graphql_file__:index format",
        ))
    }
}

impl Register for Upload {
    fn register(registry: Registry) -> Registry {
        let upload = dynamic::Scalar::new("Upload");
        registry.register_type(upload)
    }
}
