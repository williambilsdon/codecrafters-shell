mod command;

use command::Command;
use std::io::{self, Write};
fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let trimmed_input = input.trim().split_whitespace().collect();
        match Command::create_command(trimmed_input) {
            Ok(command) => command.execute(),
            Err(err) => println!("{}", err),
        }
    }
}
