use grid::*;
use std::collections::BTreeMap;
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
        // if !nodes[o].done {
        let mut temp: BTreeMap<usize, usize> = BTreeMap::new();
        if let Some(_) = nodes[o].previous {
            let current_node = nodes[o];
            for n in 0..nodes.len() {
                if o != n && current_node.previous != Some(n) {
                    if is_x_valid(&current_node, &nodes[n]) {
                        if !is_at_limit(nodes, o, n) {
                            temp.insert(nodes[n].loss, n);
                            // set_node(o, &current_node, &mut nodes[n]);
                        }
                    }
                    if is_y_valid(&current_node, &nodes[n]) {
                        if !is_at_limit(nodes, o, n) {
                            temp.insert(nodes[n].loss, n);
                            // set_node(o, &current_node, &mut nodes[n]);
                        }
                    }
                }
            }

            for t in temp.iter() {
                set_node(o, &current_node, &mut nodes[*t.1]);
            }
            nodes[o].done = true;
            // queue.push((o, nodes[o]));
        }
        // }
        let dones = nodes.iter().filter(|f| f.done).count();
        println!("{dones}");
        if dones == nodes.len() {
            break;
        }
        // if queue.len() >= nodes.len() - 1 {
        //     break;
        // }
    }
    nodes[dst].total_loss.unwrap()
}
fn set_node(n: usize, src: &Node, dst: &mut Node) {
    if dst.total_loss.is_none() {
        dst.total_loss = Some(src.total_loss.unwrap_or(0) + dst.loss);
        dst.previous = Some(n);
    } else if dst.total_loss.unwrap() >= src.total_loss.unwrap_or(0) + dst.loss {
        dst.total_loss = Some(src.total_loss.unwrap_or(0) + dst.loss);
        dst.previous = Some(n);
    };
}

fn is_at_limit(nodes: &[Node], src: usize, dst: usize) -> bool {
    let mut new_src: usize = src;
    let (x, y): (usize, usize) = (nodes[dst].x, nodes[dst].y);
    let mut xy: Vec<(usize, usize)> = Vec::new();
    xy.push((nodes[dst].x, nodes[dst].y));
    for _ in 0..=4 {
        xy.push((nodes[new_src].x, nodes[new_src].y));
        if let Some(p) = nodes[new_src].previous {
            if new_src == p {
                break;
            }
            new_src = p;
        } else {
            break;
        }
    }
    if xy.len() < 4 {
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
fn print_grid(g: &Grid<char>) {
    for n in 0..g.rows() {
        let line = g.iter_row(n).collect::<String>();
        println!("{}", line);
    }
    println!("--------------------------------------");
}

#[cfg(test)]
mod tests {
    use std::char;

    use grid::Grid;

    use super::*;

    #[test]
    fn pt1_1() {
        let s = include_str!("../test.txt");
        let mut nodes = Node::from_str(s);
        let dst = nodes.len() - 1;
        let ans = solve_1(&mut nodes, 0, dst);
        let mut g: Grid<char> = Grid::new(13, 13);
        for n in nodes.iter() {
            let no = g.get_mut(n.x, n.y).unwrap();
            *no = char::from((n.loss as u8) + b'0');
        }
        let mut dst = dst;
        loop {
            let nod = nodes[dst];
            let no = g.get_mut(nod.x, nod.y).unwrap();
            *no = '#';
            if dst == nod.previous.unwrap() {
                break;
            }
            dst = nod.previous.unwrap();
        }
        print_grid(&g);
        assert_eq!(ans, 102)
    }
    #[test]
    fn pt1_2() {
        let s = include_str!("../input.txt");
        let mut nodes = Node::from_str(s);
        let dst = nodes.len() - 1;
        let ans = solve_1(&mut nodes, 0, dst);
        let mut g: Grid<char> = Grid::new(13, 13);
        for n in nodes.iter() {
            let no = g.get_mut(n.x, n.y).unwrap();
            *no = char::from((n.loss as u8) + b'0');
        }
        let mut dst = dst;
        loop {
            let nod = nodes[dst];
            let no = g.get_mut(nod.x, nod.y).unwrap();
            *no = '#';
            if dst == nod.previous.unwrap() {
                break;
            }
            dst = nod.previous.unwrap();
        }
        print_grid(&g);
        assert_eq!(ans, 102)
    }
}
