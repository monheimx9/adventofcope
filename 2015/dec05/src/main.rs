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
            SantaString::IsNaughty => count += 0,
            SantaString::IsNice => count += 1,
        }
    }
    println!("Total nice strings {count}");

    count = the_second_impact(&strings);

    println!("The true total is {count}");
}

fn the_second_impact(ss: &[&str]) -> i32 {
    let mut count = 0;
    count
}

fn is_it_really_nice(s: &str) -> SantaString {
    unimplemented!()
}

fn double_pair(s: &str) -> SantaString {
    unimplemented!()
}

fn repeat_between(s: &str) -> SantaString {
    unimplemented!()
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
}
