use super::{BaseStruct, TupleField};

#[allow(dead_code)]
pub type TupleStruct<F = TupleField, G = ()> = BaseStruct<F, G>;
