#[derive(Debug)]
pub enum AdiError {
    NullPointerError,
    NegativeValueError(i32),
}
