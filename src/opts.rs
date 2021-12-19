use std::env;

pub enum Flags {
    Filename(String),
    Help
}

impl Flags {
    pub fn get() -> Flags {
        let args: Vec<String> = env::args().collect();
        if args.len() < 2 {
            return Flags::Help;
        }
        match args[1].as_ref() {
            "-h" => Flags::Help,
            filename => Flags::Filename(String::from(filename))
        }
    }
}
