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

pub fn as_statement(expr: &dyn Statement) -> &dyn Statement {
    expr
}

pub struct BinaryExpression {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
    operator: String,
}

impl Statement for BinaryExpression {
    fn get_type(&self) -> NodeType {
        NodeType::BinaryExpression
    }
}
impl Expression for BinaryExpression {}
impl BinaryExpression {
    pub fn new(
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

pub struct NumberLiteral {
    value: f32,
}
impl NumberLiteral {
    pub fn new(val: f32) -> NumberLiteral {
        NumberLiteral { value: val }
    }
}
impl Statement for NumberLiteral {
    fn get_type(&self) -> NodeType {
        NodeType::NumericLiteral
    }
}
impl Expression for NumberLiteral {}
