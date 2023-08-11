use anyhow::{anyhow, Ok, Result};

use crate::front::ast::{BinaryExpression, Expression, Statement};

use super::{
    eval,
    value::{RuntimeValue, Value, ValueType, MK_NUMBER},
};

pub fn eval_binray_expression(expr: &BinaryExpression) -> Result<RuntimeValue> {
    unsafe {
        let left = std::mem::transmute::<&dyn Expression, &dyn Statement>(expr.left.as_ref());
        let right = std::mem::transmute::<&dyn Expression, &dyn Statement>(expr.right.as_ref());
        let left = eval(left)?;
        let right = eval(right)?;
        let op = expr.operator.as_str();
        match left.v_type {
            ValueType::Number => match right.v_type {
                ValueType::Number => {
                    if let Value::Number(left_value) = left.value {
                        if let Value::Number(right_value) = right.value {
                            match op {
                                "+" => return Ok(MK_NUMBER(left_value + right_value)),
                                "-" => return Ok(MK_NUMBER(left_value - right_value)),
                                "*" => return Ok(MK_NUMBER(left_value * right_value)),
                                "/" => return Ok(MK_NUMBER(left_value / right_value)),
                                "%" => return Ok(MK_NUMBER(left_value % right_value)),
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
    Err(anyhow!("Invalid binray expression during runtime!"))
}
