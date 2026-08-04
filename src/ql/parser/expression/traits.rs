pub trait BinaryApply<T> {
    type Error;
    /// Applies the given binary operator to the two given values
    fn apply(&self, l: T, r: T) -> Result<T, Self::Error>;
}

pub trait UnaryApply<T> {
    type Error;
    /// Applies the given unary operator to the given value
    fn apply(&self, r: T) -> Result<T, Self::Error>;
}
