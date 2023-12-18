use std::usize;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Node {
    loss: usize,
    total_loss: Option<usize>,
    x: usize,
    y: usize,
    previous: Option<usize>,
    max_x: Option<usize>,
    max_y: Option<usize>,
}
impl Node {
    fn from_str(s: &str) -> Vec<Self> {
        let ss = s.lines().collect::<Vec<_>>();
        ss.iter()
            .enumerate()
            .flat_map(|(x, l)| {
                l.chars()
                    .enumerate()
                    .map(|(y, c)| Node {
                        loss: c.to_digit(10).unwrap() as usize,
                        total_loss: None,
                        x,
                        y,
                        previous: None,
                        max_x: None,
                        max_y: None,
                    })
                    .collect::<Vec<Node>>()
            })
            .collect()
    }
}
fn solve_1(nodes: &mut [Node], src: &mut usize, dst: usize) -> usize {
    let mut queue: Vec<(usize, &Node)> = Vec::new();
    while queue.len() < nodes.len() {
        let current_node = nodes[*src];
        for n in 0..nodes.len() {}

        todo!()
    }
    todo!()
}

fn is_x_valid(src: &Node, dst: &Node) {}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1() {
        let s = include_str!("../test.txt");
        let nodes = Node::from_str(s);
        println!("OK")
    }
}
