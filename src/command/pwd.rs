use std::env;

use super::Command;

pub fn create() -> Result<Command, String> {
    Ok(Command::Pwd)
}

pub fn execute() {
    match env::current_dir() {
        Ok(path) => println!(
            "{}",
            path.as_os_str()
                .to_str()
                .unwrap_or("Failed to print working directory")
        ),
        Err(_) => println!("Failed to print working directory"),
    }
}
