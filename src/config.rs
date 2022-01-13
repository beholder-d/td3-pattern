use std::error::Error;
use std::fmt::Debug;
use std::str::FromStr;

pub enum Mode {
    Download,
    Upload,
    // List,
}

impl FromStr for Mode {
    type Err = ();
    fn from_str(input: &str) -> Result<Mode, Self::Err> {
        match input {
            "upload" => Ok(Mode::Upload),
            "download" => Ok(Mode::Download),
            _ => Err(()),
        }
    }
}

impl Debug for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Mode::Download => write!(f, "Download"),
            Mode::Upload => write!(f, "Upload"),
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub in_port: String,
    pub out_port: String,
    pub mode: Mode,
    pub filename: String,
    pub group: u8,
    pub pnum: u8,
    pub ab: u8,
}

pub const DEFAULT_PORTNAME: &'static str = "TD-3";
const FILE: &'static str = "file";
const IN_PORT: &'static str = "in_port";
const OUT_PORT: &'static str = "out_port";

pub fn get_config() -> Result<Config, Box<dyn Error>> {
    let mut config = Config {
        in_port: DEFAULT_PORTNAME.to_owned(),
        out_port: DEFAULT_PORTNAME.to_owned(),
        mode: Mode::Download,
        filename: String::from(""),
        group: 0,
        pnum: 0,
        ab: 0,
    };
    let (args, argv) = argmap::parse(std::env::args());

    // Main args -- [verb] grpoup
    if args.len() < 2 {
        return Err("Incorrect number of arguments".into());
    }
    let mut iter = args.iter();
    let _ = iter.next();
    let mut arg = iter.next().unwrap();
    match Mode::from_str(arg) {
        Ok(Mode::Upload) => {
            config.mode = Mode::Upload;
            if args.len() < 4 {
                return Err("Invalid number of program arguments for upload".into());
            }
            if !argv.contains_key(FILE) || argv.get(FILE).unwrap().first().unwrap() == "" {
                return Err("For upload -file=\"filename\" should be specified".into());
            }
            arg = iter.next().unwrap();
        }
        _ => {
            if args.len() < 3 {
                return Err("Invalid number of program arguments for download/print".into());
            }
        }
    }
    // group
    config.group = match arg.parse::<u8>() {
        Ok(group) => match group {
            1..=4 => group - 1,
            _ => return Err("Group should be from 1 to 4".into()),
        },
        _ => return Err(format!("Group {:?} is invalid", arg.parse::<u8>()).into()),
    };
    // pattern number|ab
    let arg = iter.next().unwrap();
    if arg.len() != 2 {
        return Err("Pattern should consist of number from 1 to 8 and letter A or B".into());
    }
    config.pnum = match arg[0..1].parse::<u8>() {
        Ok(pattern) => match pattern {
            1..=8 => pattern - 1,
            _ => return Err("Pattern should start with number from 1 to 8".into()),
        },
        _ => return Err("Pattern should start with number".into()),
    };
    config.ab = match &arg[1..2] {
        "A" | "a" => 0,
        "B" | "b" => 1,
        _ => return Err("Pattern should end with letter A or B".into()),
    };
    // -filename
    if argv.contains_key(FILE) {
        config.filename = argv.get(FILE).unwrap().first().unwrap().to_string();
    }
    // -in_port
    if argv.contains_key(IN_PORT) && argv.get(IN_PORT).unwrap().first().unwrap() != "" {
        config.in_port = argv.get(IN_PORT).unwrap().first().unwrap().to_string();
    }
    // -out_port
    if argv.contains_key(OUT_PORT) && argv.get(OUT_PORT).unwrap().first().unwrap() != "" {
        config.out_port = argv.get(OUT_PORT).unwrap().first().unwrap().to_string();
    }
    if cfg!(debug_assertions) {
        println!("|| config={:?}", config);
    }
    Ok(config)
}
