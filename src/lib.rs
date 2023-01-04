mod command;
mod error;
mod parameter;

pub use command::*;
pub use error::*;
pub use parameter::*;

use std::{collections::HashMap, fmt::Display};

use rustyline::{error::ReadlineError, Editor};

pub type RunFn<C, E> =
    fn(HashMap<String, String>, &mut C) -> std::result::Result<Option<String>, E>;

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
    context: C,
}

impl<C, E> Repl<C, E>
where
    E: Display + Into<Error>,
{
    pub fn new(context: C) -> Self {
        Self {
            version: String::from(env!("CARGO_PKG_VERSION")),
            welcome_message: String::new(),
            output_prompt: String::new(),
            exit_message: String::new(),
            prompt: String::from(">> "),
            ignore_empty_line: true,
            commands: HashMap::new(),
            context,
        }
    }

    pub fn with_prompt<T>(&mut self, prompt: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.prompt = prompt.into();
        self
    }

    pub fn with_welcome_message<T>(&mut self, message: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.welcome_message = message.into();
        self
    }

    pub fn with_exit_message<T>(&mut self, message: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.exit_message = message.into();
        self
    }

    pub fn with_version<T>(&mut self, version: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.version = version.into();
        self
    }

    pub fn with_command(&mut self, command: Command<C, E>) -> &mut Self {
        self.commands.insert(command.name.clone(), command);
        self
    }

    pub fn ignore_empty_line(&mut self, ignore: bool) -> &mut Self {
        self.ignore_empty_line = ignore;
        self
    }

    pub fn with_output_prompt<T>(&mut self, prompt: Option<T>) -> &mut Self
    where
        T: Into<String>,
    {
        match prompt {
            Some(prompt) => self.output_prompt = prompt.into(),
            None => self.output_prompt = self.prompt.clone(),
        }

        self
    }

    pub fn with_default_commands(&mut self) -> &mut Self {
        fn help<C, E>(
            _args: HashMap<String, String>,
            _ctx: &mut C,
        ) -> std::result::Result<Option<String>, E> {
            Ok(Some(String::from("world")))
        }

        self.with_command(Command::new("help", help));

        self
    }

    pub fn run(&mut self) -> Result<()> {
        let mut editor = Editor::<()>::new()?;

        loop {
            let readline = editor.readline(&self.prompt);

            match readline {
                Ok(line) => {
                    let line = line.trim();

                    if self.ignore_empty_line && line.is_empty() {
                        continue;
                    }

                    self.handle_command(line)?
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

    fn handle_command(&mut self, line: &str) -> Result<()> {
        let (cmd, args) = match line.split_once(" ") {
            Some(parts) => parts,
            None => (line, ""),
        };

        match self.commands.get(cmd) {
            Some(cmd) => match (cmd.run)(HashMap::new(), &mut self.context) {
                Ok(Some(out)) => println!("{}{}", self.output_prompt, out),
                Ok(None) => (),
                Err(err) => return Err(err.into()),
            },
            None => (),
        };

        Ok(())
    }
}
