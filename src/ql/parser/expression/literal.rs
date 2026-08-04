#[derive(Debug, PartialEq)]
pub enum Literal {
    Int(i32),
    BigInt(i64),
    Float(f32),
    BigFloat(f64),
    Bool(bool),
    String(String),
    Null,
}

impl PartialOrd for Literal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Int(l), Self::Int(r)) => l.partial_cmp(r),
            (Self::Int(l), Self::BigInt(r)) => (&(*l as i64)).partial_cmp(r),
            (Self::Int(l), Self::Float(r)) => (&(*l as f32)).partial_cmp(r),
            (Self::Int(l), Self::BigFloat(r)) => (&(*l as f64)).partial_cmp(r),
            (Self::BigInt(l), Self::Int(r)) => l.partial_cmp(&(*r as i64)),
            (Self::BigInt(l), Self::BigInt(r)) => l.partial_cmp(r),
            (Self::BigInt(l), Self::Float(r)) => {
                //TODO: i64 might not fit in f64
                (&(*l as f64)).partial_cmp(&(*r as f64))
            }
            (Self::BigInt(l), Self::BigFloat(r)) => (&(*l as f64)).partial_cmp(r),

            (Self::Float(l), Self::Int(r)) => l.partial_cmp(&(*r as f32)),
            (Self::Float(l), Self::BigInt(r)) => {
                //TODO: i64 might not fit in f64
                (&(*l as f64)).partial_cmp(&(*r as f64))
            }
            (Self::Float(l), Self::Float(r)) => l.partial_cmp(r),
            (Self::Float(l), Self::BigFloat(r)) => (&(*l as f64)).partial_cmp(r),

            (Self::BigFloat(l), Self::Int(r)) => l.partial_cmp(&(*r as f64)),
            (Self::BigFloat(l), Self::BigInt(r)) => l.partial_cmp(&(*r as f64)),
            (Self::BigFloat(l), Self::Float(r)) => l.partial_cmp(&(*r as f64)),
            (Self::BigFloat(l), Self::BigFloat(r)) => l.partial_cmp(r),

            // Only impl for numeric types
            _ => None,
        }
    }
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
