pub enum NodeType {
    Program,
    ExpressionStatement,
    BinaryExpression,
    NumericLiteral,
}

pub trait Statement {
    fn get_type(&self) -> NodeType;
}

pub struct Program {
    pub body: Vec<Box<dyn Statement>>,
}
impl Statement for Program {
    fn get_type(&self) -> NodeType {
        NodeType::Program
    }
}
impl Program {
    pub fn new() -> Program {
        Program { body: vec![] }
    }
}

pub trait Expression: Statement {}

struct BinaryExpression {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
    operator: String,
}
impl Expression for BinaryExpression {}
impl Statement for BinaryExpression {
    fn get_type(&self) -> NodeType {
        NodeType::BinaryExpression
    }
}
impl BinaryExpression {
    fn new(
        left: Box<dyn Expression>,
        right: Box<dyn Expression>,
        operator: String,
    ) -> BinaryExpression {
        BinaryExpression {
            left,
            right,
            operator,
        }
    }
}
