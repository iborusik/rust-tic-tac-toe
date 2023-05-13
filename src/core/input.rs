pub struct InputResult {
    pub i: u32,
    pub j: u32,
}
pub trait Input {
    fn read_input(&self) -> InputResult;
}