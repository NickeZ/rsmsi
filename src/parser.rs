use std::fs::File;
use std::collections::HashMap;
use std::io::Read;

use makro::MacroSet;
use ast::{TmplExpr, SubsListType, Template};
//use grammar::{parse_TmplExpr, parse_SubsExpr};
use grammar::{parse_SubsExpr};
use tmpl_grammar::{parse_TmplExpr};
use lexer;

//pub fn expand_macros(template: &str, macros: MacroSet) -> String {
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
//    unimplemented!()
//}

// Returns how many bytes forward the matching paren is.
//fn find_matching_brace(input: &[u8]) -> Result<usize, ()> {
//    find_matching(input, b'{', b'}')
//}
//
//fn find_matching_paren(input: &[u8]) -> Result<usize, ()> {
//    find_matching(input, b'(', b')')
//}
//
//fn find_matching(input: &[u8], open: u8, close: u8) -> Result<usize, ()>
//{
//    input.iter()
//        .scan((0, 1usize), |state, &c| { // state = (chars, paren_counter)
//            state.0 = state.0 + 1;
//            if c == open {
//                state.1 = state.1 + 1;
//            }
//            if c == close {
//                state.1 = state.1 - 1;
//            }
//            Some(*state)
//        })
//        .take_while(|state| {
//            state.0 != 0
//        })
//        .last()
//        .map(|state| state.0 - 1)
//        .ok_or(())
//}

pub fn expand_subs(subs: &str) -> String {
    let mut res = String::new();
    for file in parse_SubsExpr(subs).unwrap() {
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
            println!("macros {:?}", macros_def);
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

#[test]
fn test_find_matching() {
    assert!(find_matching_brace(b" }") == Ok(1));
    assert!(find_matching_brace(b"  }") == Ok(2));
    assert!(find_matching_brace(b" ${ } ${ ${ } } }") == Ok(16));
    assert!(find_matching_paren(b" )") == Ok(1));
    assert!(find_matching_paren(b"  )") == Ok(2));
    assert!(find_matching_paren(b" $( ) $( $( ) ) )") == Ok(16));
}

#[test]
fn test_expand_subs() {
    let s = expand_subs("file test/test1 {{name=bob, mak2=val2}}");
    println!("{:?}", s);
    let s = expand_subs("file test/test1 {pattern{name, mak2} {bob, 1} {ben, 2}}");
    println!("{:?}", s);
    //let s = expand_subs("file test/test1 {pattern{name, mak2} {bob, ${test}} {ben, 2}}");
    //println!("{:?}", s);
}

#[test]
fn test_subs() {
    let s = parse_SubsExpr("file test/test1 {{mak1=val1, mak2=val2}}");
    //println!("{:?}", s);
    let s = parse_SubsExpr("file test/test1 {{mak1=val1, mak2=val2}{mak1=val3, mak2=val4}}");
    //println!("{:?}", s);
    let s = parse_SubsExpr("file \"test/test1\" { pattern {mak1, mak2} {val1, val2}}");
    //println!("{:?}", s);
    let s = parse_SubsExpr("file \"test/test1\" {{mak1=val1, mak2=val2}}");
    //println!("{:?}", s);
    let s = parse_SubsExpr("file test/test1 {pattern{mak1, mak2}{val1, val2}}");
    //println!("{:?}", s);
    let s = parse_SubsExpr("file test/test1 {pattern{mak1, mak2}{val1, val2}{val3,val4}}");
    //println!("{:?}", s);

    let s = parse_SubsExpr("file test/test1 {{mak1=val1}} file test/test1 {{mak1=val2}}");
    //println!("{:?}", s);
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

    let mut subs = HashMap::new();
    subs.extend(vec![(String::from("P"), String::from("Q"))].into_iter());
    let res = expand_template("${P=${P}}", &subs);
    println!("{:?}", res);
    assert!(res == "Q", "Did not expand to Q");
}
