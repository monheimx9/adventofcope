use std::collections::HashMap;
#[derive(Clone)]
struct Card {
    win: Vec<u32>,
    own: Vec<u32>,
    point: u32,
    matches: u32,
    copies: u32,
    id: u32,
}

impl Card {
    fn from_str(s: &str) -> Self {
        let (id_, w) = s.split_once(':').unwrap();
        let (win, own) = w.split_once('|').unwrap();
        let win_: Vec<u32> = win
            .split(' ')
            .filter_map(|f| f.parse::<u32>().ok())
            .collect();
        let own_: Vec<u32> = own
            .split(' ')
            .filter_map(|f| f.parse::<u32>().ok())
            .collect();
        let id_: String = id_.chars().filter(|c| c.is_ascii_digit()).collect();
        let id_: u32 = id_.parse().unwrap();
        let matches_: usize = win_
            .iter()
            .filter(|&w| own_.iter().any(|&o| o == *w))
            .count();
        let point_: u32 = {
            if matches_ > 0 {
                2u32.pow(matches_ as u32 - 1)
            } else {
                0
            }
        };
        Card {
            win: win_,
            own: own_,
            id: id_,
            point: point_,
            matches: matches_ as u32,
            copies: 1,
        }
    }
}

fn main() {
    println!("Hello, world!");
    let input: Vec<&str> = include_str!("../input.txt").lines().collect();
    let points = input.iter().map(|s| Card::from_str(s).point).sum::<u32>();
    println!("The total points is {points}");
    let mut cards: HashMap<u32, Card> = HashMap::new();
    for (i, c) in input.iter().enumerate() {
        cards.insert(i as u32, Card::from_str(c).clone());
    }
    for n in 0..cards.len() {
        let id: u32 = n as u32;
        let ma = cards.get(&id).unwrap().matches;
        let co = cards.get(&id).unwrap().copies;
        for o in 1..=ma {
            for p in 0..co {
                let idx: u32 = n as u32 + o;
                if idx as usize <= cards.len() {
                    if let Some(card) = cards.get_mut(&idx) {
                        card.copies += 1;
                    }
                }
            }
        }
    }
    let count: u32 = cards.iter().map(|f| f.1.copies).sum();
    println!("The total count is {count}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partone1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
                        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
                        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
                        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
                        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
                        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let input: Vec<&str> = input.lines().collect();
        let test: Vec<Card> = input.iter().map(|s| Card::from_str(s)).collect();
        assert_eq!(test.iter().map(|s| s.point).sum::<u32>(), 13)
    }
    #[test]
    fn parttwo() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
                        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
                        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
                        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
                        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
                        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let input: Vec<&str> = input.lines().collect();
        let mut cards: HashMap<u32, Card> = HashMap::new();
        for (i, c) in input.iter().enumerate() {
            cards.insert(i as u32, Card::from_str(c).clone());
        }
        for n in 0..cards.len() {
            let id: u32 = n as u32;
            let ma = cards.get(&id).unwrap().matches;
            let co = cards.get(&id).unwrap().copies;
            for o in 1..=ma {
                for p in 0..co {
                    let idx: u32 = n as u32 + o;
                    if idx as usize <= cards.len() {
                        if let Some(card) = cards.get_mut(&idx) {
                            card.copies += 1;
                        }
                    }
                }
            }
        }
        let count: u32 = cards.iter().map(|f| f.1.copies).sum();
        assert_eq!(count, 30)
    }
}
