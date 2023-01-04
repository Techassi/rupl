use std::fmt::Display;

use crate::{
    error::{Error, Result},
    parameter::Parameter,
    RunFn,
};

pub struct Command<C, E>
where
    E: Display + Into<Error>,
{
    pub(crate) subcommands: Vec<Command<C, E>>,
    pub(crate) parameters: Vec<Parameter>,
    pub(crate) help: Option<String>,
    pub(crate) run: RunFn<C, E>,
    pub(crate) name: String,
}

impl<C, E> Command<C, E>
where
    E: Display + Into<Error>,
{
    pub fn new<T>(name: T, run: RunFn<C, E>) -> Self
    where
        T: Into<String>,
    {
        Self {
            subcommands: vec![],
            parameters: vec![],
            name: name.into(),
            help: None,
            run,
        }
    }

    pub fn with_subcommand(&mut self, command: Command<C, E>) -> Result<&mut Self> {
        if self.parameters.len() > 0 {
            return Err(Error::IllegalSubcommandError);
        }

        self.subcommands.push(command);
        Ok(self)
    }

    pub fn with_param(&mut self, param: Parameter) -> Result<&mut Self> {
        if self.subcommands.len() > 0 {
            return Err(Error::IllegalParameterError);
        }

        self.parameters.push(param);
        Ok(self)
    }

    pub fn with_help<T>(&mut self, help: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.help = Some(help.into());
        self
    }
}
