use std::collections::BTreeMap;

fn main() {
    println!("Hello, world!");
    let input: Vec<&str> = include_str!("../input.txt").lines().collect();
    let mut count: u32 = 0;
    for s in input.iter() {
        count += get_double_digit(s);
    }
    println!("The total of part one is {count}");

    count = 0;
    for s in input {
        count += get_double_digit_with_index(s);
    }

    println!("The total of part one is {count}")
}

fn get_double_digit_with_index(s: &str) -> u32 {
    let digits_word = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let digits = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut first_digit: char = '0';
    let mut first_digit_index: usize = 1000;
    let mut second_digit: char = '0';
    let mut second_digit_index: usize = 0;
    'outer: for (i, c) in s.chars().enumerate() {
        for d in digits {
            if d == c {
                first_digit_index = i;
                first_digit = c;
                break 'outer;
            }
        }
    }
    'outer: for (i, c) in s.chars().rev().enumerate() {
        for d in digits {
            if d == c {
                second_digit_index = s.len() - (i + 1);
                second_digit = c;
                break 'outer;
            }
        }
    }

    let mut first_word: Option<&str> = None;
    let mut first_word_index: Option<u32> = None;
    let mut second_word: Option<&str> = None;
    let mut second_word_index: Option<u32> = None;

    let mut word_collection: BTreeMap<u32, &str> = BTreeMap::new();

    for word in digits_word {
        if let Some(w) = s.find(word) {
            word_collection.insert(w as u32, word);
        }
    }

    if let Some((i, w)) = word_collection.first_key_value() {
        first_word_index = Some(*i);
        first_word = Some(w);
    }

    let mut word_collection: BTreeMap<u32, &str> = BTreeMap::new();

    for word in digits_word {
        if let Some(w) = s.rfind(word) {
            word_collection.insert(w as u32, word);
        }
    }

    if let Some((i, w)) = word_collection.last_key_value() {
        second_word_index = Some(*i);
        second_word = Some(w);
    }

    let mut first: u32 = 0;
    let mut second: u32 = 0;

    match first_word {
        Some(w) => {
            if first_word_index.unwrap() < first_digit_index as u32 {
                first = transform_word_to_digit(w).unwrap();
            } else {
                first = first_digit.to_digit(10).unwrap();
            }
        }
        None => first = first_digit.to_digit(10).unwrap(),
    }
    match second_word {
        Some(w) => {
            if second_word_index.unwrap() > second_digit_index as u32 {
                second = transform_word_to_digit(w).unwrap();
            } else {
                second = second_digit.to_digit(10).unwrap();
            }
        }
        None => second = second_digit.to_digit(10).unwrap(),
    }

    let result = format!("{}{}", first, second);
    let result: u32 = result.parse().unwrap();
    result
}

fn transform_word_to_digit(s: &str) -> Option<u32> {
    let digits_word = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut result: Option<u32> = None;

    for (i, d) in digits_word.iter().enumerate() {
        if d == &s {
            result = Some(i as u32);
        }
    }
    result
}

fn get_double_digit(s: &str) -> u32 {
    let digits = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut first: u32 = 0;
    let mut second: u32 = 0;

    'outer: for c in s.chars() {
        for d in digits {
            if d == c {
                first = d.to_digit(10).unwrap();
                break 'outer;
            }
        }
    }
    'outer: for c in s.chars().rev() {
        for d in digits {
            if d == c {
                second = d.to_digit(10).unwrap();
                break 'outer;
            }
        }
    }
    let result = format!("{first}{second}");
    result.parse::<u32>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn itworks1() {
        assert_eq!(get_double_digit("1abc2"), 12)
    }
    #[test]
    fn itworks2() {
        assert_eq!(get_double_digit("pqr3stu8vwx"), 38)
    }
    #[test]
    fn itworks3() {
        assert_eq!(get_double_digit("a1b2c3d4e5f"), 15)
    }
    #[test]
    fn itworks4() {
        assert_eq!(get_double_digit("treb7uchet"), 77)
    }
    #[test]
    fn part_two1() {
        assert_eq!(get_double_digit_with_index("two1nine"), 29)
    }
    #[test]
    fn part_two2() {
        assert_eq!(get_double_digit_with_index("eightwothree"), 83)
    }
    #[test]
    fn part_two3() {
        assert_eq!(get_double_digit_with_index("abcone2threexyz"), 13)
    }
    #[test]
    fn part_two4() {
        assert_eq!(get_double_digit_with_index("xtwone3four"), 24)
    }
    #[test]
    fn part_two5() {
        assert_eq!(get_double_digit_with_index("4nineeightseven2"), 42)
    }
    #[test]
    fn part_two6() {
        assert_eq!(get_double_digit_with_index("zoneight234"), 14)
    }
    #[test]
    fn part_two7() {
        assert_eq!(get_double_digit_with_index("7pqrstsixteen"), 76)
    }
    #[test]
    fn part_two8() {
        assert_eq!(get_double_digit_with_index("xcsfkjqvln2tpm"), 22)
    }
}
