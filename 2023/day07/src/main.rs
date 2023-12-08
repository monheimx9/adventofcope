use std::cmp::*;
use std::collections::HashMap;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    One,
}
impl Card {
    fn from_char(c: &char) -> Self {
        match c {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => Card::One,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Hand {
    FiveKind(Vec<Card>),
    FourKind(Vec<Card>),
    FullHouse(Vec<Card>),
    ThreeKind(Vec<Card>),
    TwoPair(Vec<Card>),
    OnePair(Vec<Card>),
    HighCard(Vec<Card>),
}
impl Hand {
    fn from_str(s: &str) -> Self {
        let mut h: HashMap<char, u8> = HashMap::new();
        for c in s.chars() {
            let n = h.entry(c).or_insert(0);
            *n += 1;
        }
        let cards = s.chars().map(|f| Card::from_char(&f)).collect();

        if h.len() == 1 {
            Hand::FiveKind(cards)
        } else if h.len() == 5 {
            Hand::HighCard(cards)
        } else if h.len() == 4 {
            Hand::OnePair(cards)
        } else if h.len() == 3 {
            if h.iter().any(|f| f.1 == &3) {
                Hand::ThreeKind(cards)
            } else {
                Hand::TwoPair(cards)
            }
        } else {
            if h.iter().any(|f| f.1 == &1) {
                Hand::FourKind(cards)
            } else {
                Hand::FullHouse(cards)
            }
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct HandSet {
    hands: Vec<(Hand, usize)>,
}
impl HandSet {
    fn from_str(s: &str) -> Self {
        let ss = s.lines().collect::<Vec<&str>>();
        let mut h: Vec<(Hand, usize)> = Vec::new();
        for l in ss {
            let (hts, bid) = l.split_once(' ').unwrap();
            let ht = Hand::from_str(hts);
            h.push((ht, bid.parse().unwrap()))
        }
        h.sort_by(|h1, h2| h2.0.partial_cmp(&h1.0).unwrap());

        HandSet { hands: h }
    }
}

fn process_part_one(hh: &str) -> usize {
    let handset = HandSet::from_str(hh);
    handset
        .hands
        .iter()
        .enumerate()
        .map(|(r, (_, bid))| (r + 1) * bid)
        .sum()
}

fn main() {
    println!("Hello, world!");
    let input = include_str!("../input.txt");
    let r = process_part_one(input);
    println!("Part one = {r}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doublon() {
        let mut h: HashMap<&str, u8> = HashMap::new();
        let input: Vec<&str> = include_str!("../input.txt").lines().collect();
        for i in input {
            let a = i.split_once(' ').unwrap().0;
            let n = h.entry(a).or_insert(0);
            *n += 1;
        }
        assert!(h.iter().all(|f| f.1 == &1))
    }

    #[test]
    fn part_one() {
        let hh = include_str!("../test.txt");
        let r = process_part_one(hh);
        assert_eq!(r, 6440)
    }
}
