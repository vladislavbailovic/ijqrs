use super::app;

mod instructions;
use instructions::{
    Instruction,
};

pub fn run_internal(command: &str, state: &app::State) -> Result<String, String> {
    let cmd: Vec<&str> = command.splitn(2, ' ').collect();
    let mut param = "";
    if cmd.len() > 1 {
        param = cmd[1];
    }
    let param = param;
    let instruction = match cmd[0] {
        "w" => instructions::new(Instruction::WriteOut,  param.to_string()),
        "wo" => instructions::new(Instruction::WriteOut, param.to_string()),
        "wc" => instructions::new(Instruction::WriteCmd, param.to_string()),
        _=> instructions::new(Instruction::Unknown, command.to_string()),
    };
    return instruction.eval(state);
}

use std::process::Command;

pub fn run_command(command: &str, filename: &str) -> String {
    let command = Command::new("jq")
        .arg(command)
        .arg(filename)
        .output()
        .expect("Command execution failed");
    let result = String::from_utf8(command.stdout).expect("Invalid stdout");
    if result.is_empty() {
        return String::from_utf8(command.stderr).expect("Invalid stderr");
    }
    result
}

use std::env;
use std::fs::File;
use std::io::Write;

fn write_file(fname: &str, content: &str) -> String {
    let mut cwd = env::current_dir().expect("Error resolving cwd");
    cwd.push(fname);
    let path = cwd.to_str().expect("Error resolving cwd file path");
    let mut file = File::create(path).expect("Error creating file");
    file.write_all(content.as_bytes()).expect("Error writing file!");
    String::from(path)
}

use std::time::{SystemTime, UNIX_EPOCH};

pub fn write_temp(source: &str) -> String {
    let mut tmp = env::temp_dir();
    let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Error figuring out current time");
    let fname = format!("ijqrs-{}.json", now.as_nanos());
    tmp.push(fname);
    let path = tmp.to_str().expect("Error getting temporary file path");

    let mut file = File::create(path).expect("Error creating temp file");
    file.write_all(source.as_bytes()).expect("Error writing file!");
    return String::from(path);
}
