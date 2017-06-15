extern crate clap;

mod options;
mod makro;
mod grammar;
mod ast;

use options::Options;

fn main() {
    println!("Hello, world!");

    let options = Options::parse_args().expect("Failed to parse options");


    println!("Sub {:?}", options.subfile);
    println!("Makros {:?}", options.macros);
    println!("Includes {:?}", options.includes);
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

fn expand_template(template: &str, macros: Vec<(&str, &str)>) -> String {
    let t = grammar::parse_Expr(template).unwrap();
    println!("{:?}", t);
    expand_template_priv(*t, &macros)
}

use ast::Expr;

fn expand_template_priv(item: Expr, macros: &Vec<(&str, &str)>) -> String {
    match item {
        Expr::Makro(list) => {
            let mut res = String::new();
            for e in list {
                res.push_str(&expand_template_priv(*e, macros));
            }
            for s in macros {
                if res.as_str() == s.0 {
                    return res.replace(s.0, s.1);
                }
            }
            res
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
            for s in macros {
                if res.as_str() == s.0 {
                    return res.replace(s.0, s.1);
                }
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
