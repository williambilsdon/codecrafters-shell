use std::{
    env::{self, var_os},
    ffi::OsString,
};

use super::Command;

pub fn create(input: Vec<&str>) -> Result<Command, String> {
    Ok(Command::Cd(input[1].to_string()))
}

pub fn execute(path: &String) {
    let home_path = if path == "~" {
        var_os("HOME").unwrap()
    } else {
        OsString::from(path)
    };
    match env::set_current_dir(home_path) {
        Ok(_) => (),
        Err(_) => {
            let output = format!("{}: No such file or directory", path);
            println!("{}", output.trim());
        }
    }
}
