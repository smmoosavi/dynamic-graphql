use crate::{Context, Error, FieldValue, Result, ID};

pub trait ResolveOwned<'a> {
    fn resolve_owned(self, ctx: &Context) -> Result<Option<FieldValue<'a>>>;
}

pub trait ResolveRef<'a> {
    fn resolve_ref(&'a self, ctx: &Context) -> Result<Option<FieldValue<'a>>>;
}

impl<'a, T: ResolveOwned<'a>> ResolveOwned<'a> for Option<T> {
    #[inline]
    fn resolve_owned(self, ctx: &Context) -> Result<Option<FieldValue<'a>>> {
        match self {
            None => Ok(None),
            Some(value) => value.resolve_owned(ctx),
        }
    }
}

impl<'a, T: ResolveRef<'a>> ResolveRef<'a> for Option<T> {
    #[inline]
    fn resolve_ref(&'a self, ctx: &Context) -> Result<Option<FieldValue<'a>>> {
        match self {
            None => Ok(None),
            Some(value) => value.resolve_ref(ctx),
        }
    }
}

impl<'a, T, E> ResolveOwned<'a> for Result<T, E>
where
    T: ResolveOwned<'a>,
    E: Into<Error>,
{
    #[inline]
    fn resolve_owned(self, ctx: &Context) -> Result<Option<FieldValue<'a>>> {
        match self {
            Ok(value) => value.resolve_owned(ctx),
            Err(err) => Err(err.into()),
        }
    }
}

impl<'a, T, E> ResolveRef<'a> for Result<T, E>
where
    T: ResolveRef<'a>,
    E: Into<Error> + Clone,
{
    #[inline]
    fn resolve_ref(&'a self, ctx: &Context) -> Result<Option<FieldValue<'a>>> {
        match self {
            Ok(value) => value.resolve_ref(ctx),
            Err(err) => Err(err.clone().into()),
        }
    }
}

impl<'a, T: ResolveOwned<'a>> ResolveOwned<'a> for Vec<T> {
    fn resolve_owned(self, ctx: &Context) -> Result<Option<FieldValue<'a>>> {
        let iter = self.into_iter();
        let items = iter.enumerate().map(|(index, item)| {
            let ctx_idx = ctx.with_index(index);
            match item.resolve_owned(&ctx_idx) {
                Ok(Some(value)) => value,
                _ => FieldValue::NULL,
            }
        });
        Ok(Some(FieldValue::list(items)))
    }
}

impl<'a, T: ResolveRef<'a>> ResolveRef<'a> for Vec<T> {
    fn resolve_ref(&'a self, ctx: &Context) -> Result<Option<FieldValue<'a>>> {
        let iter = self.iter();
        let items = iter.enumerate().map(|(index, item)| {
            let ctx_idx = ctx.with_index(index);
            match item.resolve_ref(&ctx_idx) {
                Ok(Some(value)) => value,
                _ => FieldValue::NULL,
            }
        });
        Ok(Some(FieldValue::list(items)))
    }
}

impl<'a> ResolveOwned<'a> for ID {
    #[inline]
    fn resolve_owned(self, _ctx: &Context) -> Result<Option<FieldValue<'a>>> {
        Ok(Some(FieldValue::value(self.0)))
    }
}

impl<'a> ResolveRef<'a> for ID {
    #[inline]
    fn resolve_ref(&'a self, _ctx: &Context) -> Result<Option<FieldValue<'a>>> {
        Ok(Some(FieldValue::value(self.0.to_owned())))
    }
}

impl<'a> ResolveOwned<'a> for &str {
    #[inline]
    fn resolve_owned(self, _ctx: &Context) -> Result<Option<FieldValue<'a>>> {
        Ok(Some(FieldValue::value(self.to_string())))
    }
}

impl<'a> ResolveRef<'a> for &str {
    #[inline]
    fn resolve_ref(&'a self, _ctx: &Context) -> Result<Option<FieldValue<'a>>> {
        Ok(Some(FieldValue::value(self.to_owned())))
    }
}

macro_rules! output_value {
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
                fn resolve_ref(&'a self, _ctx: &Context) -> Result<Option<FieldValue<'a>>> {
                    Ok(Some(FieldValue::value(self.to_owned())))
                }
            }
        )*
    };
}

output_value!(String, i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, bool, f32, f64);
