use rupl::{command::Command, error::ReplResult, Repl};

struct Ctx {
    counter: usize,
}

fn main() -> ReplResult<()> {
    let mut state = ();

    let mut repl = Repl::<_>::builder(&mut state)
        .with_prompt(">>")
        .with_output_prompt("#")
        .with_command(
            Command::new("service", service).with_subcommand(
                Command::new("dns", service_dns)
                    .with_subcommand(Command::new("status", service_dns))
                    .with_arg("port", false)
                    .with_arg("mode", false),
            ),
        )
        .build();

    repl.run()
}

fn service(ctx: &mut ()) -> String {
    "Hello from service".into()
}

fn service_dns(ctx: &mut ()) -> String {
    "Hello from service_dns".into()
}
