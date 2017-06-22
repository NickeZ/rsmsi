#[derive(PartialEq, Debug)]
pub enum TmplExpr {
    Include(String),
    Substitute(Vec<(Vec<Box<TmplExpr>>, Vec<Box<TmplExpr>>)>),
    Text(String),
    Makro(Vec<Box<TmplExpr>>),
    MakroWithDefault(Vec<Box<TmplExpr>>, Vec<Box<TmplExpr>>),
}

#[derive(PartialEq, Debug)]
pub struct Template(pub String, pub Box<SubsListType>);

#[derive(PartialEq, Debug)]
pub enum SubsListType {
    RegularList(Vec<Vec<(String, String)>>),
    PatternList(Vec<String>, Vec<Vec<String>>),
}
