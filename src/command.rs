mod cd;
mod echo;
mod exit;
mod external;
mod pwd;
mod type_cmd;

pub enum Command {
    Exit(i32),
    Echo(Vec<String>),
    Type(Vec<String>),
    Pwd,
    Cd(String),
    External(String, Vec<String>),
}

impl Command {
    pub fn create_command(input: Vec<&str>) -> Result<Command, String> {
        match input[0] {
            "exit" => exit::create(input),
            "echo" => echo::create(input),
            "type" => type_cmd::create(input),
            "pwd" => pwd::create(),
            "cd" => cd::create(input),
            _ => external::create(input),
        }
    }

    pub fn execute(&self) {
        match self {
            Command::Exit(status_code) => exit::exectute(status_code),
            Command::Echo(params) => echo::execute(params),
            Command::Type(params) => type_cmd::execute(params),
            Command::Pwd => pwd::execute(),
            Command::Cd(path) => cd::execute(path),
            Command::External(cmd_str, args) => {
                let output = external::execute(cmd_str, args);
                let output_string =
                    String::from_utf8(output).expect("failed to parse external command output");
                println!("{}", output_string.trim());
            }
        }
    }
}
