use std::usize;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Node {
    loss: usize,
    total_loss: Option<usize>,
    x: usize,
    y: usize,
    previous: Option<usize>,
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
                    })
                    .collect::<Vec<Node>>()
            })
            .collect()
    }
}
fn solve_1(nodes: &mut [Node], src: usize, dst: usize) -> usize {
    let mut queue: Vec<(usize, Node)> = Vec::new();
    let mut next_src = src;
    while queue.len() < nodes.len() {
        let mut temp_nodes: Vec<(usize, usize)> = Vec::new();
        let previous = src;
        let current_node = nodes[next_src];
        let mut count: usize = 0;
        for n in 0..nodes.len() {
            if is_x_valid(&current_node, &nodes[n]) {
                if !is_at_limit(nodes, n) {
                    let loss = node_loss(&current_node, &nodes[n]);
                    temp_nodes.push((n, loss));
                    count += 1;
                }
            }
            if is_y_valid(&current_node, &nodes[n]) {
                if !is_at_limit(nodes, n) {
                    let loss = node_loss(&current_node, &nodes[n]);
                    temp_nodes.push((n, loss));
                    count += 1;
                }
            }
            if count == 4 {
                break;
            }
        }
        let (next_node, loss) = temp_nodes.iter().min_by_key(|(_, value)| value).unwrap();
        nodes[*next_node].total_loss = Some(*loss);
        nodes[next_src].previous = Some(previous);
        queue.push((next_src, nodes[next_src]));
        next_src = *next_node;
    }
    nodes[dst].total_loss.unwrap()
}
fn node_loss(src: &Node, dst: &Node) -> usize {
    src.total_loss.unwrap_or(0) + dst.loss
}

fn is_at_limit(nodes: &[Node], src: usize) -> bool {
    let mut count: usize = 0;
    let mut new_src: usize = src;
    let (x, y): (usize, usize) = (nodes[src].x, nodes[src].y);
    let mut xy: Vec<(usize, usize)> = Vec::new();
    for n in 0..3 {
        xy.push((nodes[new_src].x, nodes[new_src].y));
        if let Some(x) = nodes[new_src].previous {
            new_src = x;
        } else {
            return false;
        }
    }
    if xy.len() < 3 {
        return false;
    }
    if xy.iter().all(|(x1, _)| x1 == &x) || xy.iter().all(|(_, y1)| y1 == &y) {
        return true;
    }
    false
}

fn is_x_valid(src: &Node, dst: &Node) -> bool {
    dst.x.abs_diff(src.x) == 1 && dst.y == src.y
}
fn is_y_valid(src: &Node, dst: &Node) -> bool {
    dst.y.abs_diff(src.y) == 1 && dst.x == src.x
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1() {
        let s = include_str!("../test.txt");
        let mut nodes = Node::from_str(s);
        let dst = nodes.len() - 1;
        let ans = solve_1(&mut nodes, 0, dst);
        assert_eq!(ans, 102)
    }
}
