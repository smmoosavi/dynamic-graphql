use crate::{Context, Error, FieldValue, Result, ID};

pub trait ResolveRef<'a> {
    fn resolve_ref(&'a self, ctx: &Context) -> Result<Option<FieldValue<'a>>>;
}

pub trait ResolveOwned<'a> {
    fn resolve_owned(self, ctx: &Context) -> Result<Option<FieldValue<'a>>>;
}

pub trait Resolve<'a> {
    fn resolve(self, ctx: &Context) -> Result<Option<FieldValue<'a>>>;
}

mod resolve_ref {
    use super::*;
    // &Option<T>
    impl<'a, T> ResolveRef<'a> for Option<T>
    where
        &'a T: Resolve<'a> + 'a,
    {
        #[inline]
        fn resolve_ref(&'a self, ctx: &Context) -> Result<Option<FieldValue<'a>>> {
            match self {
                None => Ok(None),
                Some(value) => value.resolve(ctx),
            }
        }
    }

    // &Vec<T>
    impl<'a, T> ResolveRef<'a> for Vec<T>
    where
        &'a T: Resolve<'a> + 'a,
    {
        fn resolve_ref(&'a self, ctx: &Context) -> Result<Option<FieldValue<'a>>> {
            let iter = self.iter();
            let items = iter.enumerate().map(|(index, item)| {
                let ctx_idx = ctx.with_index(index);
                match item.resolve(&ctx_idx) {
                    Ok(Some(value)) => value,
                    _ => FieldValue::NULL,
                }
            });
            Ok(Some(FieldValue::list(items)))
        }
    }
    // &ID
    impl<'a> ResolveRef<'a> for ID {
        #[inline]
        fn resolve_ref(&self, _ctx: &Context) -> Result<Option<FieldValue<'a>>> {
            Ok(Some(FieldValue::value(self.0.to_owned())))
        }
    }
    // &str
    impl<'a> ResolveRef<'a> for &str {
        #[inline]
        fn resolve_ref(&'a self, _ctx: &Context) -> Result<Option<FieldValue<'a>>> {
            Ok(Some(FieldValue::value(self.to_string())))
        }
    }
}
mod resolve_own {
    use super::*;
    use std::borrow::Cow;
    // &T
    impl<'a, T> ResolveOwned<'a> for &'a T
    where
        T: ResolveRef<'a>,
    {
        #[inline]
        fn resolve_owned(self, ctx: &Context) -> Result<Option<FieldValue<'a>>> {
            self.resolve_ref(ctx)
        }
    }
    // Cow<'a, T>
    impl<'a, T> ResolveOwned<'a> for Cow<'a, T>
    where
        T: Clone + Resolve<'a>,
        &'a T: Resolve<'a>,
    {
        #[inline]
        fn resolve_owned(self, ctx: &Context) -> Result<Option<FieldValue<'a>>> {
            match self {
                Cow::Owned(value) => value.resolve(ctx),
                Cow::Borrowed(value) => value.resolve(ctx),
            }
        }
    }
    // Option<T>
    impl<'a, T> ResolveOwned<'a> for Option<T>
    where
        T: Resolve<'a>,
    {
        #[inline]
        fn resolve_owned(self, ctx: &Context) -> Result<Option<FieldValue<'a>>> {
            match self {
                None => Ok(None),
                Some(value) => value.resolve(ctx),
            }
        }
    }

    // Result<T, E>
    impl<'a, T, E> ResolveOwned<'a> for Result<T, E>
    where
        T: Resolve<'a>,
        E: Into<Error>,
    {
        #[inline]
        fn resolve_owned(self, ctx: &Context) -> Result<Option<FieldValue<'a>>> {
            match self {
                Ok(value) => value.resolve(ctx),
                Err(err) => Err(err.into()),
            }
        }
    }

    // Vec<T>
    impl<'a, T> ResolveOwned<'a> for Vec<T>
    where
        T: Resolve<'a>,
    {
        fn resolve_owned(self, ctx: &Context) -> Result<Option<FieldValue<'a>>> {
            let iter = self.into_iter();
            let items = iter.enumerate().map(|(index, item)| {
                let ctx_idx = ctx.with_index(index);
                match item.resolve(&ctx_idx) {
                    Ok(Some(value)) => value,
                    _ => FieldValue::NULL,
                }
            });
            Ok(Some(FieldValue::list(items)))
        }
    }

    // &[T]
    impl<'a, T> ResolveOwned<'a> for &'a [T]
    where
        &'a T: Resolve<'a>,
    {
        fn resolve_owned(self, ctx: &Context) -> Result<Option<FieldValue<'a>>> {
            let iter = self.iter();
            let items = iter.enumerate().map(|(index, item)| {
                let ctx_idx = ctx.with_index(index);
                match item.resolve(&ctx_idx) {
                    Ok(Some(value)) => value,
                    _ => FieldValue::NULL,
                }
            });
            Ok(Some(FieldValue::list(items)))
        }
    }

    // ID
    impl<'a> ResolveOwned<'a> for ID {
        #[inline]
        fn resolve_owned(self, _ctx: &Context) -> Result<Option<FieldValue<'a>>> {
            Ok(Some(FieldValue::value(self.0)))
        }
    }

    // &str
    impl<'a> ResolveOwned<'a> for &str {
        #[inline]
        fn resolve_owned(self, _ctx: &Context) -> Result<Option<FieldValue<'a>>> {
            Ok(Some(FieldValue::value(self.to_string())))
        }
    }
}

// T
impl<'a, T: ResolveOwned<'a>> Resolve<'a> for T {
    #[inline]
    fn resolve(self, ctx: &Context) -> Result<Option<FieldValue<'a>>> {
        self.resolve_owned(ctx)
    }
}

macro_rules! resolves {
    ($($ty:ident),*) => {
        $(
            impl <'a> ResolveOwned<'a> for $ty {
                #[inline]
                fn resolve_owned(self, _ctx: &Context) -> Result<Option<FieldValue<'a>>> {
                    Ok(Some(FieldValue::value(self)))
                }
            }
            impl <'a> ResolveRef<'a> for $ty {
                #[inline]
                fn resolve_ref(&self, _ctx: &Context) -> Result<Option<FieldValue<'a>>> {
                    Ok(Some(FieldValue::value(self.to_owned())))
                }
            }
        )*
    };
}

resolves!(String, i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, bool, f32, f64);
