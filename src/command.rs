use std::collections::HashMap;

pub struct Command<'a, C> {
    pub(crate) sub: HashMap<&'a str, Command<'a, C>>,
    pub(crate) func: Box<dyn Fn(&mut C) -> String>,
    pub(crate) args: Vec<String>,
    pub(crate) name: &'a str,
}

impl<'a, C> Command<'a, C> {
    pub fn new<F>(name: &'a str, func: F) -> Self
    where
        F: Fn(&mut C) -> String + 'static,
    {
        Self {
            func: Box::new(func),
            sub: HashMap::new(),
            args: Vec::new(),
            name,
        }
    }

    pub fn name(&self) -> &'a str {
        self.name
    }

    pub fn with_subcommand(mut self, command: Command<'a, C>) -> Self {
        self.sub.insert(command.name(), command);
        self
    }

    pub fn with_arg<T: Into<String>>(mut self, name: T) -> Self {
        self.args.push(name.into());
        self
    }

    pub fn run(&self, ctx: &mut C) -> String {
        (self.func)(ctx)
    }
}
