use super::{BaseEnum, NewtypeVariant};

#[allow(dead_code)]
pub type NewTypeEnum<V = NewtypeVariant> = BaseEnum<V>;
