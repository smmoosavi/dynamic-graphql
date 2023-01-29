use crate::{
    AnyBox, GraphqlType, Interface, InterfaceMark, Object, OutputType, Register, Registry,
    ResolveOwned,
};
use async_graphql::dynamic::FieldValue;
use async_graphql::Context;

pub struct Instance<'v, I, T = ()>
where
    I: ?Sized,
    I: Interface,
{
    pub(crate) _interface: std::marker::PhantomData<I>,
    pub(crate) _target: std::marker::PhantomData<T>,
    value: AnyBox<'v>,
}

impl<I: ?Sized> Instance<'_, I>
where
    I: Interface,
{
    #[inline]
    pub fn new_owned<'a, T>(value: T) -> Instance<'a, I>
    where
        T: InterfaceMark<I> + Object + Send + Sync + 'static,
    {
        Instance {
            _interface: std::marker::PhantomData,
            _target: std::marker::PhantomData,
            value: AnyBox::new_owned(value, <T as Object>::NAME.to_string()),
        }
    }
    #[inline]
    pub fn new_borrowed<T>(value: &T) -> Instance<I>
    where
        T: InterfaceMark<I> + Object + Send + Sync + 'static,
    {
        Instance {
            _interface: std::marker::PhantomData,
            _target: std::marker::PhantomData,
            value: AnyBox::new_borrowed(value, <T as Object>::NAME.to_string()),
        }
    }
}

impl<'a, I> ResolveOwned<'a> for Instance<'a, I>
where
    I: ?Sized + Interface,
{
    fn resolve_owned(self, ctx: &Context) -> async_graphql::Result<Option<FieldValue<'a>>> {
        self.value.resolve_owned(ctx)
    }
}

pub trait RegisterInstance<I, T>
where
    I: ?Sized,

    T: Object + 'static,
    T: Send + Sync,
{
    #[inline]
    fn register_instance(registry: Registry) -> Registry {
        registry
    }
}

impl<I, T> Register for Instance<'_, I, T>
where
    I: ?Sized,
    I: Interface + 'static,
    I: RegisterInstance<I, T>,
    T: Object + 'static,
    T: Send + Sync,
{
    #[inline]
    fn register(registry: Registry) -> Registry {
        let registry = registry.register::<I>();
        <I as RegisterInstance<I, T>>::register_instance(registry)
    }
}

impl<I> Register for Instance<'_, I, ()>
where
    I: ?Sized,
    I: Interface + 'static,
{
    #[inline]
    fn register(registry: Registry) -> Registry {
        registry.register::<I>()
    }
}

impl<I> GraphqlType for Instance<'_, I>
where
    I: Interface + 'static + ?Sized,
{
    const NAME: &'static str = <I as GraphqlType>::NAME;
}

impl<I> OutputType for Instance<'_, I>
where
    I: Interface + 'static + ?Sized,
{
    const NAME: &'static str = <I as OutputType>::NAME;
}
