use super::BaseEnum;
use super::NamedVariant;

#[allow(dead_code)]
pub type NamedEnum<V = NamedVariant, G = ()> = BaseEnum<V, G>;
