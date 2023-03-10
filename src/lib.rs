use std::io::{Stdout, Write};

use termion::{event::Key, input::TermRead, raw::RawTerminal};

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

pub struct Repl<C>
where
    C: Clone,
    // E: std::fmt::Debug + Display + Into<ReplError>,
{
    stdout: RawTerminal<Stdout>,
    buffer: CursorBuffer,
    output: String,
    context: C,
}

impl<C> Repl<C>
where
    C: Clone,
    // E: std::fmt::Debug + Display + Into<ReplError>,
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
    pub fn builder(context: C) -> ReplBuilder<C> {
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
        let mut stdin = termion::async_stdin().keys();

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
            Key::Backspace => Ok(self.handle_backspace_key()),
            Key::Left => Ok(self.handle_left_key()),
            Key::Right => Ok(self.handle_right_key()),
            Key::Up => todo!(),
            Key::Down => todo!(),
            Key::Home => todo!(),
            Key::End => todo!(),
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

        // self.write_to_stdout_flush(format!("Key: {:?}", key))
    }

    fn handle_backspace_key(&mut self) {
        if self.buffer.get_pos() == 0 {
            return;
        }

        self.buffer.remove_one(Direction::Left);
    }

    fn handle_left_key(&mut self) {
        self.buffer.move_left();
    }

    fn handle_right_key(&mut self) {
        self.buffer.move_right();
    }

    fn handle_char_key(&mut self, c: char) -> ReplResult<()> {
        match c {
            '\n' => self.handle_enter_key(),
            _ => {
                self.buffer.insert(&[c])?;
                self.display()?;
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

    fn parse_input(&mut self) -> ReplResult<()> {
        self.output.push_str("PARSE: ");

        self.display()?;
        self.newline()?;

        // Clear the current input buffer after parsing the
        // inpput and executing any matched commands.
        self.buffer.clear();

        Ok(())
    }

    fn display(&mut self) -> ReplResult<()> {
        // Erase entire line and go back to start of line
        self.output.insert_str(0, "\x1B[2K\r");

        // Append current input buffer, write to stdout
        self.output.push_str(self.buffer.to_string().as_str());
        self.stdout.write_all(self.output.as_bytes())?;

        // Flush and clear current output
        self.stdout.flush()?;
        self.output.clear();

        Ok(())
    }

    fn newline(&mut self) -> ReplResult<()> {
        Ok(self.stdout.write_all(b"\r\n")?)
    }
}
