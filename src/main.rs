extern crate clap;

mod options;
mod makro;
mod grammar;
mod ast;

use options::Options;
use makro::MacroSet;
use std::io::Read;

fn main() {
    println!("Hello, world!");

    let options = Options::parse_args().expect("Failed to parse options");

    let mut input = Vec::new();
    if let Some(template) = options.infile {
        std::fs::File::open(template).unwrap().read_to_end(&mut input).unwrap();
    } else {
        std::io::stdin().read_to_end(&mut input).unwrap();
    };

    let res = expand_template(&String::from_utf8(input).unwrap(), &options.macros);

    println!("{}", res);


    //println!("Sub {:?}", options.subfile);
    //println!("Makros {:?}", options.macros);
    //println!("Includes {:?}", options.includes);
}

#[test]
fn grammar() {
    let t = grammar::parse_Expr("${test}");
    println!("{:?}", t);
    assert!(t.unwrap() == Box::new(Expr::List(vec![Box::new(Expr::Makro(vec![Box::new(Expr::Final(String::from("test")))]))])));
    let t = grammar::parse_Expr("${test}${test}");
    println!("{:?}", t);
    let t = grammar::parse_Expr("${${test}}");
    println!("{:?}", t);
    let t = grammar::parse_Expr("${test=sda}");
    println!("{:?}", t);
    let t = grammar::parse_Expr("${test=}");
    println!("{:?}", t);
    let t = grammar::parse_Expr("${tes${TEST}=}");
    println!("{:?}", t);
    let t = grammar::parse_Expr("${t${TA}s${TEST}=}");
    println!("{:?}", t);

    let res = expand_template("${TE${IN}}", vec![("TEST", "APA"), ("IN", "ST")]);
    println!("{:?}", res);
    assert!(res == "APA", "Did not expand to APA");

    let res = expand_template("${TE${IN=ST}}", vec![("TEST", "APA")]);
    println!("{:?}", res);
    assert!(res == "APA", "Did not expand to APA");
}

fn expand_template(template: &str, macros: &MacroSet) -> String {
    let t = grammar::parse_Expr(template).unwrap();
    expand_template_priv(*t, macros)
}

use ast::Expr;

fn expand_template_priv(item: Expr, macros: &MacroSet) -> String {
    match item {
        Expr::Makro(list) => {
            let mut res = String::new();
            for e in list {
                res.push_str(&expand_template_priv(*e, macros));
            }
            if let Some(sub) = macros.get(&res) {
                return sub.value.clone().unwrap();
            }
            String::from("undefined")
        },
        Expr::List(list) => {
            let mut res = String::new();
            for e in list {
                res.push_str(&expand_template_priv(*e, macros));
            }
            res
        },
        Expr::MakroWithDefault(name_list, default_list) => {
            let mut res = String::new();
            for e in name_list {
                res.push_str(&expand_template_priv(*e, macros));
            }
            if let Some(sub) = macros.get(&res) {
                return sub.value.clone().unwrap();
            }
            res.clear();
            for e in default_list {
                res.push_str(&expand_template_priv(*e, macros));
            }
            res
        }
        Expr::Final(s) => {
            s
        },
    }
}
