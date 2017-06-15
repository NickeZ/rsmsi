#[macro_use]
extern crate nom;

use nom::{alphanumeric, IResult, space, is_space};
use nom::IResult::*;
use std::str;
use std::ops::{Range,RangeFrom,RangeTo};

#[derive(PartialEq, Debug, Clone)]
pub enum Expr {
    // Macro name without any value
    Makro(String),

    // Macro name with default value
    MakroWithDef(String, String),

    // Name, Value
    MakroSub(String, String),
}

//#[derive(Debug)]
//struct A<'a> {makro: &'a [u8]}

//named!(makro_string<&str>,
//    map_res!(
//        escaped!(is_not!("\\"), '\\', is_a!("\"n\\")),
//        str::from_utf8
//    )
//);

named!(makro_name<&str>,
    alt_complete!(
        unquoted_name |
        quoted_name
    )
);

named!(makro_value<&str>,
    alt_complete!(
        unquoted_value |
        quoted_value
    )
);

named!(quoted_name<&str>,
  delimited!(
    tag!("\""),
    map_res!(
        escaped!(is_not!("\\\""), '\\', one_of!("\"n\\")),
        str::from_utf8
    ),
    tag!("\"")
  )
);

named!(quoted_value<&str>,
  delimited!(
    tag!("\""),
    map_res!(
        escaped!(is_not!("\\\""), '\\', one_of!("\"n\\")),
        str::from_utf8
    ),
    tag!("\"")
  )
);

named!(unquoted_name<&str>,
    alt_complete!(
        map_res!(is_not!("=\", \t$}"), str::from_utf8) |
        makro_inst_brace
    )
);

named!(unquoted_value<&str>,
    alt_complete!(
        map_res!(is_not!("\"=, \t}$"), str::from_utf8) |
        makro_inst_brace |
        makro_inst_paren
    )
);

named!(makro_inst_brace<&str>,
    alt_complete!(
        do_parse!(
            tag!("${") >>
            tmp: map_res!(
                alt_complete!(
                    take_until!(tag!("${"))) |
                    take_until!(tag!("${"))) |
                    )is_not!("$}"), str::from_utf8) >>
            tag!("}") >>

            (tmp)
        ) |
        makro_inst_brace
    )
);

named!(makro_inst_paren<&str>,
    delimited!(
        tag!("$("),
        map_res!(is_not!(")"), str::from_utf8),
        tag!(")")
    )
);


named!(makro_with_default<Expr>,
    do_parse!(
        name: makro_name >>
        tag!("=") >>
        value: makro_value >>

        (Expr::MakroWithDef(String::from(name), String::from(value)))
    )
);

named!(makro_sub<Expr>,
    do_parse!(
        name: makro_name >>
        tag!("=") >>
        value: makro_value >>

        (Expr::MakroSub(String::from(name), String::from(value)))
    )
);

named!(makro<Expr>,
    do_parse!(
        name: makro_name >>

        (Expr::Makro(String::from(name)))
    )
);

named!(braced<Vec<Expr>>,
    ws!(
        delimited!(
            tag!("{"),
            separated_list!(tag!(","), makro_sub),
            tag!("}")
        )
    )
);

//named!(makros,

//named!(pattern (&[u8]) -> Vec<&[u8]>,
//    do_parse!(
//        tag!("pattern") >>
//        take_until!("{") >>
//        tag!("{") >>
//        makro: take_until!("}") >>
//        tag!("}") >>
//
//        (A{makro: makro})
//    )
//);

#[cfg(test)]
mod tests {
    use nom;
    use ::Expr;

    fn single_test(input: &[u8]) {
        println!("Test {}", String::from_utf8(input.to_vec()).unwrap());
        match ::braced(input) {
            nom::IResult::Done(_, res) => {
                println!("Success");
                for expr in res {
                    match expr {
                        Expr::MakroSub(nam, val) => {
                            println!("{} {}", nam, val)
                        },
                        _ => (),
                    }
                }
            },
            fail => println!("Failed {:?}", fail),
        }
    }

    #[test]
    fn it_works() {
        single_test(&b"{P=TEST}"[..]);
        single_test(&b"{${P}=TEST}"[..]);
        single_test(&b"{P=${TEST}}"[..]);
        single_test(&b"{$(P)=TEST}"[..]);
        single_test(&b"{P=TEST,var=VAL}"[..]);
        single_test(&b"{ P=TEST$ , var=VAL }"[..]);
        single_test(&b"{ \"P $\"=\"|TEST\" }"[..]);
    }
}
