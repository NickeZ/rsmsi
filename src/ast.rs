#[derive(PartialEq, Debug)]
pub enum Expr {
    List(Vec<Box<Expr>>),
    Makro(Vec<Box<Expr>>),
    MakroWithDefault(Vec<Box<Expr>>, Vec<Box<Expr>>),
    Final(String),
}
