use std::path::PathBuf;

use clap::{Arg, App};

use makro::{Macro, parse_macros};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub struct Options {
    // Increase warning of undefined to error
    pub werr: bool,

    // Write the output to this file
    pub outfile: Option<PathBuf>,

    // Include from the following paths
    pub includes: Vec<PathBuf>,

    // Extra macros
    pub macros: Vec<Macro<'static>>,

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
            let macrosv = macrosv.map(String::from);
            macrosv.map(|m| options.macros.append(&mut parse_macros(m).unwrap())).collect::<Vec<()>>();
        }
        if let Some(subfile) = matches.value_of("subfile") {
            options.subfile = Some(PathBuf::from(subfile));
        }

        options
    }
}
