use std::str::CharIndices;
use std::iter::Peekable;

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

const SPECIAL_CHARS: &str = "\n\"=, \t${}()";
const KEYWORDS: &[&str] = &["include", "substitute", "${", "$("];

#[derive(Debug, PartialEq)]
pub enum Tok<'input> {
    Text(&'input str),
    Newline,
    Quote,
    Equals,
    Comma,
    Space,
    Tab,
    CommandInclude,
    CommandSubstitute,
    MacroBrBegin,
    MacroBrEnd,
    MacroPaBegin,
    MacroPaEnd,
}

#[derive(Debug)]
pub enum LexicalError {
    // not possible
}

#[derive(Debug)]
pub struct Lexer<'input> {
    chars: CharIndices<'input>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer { chars: input.char_indices() }
    }

    /// Returns true if any keyword is found
    fn lookahead_keywords(&mut self) -> bool {
        for keyword in KEYWORDS {
            let ahead = self.chars.as_str().chars();
            let mut kc = keyword.chars();
            for ac in ahead {
                let kc = kc.next();
                match kc {
                    Some(kc) => {
                        if kc != ac {
                            break;
                        }
                    },
                    None => return true,
                }
            }
        }
        false
    }
    fn keyword_match(&mut self, keyword: &str, token: Tok<'input>, i:usize) -> Option<Spanned<Tok<'input>, usize, LexicalError>> {
        let ahead = self.chars.as_str().chars();
        let mut match_str = keyword[1..].chars();
        for c in ahead {
            let g = match_str.next();
            match g {
                Some(g) => {
                    if c != g {
                        return None
                    }
                },
                None => {
                    self.chars.nth(keyword.len() - 2);
                    return Some(Ok((i, token, i + keyword.len())))
                },
            }
        }
        None
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Tok<'input>, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut collect = None;
        loop {
            let current = self.chars.as_str();
            match self.chars.next() {
                Some((i, c)) => {
                    match c {
                        '$' => {
                            let ahead = self.chars.as_str();
                            match ahead.chars().next() {
                                Some('{') => {
                                    self.chars.next();
                                    return Some(Ok((i, Tok::MacroBrBegin, i+2)))
                                },
                                Some('(') => {
                                    self.chars.next();
                                    return Some(Ok((i, Tok::MacroPaBegin, i+2)))
                                },
                                Some(_) | None => {
                                    if collect.is_none() {
                                        collect = Some((i, current));
                                    }
                                    continue
                                },
                            }
                        },
                        '}' => return Some(Ok((i, Tok::MacroBrEnd, i+1))),
                        ')' => return Some(Ok((i, Tok::MacroPaEnd, i+1))),
                        '\n' => return Some(Ok((i, Tok::Newline, i+1))),
                        '"' => return Some(Ok((i, Tok::Quote, i+1))),
                        '=' => return Some(Ok((i, Tok::Equals, i+1))),
                        ',' => return Some(Ok((i, Tok::Comma, i+1))),
                        ' ' => return Some(Ok((i, Tok::Space, i+1))),
                        '\t' => return Some(Ok((i, Tok::Tab, i+1))),
                        c => {
                            //println!("text {} {:?}", c, first_text);
                            match c {
                                'i' => {
                                    if let Some(res) = self.keyword_match(KEYWORDS[0], Tok::CommandInclude, i) {
                                        return Some(res);
                                    } else {
                                        if collect.is_none() {
                                            collect = Some((i, current));
                                        }
                                    }
                                },
                                's' => {
                                    if let Some(res) = self.keyword_match(KEYWORDS[1], Tok::CommandSubstitute, i) {
                                        return Some(res);
                                    } else {
                                        if collect.is_none() {
                                            collect = Some((i, current));
                                        }
                                    }
                                },
                                _ => {
                                    if collect.is_none() {
                                        collect = Some((i, current));
                                    }
                                    let mut ahead = self.chars.as_str().chars();
                                    let invalid = SPECIAL_CHARS.chars();
                                    match ahead.next() {
                                        Some(c) => {
                                            for g in invalid {
                                                if c == g {
                                                    let (idx, chars_start) = collect.unwrap();
                                                    println!("1 {}", i+1-idx);
                                                    return Some(Ok((idx, Tok::Text(&chars_start[..i+1-idx]), i+1)));
                                                }
                                            }
                                        },
                                        None => {},
                                    }
                                    if self.lookahead_keywords() {
                                        let (idx, chars_start) = collect.unwrap();
                                        println!("2 {}", i+1-idx);
                                        return Some(Ok((idx, Tok::Text(&chars_start[..i+1]), i+1-idx)));
                                    }
                                }
                            }
                        },
                    }
                },
                None => {
                    if let Some((idx, chars_start)) = collect {
                        println!("3 {}", 1);
                        return Some(Ok((idx, Tok::Text(&chars_start[..]), idx + chars_start.len())))
                    }
                    return None
                },
            }
        }
    }
}

