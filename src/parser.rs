use std::fs::File;
use std::collections::HashMap;
use std::io::Read;

use makro::MacroSet;
use ast::{TmplExpr, SubsExpr, Template};
//use grammar::{parse_TmplExpr, parse_SubsExpr};
use grammar::{parse_SubsExpr};
use tmpl_grammar::{parse_TmplExpr};
use lexer;

pub fn expand_macros(template: &str, macros: MacroSet) -> String {
    //template.bytes().scan((0, Vec::new()), |state, c| {
    //    let (counter, list) = *state;
    //    if c == b'$' {
    //        if template.as_bytes()[state.0+1] == b'{' {
    //            let res = find_matching_brace(&template.as_bytes()[state.0+2..]);
    //            state.1.push((state.0.clone(), res));
    //        }
    //    }
    //    state.0 = state.0 + 1;
    //    *state = *state;
    //    Some(*state)
    //});
    unimplemented!()
}

// Returns how many bytes forward the matching paren is.
fn find_matching_brace(input: &[u8]) -> Result<usize, ()> {
    find_matching(input, b'{', b'}')
}

fn find_matching_paren(input: &[u8]) -> Result<usize, ()> {
    find_matching(input, b'(', b')')
}

fn find_matching(input: &[u8], open: u8, close: u8) -> Result<usize, ()>
{
    input.iter()
        .scan((0, 1usize), |state, &c| { // state = (chars, paren_counter)
            state.0 = state.0 + 1;
            if c == open {
                state.1 = state.1 + 1;
            }
            if c == close {
                state.1 = state.1 - 1;
            }
            Some(*state)
        })
        .take_while(|state| {
            state.0 != 0
        })
        .last()
        .map(|state| state.0 - 1)
        .ok_or(())
}

pub fn expand_subs(subs: &str) -> String {
    let mut res = String::new();
    for file in parse_subs(subs) {
        let pair = *file; // Work around a bug. See issue #16223
        let Template(filename, macros) = pair;
        let mut fh = File::open(filename).unwrap();
        let mut buf = Vec::new();
        fh.read_to_end(&mut buf).unwrap();
        res.push_str(&String::from_utf8(buf).unwrap());
    }
    res
}

#[test]
fn test_find_matching() {
    assert!(find_matching_brace(b" }") == Ok(1));
    assert!(find_matching_brace(b"  }") == Ok(2));
    assert!(find_matching_brace(b" ${ } ${ ${ } } }") == Ok(16));
    assert!(find_matching_paren(b" )") == Ok(1));
    assert!(find_matching_paren(b"  )") == Ok(2));
    assert!(find_matching_paren(b" $( ) $( $( ) ) )") == Ok(16));
}

fn parse_subs(subs: &str) -> Vec<Box<Template>> {
    parse_SubsExpr(subs).unwrap()
}

#[test]
fn test_expand_subs() {
    let s = expand_subs("file test {{firstname=bob, mak2=val2}}");
    //println!("{:?}", s);
}

#[test]
fn test_subs() {
    let s = parse_subs("file test {{mak1=val1, mak2=val2}}");
    //println!("{:?}", s);
    let s = parse_subs("file test {{mak1=val1, mak2=val2}{mak1=val3, mak2=val4}}");
    //println!("{:?}", s);
    let s = parse_subs("file \"test\" { pattern {mak1, mak2} {val1, val2}}");
    //println!("{:?}", s);
    let s = parse_subs("file \"test\" {{mak1=val1, mak2=val2}}");
    //println!("{:?}", s);
    let s = parse_subs("file test {pattern{mak1, mak2}{val1, val2}}");
    //println!("{:?}", s);
    let s = parse_subs("file test {pattern{mak1, mak2}{val1, val2}{val3,val4}}");
    //println!("{:?}", s);

    let s = parse_subs("file test {{mak1=val1}} file test {{mak1=val2}}");
    //println!("{:?}", s);
}

pub fn expand_template(template: &str, macros: &MacroSet) -> String {
    //let l = lexer::Lexer::new(template);
    //for l in l {
    //    println!("{:?}", l);
    //}
    let l = lexer::Lexer::new(template);
    let t = parse_TmplExpr(l).unwrap();
    let mut res = String::new();
    for t in t {
        res.push_str(expand_template_priv(*t, macros).as_str());
    }
    res
}

fn expand_template_priv(item: TmplExpr, macros: &MacroSet) -> String {
    match item {
        TmplExpr::Makro(list) => {
            let mut res = String::new();
            for e in list {
                res.push_str(&expand_template_priv(*e, macros));
            }
            if let Some(sub) = macros.get(&res) {
                return sub.clone();
            }
            String::from("undefined")
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
            String::from("")
        }
        TmplExpr::Include(file) => {
            String::from("")
        }
    }
}

#[test]
fn macro_expansion_test() {
    //let t = parse_TmplExpr("${test}");
    //println!("{:?}", t);
    //let t = parse_TmplExpr("substitute \"test=${te}st, test${2}=val\"");
    //println!("{:?}", t);
    ////assert!(t.unwrap() == Box::new(Expr::List(vec![Box::new(Expr::Makro(vec![Box::new(Expr::Final(String::from("test")))]))])));
    //let t = parse_TmplExpr("${t e s t }");
    //println!("{:?}", t);
    //let t = parse_TmplExpr("${test}${test}");
    //println!("{:?}", t);
    //let t = parse_TmplExpr("${${test}}");
    //println!("{:?}", t);
    //let t = parse_TmplExpr("${test=sda}");
    //println!("{:?}", t);
    //let t = parse_TmplExpr("${test=}");
    //println!("{:?}", t);
    //let t = parse_TmplExpr("${tes${TEST}=}");
    //println!("{:?}", t);
    //let t = parse_TmplExpr("${t${TA}s${TEST}=}");
    //println!("{:?}", t);

    let mut subs = HashMap::new();
    subs.extend(vec![(String::from("TEST"), String::from("APA")), (String::from("IN"), String::from("ST"))].into_iter());
    let res = expand_template("${TE${IN}}", &subs);
    println!("{:?}", res);
    assert!(res == "APA", "Did not expand to APA");

    let mut subs = HashMap::new();
    subs.extend(vec![(String::from("TEST"), String::from("APA"))].into_iter());
    let res = expand_template("${TE${IN=ST}}", &subs);
    println!("{:?}", res);
    assert!(res == "APA", "Did not expand to APA");

    let mut subs = HashMap::new();
    subs.extend(vec![(String::from("TEST"), String::from("APA"))].into_iter());
    let res = expand_template("detta ar en random text", &subs);
    println!("{:?}", res);
    assert!(res == "detta ar en random text", "Did not expand");

    let mut subs = HashMap::new();
    subs.extend(vec![(String::from("TEST"), String::from("APA"))].into_iter());
    let res = expand_template("substitute \"hej=da\"hej ${TE${IN=ST}}", &subs);
    println!("{:?}", res);
    assert!(res == "hej APA", "Did not expand to APA");
}
