#[derive(Debug)]

pub enum ValueType {
    Number,
    Null,
}
#[derive(Debug)]

pub enum Value {
    Number(Number),
    Null,
}

#[derive(Debug)]
pub struct RuntimeValue {
    pub v_type: ValueType,
    pub value: Value,
}

type Number = f32;

pub fn MK_NUMBER(val: f32) -> RuntimeValue {
    RuntimeValue {
        v_type: ValueType::Number,
        value: Value::Number(val),
    }
}
pub fn MK_NULL() -> RuntimeValue {
    RuntimeValue {
        v_type: ValueType::Null,
        value: Value::Null,
    }
}
