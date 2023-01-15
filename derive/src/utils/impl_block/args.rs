use crate::utils::impl_block::FromFnArg;
use crate::utils::with_context::SetContext;
use crate::utils::with_index::SetIndex;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct Args<MethodArg> {
    pub args: Vec<MethodArg>,
}

impl<MethodArg> Deref for Args<MethodArg> {
    type Target = Vec<MethodArg>;

    fn deref(&self) -> &Self::Target {
        &self.args
    }
}

impl<MethodArg: SetContext> SetContext for Args<MethodArg> {
    type Context = MethodArg::Context;

    fn set_context(&mut self, context: Self::Context) {
        self.args.set_context(context);
    }
}

impl<MethodArg: FromFnArg + SetIndex> Args<MethodArg> {
    pub fn from_fn_args<'a>(
        args: &mut impl Iterator<Item = &'a mut syn::FnArg>,
    ) -> darling::Result<Self> {
        Ok(Self {
            args: args
                .enumerate()
                .map(|(index, arg)| MethodArg::from_fn_arg(arg).with_index(index))
                .collect::<darling::Result<Vec<_>>>()?,
        })
    }
}
