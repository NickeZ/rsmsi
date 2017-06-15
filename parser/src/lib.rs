#[macro_use]
extern crate nom;

use nom::alphanumeric;

//#[derive(Debug)]
//struct A<'a> {makro: &'a [u8]}

//named!(makro_name,
//    ws!(
//        alphanumeric
//    )
//);

named!(makro<(&[u8], &[u8])>,
    do_parse!(
        name: alphanumeric >>
        tag!("=") >>
        value: alphanumeric >>

        (name, value)
    )
);

named!(braced<Vec<(&[u8], &[u8])>>,
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
                             String::from_utf8(nam.to_vec()).unwrap(),
                             String::from_utf8(val.to_vec()).unwrap())
                }
            },
            _ => println!("Failed"),
        }
        match ::braced(&b"{ P=TEST , var=VAL }"[..]) {
            nom::IResult::Done(_, res) => {
                println!("success");
                for (nam, val) in res {
                    println!("{} {}", 
                             String::from_utf8(nam.to_vec()).unwrap(),
                             String::from_utf8(val.to_vec()).unwrap())
                }
            },
            _ => println!("Failed"),
        }
    }
}
