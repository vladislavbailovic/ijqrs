use super::{app, write_file};
use super::super::ui::Pane;

pub enum Instruction {
    Unknown,
    WriteOut,
    WriteCmd,
    // YankOut,
    // YankCmd,
}

pub fn new(inst: Instruction, param: String) -> Box<dyn Instr> {
    match inst {
        Instruction::WriteOut => Box::new(WriteOut{ param }),
        Instruction::WriteCmd => Box::new(WriteCmd{ param }),
        Instruction::Unknown => Box::new(Unknown{ param }),
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

impl<T> Instr for T where T: InstrWrite {
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

struct WriteOut {
    param: String,
}
impl InstrWrite for WriteOut {
    fn param(&self) -> String {
        self.param.as_str().to_string()
    }
    fn default_filename(&self) -> String {
        String::from("ijqrs.out")
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
        String::from("ijqrs.cmd")
    }
    fn content(&self, state: &app::State) -> String {
        state.jq().get_content()
    }
}
