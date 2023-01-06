use rupl::{Arg, Command, FnContext, Repl, ReplResult};

fn main() -> ReplResult<()> {
    let mut repl = Repl::new(());

    repl.with_prompt(">>")
        .with_version("1.0.1-rc2")
        .with_welcome_message("This basic REPL says 'Hello, world!'")
        .with_exit_message("Exiting... Bye!")
        .with_builtins(true)
        .ignore_empty_line(true)
        .with_output_prompt(Some(""))
        .with_command(
            Command::new("hello", hello)
                .with_arg(Arg::new("name"))
                .with_arg(Arg::new("end")),
        )
        .with_command(
            Command::new("bye", bye)
                .with_arg(Arg::new("name"))
                .with_arg(Arg::new("end")),
        );

    repl.run()
}

fn hello(ctx: FnContext<()>) -> ReplResult<Option<String>> {
    let name: String = ctx.args().get("name")?;
    let end: String = ctx.args().get("end")?;

    Ok(Some(format!("Hello, {}{}", name, end)))
}

fn bye(ctx: FnContext<()>) -> ReplResult<Option<String>> {
    let name: String = ctx.args().get("name")?;
    let end: String = ctx.args().get("end")?;

    Ok(Some(format!("Bye, {}{}", name, end)))
}
