use crate::Args;

pub struct FnContext<'a, C> {
    parameters: Args,
    context: &'a mut C,
}

impl<'a, C> FnContext<'a, C> {
    pub fn new(parameters: Args, context: &'a mut C) -> Self {
        Self {
            parameters,
            context,
        }
    }

    pub fn args(&self) -> &Args {
        &self.parameters
    }

    pub fn ctx(&self) -> &C {
        &self.context
    }

    pub fn ctx_mut(&mut self) -> &mut C {
        &mut self.context
    }
}
