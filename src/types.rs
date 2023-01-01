use crate::dynamic;
use crate::registry::Registry;

pub trait Register {
    fn register(registry: Registry) -> Registry;
}

pub trait Object {
    const NAME: &'static str;
}

pub trait Enum {
    const NAME: &'static str;
}

pub trait Scalar {
    const NAME: &'static str;
}

pub trait Union {
    const NAME: &'static str;
}

pub trait Interface {
    const NAME: &'static str;

    fn register_fields(interface: dynamic::Interface) -> dynamic::Interface;
}

pub trait InputObject {
    const NAME: &'static str;
}

pub trait Mutation: Object {}

pub trait ExpandObject {
    type Target: Object;
}
