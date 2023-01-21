macro_rules! define {
    ($name:ident, $ty:ty) => {
        #[derive(Debug, Clone)]
        pub struct $name(pub $ty);
    };
}

macro_rules! deref {
    ($name:ident, $ty:ty) => {
        impl std::ops::Deref for $name {
            type Target = $ty;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

macro_rules! set_index {
    ($name:ident) => {
        impl crate::utils::with_index::SetIndex for $name {
            fn with_index(self, index: usize) -> Self {
                Self(crate::utils::with_index::SetIndex::with_index(
                    self.0, index,
                ))
            }
        }
    };
}

macro_rules! set_context {
    ($name:ident, $ty:ty) => {
        impl crate::utils::with_context::SetContext for $name {
            type Context = <<Self as std::ops::Deref>::Target as crate::utils::with_context::SetContext>::Context;

            fn set_context(&mut self, context: Self::Context) {
                self.0.set_context(context);
            }
        }
    };
    ($name:ident, $ty:ty, inner=$( $field_path:ident ).+) => {
        impl crate::utils::with_context::SetContext for $name {
            type Context = <<Self as std::ops::Deref>::Target as crate::utils::with_context::SetContext>::Context;

            fn set_context(&mut self, context: Self::Context) {
                self.0.set_context(context);
                let ctx = MakeContext::make_context(self);
                self.0. $( $field_path ).+ .set_context(ctx);
            }
        }
    };
}

macro_rules! impl_from_mut {
    ($trayt: ident $( :: $rest : ident )*  , $method:ident, $syn:path, $name:ident,) => {
        impl $trayt$(::$rest)* for $name {
            fn $method(input: &mut $syn) -> darling::Result<Self> {
                Ok(Self($trayt$(::$rest)*::$method(input)?))
            }
        }
    };
    ($trayt: ident $( :: $rest : ident )*  , $method:ident, $syn:path, $name:ident, ctx,) => {
        impl $trayt$(::$rest)* for $name {
            fn $method(input: &mut $syn) -> darling::Result<Self> {
                let mut value = Self($trayt$(::$rest)*::$method(input)?);
                let ctx = crate::utils::with_context::MakeContext::make_context(&value);
                crate::utils::with_context::SetContext::set_context(&mut value.0, ctx);
                Ok(value)
            }
        }
    };
        ($trayt: ident $( :: $rest : ident )*  , $method:ident, $syn:path, $name:ident, inner=$( $field_path:ident ).+,) => {
        impl $trayt$(::$rest)* for $name {
            fn $method(input: &mut $syn) -> darling::Result<Self> {
                let mut value = Self($trayt$(::$rest)*::$method(input)?);
                let ctx = crate::utils::with_context::MakeContext::make_context(&value);
                crate::utils::with_context::SetContext::set_context(&mut value.0. $( $field_path ).+, ctx);
                Ok(value)
            }
        }
    };
}

macro_rules! impl_from {
    ($trayt: ident $( :: $rest : ident )*  , $method:ident, $syn:path, $name:ident,) => {
        impl $trayt$(::$rest)* for $name {
            fn $method(input: &$syn) -> darling::Result<Self> {
                Ok(Self($trayt$(::$rest)*::$method(input)?))
            }
        }
    };
    ($trayt: ident $( :: $rest : ident )*  , $method:ident, $syn:path, $name:ident, ctx,) => {
        impl $trayt$(::$rest)* for $name {
            fn $method(input: &$syn) -> darling::Result<Self> {
                let mut value = Self($trayt$(::$rest)*::$method(input)?);
                let ctx = crate::utils::with_context::MakeContext::make_context(&value);
                crate::utils::with_context::SetContext::set_context(&mut value.0, ctx);
                Ok(value)
            }
        }
    };
        ($trayt: ident $( :: $rest : ident )*  , $method:ident, $syn:path, $name:ident, inner=$( $field_path:ident ).+,) => {
        impl $trayt$(::$rest)* for $name {
            fn $method(input: &$syn) -> darling::Result<Self> {
                let mut value = Self($trayt$(::$rest)*::$method(input)?);
                let ctx = crate::utils::with_context::MakeContext::make_context(&value);
                crate::utils::with_context::SetContext::set_context(&mut value.0. $( $field_path ).+, ctx);
                Ok(value)
            }
        }
    };
}

pub(crate) use define;
pub(crate) use deref;
pub(crate) use impl_from;
pub(crate) use impl_from_mut;
pub(crate) use set_context;
pub(crate) use set_index;
