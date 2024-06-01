#[allow(unused_imports)]
use std::io::{self, Write};
use std::{
    env::{self, var_os},
    ffi::OsString,
    path::Path,
    process,
};

enum Command {
    Exit(i32),
    Echo(Vec<String>),
    Type(Vec<String>),
    Pwd,
    Cd(String),
    External(String, Vec<String>),
}

impl Command {
    fn create_command(input: Vec<&str>) -> Result<Command, String> {
        match input[0] {
            "exit" => {
                if input.len() > 2 {
                    return Err(format!("{}: too many args provided", input[0]).to_string());
                } else if input.len() == 1 {
                    return Err(format!("{}: too few args provided", input[0]).to_string());
                }
                match input[1].parse() {
                    Ok(parse_result) => return Ok(Command::Exit(parse_result)),
                    Err(_) => return Err("Failed to parse input".to_string()),
                };
            }
            "echo" => Ok(Command::Echo(
                input[1..]
                    .into_iter()
                    .map(|input| input.to_string())
                    .collect(),
            )),
            "type" => Ok(Command::Type(
                input[1..]
                    .into_iter()
                    .map(|input| input.to_string())
                    .collect(),
            )),
            "pwd" => Ok(Command::Pwd),
            "cd" => Ok(Command::Cd(input[1].to_string())),
            param_path => {
                let args: Vec<String> = input[1..].into_iter().map(|arg| arg.to_string()).collect();
                Ok(Command::External(param_path.to_string(), args))
            }
        }
    }

    fn is_builtin(&self, param: &str) -> bool {
        matches!(param, "exit" | "echo" | "type" | "cd" | "pwd")
    }
    pub fn execute(&self) {
        match self {
            Command::Exit(status_code) => std::process::exit(status_code.to_owned()),
            Command::Echo(params) => {
                let echo_output = params.join(" ");
                println!("{}", echo_output);
                io::stdout().flush().unwrap();
            }
            Command::Type(params) => {
                for param in params {
                    if self.is_builtin(param) {
                        let output = format!("{} is a shell builtin", param);
                        println!("{}", output);
                        continue;
                    }
                    match is_in_path(param) {
                        Some(path) => {
                            let trimmed_output =
                                format!("{} is {}/{}", param, path, param).to_string();
                            println!("{}", trimmed_output.trim());
                        }
                        None => println!("{}: not found", param),
                    }
                }
            }
            Command::Pwd => match env::current_dir() {
                Ok(path) => println!(
                    "{}",
                    path.as_os_str()
                        .to_str()
                        .unwrap_or("Failed to print working directory")
                ),
                Err(_) => println!("Failed to print working directory"),
            },
            Command::Cd(path) => {
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
            Command::External(cmd_str, args) => {
                let mut command = process::Command::new(cmd_str);
                if !args.is_empty() {
                    command.args(args);
                }
                let output_result = command.output();
                let output = match output_result {
                    Ok(output) => output.stdout,
                    _ => return println!("{}: command not found", cmd_str),
                };
                let output_string =
                    String::from_utf8(output).expect("failed to parse external command output");
                println!("{}", output_string.trim());
            }
        }
    }
}
fn is_in_path(param: &str) -> Option<String> {
    match var_os("PATH") {
        Some(path) => {
            if let Ok(path_string) = path.into_string() {
                let each_path: Vec<&Path> =
                    path_string.split(':').map(|path| Path::new(path)).collect();
                let result = each_path.into_iter().fold(None, |mut acc, path| {
                    for dir_entry in path.read_dir().unwrap() {
                        let file_name = dir_entry.unwrap().file_name();
                        if file_name == OsString::from(param) {
                            acc = Some(path.to_str().unwrap().to_string())
                        }
                    }
                    acc
                });
                result
            } else {
                None
            }
        }
        _ => None,
    }
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
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
