use std::fs::File;
use std::collections::HashMap;
use std::io::Read;

use makro::MacroSet;
use ast::{TmplExpr, SubsListType, Template};
//use grammar::{parse_TmplExpr, parse_SubsExpr};
use grammar::{parse_SubsExpr};
use tmpl_grammar::{parse_TmplExpr};
use lexer;
use lalrpop_util::{ParseError};

pub fn expand_subs(subs: &str) -> String {
    let mut res = String::new();
    match parse_SubsExpr(subs) {
        Ok(files) => {
            for file in files {
                let pair = *file; // Work around a bug. See issue #16223
                let Template(filename, macros) = pair;
                let mut fh = File::open(filename).unwrap_or_else(|e| die!("Failed to open file: {}", e));
                let mut buf = Vec::new();
                fh.read_to_end(&mut buf).unwrap_or_else(|e| die!("Failed to read from filr: {}", e));
                let template = String::from_utf8(buf).unwrap_or_else(|e| die!("Invalid utf8 in file: {}", e));
                let macros_v = create_hashmap(*macros);
                for mut macros in macros_v {
                    let expanded = expand_template(&template, &mut macros);
                    res.push_str(expanded.as_str());
                }
            }
            res
        },
        Err(e) => {
            match e {
                ParseError::InvalidToken{location} => {
                    println!("Invalid token '{}' at {}", subs.chars().nth(location).unwrap(), location);
                },
                _ => (),
            }
            panic!();
        }
    }
}

fn create_hashmap(expr: SubsListType) -> Vec<MacroSet> {
    match expr {
        SubsListType::RegularList(macro_sets) => {
            let mut res = Vec::new();
            for macros in macro_sets {
                let mut hm = HashMap::new();
                for (k, v) in macros {
                    hm.insert(k, v);
                }
                res.push(hm);
            }
            return res
        },
        SubsListType::PatternList(macros_def, macros_val_sets) => {
            //println!("macros {:?}", macros_def);
            let mut res = Vec::new();
            for macros_val in macros_val_sets {
                let mut hm = HashMap::new();
                for (i, v) in macros_val.into_iter().enumerate() {
                    hm.insert(macros_def[i].clone(), v);
                }
                res.push(hm);
            }
            return res
        },
    }
}

#[cfg(test)]
const SUBS_EXP_TEST_DATA: &'static [(&'static str, &'static str)] = &[
    ("file test/trivial.tmpl {{name=niklas, age=30}}", "My name is niklas\nMy age is 30\n"),
    ("file test/trivial.tmpl {pattern{name, age} {bob, 1} {ben, 2}}", "My name is bob\nMy age is 1\nMy name is ben\nMy age is 2\n"),
    ("file test/trivial_with_default.tmpl {{name=niklas, mak2=val2}}", "My name is niklas\nMy age is none of your business\n"),
    ("file test/trivial.tmpl {{mak1=val1, mak2=val2}{mak1=val3, mak2=val4}}", ""),
    ("file \"test/trivial.tmpl\" { pattern {mak1, mak2} {val1, val2}}", ""),
    ("file \"test/trivial.tmpl\" {{mak1=val1, mak2=val2}}", ""),
    ("file test/trivial.tmpl {pattern{mak1, mak2}{val1, val2}}", ""),
    ("file test/trivial.tmpl {pattern{mak1, mak2}{val1, val2}{val3,val4}}", ""),
    ("file test/trivial.tmpl {{mak1=val1}} file test/test1 {{mak1=val2}}", ""),
];

#[test]
fn test_expand_subs() {
    for &(sub, res) in SUBS_EXP_TEST_DATA {
        println!("test {:?}", sub);
        let s = expand_subs(sub);
        assert!(s == res, format!("'{}' is not '{}'", s, res));
    }
}

pub fn expand_template(template: &str, macros: &mut MacroSet) -> String {
    //let l = lexer::Lexer::new(template);
    //for l in l {
    //    println!("{:?}", l);
    //}
    let l = lexer::Lexer::new(template);
    let t = parse_TmplExpr(l).unwrap();
    let mut res = String::new();
    for t in t {
        //println!("{:?}", t);
        res.push_str(expand_template_priv(*t, macros).as_str());
    }
    res
}

fn expand_template_priv(item: TmplExpr, macros: &mut MacroSet) -> String {
    match item {
        TmplExpr::Makro(list) => {
            let mut res = String::new();
            for e in list {
                res.push_str(&expand_template_priv(*e, macros));
            }
            if let Some(sub) = macros.get(&res) {
                return sub.clone();
            }
            String::from(format!("${{{},undefined}}", res))
        },
        TmplExpr::MakroWithDefault(name_list, default_list) => {
            let mut res = String::new();
            for e in name_list {
                res.push_str(&expand_template_priv(*e, macros));
            }
            if let Some(sub) = macros.get(&res) {
                return sub.clone();
            }
            res.clear();
            for e in default_list {
                res.push_str(&expand_template_priv(*e, macros));
            }
            res
        }
        TmplExpr::Text(s) => {
            s
        },
        TmplExpr::Substitute(v) => {
            for (name_v, value_v) in v {
                let mut name = String::new();
                for item in name_v {
                    name.push_str(&expand_template_priv(*item, macros));
                }
                let mut value = String::new();
                for item in value_v {
                    value.push_str(&expand_template_priv(*item, macros));
                }
                macros.insert(name, value);
            }
            String::from("")
        }
        TmplExpr::Include(file) => {
            String::from(format!("TODO {}", file))
        }
    }
}

#[cfg(test)]
const TMPL_EXP_TEST_DATA: &'static [(&'static [(&'static str, &'static str)], &'static str, &'static str)] = &[
    (&[("name", "niklas"), ("age", "30")], "nothing to expand\t\n", "nothing to expand\t\n"),
    (&[("name", "niklas"), ("age", "30")], "${name} ${age}", "niklas 30"),
    (&[("name", "niklas")], "${name} ${age=20}", "niklas 20"),
    (&[("name1", "niklas"), ("num", "1")], "${name${num}}", "niklas"),
    (&[("name1", "niklas")], "${name${num=1}}", "niklas"),
    (&[("n", "n")], "substitute \"name=niklas\"\n${name}", "niklas"),
    (&[("n", "n")], "substitute \"name=niklas,age=30\"\n${name} ${age}", "niklas 30"),
    (&[("n", "n")], "substitute \"name=\\\"niklas\\\",age=30\"\n${name} ${age}", "\"niklas\" 30"),
    (&[("n", "n")], "substitute \"name=niklas,age=\\\"30\\\"\"\n${name} ${age}", "niklas \"30\""),
    (&[("n", "n")], "substitute \"name=niklas\"\nsubstitute \"name=pelle\"\n${name}", "pelle"),
    (&[("P", "Q")], "${P=${P}}", "Q"),
];

#[test]
fn macro_expansion_test() {
    for entry in TMPL_EXP_TEST_DATA {
        //println!("test {:?}", entry);
        let mut subs = HashMap::new();
        for &(k,v) in entry.0 {
            subs.insert(k.to_string(),v.to_string());
        }
        let res = expand_template(entry.1, &mut subs);
        assert!(res == entry.2, format!("'{}' is not '{}' expanded to '{}'", res, entry.1, entry.2));
    }
}
