use std::process::{Command, Stdio, ExitStatus};

use crate::cli::InputOptions;

pub fn create_command(opt: &InputOptions) -> Result<Command, &str> {
    let program = match opt.command.first() {
        None => return Err("no command specified"),
        Some(c) => c.clone()
    };
    let args = &opt.command[1..];
    let mut command = Command::new(program);

    command.args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    Ok(command)
}

pub fn run_command(command: &mut Command) -> std::io::Result<ExitStatus> {
    command.spawn()?.wait()
}
