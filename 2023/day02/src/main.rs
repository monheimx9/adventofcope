#[derive(Default)]
struct SantaCube {
    red: u32,
    green: u32,
    blue: u32,
}

impl SantaCube {
    fn from_str(s: &str) -> SantaCube {
        let cubes: Vec<&str> = s.split(',').collect();
        let (mut r, mut g, mut b) = (0, 0, 0);
        for cube in cubes {
            match cube {
                x if x.contains("red") => {
                    r = SantaCube::find_color(x, "red");
                }
                x if x.contains("green") => {
                    g = SantaCube::find_color(x, "green");
                }
                x if x.contains("blue") => {
                    b = SantaCube::find_color(x, "blue");
                }
                _ => {}
            }
        }
        SantaCube {
            red: r,
            green: g,
            blue: b,
        }
    }
    fn find_color(s: &str, color: &str) -> u32 {
        if s.contains(color) {
            parse_digits(s)
        } else {
            0
        }
    }
}

struct SantaGame(u32, Vec<SantaCube>);

impl SantaGame {
    // add code here
    fn from_str(s: &str) -> SantaGame {
        let game: Vec<&str> = s.split(':').collect();
        let id = parse_digits(game[0]);
        let draws: Vec<&str> = game[1].split(';').collect();
        let mut cubes: Vec<SantaCube> = Vec::new();
        for draw in draws {
            cubes.push(SantaCube::from_str(draw))
        }

        SantaGame(id, cubes)
    }
    fn is_game_possible(&self, max_r: u32, max_g: u32, max_b: u32) -> bool {
        for cube in &self.1 {
            if max_r < cube.red || max_g < cube.green || max_b < cube.blue {
                return false;
            }
        }
        true
    }
}

fn parse_digits(s: &str) -> u32 {
    let id: Vec<u32> = s
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    id.iter().fold(0, |acc, elem| acc * 10 + elem)
}

fn main() {
    println!("Hello, world!");
    let input: Vec<SantaGame> = include_str!("../input.txt")
        .lines()
        .map(SantaGame::from_str)
        .collect();
    let total_part_one = input
        .iter()
        .filter(|f| f.is_game_possible(12, 13, 14))
        .map(|j| j.0)
        .sum::<u32>();
    println!("Cumulated games id's from part one is {total_part_one}");
}

//Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
//Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
//Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
//Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
//Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one1() {
        assert!(
            SantaGame::from_str("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")
                .is_game_possible(12, 13, 14)
        )
    }
    #[test]
    fn part_one2() {
        assert!(SantaGame::from_str(
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"
        )
        .is_game_possible(12, 13, 14))
    }
    #[test]
    fn part_one3() {
        assert!(!SantaGame::from_str(
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
"
        )
        .is_game_possible(12, 13, 14))
    }
    #[test]
    fn part_one() {
        let input = [
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];
        let santa_games: Vec<SantaGame> = input.into_iter().map(SantaGame::from_str).collect();
        assert_eq!(
            santa_games
                .iter()
                .filter(|s| s.is_game_possible(12, 13, 14))
                .map(|f| f.0)
                .sum::<u32>(),
            8
        )
    }
}
