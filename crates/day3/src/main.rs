use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

fn main() {
    let input = include_str!("input.txt");
    let (uniques, connections) = get_part_numbers(input);

    // let n = part1(uniques);
    let n = part2(connections);

    println!("{n}");
}

fn part1(parts: Vec<PartNumber>) -> u32 {
    parts.into_iter().map(|x| x.number).sum()
}

fn part2(parts: HashMap<(usize, usize, char), HashSet<PartNumber>>) -> u32 {
    parts
        .into_iter()
        .filter(|((_, _, c), v)| c == &'*' && v.len() == 2)
        .map(|(_, v)| v.into_iter().map(|p| p.number).product::<u32>())
        .sum()
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PartNumber {
    pub location: (RangeInclusive<usize>, usize),
    pub number: u32,
}

///NB: assumes that (x, y) is inbounds, and (dx, dy) is in [-1, 1]
fn get_coord_in_bounds(
    (x, y): (usize, usize),
    (dx, dy): (i32, i32),
    width: usize,
    height: usize,
) -> Option<(usize, usize)> {
    let mut nx = None;
    let mut ny = None;

    if (x == 0 && dx >= 0) || (x == width - 1 && dx <= 0) || (x > 0 && x < width - 1) {
        nx = Some((x as i32 + dx) as usize);
    }
    if (y == 0 && dy >= 0) || (y == height - 1 && dy <= 0) || (y > 0 && y < height - 1) {
        ny = Some((y as i32 + dy) as usize);
    }

    nx.zip(ny)
}

//returns unique list, then the ones with the parts they're linked to
fn get_part_numbers(
    input: &str,
) -> (
    Vec<PartNumber>,
    HashMap<(usize, usize, char), HashSet<PartNumber>>,
) {
    let mut uniques = vec![];
    let mut already_found = HashSet::new();

    let mut connections: HashMap<(usize, usize, char), HashSet<PartNumber>> = HashMap::new();

    let lines = input.lines().collect::<Vec<_>>();

    let width = lines.first().map(|x| x.len()).unwrap();
    let height = lines.len();

    let lines_as_char_arrays = lines
        .into_iter()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for (y, chars) in lines_as_char_arrays.clone().into_iter().enumerate() {
        for (x, discovering_char) in chars.into_iter().enumerate() {
            if discovering_char != '.' && !discovering_char.is_ascii_digit() {
                for delta in [
                    (-1, -1),
                    (0, -1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                    (0, 1),
                    (-1, 1),
                    (-1, 0),
                ] {
                    if let Some(location) = get_coord_in_bounds((x, y), delta, width, height) {
                        let (search_x, search_y) = location;
                        let char = lines_as_char_arrays[search_y][search_x];

                        if char.is_ascii_digit() {
                            let mut left_most = search_x;
                            let mut right_most = search_x;

                            loop {
                                if left_most == 0 {
                                    break;
                                }
                                if lines_as_char_arrays[search_y][left_most - 1].is_ascii_digit() {
                                    left_most -= 1;
                                } else {
                                    break;
                                }
                            }

                            loop {
                                if right_most == width - 1 {
                                    break;
                                }
                                if lines_as_char_arrays[search_y][right_most + 1].is_ascii_digit() {
                                    right_most += 1;
                                } else {
                                    break;
                                }
                            }

                            let location = (left_most..=right_most, search_y);
                            let number = (lines_as_char_arrays[search_y][left_most..=right_most])
                                .iter()
                                .collect::<String>()
                                .parse()
                                .unwrap();

                            let part = PartNumber {
                                location: location.clone(),
                                number,
                            };

                            if !already_found.contains(&location) {
                                already_found.insert(location);
                                uniques.push(part.clone());
                            }

                            let current = connections.entry((x, y, discovering_char)).or_default();
                            if !current.contains(&part) {
                                current.insert(part);
                            }
                        }
                    }
                }
            }
        }
    }

    (uniques, connections)
}
