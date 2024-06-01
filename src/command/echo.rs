use std::io::{self, Write};

use super::Command;

pub fn create(input: Vec<&str>) -> Result<Command, String> {
    Ok(Command::Echo(
        input[1..]
            .into_iter()
            .map(|input| input.to_string())
            .collect(),
    ))
}

pub fn execute(params: &Vec<String>) {
    let echo_output = params.join(" ");
    println!("{}", echo_output);
    io::stdout().flush().unwrap();
}