#[test]
fn test_lexer() {
    //let stim = "my test $(hej) ${hej} ${he${hej}} $ test $test include inc substitute s";
    //let lex = Lexer::new(stim);
    //println!("{}", stim);
    //for l in lex {
    //    println!("{:?} ", l);
    //}
    let stim = "include \"file.tmp\"";
    let mut lex = Lexer::new(stim);
    //assert!(lex.next().unwrap().unwrap() == ((0, Tok::CommandInclude, 7)));
    //assert!(lex.next().unwrap().unwrap() == ((7, Tok::Space, 8)));
    //assert!(lex.next().unwrap().unwrap() == ((8, Tok::Quote, 9)));
    //assert!(lex.next().unwrap().unwrap() == ((9, Tok::Text, 17)));
    //assert!(lex.next().unwrap().unwrap() == ((17, Tok::Quote, 18)));
    //assert!(lex.next().is_none());
    println!("{}", stim);
    for l in lex {
        println!("{:?} ", l);
    }
    let stim = "substitute \"mak1=val1, mak2=val2\"";
    let mut lex = Lexer::new(stim);
    //assert!(lex.next().unwrap().unwrap() == ((0, Tok::CommandSubstitute, 10)));
    //assert!(lex.next().unwrap().unwrap() == ((10, Tok::Space, 11)));
    //assert!(lex.next().unwrap().unwrap() == ((11, Tok::Quote, 12)));
    //assert!(lex.next().unwrap().unwrap() == ((12, Tok::Text, 16)));
    //assert!(lex.next().unwrap().unwrap() == ((16, Tok::Equals, 17)));
    //assert!(lex.next().unwrap().unwrap() == ((17, Tok::Text, 21)));
    //assert!(lex.next().unwrap().unwrap() == ((21, Tok::Comma, 22)));
    //assert!(lex.next().unwrap().unwrap() == ((22, Tok::Space, 23)));
    //assert!(lex.next().unwrap().unwrap() == ((23, Tok::Text, 27)));
    //assert!(lex.next().unwrap().unwrap() == ((27, Tok::Equals, 28)));
    //assert!(lex.next().unwrap().unwrap() == ((28, Tok::Text, 32)));
    //assert!(lex.next().unwrap().unwrap() == ((32, Tok::Quote, 33)));
    //assert!(lex.next().is_none());
    println!("{}", stim);
    for l in lex {
        println!("{:?} ", l);
    }
    let stim = "substitute \t\"mak1=val1,\tmak2=val2\"";
    let mut lex = Lexer::new(stim);
    //assert!(lex.next().unwrap().unwrap() == ((0, Tok::CommandSubstitute, 10)));
    //assert!(lex.next().unwrap().unwrap() == ((10, Tok::Space, 11)));
    //assert!(lex.next().unwrap().unwrap() == ((11, Tok::Tab, 12)));
    //assert!(lex.next().unwrap().unwrap() == ((12, Tok::Quote, 13)));
    //assert!(lex.next().unwrap().unwrap() == ((13, Tok::Text, 17)));
    //assert!(lex.next().unwrap().unwrap() == ((17, Tok::Equals, 18)));
    //assert!(lex.next().unwrap().unwrap() == ((18, Tok::Text, 22)));
    //assert!(lex.next().unwrap().unwrap() == ((22, Tok::Comma, 23)));
    //assert!(lex.next().unwrap().unwrap() == ((23, Tok::Tab, 24)));
    //assert!(lex.next().unwrap().unwrap() == ((24, Tok::Text, 28)));
    //assert!(lex.next().unwrap().unwrap() == ((28, Tok::Equals, 29)));
    //assert!(lex.next().unwrap().unwrap() == ((29, Tok::Text, 33)));
    //assert!(lex.next().unwrap().unwrap() == ((33, Tok::Quote, 34)));
    //assert!(lex.next().is_none());
    println!("{}", stim);
    for l in lex {
        println!("{:?} ", l);
    }
    //let stim = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
    //let lex = Lexer::new(stim);
    //for l in lex {
    //    println!("{:?} ", l);
    //}
}
