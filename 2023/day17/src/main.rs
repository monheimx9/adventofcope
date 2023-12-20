use std::usize;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Node {
    loss: usize,
    total_loss: Option<usize>,
    x: usize,
    y: usize,
    previous: Option<usize>,
    done: bool,
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
                        done: false,
                    })
                    .collect::<Vec<Node>>()
            })
            .collect()
    }
}
fn solve_1(nodes: &mut [Node], src: usize, dst: usize) -> usize {
    let mut queue: Vec<(usize, Node)> = Vec::new();
    nodes[src].previous = Some(src);
    for o in (0..nodes.len()).cycle() {
        if !nodes[o].done {
            if let Some(_x) = nodes[o].previous {
                let current_node = nodes[o];
                for n in 0..nodes.len() {
                    if o != n && nodes[o].previous != Some(n) {
                        if is_x_valid(&current_node, &nodes[n]) {
                            if !is_at_limit(nodes, n) {
                                set_node(o, &current_node, &mut nodes[n]);
                            }
                        }
                        if is_y_valid(&current_node, &nodes[n]) {
                            if !is_at_limit(nodes, n) {
                                set_node(o, &current_node, &mut nodes[n]);
                            }
                        }
                    }
                }
                nodes[o].done = true;
                queue.push((o, nodes[o]));
            }
        }
        if queue.len() >= nodes.len() {
            break;
        }
    }
    nodes[dst].total_loss.unwrap()
}
fn set_node(n: usize, src: &Node, dst: &mut Node) {
    if dst.total_loss.is_none() {
        dst.total_loss = Some(src.total_loss.unwrap_or(0) + dst.loss);
        dst.previous = Some(n);
    } else if dst.total_loss.unwrap() > src.total_loss.unwrap_or(0) + dst.loss {
        dst.total_loss = Some(src.total_loss.unwrap_or(0) + dst.loss);
        dst.previous = Some(n);
    };
}

fn is_at_limit(nodes: &[Node], src: usize) -> bool {
    let mut new_src: usize = src;
    let (x, y): (usize, usize) = (nodes[src].x, nodes[src].y);
    let mut xy: Vec<(usize, usize)> = Vec::new();
    for _ in 0..4 {
        xy.push((nodes[new_src].x, nodes[new_src].y));
        if let Some(p) = nodes[new_src].previous {
            new_src = p;
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
    fn pt1_1() {
        let s = include_str!("../test.txt");
        let mut nodes = Node::from_str(s);
        let dst = nodes.len() - 1;
        let ans = solve_1(&mut nodes, 0, dst);
        assert_eq!(ans, 102)
    }
    #[test]
    fn pt1_2() {
        let s = include_str!("../input.txt");
        let mut nodes = Node::from_str(s);
        let dst = nodes.len() - 1;
        let ans = solve_1(&mut nodes, 0, dst);
        assert_eq!(ans, 102)
    }
}
