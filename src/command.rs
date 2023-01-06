use std::fmt::Display;

use crate::{args::Arg, error::Error, RunFn};

#[derive(Clone)]
pub struct Command<C, E>
where
    C: Clone,
    E: Clone + Display + Into<Error>,
{
    pub(crate) help: Option<String>,
    pub(crate) run: RunFn<C, E>,
    pub(crate) args: Vec<Arg>,
    pub(crate) name: String,
}

impl<C, E> Command<C, E>
where
    C: Clone,
    E: Clone + Display + Into<Error>,
{
    pub fn new<T>(name: T, run: RunFn<C, E>) -> Self
    where
        T: Into<String>,
    {
        Self {
            name: name.into(),
            args: vec![],
            help: None,
            run,
        }
    }

    pub fn with_arg(mut self, param: Arg) -> Self {
        self.args.push(param);
        self
    }

    pub fn with_help<T>(mut self, help: T) -> Self
    where
        T: Into<String>,
    {
        self.help = Some(help.into());
        self
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub(crate) fn has_args(&self) -> bool {
        self.args.len() > 0
    }
}
