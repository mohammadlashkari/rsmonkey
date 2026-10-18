#[derive(Debug, PartialEq, Default)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Let(LetStatement),
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Identifier(Identifier),
    Integer(i64),
}

#[derive(Debug, PartialEq)]
pub struct LetStatement {
    pub ident: Identifier,
    pub value: Expression,
}

#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub name: String,
}
