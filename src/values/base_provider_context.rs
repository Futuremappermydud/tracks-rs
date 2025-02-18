use std::cell::RefCell;

pub struct BaseProviderContext {
    base_combo: RefCell<Vec<f32>>,
}

impl BaseProviderContext {
    pub fn new() -> Self {
        Self { base_combo: RefCell::new(vec![0f32]) }
    }

    pub fn get_values(&self, base: &str) -> Vec<f32> {
      match base {
        "baseCombo" => self.base_combo.borrow().to_vec(),
        _ => panic!("Base provider not found"),
      }
    }

    pub fn set_values(&self, base: &str, values: Vec<f32>) {
      match base {
        "baseCombo" => { self.base_combo.replace(values); }
        _ => panic!("Base provider not found"),
      }
    }
}
