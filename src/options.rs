use std::str::{FromStr};
use std::path::PathBuf;

use clap::{Arg, App};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub struct Options {
    // Increase warning of undefined to error
    pub werr: bool,

    // Write the output to this file
    pub outfile: Option<PathBuf>,

    // Include from the following paths
    pub includes: Vec<PathBuf>,

    // Extra macros
    pub macros: Vec<Macro>,

    // Substitutions file
    pub subfile: Option<PathBuf>,

    // Template file
    pub infile: Option<PathBuf>,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            werr: false,
            outfile: None,
            includes: Vec::new(),
            macros: Vec::new(),
            subfile: None,
            infile: None,
        }
    }
}

impl Options {
    pub fn parse_args() -> Options {
        let mut options = Options::default();
        let matches = App::new("rsmsi")
            .version(VERSION)
            .author("Niklas Claesson <nicke.claesson@gmail.com>")
            .about("Macro Substitution and Include Tool")
            .arg(Arg::with_name("werr")
                .short("V")
                .help("Undefined macros is considered an error"))
            .arg(Arg::with_name("outfile")
                .long("outfile")
                .short("o")
                .takes_value(true)
                .help("Output will be written to this file instead of stdout"))
            .arg(Arg::with_name("includes")
                .short("I")
                .multiple(true)
                .takes_value(true)
                .help("Search path for the include statement"))
            .arg(Arg::with_name("macros")
                .short("M")
                .multiple(true)
                .takes_value(true)
                .help("Macro values. Multiple macro values can be specified in one argument or using multiple instances of `-M`"))
            .arg(Arg::with_name("subfile")
                .short("S")
                .takes_value(true)
                .help("The substitutions file"))
            .arg(Arg::with_name("template")
                .help("The input file")
                .takes_value(true))
            .get_matches();

        options.werr = matches.is_present("werr");

        if let Some(outfile) = matches.value_of("outfile") {
            options.outfile = Some(PathBuf::from(outfile));
        }
        if let Some(includev) = matches.values_of("include") {
            includev.map(|b| options.includes.push(PathBuf::from(b))).collect::<Vec<()>>();
        }
        if let Some(macrosv) = matches.values_of("macros") {
            macrosv.map(|m| options.macros.append(&mut parse_macros(m).unwrap())).collect::<Vec<()>>();
        }
        if let Some(subfile) = matches.value_of("subfile") {
            options.subfile = Some(PathBuf::from(subfile));
        }

        options
    }
}

#[derive(Debug)]
pub enum Error {
    ParseError(String),
}

#[derive(Debug)]
pub struct Macro {
    name: String,
    default: Option<String>,
    value: Option<String>,
}

fn parse_macros(input: &str) -> Result<Vec<Macro>, Error> {
    let result = input.split(',').map(|m| {
        parse_macro(m).unwrap_or_else(|e| panic!("Error: {:?}", e))
    }).filter(|m| m.is_some()).map(|m| m.unwrap()).collect();
    Ok(result)
}

#[test]
fn test_parse_macro_ws() {
    for m in &[
        "macro=value",
        " macro=value",
        "macro=value ",
        "macro =value",
        "macro= value",
        "macro = value",
        " macro = value ",
        "macro\t=\tvalue",
    ] {
        let m = parse_macro(m).unwrap().unwrap();
        let (name, value) = (m.name, m.value.unwrap());
        assert!(name == "macro", format!("name was {:?}", name));
        assert!(value == "value", format!("value was {:?}", value));
    }
}

#[test]
fn test_parse_macro_empty_string() {
    for m in &[
        "macro=",
        " macro= ",
    ] {
        let m = parse_macro(m).unwrap().unwrap();
        let (name, value) = (m.name, m.value.unwrap());
        assert!(name == "macro", format!("name was {:?}", name));
        assert!(value == "", format!("value was {:?}", value));
    }
}

#[test]
fn test_parse_macro_none() {
    for m in &[
        "macro",
        " macro ",
    ] {
        let m = parse_macro(m).unwrap().unwrap();
        let (name, value) = (m.name, m.value);
        assert!(name == "macro", format!("name was {:?}", name));
        assert!(value == None, format!("value was {:?}", value));
    }
}

fn parse_macro(input: &str) -> Result<Option<Macro>, Error> {
    let input = input.trim();
    if input.len() == 0 {
        return Ok(None);
    }
    match input.find("=") {
        Some(idx) if idx == 0 => Err(Error::ParseError(String::from(input))),
        Some(idx) => {
            let (name, value) = input.split_at(idx);
            let (name, value) = (name.trim(), String::from(value[1..].trim()));
            Ok(Some(Macro {name: String::from(name), default: None, value: Some(value)}))
        },
        None => Ok(Some(Macro {name: String::from(input), default: None, value: None})),
    }
}
