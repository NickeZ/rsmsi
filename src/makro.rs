#[derive(Debug)]
pub enum Error {
    ParseError(String),
}

#[derive(Debug)]
pub struct Macro {
    name: String,
    default: Option<String>,
    value: Option<String>,
}

pub fn parse_macros(input: &str) -> Result<Vec<Macro>, Error> {
    let result = input.split(',').map(|m| {
        parse_macro(m).unwrap_or_else(|e| panic!("Error: {:?}", e))
    }).filter(|m| m.is_some()).map(|m| m.unwrap()).collect();
    Ok(result)
}

fn parse_macro(input: &str) -> Result<Option<Macro>, Error> {
    let input = input.trim();
    if input.len() == 0 {
        return Ok(None);
    }
    match input.find("=") {
        Some(idx) if idx == 0 => Err(Error::ParseError(String::from(input))),
        Some(idx) => {
            let (name, value) = input.split_at(idx);
            let (name, value) = (name.trim(), String::from(value[1..].trim()));
            Ok(Some(Macro {name: String::from(name), default: None, value: Some(value)}))
        },
        None => Ok(Some(Macro {name: String::from(input), default: None, value: None})),
    }
}

#[test]
fn test_parse_macro_ws() {
    for m in &[
        "macro=value",
        " macro=value",
        "macro=value ",
        "macro =value",
        "macro= value",
        "macro = value",
        " macro = value ",
        "macro\t=\tvalue",
    ] {
        let m = parse_macro(m).unwrap().unwrap();
        let (name, value) = (m.name, m.value.unwrap());
        assert!(name == "macro", format!("name was {:?}", name));
        assert!(value == "value", format!("value was {:?}", value));
    }
}

#[test]
fn test_parse_macro_empty_string() {
    for m in &[
        "macro=",
        " macro= ",
    ] {
        let m = parse_macro(m).unwrap().unwrap();
        let (name, value) = (m.name, m.value.unwrap());
        assert!(name == "macro", format!("name was {:?}", name));
        assert!(value == "", format!("value was {:?}", value));
    }
}

#[test]
fn test_parse_macro_none() {
    for m in &[
        "macro",
        " macro ",
    ] {
        let m = parse_macro(m).unwrap().unwrap();
        let (name, value) = (m.name, m.value);
        assert!(name == "macro", format!("name was {:?}", name));
        assert!(value == None, format!("value was {:?}", value));
    }
}
