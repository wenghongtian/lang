use std::any::Any;

pub enum NodeType {
    Program,
    ExpressionStatement,
    BinaryExpression,
    NumericLiteral,
}

pub trait Statement {
    fn get_type(&self) -> NodeType;
    fn as_any(&self) -> &dyn Any;
}

pub struct Program {
    pub body: Vec<Box<dyn Statement>>,
}
impl Statement for Program {
    fn get_type(&self) -> NodeType {
        NodeType::Program
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl Program {
    pub fn new() -> Program {
        Program { body: vec![] }
    }
}

pub trait Expression: Statement {}

pub struct BinaryExpression {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
    pub operator: String,
}

impl Statement for BinaryExpression {
    fn get_type(&self) -> NodeType {
        NodeType::BinaryExpression
    }

    fn as_any(&self) -> &dyn Any {
        self
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
    pub value: f32,
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

    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl Expression for NumberLiteral {}
