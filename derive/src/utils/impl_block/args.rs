use crate::utils::with_context::SetContext;
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
