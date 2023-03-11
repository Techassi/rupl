use std::{
    collections::HashMap,
    io::{stdin, Stdout, Write},
};

use nom::{
    character::complete::{alpha1, alphanumeric1, char},
    combinator::cut,
    multi::many0,
    sequence::separated_pair,
    IResult,
};
use termion::{event::Key, input::TermRead, raw::RawTerminal};
use thiserror::Error;

pub mod args;
pub mod buffer;
pub mod builder;
pub mod command;
pub mod context;
pub mod error;

use args::*;
use buffer::*;
use builder::*;
use command::*;
use context::*;
use error::*;

pub type RunFn<C> = fn(FnContext<C>) -> std::result::Result<Option<String>, ReplError>;

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("Empty input")]
    EmptyInput,

    #[error("Invalid arguments")]
    InvalidArgs,
}

pub struct Repl<'a, C> {
    commands: HashMap<String, Command<'a, C>>,
    stdout: RawTerminal<Stdout>,
    buffer: CursorBuffer,
    context: &'a mut C,
    output: String,
}

impl<'a, C> Repl<'a, C> {
    /// Creates a new default REPL with a context.
    ///
    /// ### Example
    ///
    /// ```no_run
    /// let mut repl = Repl::new(());
    /// repl.run();
    /// ```
    pub fn new(context: &'a mut C) -> Self {
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
    pub fn builder(context: &'a mut C) -> ReplBuilder<'a, C> {
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
        let mut stdin = stdin().keys();

        loop {
            match stdin.next() {
                Some(result) => match result {
                    Ok(key) => self.handle_key(key)?,
                    Err(err) => panic!("{err}"),
                },
                None => continue,
            };
        }
    }

    fn handle_key(&mut self, key: Key) -> ReplResult<()> {
        match key {
            Key::Backspace => self.handle_backspace_key(),
            Key::Left => self.handle_left_key(),
            Key::Right => self.handle_right_key(),
            Key::Up => self.handle_up_key(),
            Key::Down => self.handle_down_key(),
            Key::Home => self.handle_home_key(),
            Key::End => self.handle_end_key(),
            Key::PageUp => todo!(),
            Key::PageDown => todo!(),
            Key::BackTab => todo!(),
            Key::Delete => todo!(),
            Key::Insert => todo!(),
            Key::F(_) => todo!(),
            Key::Char(c) => self.handle_char_key(c),
            Key::Alt(_) => todo!(),
            Key::Ctrl(_) => todo!(),
            Key::Null => todo!(),
            Key::Esc => todo!(),
            _ => todo!(),
        }
    }

    fn handle_backspace_key(&mut self) -> ReplResult<()> {
        // We are all the way left, pressing backspace does nothing
        if self.buffer.get_pos() == 0 {
            return Ok(());
        }

        let _ = self.buffer.remove_one(Direction::Left)?;
        self.display_input()
    }

    fn handle_left_key(&mut self) -> ReplResult<()> {
        self.left()
    }

    fn handle_right_key(&mut self) -> ReplResult<()> {
        self.right()
    }

    fn handle_up_key(&mut self) -> ReplResult<()> {
        Ok(())
    }

    fn handle_down_key(&mut self) -> ReplResult<()> {
        Ok(())
    }

    fn handle_home_key(&mut self) -> ReplResult<()> {
        Ok(())
    }

    fn handle_end_key(&mut self) -> ReplResult<()> {
        Ok(())
    }

    fn handle_char_key(&mut self, c: char) -> ReplResult<()> {
        match c {
            '\n' => self.handle_enter_key(),
            '\t' => self.handle_tab_key(),
            _ => {
                self.buffer.insert(&[c])?;
                self.display_input()?;
                Ok(())
            }
        }
    }

    fn handle_enter_key(&mut self) -> ReplResult<()> {
        // No input, do nothing
        if self.buffer.is_empty() {
            return self.newline();
        }

        // Else handle the input
        self.newline()?;
        self.parse_input()
    }

    fn handle_tab_key(&mut self) -> ReplResult<()> {
        Ok(())
    }

    /// Parses the input. The function tries to match commands, subcommands
    /// and arguments.
    fn parse_input(&mut self) -> ReplResult<()> {
        let input = self.buffer.to_string();
        let input = input.as_str();

        // TODO (Techassi): Introduce standalone args and kv args
        let res = match parse(input, &self.commands) {
            Ok(res) => res,
            Err(_) => {
                self.output = "Invalid number of args".into();
                self.buffer.clear();
                self.display_input()?;
                self.newline()?;
                return Ok(());
            }
        };

        self.output = match res {
            (Some(cmd), args) => {
                if !cmd.parse_args(args) {
                    "Invalid arguments".into()
                } else {
                    cmd.run(self.context)
                }
            }
            _ => "Unknown command".into(),
        };

        // Clear the current input buffer after parsing the
        // inpput and executing any matched commands.
        self.buffer.clear();

        self.display_input()?;
        self.newline()?;

        Ok(())
    }

    /// Displays the user input on stdout. This is achieved by first erasing
    /// the contents of the current line, writing the refreshed input to
    /// stdout, flushing it and then clearing the output buffer.
    fn display_input(&mut self) -> ReplResult<()> {
        // Erase entire line and go back to start of line
        self.output.insert_str(0, "\x1B[2K\r");

        // Append current input buffer, write to stdout
        self.output.push_str(self.buffer.to_string().as_str());
        write!(self.stdout, "{}", self.output)?;

        // Position the cursor correctly again
        let diff = self.buffer.len() - self.buffer.get_pos();
        if diff != 0 {
            write!(self.stdout, "{}", termion::cursor::Left(diff as u16))?;
        }

        // Flush and clear current output
        self.stdout.flush()?;
        self.output.clear();

        Ok(())
    }

    fn display_output() {}

    /// Inserts a newline into stdout
    fn newline(&mut self) -> ReplResult<()> {
        Ok(write!(self.stdout, "\r\n")?)
    }

    /// Moves the cursor left. This moves the cursor in the
    /// terminal and the input buffer.
    fn left(&mut self) -> ReplResult<()> {
        if self.buffer.move_left() {
            write!(self.stdout, "{}", termion::cursor::Left(1))?;
            self.stdout.flush()?
        }

        Ok(())
    }

    /// Moves the cursor right. This moves the cursor in the
    /// terminal and the input buffer.
    fn right(&mut self) -> ReplResult<()> {
        if self.buffer.move_right() {
            write!(self.stdout, "{}", termion::cursor::Right(1))?;
            self.stdout.flush()?
        }

        Ok(())
    }
}

fn parse<'a, C>(
    input: &'a str,
    commands: &'a HashMap<String, Command<'a, C>>,
) -> Result<(Option<&'a Command<'a, C>>, Vec<(&'a str, &'a str)>), ParserError> {
    let mut input = input;

    let mut cmds = commands;
    let mut cmd = None;

    loop {
        let (part, rest) = match input.split_once(' ') {
            Some(split) => split,
            None => (input, ""),
        };

        if let Some(c) = cmds.get(part) {
            cmds = &c.sub;
            cmd = Some(c);
            input = rest;
            continue;
        }

        break;
    }

    if cmd.is_none() {
        return Ok((None, vec![]));
    }

    let (_, args) = match arg_pair_parser(input) {
        Ok(pairs) => pairs,
        Err(_) => return Err(ParserError::InvalidArgs),
    };

    Ok((cmd, args))
}

fn arg_pair_parser(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    many0(separated_pair(alpha1, cut(char(' ')), cut(alphanumeric1)))(input)
}
