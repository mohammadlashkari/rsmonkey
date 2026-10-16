struct Program {
    statements: Vec<Statement>,
}

enum Statement {
    Let(LetStatement),
}

enum Expression {
    Identifier(Identifier),
}

struct LetStatement {
    name: Identifier,
    value: Expression,
}

struct Identifier {
    value: String,
}
