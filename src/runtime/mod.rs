mod expression;
mod statement;
mod value;

use anyhow::Result;

use crate::front::ast::{BinaryExpression, NodeType, NumberLiteral, Program, Statement};

use self::{
    expression::eval_binray_expression,
    statement::eval_program,
    value::{RuntimeValue, MK_NULL, MK_NUMBER},
};

pub fn eval(stmt: &dyn Statement) -> Result<RuntimeValue> {
    match stmt.get_type() {
        NodeType::NumericLiteral => Ok(stmt
            .as_any()
            .downcast_ref::<NumberLiteral>()
            .map_or(MK_NULL(), |number| MK_NUMBER(number.value))),
        NodeType::Program => stmt
            .as_any()
            .downcast_ref::<Program>()
            .map_or(Ok(MK_NULL()), |program| eval_program(program)),
        NodeType::BinaryExpression => stmt
            .as_any()
            .downcast_ref::<BinaryExpression>()
            .map_or(Ok(MK_NULL()), |bin_expr| eval_binray_expression(bin_expr)),
        _ => Ok(MK_NULL()),
    }
}
