use std::collections::HashMap;

pub struct Command<'a, C> {
    pub(crate) sub: HashMap<String, Command<'a, C>>,
    pub(crate) func: Box<dyn Fn(&mut C) -> String>,
    pub(crate) args: Vec<&'a str>,
    pub(crate) name: String,
}

impl<'a, C> Command<'a, C> {
    pub fn new<N, F>(name: N, func: F) -> Self
    where
        N: Into<String>,
        F: Fn(&mut C) -> String + 'static,
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

    pub fn list_subcommands(&self) -> Vec<String> {
        self.sub.keys().map(|k| k.to_string()).collect()
    }

    pub fn with_subcommand(mut self, command: Command<'a, C>) -> Self {
        self.sub.insert(command.name().clone(), command);
        self
    }

    pub fn with_arg(mut self, name: &'a str) -> Self {
        self.args.push(name);
        self
    }

    pub fn run(&self, ctx: &mut C) -> String {
        (self.func)(ctx)
    }

    pub(crate) fn parse_args(&self, args: Vec<(&'a str, &'a str)>) -> bool {
        args.iter().all(|arg| self.args.contains(&arg.0))
    }
}
