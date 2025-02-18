use std::cell::RefCell;

#[derive(Clone)]
pub struct BaseProviderContext {
    pub base_combo: RefCell<Vec<f32>>,
}