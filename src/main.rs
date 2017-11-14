extern crate lalrpop_util;
extern crate clap;

macro_rules! die {
    ($($arg:tt)*) => {{
        eprintln!($($arg)*);
        ::std::process::exit(1);
    }}
}

mod grammar;
mod tmpl_grammar;

mod options;
mod makro;
mod ast;
mod parser;
mod lexer;

use options::Options;
use std::io::{Read, Write};
use parser::{expand_template, expand_subs};
use std::fs::OpenOptions;

enum InputType {
    Template,
    Substitutions,
}

fn main() {
    let options = Options::parse_args().unwrap_or_else(|_| die!("Failed to parse options"));

    let mut input = Vec::new();
    let input_type;
    if let Some(template) = options.infile {
        input_type = InputType::Template;
        let mut infile = std::fs::File::open(template).unwrap_or_else(|_| die!("Failed to open template file"));
        infile.read_to_end(&mut input).unwrap_or_else(|_| die!("Failed to read from template file"));
    } else if let Some(subfile) = options.subfile {
        input_type = InputType::Substitutions;
        let mut infile = std::fs::File::open(subfile).unwrap_or_else(|_| die!("Failed to open substitutions file"));
        infile.read_to_end(&mut input).unwrap_or_else(|_| die!("Failed to read from substitutions file"));
    } else {
        input_type = InputType::Template;
        std::io::stdin().read_to_end(&mut input).unwrap_or_else(|_| die!("Failed to read from stdin"));
    };

    let input_as_utf8 = String::from_utf8(input).unwrap_or_else(|_| die!("Not valid utf8 in input"));

    let mut macros = options.macros.clone();

    let res = match input_type {
        InputType::Template => expand_template(&input_as_utf8, &mut macros),
        InputType::Substitutions => expand_subs(&input_as_utf8),
    };

    if let Some(outfile) = options.outfile {
        let mut out = OpenOptions::new()
            .write(true)
            .create(true)
            .open(outfile)
            .unwrap_or_else(|_| die!("Failed to open outfile for writing"));
        out.write(res.as_bytes())
            .unwrap_or_else(|_| die!("Failed to write to outfile"));
    } else {
        print!("{}", res);
    }
    //println!("Sub {:?}", options.subfile);
    //println!("Makros {:?}", options.macros);
    //println!("Includes {:?}", options.includes);
}

