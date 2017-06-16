use std::collections::HashMap;


pub type MacroSet = HashMap<String, Macro>;

#[derive(Debug)]
pub enum Error {
    ParseError(String),
}

#[derive(Debug)]
pub struct Macro {
    // Name of the macro
    pub name: String,

    // Default value of the macro. E.g. $(TEST=default)
    pub default: Option<String>,

    // Value of the macro.
    pub value: Option<String>,
}

/// Function to parse comma sperated macros , i.e. A=C,B=D
pub fn parse_macros(input: &str) -> Result<MacroSet, Error> {
    let mut result = HashMap::new();
    result.extend(input.split(',')
        .map(parse_macro).collect::<Result<Vec<Option<Macro>>,_>>()?
        .into_iter()
        .filter_map(|m| m)
        .map(|m| (m.name.clone(), m)));
    Ok(result)
}

/// Function to parse a single macro, i.e. A=C. Removes all whitespace
fn parse_macro(input: &str) -> Result<Option<Macro>, Error> {
    let input = input.trim();
    if input.len() == 0 {
        return Ok(None);
    }
    match input.find("=") {
        Some(idx) => {
            let (name, value) = input.split_at(idx);
            let (name, value) = (name.trim(), String::from(value[1..].trim()));
            if name.len() == 0 {
                Err(Error::ParseError(String::from(input)))
            } else {
                Ok(Some(Macro {name: String::from(name), default: None, value: Some(value)}))
            }
        },
        None => Err(Error::ParseError(String::from(input))),
        //None => Ok(Some(Macro {name: String::from(input), default: None, value: None})),
    }
}

#[test]
fn test_parse_macros_multi1() {
    let m = parse_macros("macro=value ,macro2=value").unwrap();
    let mut m = m.iter();
    let (name, mak) =  m.next().unwrap();
    let value = mak.value.clone().unwrap();
    // The order we get out the macros is undefined..
    if name == "macro" {
        assert!(name == "macro", format!("name was {:?}", name));
        assert!(value == "value", format!("value was {:?}", value));
        let (name, mak) =  m.next().unwrap();
        let value = mak.value.clone().unwrap();
        assert!(name == "macro2", format!("name was {:?}", name));
        assert!(value == "value", format!("value was {:?}", value));
    } else {
        assert!(name == "macro2", format!("name was {:?}", name));
        assert!(value == "value", format!("value was {:?}", value));
        let (name, mak) =  m.next().unwrap();
        let value = mak.value.clone().unwrap();
        assert!(name == "macro", format!("name was {:?}", name));
        assert!(value == "value", format!("value was {:?}", value));
    }
    assert!(m.next().is_none());
}

#[test]
fn test_parse_macros_multi2() {
    let m = parse_macros("macro=value ,macro=value2").unwrap();
    let mut m = m.iter();
    let (name, mak) =  m.next().unwrap();
    let value = mak.value.clone().unwrap();
    assert!(name == "macro", format!("name was {:?}", name));
    assert!(value == "value2", format!("value was {:?}", value));
    assert!(m.next().is_none());
}

#[test]
fn test_parse_macros_multi3() {
    let m = parse_macros("macro=value,,").unwrap();
    let mut m = m.iter();
    let (name, mak) =  m.next().unwrap();
    let value = mak.value.clone().unwrap();
    assert!(name == "macro", format!("name was {:?}", name));
    assert!(value == "value", format!("value was {:?}", value));
    assert!(m.next().is_none());
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
        let m = parse_macro(m);
        assert!(m.is_err(), format!("m was {:?}", m));
    }
}
