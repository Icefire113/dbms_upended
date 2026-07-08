#[derive(Debug, PartialEq, PartialOrd)]
pub enum Literal {
    Int(i32),
    BigInt(i64),
    Float(f32),
    BigFloat(f64),
    Bool(bool),
    String(String),
    Null,
}

impl From<bool> for Literal {
    fn from(b: bool) -> Self {
        Literal::Bool(b)
    }
}

impl From<i32> for Literal {
    fn from(i: i32) -> Self {
        Literal::Int(i)
    }
}

impl From<i64> for Literal {
    fn from(i: i64) -> Self {
        Literal::BigInt(i)
    }
}

impl From<f32> for Literal {
    fn from(f: f32) -> Self {
        Literal::Float(f)
    }
}

impl From<f64> for Literal {
    fn from(f: f64) -> Self {
        Literal::BigFloat(f)
    }
}

impl From<String> for Literal {
    fn from(s: String) -> Self {
        Literal::String(s)
    }
}
