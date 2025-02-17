#[derive(Clone, Copy, PartialEq)]
pub enum Operation {
    None = 0,
    Add,
    Sub,
    Mul,
    Div,
}

impl Operation {
    pub fn from_str(s: &str) -> Self {
        match s {
            "opAdd" => Self::Add,
            "opSub" => Self::Sub,
            "opMul" => Self::Mul,
            "opDiv" => Self::Div,
            _ => Self::None,
        }
    }
}
