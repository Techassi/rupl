use rupl::{command::Command, error::ReplResult, Repl};

struct Ctx {
    counter: usize,
}

fn main() -> ReplResult<()> {
    let mut context = Ctx { counter: 0 };

    let mut repl = Repl::<_>::builder(&mut context)
        .with_prompt(">>")
        .with_output_prompt("#")
        .with_command(
            Command::new("service", service)
                .with_subcommand(Command::new("dns", service_dns))
                .with_arg("name"),
        )
        // .with_version("1.0.1-rc2")
        // .with_welcome_message("This basic REPL says 'Hello, world!'")
        // .with_exit_message("Exiting... Bye!")
        // .with_builtins(true)
        // .ignore_empty_line(true)
        // .with_command(
        //     Command::new("hello", hello)
        //         .with_arg(Arg::new("name"))
        //         .with_arg(Arg::new("end")),
        // )
        // .with_command(
        //     Command::new("bye", bye)
        //         .with_arg(Arg::new("name"))
        //         .with_arg(Arg::new("end")),
        // )
        .build();

    repl.run()
}

fn service(ctx: &mut Ctx) -> String {
    ctx.counter += 1;

    format!("Hello from service {}", ctx.counter)
}

fn service_dns(ctx: &mut Ctx) -> String {
    ctx.counter += 1;

    "Hello from service_dns".into()
}
