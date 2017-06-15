#[macro_use]
extern crate nom;

use nom::alphanumeric;
use std::str;

//#[derive(Debug)]
//struct A<'a> {makro: &'a [u8]}

named!(makro_name<&str>,
    alt_complete!(
        unquoted_string |
        string
    )
);

named!(string<&str>,
  delimited!(
    tag!("\""),
    map_res!(escaped!(call!(alphanumeric), '\\', is_a!("\"n\\")), str::from_utf8),
    tag!("\"")
  )
);

named!(unquoted_string<&str>,
    map_res!(call!(alphanumeric), str::from_utf8)
);


named!(makro<(&str, &str)>,
    do_parse!(
        name: makro_name >>
        tag!("=") >>
        value: makro_name >>

        (name, value)
    )
);

named!(braced<Vec<(&str, &str)>>,
    ws!(
        delimited!(
            tag!("{"),
            separated_list!(tag!(","), makro),
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
    #[test]
    fn it_works() {
        match ::braced(&b"{P=TEST,var=VAL}"[..]) {
            nom::IResult::Done(_, res) => {
                println!("success");
                for (nam, val) in res {
                    println!("{} {}", 
                             nam,
                             val)
                }
            },
            _ => println!("Failed"),
        }
        match ::braced(&b"{ P=TEST , var=VAL }"[..]) {
            nom::IResult::Done(_, res) => {
                println!("success");
                for (nam, val) in res {
                    println!("{} {}", 
                             nam,
                             val)
                }
            },
            _ => println!("Failed"),
        }
        match ::braced(&b"{ \"P\"=\"TEST\" }"[..]) {
            nom::IResult::Done(_, res) => {
                println!("success");
                for (nam, val) in res {
                    println!("{} {}", 
                             nam,
                             val)
                }
            },
            _ => println!("Failed"),
        }
    }
}
