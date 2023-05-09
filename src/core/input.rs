pub struct InputResult {
}
pub trait Input {
    fn input(&self) -> InputResult;
}