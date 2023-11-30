use regex::{self, bytes::Regex};
fn main() {
    println!("Hello, world!");
}

fn is_this_string_nice(s: &str) -> bool {
    let re = Regex::new(r"");
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
