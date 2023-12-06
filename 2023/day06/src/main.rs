fn main() {
    println!("Hello, world!");
    let r1 = wins(59, 597);
    let r2 = wins(79, 1234);
    let r3 = wins(65, 1032);
    let r4 = wins(75, 1328);
    let r = r1 * r2 * r3 * r4;
    println!("Part one: {r}");

    let r = wins(59796575, 597123410321328);
    println!("Part two: {r}");
}

fn wins(t: u64, d: u64) -> u32 {
    let mut count: u32 = 0;
    for i in 1..t {
        let o = (t - i) * i;
        if o > d {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partone1() {
        let r = wins(7, 9);
        assert_eq!(r, 4)
    }
    #[test]
    fn partone2() {
        let r = wins(15, 40);
        assert_eq!(r, 8)
    }
    #[test]
    fn partone3() {
        let r = wins(30, 200);
        assert_eq!(r, 9)
    }
    #[test]
    fn parttwo() {
        let r = wins(71530, 970200);
        assert_eq!(r, 71503)
    }
}
