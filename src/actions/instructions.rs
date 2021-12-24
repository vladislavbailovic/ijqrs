use super::super::ui::Pane;
use super::{app, write_file};

const OUTFILE_CMD: &str = "ijqrs.cmd";
const OUTFILE_OUT: &str = "ijqrs.out";

pub enum Instruction {
    Unknown,
    Jq,
    WriteOut,
    WriteCmd,
    // YankOut,
    // YankCmd,
}

pub fn new(inst: Instruction, param: String) -> Box<dyn Instr> {
    match inst {
        Instruction::Jq => Box::new(Jq {}),
        Instruction::WriteOut => Box::new(WriteOut { param }),
        Instruction::WriteCmd => Box::new(WriteCmd { param }),
        Instruction::Unknown => Box::new(Unknown { param }),
    }
}

pub trait Instr {
    fn eval(&self, state: &app::State) -> Result<String, String>;
}

pub trait InstrWrite {
    fn param(&self) -> String;
    fn default_filename(&self) -> String;
    fn content(&self, state: &app::State) -> String;
    fn write(&self, state: &app::State) -> Result<String, String> {
        let cmd = self.param();
        let mut fname = self.default_filename();
        if !cmd.is_empty() {
            fname = cmd;
        }
        let fname = fname;
        Ok(write_file(&fname, &self.content(state)))
    }
}

impl<T> Instr for T
where
    T: InstrWrite,
{
    fn eval(&self, state: &app::State) -> Result<String, String> {
        self.write(state)
    }
}

struct Unknown {
    param: String,
}
impl Instr for Unknown {
    fn eval(&self, _state: &app::State) -> Result<String, String> {
        Err(format!("Unknown command: {}", self.param))
    }
}

use std::process::Command;
struct Jq;
impl Instr for Jq {
    fn eval(&self, state: &app::State) -> Result<String, String> {
        let command = &state.jq().get_content();
        let filename = state.filename.as_str();
        let command = Command::new("jq")
            .arg(command)
            .arg(filename)
            .output()
            .expect("Command execution failed");
        let result = String::from_utf8(command.stdout).expect("Invalid stdout");
        if result.is_empty() {
            return Ok(String::from_utf8(command.stderr).expect("Invalid stderr"));
        }
        Ok(result)
    }
}

struct WriteOut {
    param: String,
}
impl InstrWrite for WriteOut {
    fn param(&self) -> String {
        self.param.as_str().to_string()
    }
    fn default_filename(&self) -> String {
        String::from(OUTFILE_OUT)
    }
    fn content(&self, state: &app::State) -> String {
        state.output.get_content()
    }
}

struct WriteCmd {
    param: String,
}
impl InstrWrite for WriteCmd {
    fn param(&self) -> String {
        self.param.as_str().to_string()
    }
    fn default_filename(&self) -> String {
        String::from(OUTFILE_CMD)
    }
    fn content(&self, state: &app::State) -> String {
        state.jq().get_content()
    }
}
