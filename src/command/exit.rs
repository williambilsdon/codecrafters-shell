use super::Command;

pub fn create(input: Vec<&str>) -> Result<Command, String> {
    if input.len() > 2 {
        return Err(format!("{}: too many args provided", input[0]).to_string());
    } else if input.len() == 1 {
        return Err(format!("{}: too few args provided", input[0]).to_string());
    }
    match input[1].parse() {
        Ok(parse_result) => return Ok(Command::Exit(parse_result)),
        Err(_) => return Err("Failed to parse args".to_string()),
    };
}

pub fn exectute(status_code: &i32) {
    std::process::exit(status_code.to_owned())
}
