use std::collections::HashMap;

use crate::args::Arg;

pub struct Command<S> {
    pub(crate) sub: HashMap<String, Command<S>>,
    pub(crate) func: Box<dyn Fn(&mut S) -> String>,
    pub(crate) args: Vec<Arg>,
    pub(crate) name: String,
}

impl<S> Command<S> {
    pub fn new<N, F>(name: N, func: F) -> Self
    where
        N: Into<String>,
        F: Fn(&mut S) -> String + 'static,
    {
        Self {
            func: Box::new(func),
            sub: HashMap::new(),
            name: name.into(),
            args: Vec::new(),
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn list_subcommands(&self) -> Vec<&String> {
        self.sub.keys().collect()
    }

    pub fn with_subcommand(mut self, command: Command<S>) -> Self {
        self.sub.insert(command.name().clone(), command);
        self
    }

    pub fn with_arg<N>(mut self, name: N, standalone: bool) -> Self
    where
        N: Into<String>,
    {
        self.args.push(Arg::new(name, standalone));
        self
    }

    pub fn run(&self, state: &mut S) -> String {
        (self.func)(state)
    }

    pub(crate) fn parse_args<'a>(&self, args: Vec<(&'a str, &'a str)>) -> bool {
        args.iter().all(|arg| {
            self.args.iter().any(|a| {
                if !a.is_standalone() && arg.1.is_empty() {
                    return false;
                }

                a == arg.0
            })
        })
    }
}
