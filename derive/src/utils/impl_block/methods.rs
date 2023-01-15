use crate::utils::with_context::SetContext;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct Methods<Method> {
    pub methods: Vec<Method>,
}

impl<Method> Deref for Methods<Method> {
    type Target = Vec<Method>;

    fn deref(&self) -> &Self::Target {
        &self.methods
    }
}

impl<Method: SetContext> SetContext for Methods<Method> {
    type Context = Method::Context;

    fn set_context(&mut self, context: Self::Context) {
        self.methods.set_context(context);
    }
}
