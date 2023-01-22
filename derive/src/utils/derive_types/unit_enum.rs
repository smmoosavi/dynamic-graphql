use crate::utils::derive_types::BaseEnum;

use super::unit_variant::UnitVariant;

#[allow(dead_code)]
pub type UnitEnum<V = UnitVariant, G = ()> = BaseEnum<V, G>;
