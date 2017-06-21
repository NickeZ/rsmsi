use std::str::CharIndices;
use std::iter::Peekable;

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

const SPECIAL_CHARS: &str = "\n\"=, \tis${}()";
const KEYWORDS: &[&str] = &["include", "substitute"];

#[derive(Debug)]
pub enum Tok {
    Text,
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
    fn new(input: &'input str) -> Self {
        Lexer { chars: input.char_indices() }
    }

    fn keyword_match(&mut self, keyword: &str, token: Tok, i:usize) -> Option<Spanned<Tok, usize, LexicalError>> {
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
    type Item = Spanned<Tok, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut collect = None;
        loop {
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
                                        collect = Some(i);
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
                                            collect = Some(i);
                                        }
                                    }
                                },
                                's' => {
                                    if let Some(res) = self.keyword_match(KEYWORDS[1], Tok::CommandSubstitute, i) {
                                        return Some(res);
                                    } else {
                                        if collect.is_none() {
                                            collect = Some(i);
                                        }
                                    }
                                },
                                _ => {
                                    if collect.is_none() {
                                        collect = Some(i);
                                    }
                                    let mut ahead = self.chars.as_str().chars();
                                    let invalid = SPECIAL_CHARS.chars();
                                    match ahead.next() {
                                        Some(c) => {
                                            for g in invalid {
                                                if c == g {
                                                    return Some(Ok((collect.unwrap(), Tok::Text, i+1)));
                                                }
                                            }
                                        },
                                        None => continue,
                                    }
                                }
                            }
                        },
                    }
                },
                None => return None,
            }
        }
    }
}

#[test]
fn test_lexer() {
    let stim = "my test $(hej) ${hej} ${he${hej}} $ test $test include inc substitute s";
    let lex = Lexer::new(stim);
    println!("{}", stim);
    for l in lex {
        println!("{:?} ", l);
    }
    let stim = "include \"file.tmp\"";
    let lex = Lexer::new(stim);
    println!("{}", stim);
    for l in lex {
        println!("{:?} ", l);
    }
    let stim = "substitute \"mak1=val1, mak2=val2\"";
    let lex = Lexer::new(stim);
    println!("{}", stim);
    for l in lex {
        println!("{:?} ", l);
    }
    let stim = "substitute \t\"mak1=val1,\tmak2=val2\"";
    let lex = Lexer::new(stim);
    println!("{}", stim);
    for l in lex {
        println!("{:?} ", l);
    }
}
