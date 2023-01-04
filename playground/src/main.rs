use std::collections::HashMap;

use rupl::{Command, EmptyLineBehaviour, Repl, Result};

fn main() {
    fn hello<C>(args: HashMap<String, String>, ctx: &mut C) -> Result<Option<String>> {
        Ok(Some(String::from("world")))
    }

    let mut repl = Repl::new(());
    repl.with_prompt("# ")
        .with_welcome_message("This basic REPL says 'Hello, world!'")
        .with_exit_message("Exiting... Bye!")
        .with_default_commands()
        .with_empty_line_behaviour(EmptyLineBehaviour::Ignore)
        .with_output_prompt(Some("> "))
        .with_command(Command::new("hello", hello));

    match repl.run() {
        Ok(_) => (),
        Err(err) => println!("{}", err),
    }
}
