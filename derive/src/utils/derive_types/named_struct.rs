use super::BaseStruct;
use super::NamedField;

#[allow(dead_code)]
pub type NamedStruct<F = NamedField, G = ()> = BaseStruct<F, G>;
