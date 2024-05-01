pub enum Scoring {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

#[derive(Copy, Clone)]
pub enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}
impl Hand {
    pub fn from_char(c: char) -> Hand {
        match c {
            'A' | 'X' => Hand::Rock,
            'B' | 'Y' => Hand::Paper,
            'C' | 'Z' => Hand::Scissors,
            _ => panic!("Invalid hand character: {}", c),
        }
    }
    pub fn compare(&self, other: &Hand) -> u32 {
        match self {
            Self::Rock => match other {
                Hand::Rock => Scoring::Draw as u32,
                Hand::Paper => Scoring::Loss as u32,
                Hand::Scissors => Scoring::Win as u32,
            },
            Self::Paper => match other {
                Hand::Rock => Scoring::Win as u32,
                Hand::Paper => Scoring::Draw as u32,
                Hand::Scissors => Scoring::Loss as u32,
            },
            Self::Scissors => match other {
                Hand::Rock => Scoring::Loss as u32,
                Hand::Paper => Scoring::Win as u32,
                Hand::Scissors => Scoring::Draw as u32,
            },
        }
    }
    pub fn hand_value(&self) -> u32 {
        *self as u32
    }
}

pub struct Game {
    rounds: Vec<Round>,
}

pub struct Round {
    hand1: Hand,
    hand2: Hand,
}

pub trait StringManipulation {
    fn split_blank(&self) -> Vec<&str>;
}

impl StringManipulation for str {
    fn split_blank(&self) -> Vec<&str> {
        self.split("\n\n").collect()
    }
}
