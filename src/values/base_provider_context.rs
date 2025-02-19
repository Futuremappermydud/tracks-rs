use std::cell::RefCell;

use super::Value;

pub struct BaseProviderContext {
    base_combo: RefCell<Value>,
}

impl BaseProviderContext {
    pub fn new() -> Self {
        Self {
            base_combo: RefCell::new(Value::Float(0.0)),
        }
    }

    pub fn get_values(&self, base: &str) -> Value {
        match base {
            "baseCombo" => self.base_combo.borrow().clone(),
            _ => panic!("Base provider not found"),
        }
    }

    pub fn set_values(&self, base: &str, values: Value) {
        match base {
            "baseCombo" => {
                self.base_combo.replace(values);
            }
            _ => panic!("Base provider not found"),
        }
    }
}
