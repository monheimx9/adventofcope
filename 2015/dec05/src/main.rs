enum SantaString {
    IsNice,
    IsNaughty,
}
fn main() {
    //It contains at least three vowels (aeiou only)
    //It contains at least one letter that appears twice in a row
    //It does not contain the strings ab, cd, pq, or xy
    let input = include_str!("../input.txt");
    println!("Hello, world!");
    let mut count = 0;
    let strings: Vec<&str> = input.lines().collect();
    for s in strings.iter() {
        match is_this_string_nice(s) {
            SantaString::IsNaughty => {}
            SantaString::IsNice => count += 1,
        }
    }
    println!("Total nice strings {count}");

    count = the_second_impact(&strings);

    println!("The true total is {count}");
}

fn the_second_impact(ss: &[&str]) -> i32 {
    let mut count = 0;
    for s in ss.iter() {
        match is_it_really_nice(s) {
            SantaString::IsNice => count += 1,
            SantaString::IsNaughty => {}
        }
    }
    count
}

fn is_it_really_nice(s: &str) -> SantaString {
    match double_pair(s) {
        SantaString::IsNaughty => SantaString::IsNaughty,
        SantaString::IsNice => match repeat_between(s) {
            SantaString::IsNaughty => SantaString::IsNaughty,
            SantaString::IsNice => SantaString::IsNice,
        },
    }
}

fn double_pair(s: &str) -> SantaString {
    let mut first_char: char = '_';
    for (i, c) in s.chars().enumerate() {
        if let Some(a) = s.rfind(format!("{}{}", first_char, c).as_str()) {
            if a > (i) {
                println!(
                    "Nice pair: {} = {}",
                    s,
                    format!("{}{}", first_char, c).as_str()
                );
                return SantaString::IsNice;
            }
        }
        first_char = c;
    }

    SantaString::IsNaughty
}

fn repeat_between(s: &str) -> SantaString {
    let vecc: Vec<char> = s.chars().collect();
    for (i, c) in vecc.iter().enumerate() {
        let between = vecc[i + 1];
        let next = vecc[i + 2];
        if *c == next && *c != between {
            println!(
                "Nice repeat: {} = {}",
                s,
                format!("{}{}{}", c, between, next)
            );
            return SantaString::IsNice;
        }
        if i == vecc.len() - 3 {
            return SantaString::IsNaughty;
        }
    }
    SantaString::IsNaughty
}

fn is_this_string_nice(s: &str) -> SantaString {
    match unauthoriszed_strings(s) {
        SantaString::IsNaughty => SantaString::IsNaughty,
        SantaString::IsNice => match twice_in_a_row(s) {
            SantaString::IsNaughty => SantaString::IsNaughty,
            SantaString::IsNice => match vowel(s) {
                SantaString::IsNaughty => SantaString::IsNaughty,
                SantaString::IsNice => SantaString::IsNice,
            },
        },
    }
}

fn unauthoriszed_strings(s: &str) -> SantaString {
    let nono = ["ab", "cd", "pq", "xy"];
    let mut result: SantaString = SantaString::IsNice;
    for no in nono {
        if s.contains(no) {
            result = SantaString::IsNaughty;
            break;
        }
    }
    result
}

fn twice_in_a_row(s: &str) -> SantaString {
    let mut cprev = '1';
    for c in s.chars() {
        if c == cprev {
            return SantaString::IsNice;
        }
        cprev = c;
    }
    SantaString::IsNaughty
}

fn vowel(s: &str) -> SantaString {
    let mut count = 0;
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    for c in s.chars() {
        for v in vowels {
            if c == v {
                count += 1
            };
            if count == 3 {
                return SantaString::IsNice;
            }
        }
    }
    SantaString::IsNaughty
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert!(matches!(
            is_this_string_nice("ugknbfddgicrmopn"),
            SantaString::IsNice
        ));
    }
    #[test]
    fn it_works2() {
        assert!(matches!(
            is_this_string_nice("jchzalrnumimnmhp"),
            SantaString::IsNaughty
        ));
    }
    #[test]
    fn it_works3() {
        assert!(matches!(is_this_string_nice("aaa"), SantaString::IsNice));
    }
    #[test]
    fn it_works4() {
        assert!(matches!(
            is_this_string_nice("haegwjzuvuyypxyu"),
            SantaString::IsNaughty
        ));
    }
    #[test]
    fn it_works5() {
        assert!(matches!(
            is_this_string_nice("dvszwmarrgswjxmb"),
            SantaString::IsNaughty
        ));
    }
    #[test]
    fn part_two1() {
        let inp = is_it_really_nice("qjhvhtzxzqqjkmpb");
        assert!(matches!(inp, SantaString::IsNice))
    }
    #[test]
    fn part_two2() {
        let inp = is_it_really_nice("xxyxx");
        assert!(matches!(inp, SantaString::IsNice))
    }
    #[test]
    fn part_two3() {
        let inp = is_it_really_nice("uurcxstgmygtbstg");
        assert!(matches!(inp, SantaString::IsNaughty))
    }
    #[test]
    fn part_two4() {
        let inp = is_it_really_nice("ieodomkazucvgmuy");
        assert!(matches!(inp, SantaString::IsNaughty))
    }
    #[test]
    fn part_two5() {
        let inp = is_it_really_nice("vxlbxagsmsuuchod");
        assert!(matches!(inp, SantaString::IsNaughty))
    }
    #[test]
    fn part_two6() {
        let inp = is_it_really_nice("xxxddetvrlpzsfpq");
        assert!(matches!(inp, SantaString::IsNaughty))
    }
}
