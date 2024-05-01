pub enum Scoring {
    Win = 6,
    Draw = 3,
    Loss = 0,
}
pub trait StringManipulation {
    fn split_blank(&self) -> Vec<&str>;
}

impl StringManipulation for str {
    fn split_blank(&self) -> Vec<&str> {
        self.split("\n\n").collect()
    }
}
