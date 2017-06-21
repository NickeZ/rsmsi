extern crate clap;

mod grammar;
mod tmpl_grammar;

mod options;
mod makro;
mod ast;
mod parser;
mod lexer;

use options::Options;
use std::io::Read;
use parser::expand_template;

fn main() {
    let options = Options::parse_args().expect("Failed to parse options");

    let mut input = Vec::new();
    if let Some(template) = options.infile {
        std::fs::File::open(template).unwrap().read_to_end(&mut input).unwrap();
    } else {
        std::io::stdin().read_to_end(&mut input).unwrap();
    };

    let res = expand_template(&String::from_utf8(input).unwrap(), &options.macros);

    print!("{}", res);
    //println!("Sub {:?}", options.subfile);
    //println!("Makros {:?}", options.macros);
    //println!("Includes {:?}", options.includes);
}

