#[derive(Debug, PartialEq, Default)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Let(LetStatement),
    // return <expression>;
    Return(Expression),
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Ident(Ident),
    Integer(i64),
}

#[derive(Debug, PartialEq)]
pub struct LetStatement {
    pub ident: Ident,
    pub value: Expression,
}

#[derive(Debug, PartialEq)]
pub struct Ident {
    pub name: String,
}
