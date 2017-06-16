#[derive(PartialEq, Debug)]
pub enum TmplExpr {
    Include(String),
    Substitute(Vec<(String, String)>),
    Text(String),
}

#[derive(PartialEq, Debug)]
pub struct Template(pub String, pub Box<SubsExpr>);

#[derive(PartialEq, Debug)]
pub enum SubsExpr {
    Makro(String, String),
    MakroList(Vec<Box<SubsExpr>>),
    RegularList(Vec<Box<SubsExpr>>),
    PatternList(Box<SubsExpr>, Vec<Box<SubsExpr>>),
    PatternListDef(Vec<String>),
    PatternListInst(Vec<String>),
    Literal(String),
}
