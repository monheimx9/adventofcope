use std::cmp::{self, *};
use std::collections::HashMap;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandType {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard,
}
impl HandType {
    fn is_win(&self, t2: &HandType) -> Option<bool> {
        match t2.cmp(self) {
            Ordering::Less => Some(false),
            Ordering::Equal => None,
            Ordering::Greater => Some(true),
        }
    }
    fn from_str(s: &str) -> Self {
        let mut h: HashMap<char, u8> = HashMap::new();
        for c in s.chars() {
            let n = h.entry(c).or_insert(0);
            *n += 1;
        }
        HandType::identity(&h)
    }
    fn identity(h: &HashMap<char, u8>) -> Self {
        if h.len() == 1 {
            HandType::FiveKind
        } else if h.len() == 5 {
            HandType::HighCard
        } else if h.len() == 4 {
            HandType::OnePair
        } else if h.len() == 3 {
            if h.iter().any(|f| f.1 == &3) {
                HandType::ThreeKind
            } else {
                HandType::TwoPair
            }
        } else {
            if h.iter().any(|f| f.1 == &1) {
                HandType::FourKind
            } else {
                HandType::FullHouse
            }
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum CardStrenght {
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
impl CardStrenght {
    fn is_win(&self, t2: &CardStrenght) -> Option<bool> {
        match t2.cmp(self) {
            Ordering::Less => Some(false),
            Ordering::Equal => Some(false),
            Ordering::Greater => Some(true),
        }
    }
    fn from_char(c: &char) -> Self {
        match c {
            'A' => CardStrenght::A,
            'K' => CardStrenght::K,
            'Q' => CardStrenght::Q,
            'J' => CardStrenght::J,
            'T' => CardStrenght::T,
            '9' => CardStrenght::Nine,
            '8' => CardStrenght::Eight,
            '7' => CardStrenght::Seven,
            '6' => CardStrenght::Six,
            '5' => CardStrenght::Five,
            '4' => CardStrenght::Four,
            '3' => CardStrenght::Three,
            '2' => CardStrenght::Two,
            _ => CardStrenght::One,
        }
    }
}
#[derive(Clone)]
struct Hand {
    ht: HandType,
    cards: Vec<CardStrenght>,
    bid: i32,
}
impl Hand {
    fn from_str(s: &str) -> Self {
        let (hts, bid) = s.split_once(' ').unwrap();
        Hand {
            bid: bid.parse::<i32>().unwrap(),
            ht: HandType::from_str(hts),
            cards: hts.chars().map(|f| CardStrenght::from_char(&f)).collect(),
        }
    }
    fn is_win(&self, h2: &Hand) -> Option<bool> {
        match h2.ht.cmp(&self.ht) {
            Ordering::Greater => Some(true),
            Ordering::Less => Some(false),
            Ordering::Equal => Some(
                self.cards
                    .iter()
                    .zip(h2.cards.iter())
                    .any(|(c1, c2)| c1.is_win(c2).unwrap()),
            ),
        }
    }
}

fn process_part_one(hh: &[&str]) -> i32 {
    let mut hands: Vec<(i32, Hand)> = Vec::new();
    for h in hh {
        hands.push((0, Hand::from_str(h)));
    }
    for n in 0..hands.len() {
        for o in 0..hands.len() {
            if n != o {
                if hands[n].1.is_win(&hands[o].1).unwrap() {
                    let c = hands.get_mut(n).unwrap();
                    c.0 += 1;
                    //let c = hands.get_mut(o).unwrap();
                    //c.0 += -1;
                } else {
                    let c = hands.get_mut(n).unwrap();
                    c.0 += -1;
                    //let c = hands.get_mut(o).unwrap();
                    //c.0 += 1;
                }
            }
        }
    }
    hands.sort_by_key(|&(key, _)| key);
    for n in 0..hands.len() {
        let c = hands.get_mut(n).unwrap();
        c.0 = n as i32;
    }
    hands.iter().fold(0, |acc, (x, y)| acc + x * y.bid)
}

fn main() {
    println!("Hello, world!");
    let input: Vec<&str> = include_str!("../test.txt").lines().collect();
    process_part_one(&input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cardtypewin() {
        let t1 = HandType::FiveKind;
        let t2 = HandType::FourKind;
        assert!(t1.is_win(&t2).unwrap())
    }
    #[test]
    fn cardtypeloose() {
        let t1 = HandType::TwoPair;
        let t2 = HandType::HighCard;
        assert!(!t2.is_win(&t1).unwrap())
    }
    #[test]
    fn cardtypeequal() {
        let t1 = HandType::TwoPair;
        let t2 = HandType::TwoPair;
        assert!(t1.is_win(&t2).is_none())
    }
    #[test]
    fn card_ident() {
        let card = HandType::from_str("32T3K");
        assert!(matches!(card, HandType::OnePair))
    }
    #[test]
    fn card_fivetype() {
        let card = HandType::from_str("AAAAA");
        assert!(matches!(card, HandType::FiveKind))
    }
    #[test]
    fn fourkind() {
        let card = HandType::from_str("AA8AA");
        assert!(matches!(card, HandType::FourKind))
    }
    #[test]
    fn fullhouse() {
        let card = HandType::from_str("23332");
        assert!(matches!(card, HandType::FullHouse))
    }
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
    fn same_type() {
        let h1 = Hand::from_str("KK677 28");
        let h2 = Hand::from_str("KTJJT 220");
        assert!(h1.is_win(&h2).unwrap())
    }
    #[test]
    fn win1() {
        let h1 = Hand::from_str("QQQJA 483");
        let h2 = Hand::from_str("T55J5 684");
        assert!(h1.is_win(&h2).unwrap())
    }
    #[test]
    fn part_one() {
        let hh: Vec<&str> = include_str!("../test.txt").lines().collect();
        let r = process_part_one(&hh);
        assert_eq!(r, 6440)
    }
}
