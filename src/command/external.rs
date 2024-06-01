use std::process;

use super::Command;

pub fn create(input: Vec<&str>) -> Result<Command, String> {
    let param_path = input[0];
    let args: Vec<String> = input[1..].into_iter().map(|arg| arg.to_string()).collect();
    Ok(Command::External(param_path.to_string(), args))
}

pub fn execute(cmd_str: &String, args: &Vec<String>) -> Vec<u8> {
    let mut command = process::Command::new(cmd_str);
    if !args.is_empty() {
        command.args(args);
    }
    command.output().map_or(
        format!("{}: command not found", cmd_str)
            .as_bytes()
            .to_vec(),
        |output| output.stdout,
    )
}
