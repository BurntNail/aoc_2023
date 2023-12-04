use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let pns = get_part_numbers(input);

    let n = part1(pns);

    println!("{n}");
}

fn part1 (parts: HashMap<(usize, usize), PartNumber>) -> u32 {
    parts.values().map(|x| x.number).sum()
}

#[derive(Copy, Clone, Debug)]
pub struct PartNumber {
    pub number: u32,
    pub symbol_discovered_by: char,
}

fn get_coord_in_bounds ((x, y): (usize, usize), (dx, dy): (i32, i32), width: usize, height: usize) -> Option<(usize, usize)> {
    let mut nx = None;
    let mut ny = None;

    if (dx < 0 && x > 1) || (dx > 0 && x < (width - 2)){
        nx = Some((x as i32 + dx) as usize);
    }
    if (dy < 0 && y > 1) || (dy > 0 && y < (height - 2)) {
        ny = Some((y as i32 + dy) as usize);
    }

    nx.zip(ny)
}

fn get_part_numbers (input: &str) -> HashMap<(usize, usize), PartNumber> {
    let mut list = HashMap::new();

    let lines = input.lines().collect::<Vec<_>>();

    let width = lines.get(0).map(|x| x.len()).unwrap();
    let height = lines.len();

    let lines_as_char_arrays = lines.into_iter().map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    for (y, chars) in lines_as_char_arrays.clone().into_iter().enumerate() {
        for (x, discovering_char) in chars.into_iter().enumerate() {
            if discovering_char != '.' && !discovering_char.is_ascii_digit() {
                //we're at a symbol

                for delta in [(-1, -1), (0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0)] {
                    if let Some(location) = get_coord_in_bounds((x, y), delta, width, height) {
                        let (x, y) = location;
                        let char = lines_as_char_arrays[y][x];

                        if char.is_ascii_digit() {                           
                            let mut left_most = x;
                            let mut right_most = x;

                            loop {
                                if left_most == 0 {
                                    break;
                                }
                                if lines_as_char_arrays[y][left_most - 1].is_ascii_digit() {
                                    left_most -= 1;
                                } else {
                                    break;
                                }
                            }

                            loop {
                                if right_most == width - 1 {
                                    break;
                                }
                                if lines_as_char_arrays[y][right_most + 1].is_ascii_digit() {
                                    right_most += 1;
                                } else {
                                    break;
                                }
                            }

                            let st = (&lines_as_char_arrays[y][left_most..=right_most]).iter().collect::<String>();
                            println!("{st:?}");
                            let number = st.parse().unwrap();
                            println!("{number}");
                            

                            list.insert(location, PartNumber { number, symbol_discovered_by: discovering_char });
                        }
                    }
                }
            }
        }
    }

    list
}