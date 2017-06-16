#[derive(PartialEq, Debug)]
pub enum TmplExpr {
    Include(String),
    Substitute(Vec<(String, String)>),
    Text(String),
}

#[derive(PartialEq, Debug)]
pub enum SubsExpr {
    Template(String, Box<SubsExpr>),
    Makro(String, String),
    MakroList(Vec<Box<SubsExpr>>),
    RegularList(Vec<Box<SubsExpr>>),
    PatternList(Box<SubsExpr>, Vec<Box<SubsExpr>>),
    PatternListDef(Vec<String>),
    PatternListInst(Vec<String>),
    Literal(String),
}
