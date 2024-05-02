#[derive(Copy, Clone)]
pub enum Scoring {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

impl Scoring {
    pub fn from_char(c: char) -> Scoring {
        match c { 'X' => Scoring::Loss,
        'Y' => Scoring::Draw,
        'Z' => Scoring::Win,
        _ => panic!("Invalid strategy character: {}", c)}
    }
    pub fn get_value(&self) -> u32 {
        *self as u32
    }
    pub fn get_hand_from(&self, hand: Hand) -> Hand {
        match hand {
            Hand::Paper => match self {
                Scoring::Win => {Hand::Scissors},
                Scoring::Draw => {Hand::Paper},
                Scoring::Loss => {Hand::Rock},
            },
            Hand::Rock => match self {
                Scoring::Win => {Hand::Paper},
                Scoring::Draw => {Hand::Rock},
                Scoring::Loss => {Hand::Scissors},
            },
            Hand::Scissors => match self {
                Scoring::Win => {Hand::Rock},
                Scoring::Draw => {Hand::Scissors},
                Scoring::Loss => {Hand::Paper},
            },
        }
    }
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
        match other {
            Self::Rock => match self {
                Hand::Rock => Scoring::Draw as u32,
                Hand::Paper => Scoring::Loss as u32,
                Hand::Scissors => Scoring::Win as u32,
            },
            Self::Paper => match self {
                Hand::Rock => Scoring::Win as u32,
                Hand::Paper => Scoring::Draw as u32,
                Hand::Scissors => Scoring::Loss as u32,
            },
            Self::Scissors => match self {
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
impl Game {
    pub fn from_str(s: &str) -> Self {
        Game {rounds : s.lines().map(|g| Round::from_str(g)).collect::<Vec<_>>()}
    }
    pub fn score(&self) -> u32 {
        self.rounds.iter().map(|s| s.score()).sum()
    }
    pub fn score_new(&self) -> u32 {
    self.rounds.iter().map(|s| s.score_new_strategy()).sum()
    }
}

pub struct Round {
    hand1: Hand,
    hand2: Hand,
    strategy: Scoring
}

impl Round {
    pub fn from_str(s: &str) -> Self{
        let (h1, h2) = s.split_once(' ').unwrap();
        Round {hand1 : Hand::from_char(h1.chars().last().unwrap()),
            hand2 : Hand::from_char(h2.chars().last().unwrap()),
            strategy : Scoring::from_char(h2.chars().last().unwrap())
        }
    }
    pub fn score(&self) -> u32 {
        let r = self.hand1.compare(&self.hand2);
        r + self.hand2.hand_value()
    }
    pub fn score_new_strategy(&self) -> u32 {
        let r = self.strategy.get_hand_from(self.hand1);
        let score = self.strategy.get_value();
        score + r.hand_value()


    }
}

pub trait StringManipulation {
    fn split_blank(&self) -> Vec<&str>;
}

impl StringManipulation for str {
    fn split_blank(&self) -> Vec<&str> {
        self.split("\n\n").collect()
    }
}
