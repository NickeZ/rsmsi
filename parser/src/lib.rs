#[macro_use]
extern crate nom;

pub mod parser;

#[derive(Debug)]
struct A<'a> {makro: &'a [u8]}

named!(brace_makro (&[u8]) -> A,
    do_parse!(
        tag!("${") >>
        makro: take_until!("}") >>
        tag!("}") >>

        (A{makro: makro})
    )
);

named!(paren_makro (&[u8]) -> A,
    do_parse!(
        tag!("$(") >>
        makro: take_until!(")") >>
        tag!(")") >>

        (A{makro: makro})
    )
);

#[cfg(test)]
mod tests {
    use parser;
    use nom;
    #[test]
    fn it_works() {
        match ::brace_makro(&b"${TEST1}"[..]) {
            nom::IResult::Done(_, res) => println!("{}", String::from_utf8(res.makro.to_vec()).unwrap()),
            _ => (),
        }
        match ::paren_makro(&b"$(TEST2)"[..]) {
            nom::IResult::Done(_, res) => println!("{}", String::from_utf8(res.makro.to_vec()).unwrap()),
            _ => (),
        }
    }
}
