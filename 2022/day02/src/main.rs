use day02::Game;

fn main() {
    println!("Hello, world!");
    pt1();
    pt2();
}
fn pt1() {
    let s = include_str!("../input.txt");
    assert_eq!(Game::from_str(s).score(), 14531)
}
fn pt2() {
    let s = include_str!("../input.txt");
    assert_eq!(Game::from_str(s).score_new(), 11258)
}

#[cfg(test)]
mod tests {
    use day02::Game;
    use super::*;

    #[test]
    fn pt1_1() {
        let s = include_str!("../sample.txt");
        assert_eq!(Game::from_str(s).score(), 15)
    }
    #[test]
    fn pt2_1() {
        let s = include_str!("../sample.txt");
        assert_eq!(Game::from_str(s).score_new(), 12)
    }

}
