use std::env;

pub enum Flags {
    Filename(String),
    Stdin,
    Help,
    Version,
}

impl Flags {
    pub fn get() -> Flags {
        let args: Vec<String> = env::args().collect();
        if args.len() < 2 {
            return Flags::Stdin;
        }
        match args[1].as_ref() {
            "-h" => Flags::Help,
            "--help" => Flags::Help,
            "-v" => Flags::Version,
            "--version" => Flags::Version,
            filename => Flags::Filename(String::from(filename)),
        }
    }
}
