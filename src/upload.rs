use async_graphql::dynamic;
use async_graphql::dynamic::Type;
use async_graphql::dynamic::ValueAccessor;
use async_graphql::Upload;
use std::borrow::Cow;

use crate::errors::InputValueResult;
use crate::from_value::FromValue;
use crate::registry::Registry;
use crate::types::InputTypeName;
use crate::types::Register;
use crate::types::TypeName;

impl TypeName for Upload {
    fn get_type_name() -> Cow<'static, str> {
        dynamic::TypeRef::UPLOAD.into()
    }
}

impl InputTypeName for Upload {}

impl FromValue for Upload {
    fn from_value(value: async_graphql::Result<ValueAccessor>) -> InputValueResult<Self> {
        Ok(value?.upload()?)
    }
}

impl Register for Upload {
    fn register(registry: Registry) -> Registry {
        registry.register_type(Type::Upload)
    }
}
