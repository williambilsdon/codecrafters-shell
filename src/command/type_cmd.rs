use std::{env::var_os, path::Path};

use super::Command;

pub fn create(input: Vec<&str>) -> Result<Command, String> {
    Ok(Command::Type(
        input[1..].iter().map(|input| input.to_string()).collect(),
    ))
}

pub fn execute(params: &Vec<String>) {
    for param in params {
        if is_builtin(param) {
            let output = format!("{} is a shell builtin", param);
            println!("{}", output);
            continue;
        }
        match is_in_path(param) {
            Some(path) => {
                let trimmed_output = format!("{} is {}/{}", param, path, param).to_string();
                println!("{}", trimmed_output.trim());
            }
            None => println!("{}: not found", param),
        }
    }
}

fn is_builtin(param: &str) -> bool {
    matches!(param, "exit" | "echo" | "type" | "cd" | "pwd")
}

fn is_in_path(param: &str) -> Option<String> {
    match var_os("PATH") {
        Some(path) => {
            if let Ok(path_string) = path.into_string() {
                let each_path: Vec<&Path> = path_string.split(':').map(Path::new).collect();
                each_path.into_iter().fold(None, |mut acc, path| {
                    for dir_entry in path.read_dir().unwrap() {
                        let file_name = dir_entry.unwrap().file_name();
                        if file_name == *param {
                            acc = Some(path.to_str().unwrap().to_string())
                        }
                    }
                    acc
                })
            } else {
                None
            }
        }
        _ => None,
    }
}
