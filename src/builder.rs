use std::{collections::HashMap, fmt::Display};

use crate::{Command, Error, Repl};

pub struct ReplBuilder<C, E>
where
    C: Clone,
    E: Clone + Display + Into<Error>,
{
    commands: HashMap<String, Command<C, E>>,
    ignore_empty_line: bool,
    welcome_message: String,
    output_prompt: String,
    exit_message: String,
    use_builtins: bool,
    version: String,
    prompt: String,
    context: C,
}

impl<C, E> ReplBuilder<C, E>
where
    C: Clone,
    E: Clone + Display + Into<Error>,
{
    pub fn new(context: C) -> Self {
        Self {
            version: String::from(env!("CARGO_PKG_VERSION")),
            welcome_message: String::new(),
            output_prompt: String::new(),
            exit_message: String::new(),
            prompt: String::from(">>"),
            commands: HashMap::new(),
            ignore_empty_line: true,
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
    /// let repl = Repl::builder().with_prompt("#");
    /// ```
    pub fn with_prompt<P>(mut self, prompt: P) -> Self
    where
        P: Into<String>,
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
    /// let repl = Repl::builder(()).with_welcome_message("Welcome from your REPL!");
    /// ```
    pub fn with_welcome_message<M>(mut self, message: M) -> Self
    where
        M: Into<String>,
    {
        self.welcome_message = message.into();
        self
    }

    /// Adds an exit message which gets printed when the user exists the REPL.
    ///
    /// ### Example
    ///
    /// ```no_run
    /// let repl = Repl::builder(()).with_exit_message("Exiting... Bye!");
    /// ```
    pub fn with_exit_message<M>(mut self, message: M) -> Self
    where
        M: Into<String>,
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
    /// let repl = Repl::builder(()).with_version("1.3.4");
    /// ```
    pub fn with_version<V>(mut self, version: V) -> Self
    where
        V: Into<String>,
    {
        self.version = version.into();
        self
    }

    /// Sets if empty lines (all whitespace) should be ignored.
    ///
    /// ### Example
    ///
    /// ```no_run
    /// let repl = Repl::builder(()).ignore_empty_line(true);
    /// ```
    pub fn ignore_empty_line(mut self, ignore: bool) -> Self {
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
    /// let repl = Repl::builder(()).with_output_prompt(Some("#"));
    /// ```
    pub fn with_output_prompt<P>(mut self, prompt: Option<P>) -> Self
    where
        P: Into<String>,
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
    /// fn hello(params: Parameters, _ctx: &mut ()) -> ReplResult<Option<String>> {
    ///     let name: String = params.get("name")?;
    ///     let end: String = params.get("end")?;
    ///
    ///     Ok(Some(format!("Hello, {}{}", name, end)))
    /// }
    ///
    /// let mut repl = Repl::builder(())
    ///     .with_command(
    ///         Command::new("hello", hello)
    ///             .with_param(Parameter::new("name"))
    ///             .with_param(Parameter::new("end")),
    ///     )
    ///     .build();
    ///
    /// repl.run();
    /// ```
    pub fn with_command(mut self, command: Command<C, E>) -> Self {
        self.commands.insert(command.name.clone(), command);
        self
    }

    /// Enables or disables builtin commands, like `help` or `version`.
    ///
    /// ### Example
    ///
    /// ```no_run
    /// let repl = Repl::builder(()).with_builtins(true);
    /// ```
    pub fn with_builtins(mut self, use_builtins: bool) -> Self {
        self.use_builtins = use_builtins;
        self
    }

    /// Build the [`Repl`] based on the configured [`ReplBuilder`]. This is
    /// function is a finalizer and should be called last.
    ///
    /// ### Example
    ///
    /// ```
    /// let mut repl = Repl::builder(())
    ///     .with_version("0.1.4")
    ///     .with_prompt(">")
    ///     .build();
    ///
    /// repl.run();
    /// ```
    pub fn build(self) -> Repl<C, E> {
        Repl {
            commands: self.commands,
            ignore_empty_line: self.ignore_empty_line,
            welcome_message: self.welcome_message,
            output_prompt: self.output_prompt,
            exit_message: self.exit_message,
            use_builtins: self.use_builtins,
            version: self.version,
            context: self.context,
            prompt: self.prompt,
        }
    }
}
