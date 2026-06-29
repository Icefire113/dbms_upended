use bitcode::{Decode, Encode};

#[derive(Debug, Encode, Decode, PartialEq, Eq)]
pub enum ColType {
    Int,
    BigInt,
    Float,
    BigFloat,
    Bool,
    String,
}
