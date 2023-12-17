use std::collections::HashMap;

pub type Node = [u8; 3];

fn main() {
    let input = include_str!("input.txt");
    let mut lines = input.lines();

    let directions: Vec<bool> = lines.next().unwrap().chars().map(|c| c == 'L').collect();
    skip_non_owning(&mut lines, 1);
    let (start, graph) = parse_graph(lines);
    let steps = steps_till(graph, directions, start, [25, 25, 25]);

    println!("{steps}");
}

fn steps_till (graph: HashMap<Node, NodeContents>, directions: Vec<bool>, start: Node, target: Node) -> u32 {
    let mut steps = 0;

    let mut i = 0;
    let mut current = start;
    loop {
        steps += 1;
        let node_contents = graph.get(&current).unwrap();
        let new_node = if directions[i] {
            node_contents.left
        } else {
            node_contents.right
        };

        if new_node == target {
            break;
        }

        current = new_node;
        i += 1;
        i %= directions.len();
    }


    steps
}

fn parse_graph<'a> (lines: impl Iterator<Item = &'a str>) -> (Node, HashMap<Node, NodeContents>) {
    fn str_to_node (s: impl Iterator<Item = char>) -> Node {
        let mapped = s.map(|c| c as u8 - b'A').collect::<Vec<_>>();
        mapped.try_into().unwrap()
    }

    let mut map = HashMap::new();
    let mut first = None;

    for line in lines {
        let mut chars = line.chars();

        let start = str_to_node(take_non_owning(&mut chars, 3).into_iter());
        skip_non_owning(&mut chars, 4);
        let left = str_to_node(take_non_owning(&mut chars, 3).into_iter());
        skip_non_owning(&mut chars, 2);
        let right = str_to_node(take_non_owning(&mut chars, 3).into_iter());

        if first.is_none() {
            first = Some(start);
        }

        map.insert(start, NodeContents{left, right});
    }

    (first.unwrap(), map)
}

fn take_non_owning<T> (i: &mut impl Iterator<Item = T>, n: usize) -> Vec<T> {
    let mut v = vec![];
    for _ in 0..n {
        v.push(i.next().unwrap());
    }
    v
}

fn skip_non_owning(i: &mut impl Iterator, n: usize) {
    for _ in 0..n {
        let _ = i.next();
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct NodeContents {
    pub left: Node,
    pub right: Node,
}