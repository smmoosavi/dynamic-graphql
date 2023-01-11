use super::{BaseEnum, NewtypeVariant};

#[allow(dead_code)]
pub type NewTypeEnum<V = NewtypeVariant, G = ()> = BaseEnum<V, G>;
