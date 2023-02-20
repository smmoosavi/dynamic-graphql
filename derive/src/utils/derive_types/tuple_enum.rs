use super::BaseEnum;
use super::TupleField;

#[allow(dead_code)]
pub type TupleEnum<F = TupleField, G = ()> = BaseEnum<F, G>;
