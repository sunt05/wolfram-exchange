use crate::{ToWolfram, WolframValue};
use serde_json::{Number, Value};

impl ToWolfram for Value {
    fn to_wolfram(&self) -> WolframValue {
        match self {
            Value::Null => WolframValue::new_symbol("None"),
            Value::Bool(o) => o.to_wolfram(),
            Value::Number(n) => n.to_wolfram(),
            Value::String(s) => s.to_wolfram(),
            Value::Array(_) => unimplemented!(),
            Value::Object(_) => unimplemented!(),
        }
    }
}

impl ToWolfram for Number {
    fn to_wolfram(&self) -> WolframValue {
        if self.is_u64() {
            self.as_u64().unwrap_or(0).to_wolfram()
        }
        else if self.is_i64() {
            self.as_i64().unwrap_or(0).to_wolfram()
        }
        else {
            self.as_f64().unwrap_or(0.0).to_wolfram()
        }
    }
}
