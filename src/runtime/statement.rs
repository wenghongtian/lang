use anyhow::Result;

use crate::front::ast::Program;

use super::{
    eval,
    value::{RuntimeValue, MK_NULL},
};

pub fn eval_program(program: &Program) -> Result<RuntimeValue> {
    let mut val = MK_NULL();
    for stmt in &program.body {
        val = eval(stmt.as_ref())?;
    }
    Ok(val)
}
