use rupl::{Command, Parameter, Parameters, Repl, ReplResult};

fn main() -> ReplResult<()> {
    let mut repl = Repl::new(());

    repl.with_prompt(">>")
        .with_version("1.0.1-rc2")
        .with_welcome_message("This basic REPL says 'Hello, world!'")
        .with_exit_message("Exiting... Bye!")
        .with_builtins(true)
        .ignore_empty_line(true)
        .with_output_prompt(Some(":> "))
        .with_command(
            Command::new("hello", hello)
                .with_param(Parameter::new("name"))?
                .with_param(Parameter::new("punctation"))?,
        )
        .with_command(
            Command::new("service", service)
                .with_subcommand(Command::new("status", service_status))?,
        );

    repl.run()
}

fn hello(params: Parameters, _ctx: &mut ()) -> ReplResult<Option<String>> {
    let name: String = params.get("name")?;
    let punctation: String = params.get("punctation")?;

    Ok(Some(format!("Hello, {}{}", name, punctation)))
}

fn service(params: Parameters, ctx: &mut ()) -> ReplResult<Option<String>> {
    service_status(params, ctx)
}

fn service_status(_params: Parameters, _ctx: &mut ()) -> ReplResult<Option<String>> {
    Ok(None)
}
