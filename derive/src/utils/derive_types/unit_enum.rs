use super::unit_variant::UnitVariant;
use crate::utils::derive_types::BaseEnum;

#[allow(dead_code)]
pub type UnitEnum<V = UnitVariant, G = ()> = BaseEnum<V, G>;
