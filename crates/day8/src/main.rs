use std::collections::HashMap;

type Node = usize;

fn main() {
    let input = include_str!("input.txt");
    let mut lines = input.lines();

    let directions: Vec<bool> = lines.next().unwrap().chars().map(|c| c == 'L').collect();
    let _ = lines.next();
    // let (start, target, graph) = parse_graph_p1(lines.map(|x| x.to_string() + "\n").collect());
    let (start, target, graph) = parse_graph_p2(lines.map(|x| x.to_string() + "\n").collect());

    println!("Navigating from {start:?} to {target:?}");

    let steps = steps_till(graph, directions, start, target);
    println!("{steps}");
}

fn steps_till(
    graph: Vec<NodeContents>,
    directions: Vec<bool>,
    start: Vec<Node>,
    target: Vec<Node>,
) -> u128 {
    if start.is_empty() {
        return 0;
    }

    let mut steps = 1;

    let mut current = start;
    let mut directions = directions.into_iter().cycle();
    loop {
        for current in current.iter_mut() {
            let node_contents = graph[*current];
            let new_node = if directions.next().unwrap() {
                node_contents.left
            } else {
                node_contents.right
            };
            *current = new_node;
        }

        if current.iter().all(|x| target.contains(x)) {
            break;
        }

        steps += 1;
    }

    steps
}

//first, target, nc
fn parse_graph_p1(lines: String) -> (Vec<Node>, Vec<Node>, Vec<NodeContents>) {
    let variants: HashMap<String, usize> = lines
        .lines()
        .map(|l| (l[0..3]).to_string())
        .enumerate()
        .map(|(a, b)| (b, a))
        .collect();

    let mut first = None;
    let mut map = vec![None; variants.len()];

    for line in lines.lines() {
        let start = variants.get(&line[0..3]).unwrap();
        if first.is_none() {
            first = Some(*start);
        }

        let left = variants.get(&line[7..10]).unwrap();
        let right = variants.get(&line[12..15]).unwrap();

        map[*start] = Some(NodeContents {
            left: *left,
            right: *right,
        });
    }

    (
        vec![variants.get("AAA").copied().unwrap()],
        vec![variants.get("ZZZ").copied().unwrap()],
        map.into_iter().map(|x| x.unwrap()).collect(),
    )
}

fn parse_graph_p2(lines: String) -> (Vec<Node>, Vec<Node>, Vec<NodeContents>) {
    let variants: HashMap<String, usize> = lines
        .lines()
        .map(|l| (l[0..3]).to_string())
        .enumerate()
        .map(|(a, b)| (b, a))
        .collect();
    let starts = variants
        .clone()
        .into_iter()
        .filter_map(|(s, idx)| if s.ends_with('A') { Some(idx) } else { None })
        .collect();
    let finishes = variants
        .clone()
        .into_iter()
        .filter_map(|(s, idx)| if s.ends_with('A') { Some(idx) } else { None })
        .collect();

    let mut map = vec![None; variants.len()];

    for line in lines.lines() {
        let start = variants.get(&line[0..3]).unwrap();
        let left = variants.get(&line[7..10]).unwrap();
        let right = variants.get(&line[12..15]).unwrap();

        map[*start] = Some(NodeContents {
            left: *left,
            right: *right,
        });
    }

    (
        starts,
        finishes,
        map.into_iter().map(|x| x.unwrap()).collect(),
    )
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct NodeContents {
    pub left: Node,
    pub right: Node,
}
