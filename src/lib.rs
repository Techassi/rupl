mod command;
mod error;
mod parameters;

pub use command::*;
pub use error::*;
pub use parameters::*;

use std::{collections::HashMap, fmt::Display};

use rustyline::{error::ReadlineError, Editor};

pub type RunFn<C, E> = fn(Parameters, &mut C) -> std::result::Result<Option<String>, E>;

pub struct Repl<C, E>
where
    E: Display + Into<Error>,
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
    E: Display + Into<Error>,
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
        Self {
            version: String::from(env!("CARGO_PKG_VERSION")),
            welcome_message: String::new(),
            output_prompt: String::new(),
            exit_message: String::new(),
            prompt: String::from(">> "),
            ignore_empty_line: true,
            commands: HashMap::new(),
            use_builtins: true,
            context,
        }
    }

    /// Change the prompt which appears in front of every input line. The
    /// default is `>>`. This function automatically adds a space to the
    /// end of the prompt. Trailing whitespace is removed from the provided
    /// prompt beforehand.
    ///
    /// ### Example
    ///
    /// ```no_run
    /// let mut repl = Repl::new(());
    ///
    /// repl.with_prompt("#");
    /// repl.run();
    /// ```
    pub fn with_prompt<T>(&mut self, prompt: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.prompt = prompt.into().trim_end().to_string() + " ";
        self
    }

    /// Adds a welcome message which gets printed once at the start of the
    /// REPL.
    ///
    /// ### Example
    ///
    /// ```no_run
    /// let mut repl = Repl::new(());
    ///
    /// repl.with_welcome_message("Welcome from your REPL!");
    /// repl.run();
    /// ```
    pub fn with_welcome_message<T>(&mut self, message: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.welcome_message = message.into();
        self
    }

    /// Adds an exit message which gets printed when the user exists the REPL.
    ///
    /// ### Example
    ///
    /// ```no_run
    /// let mut repl = Repl::new(());
    ///
    /// repl.with_exit_message("Exiting... Bye!");
    /// repl.run();
    /// ```
    pub fn with_exit_message<T>(&mut self, message: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.exit_message = message.into();
        self
    }

    /// Adds a version string to the REPL. When builtin commands are enabled,
    /// the version can be printed with the `version` command.
    ///
    /// ### Example
    ///
    /// ```no_run
    /// let mut repl = Repl::new(());
    ///
    /// repl.with_version("1.3.4");
    /// repl.run();
    /// ```
    pub fn with_version<T>(&mut self, version: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.version = version.into();
        self
    }

    /// Sets if empty lines (all whitespace) should be ignored.
    ///
    /// ### Example
    ///
    /// ```no_run
    /// let mut repl = Repl::new(());
    ///
    /// repl.ignore_empty_line(true);
    /// repl.run();
    /// ```
    pub fn ignore_empty_line(&mut self, ignore: bool) -> &mut Self {
        self.ignore_empty_line = ignore;
        self
    }

    /// Set the output prompt. When [`Some`] is provided, this value will be
    /// used as the output prompt. Providing [`None`] will instead fallback to
    /// the input prompt. Disabling the output prompt can be achieved by
    /// providing `Some("")`.
    ///
    /// ### Example
    ///
    /// ```no_run
    /// let mut repl = Repl::new(());
    ///
    /// repl.with_output_prompt(Some("#"));
    /// repl.run();
    /// ```
    pub fn with_output_prompt<T>(&mut self, prompt: Option<T>) -> &mut Self
    where
        T: Into<String>,
    {
        match prompt {
            Some(prompt) => self.output_prompt = prompt.into().trim_end().to_string() + " ",
            None => self.output_prompt = self.prompt.clone(),
        }

        self
    }

    /// Adds a command to the REPL. See [`Command`] for more information on how
    /// to construct commands.
    ///
    /// ### Example
    ///
    /// ```no_run
    /// let mut repl = Repl::new(());
    ///
    /// fn hello(params: Parameters, _ctx: &mut ()) -> ReplResult<Option<String>> {
    ///     let name: String = params.get("name")?;
    ///     let punctation: String = params.get("punctation")?;
    ///
    ///     Ok(Some(format!("Hello, {}{}", name, punctation)))
    /// }
    ///
    /// repl.with_command(
    ///     Command::new("hello", hello)
    ///         .with_param(Parameter::new("name"))?
    ///         .with_param(Parameter::new("punctation"))?,
    /// );
    /// repl.run();
    /// ```
    pub fn with_command(&mut self, command: Command<C, E>) -> &mut Self {
        self.commands.insert(command.name.clone(), command);
        self
    }

    /// Enables or disables builtin commands, like `help` or `version`.
    ///
    /// ### Example
    ///
    /// ```no_run
    /// let mut repl = Repl::new(());
    ///
    /// repl.with_builtins(true);
    /// repl.run();
    /// ```
    pub fn with_builtins(&mut self, use_builtins: bool) -> &mut Self {
        self.use_builtins = use_builtins;
        self
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
        let mut editor = Editor::<()>::new()?;
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
                        Err(Error::ParameterError(err)) => self.handle_parameter_error(err),
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
                Err(err) => return Err(Error::EditorError(err)),
            }
        }

        if !self.exit_message.trim().is_empty() {
            println!("{}", self.exit_message);
        }

        Ok(())
    }

    fn handle_command(&mut self, line: &str) -> ReplResult<Option<String>> {
        let (cmd, args) = match line.split_once(" ") {
            Some(parts) => parts,
            None => (line, ""),
        };

        match self.commands.get(cmd) {
            Some(cmd) => {
                let mut params = Parameters::default();

                if cmd.has_params() {
                    params = match Parameters::new(args, cmd.parameters.clone()) {
                        Ok(p) => p,
                        Err(err) => return Err(err.into()), // TODO (Techassi): Make this configurable
                    };
                }

                match (cmd.run)(params, &mut self.context) {
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
                    "help" => self.handle_help_builtin(args),
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

    fn handle_parameter_error(&self, err: ParameterError) {
        self.handle_output(Some(err.to_string()))
    }

    fn print_welcome_message(&self) {
        if !self.welcome_message.is_empty() {
            println!("{}", self.welcome_message)
        }
    }
}
