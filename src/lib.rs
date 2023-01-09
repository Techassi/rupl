use std::{collections::HashMap, fmt::Display};

use rustyline::{completion::Completer, error::ReadlineError, Editor};
use rustyline_derive::{Helper, Highlighter, Hinter, Validator};

mod args;
mod builder;
mod command;
mod context;
mod error;

pub use args::*;
pub use builder::*;
pub use command::*;
pub use context::*;
pub use error::*;

pub type RunFn<C, E> = fn(FnContext<C>) -> std::result::Result<Option<String>, E>;

pub struct Repl<C, E>
where
    C: Clone,
    E: Clone + Display + Into<Error>,
{
    commands: HashMap<String, Command<C, E>>,
    ignore_empty_line: bool,
    welcome_message: String,
    output_prompt: String,
    exit_message: String,
    version: String,
    prompt: String,
    use_builtins: bool,
    context: C,
}

impl<C, E> Repl<C, E>
where
    C: Clone,
    E: Clone + Display + Into<Error>,
{
    /// Creates a new default REPL with a context.
    ///
    /// ### Example
    ///
    /// ```no_run
    /// let mut repl = Repl::new(());
    /// repl.run();
    /// ```
    pub fn new(context: C) -> Self {
        Self::builder(context).build()
    }

    /// Create a [`ReplBuilder`] to configure a [`Repl`].
    ///
    /// ### Example
    ///
    /// ```no_run
    /// let mut repl = Repl::builder(())
    ///     .with_version("0.1.4")
    ///     .with_prompt(">")
    ///     .build();
    ///
    /// repl.run();
    /// ```
    pub fn builder(context: C) -> ReplBuilder<C, E> {
        ReplBuilder::new(context)
    }

    /// Runs the REPL. This will block until the user exists the REPL with
    /// CTRL-C or CTROL-D for example. This behaviour can be customized.
    ///
    /// ### Example
    ///
    /// ```no_run
    /// let mut repl = Repl::new(());
    /// repl.run();
    /// ```
    pub fn run(&mut self) -> ReplResult<()> {
        let mut editor = match Editor::<Helper<C, E>>::new() {
            Ok(e) => e,
            Err(err) => return Err(Error::EditorError(err.to_string())),
        };

        let helper = Helper::new(self.commands.clone());
        editor.set_helper(Some(helper));
        self.print_welcome_message();

        loop {
            let readline = editor.readline(&self.prompt);

            match readline {
                Ok(line) => {
                    let line = line.trim();

                    if self.ignore_empty_line && line.is_empty() {
                        continue;
                    }

                    match self.handle_command(line) {
                        Err(Error::ArgError(err)) => self.handle_parameter_error(err),
                        Err(err) => return Err(err),
                        Ok(out) => self.handle_output(out),
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    #[cfg(debug_assertions)]
                    break;

                    #[cfg(not(debug_assertions))]
                    continue;
                }
                Err(ReadlineError::Eof) => {
                    #[cfg(debug_assertions)]
                    break;

                    #[cfg(not(debug_assertions))]
                    continue;
                }
                Err(err) => return Err(Error::EditorError(err.to_string())),
            }
        }

        if !self.exit_message.trim().is_empty() {
            println!("{}", self.exit_message);
        }

        Ok(())
    }

    fn handle_command(&mut self, line: &str) -> ReplResult<Option<String>> {
        let (cmd, args_str) = match line.split_once(" ") {
            Some(parts) => parts,
            None => (line, ""),
        };

        match self.commands.get(cmd) {
            Some(cmd) => {
                let mut parsed_args = Args::default();

                if cmd.has_args() {
                    parsed_args = match Args::new(args_str, cmd.args.clone()) {
                        Ok(p) => p,
                        Err(err) => return Err(err.into()), // TODO (Techassi): Make this configurable
                    };
                }

                match (cmd.run)(FnContext::new(parsed_args, &mut self.context)) {
                    Ok(Some(out)) => return Ok(Some(out)),
                    Ok(None) => return Ok(None),
                    Err(err) => return Err(err.into()),
                }
            }
            None => {
                if !self.use_builtins {
                    return Err(Error::NoSuchCommandError(cmd.into()));
                }

                match cmd {
                    "help" => self.handle_help_builtin(args_str),
                    "version" => self.handle_version_builtin(),
                    _ => return Err(Error::NoSuchCommandError(cmd.into())),
                }
            }
        }
    }

    fn handle_output(&self, out: Option<String>) {
        if out.is_some() {
            println!("{}{}", self.output_prompt, out.unwrap());
        }
    }

    fn handle_help_builtin<A>(&self, _args: A) -> ReplResult<Option<String>>
    where
        A: Into<String>,
    {
        Ok(Some(String::from("Help requested!")))
    }

    fn handle_version_builtin(&self) -> ReplResult<Option<String>> {
        Ok(Some(self.version.clone()))
    }

    fn handle_parameter_error(&self, err: ArgError) {
        self.handle_output(Some(err.to_string()))
    }

    fn print_welcome_message(&mut self) {
        if !self.welcome_message.is_empty() {
            println!("{}", self.welcome_message)
        }
    }
}

#[derive(Helper, Hinter, Highlighter, Validator)]
struct Helper<C, E>
where
    C: Clone,
    E: Clone + Display + Into<Error>,
{
    pub(crate) commands: HashMap<String, Command<C, E>>,
}

impl<C, E> Helper<C, E>
where
    C: Clone,
    E: Clone + Display + Into<Error>,
{
    pub(crate) fn new(commands: HashMap<String, Command<C, E>>) -> Self {
        Self { commands }
    }
}

impl<C, E> Completer for Helper<C, E>
where
    C: Clone,
    E: Clone + Display + Into<Error>,
{
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let line = line.trim();
        // The user didn't type anything and pressed tab. In this case we
        // display a list of available commands
        if pos == 0 || line.is_empty() {
            let cmds: Vec<Self::Candidate> =
                self.commands.iter().map(|c| c.1.name.clone()).collect();
            return Ok((0, cmds));
        }

        // If we have some input, try to find the correct command and display
        // the arguments as tab completions options. If we didn't match any
        // command, we have to deal with partial input: try to match commands
        // starting with the current input
        let (start, _) = match line.split_once(' ') {
            Some(parts) => parts,
            None => (line, ""),
        };

        match self.commands.get(start) {
            Some(cmd) => {
                let args: Vec<Self::Candidate> =
                    cmd.args.iter().map(|a| format!("--{}", a.name)).collect();
                return Ok((pos, args));
            }
            None => return Ok((pos, Vec::with_capacity(0))),
        };

        // Ok((pos, Vec::with_capacity(0)))
    }
}
