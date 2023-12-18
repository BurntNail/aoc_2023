use std::{fmt::Debug, str::FromStr, collections::HashMap};

fn main() {
    let input = include_str!("input.txt");

    let (games, last_id) = get_games(input);
    let scores = get_scores(&games);

    // let n = part1(scores);
    let n = part2(scores, last_id);
    println!("{n}");
}

fn get_scores (games: &HashMap<u32, Game>) -> HashMap<u32, u32> {
    games.iter().map(|(id, Game { winning_numbers, my_numbers })| (*id, my_numbers.iter().map(|x| if winning_numbers.contains(x) {1} else {0}).sum())).collect()
}

fn part1(scores: HashMap<u32, u32>) -> u32 {
    scores.values().copied().map(|points| {
        if points > 0 {
            let mut i = 1;
            for _ in 1..points {
                i *= 2;
            }
            i
        } else {
            0
        }
    }).sum()
}

fn part2 (scores: HashMap<u32, u32>, last_id: u32) -> u32 {
    let mut counts = HashMap::new();
    for k in scores.keys() {
        counts.insert(*k, 1);
    }


    for id in 1..=last_id {
        let score = scores.get(&id).copied().unwrap();
        let current_count = counts.get(&id).copied().unwrap();

        for new_id in (1..=score).map(|x| x + id) {
            counts.entry(new_id).and_modify(|x| *x += current_count);
        }
    }


    counts.values().sum()
}

fn advance_iter<I: Iterator> (i: &mut I, n: usize) {
    for _ in 0..n {
        let _ = i.next();
    }
}

fn get_t<T: FromStr> (i: &mut impl Iterator<Item = char>, len: usize) -> T
where <T as FromStr>::Err: Debug {
    let mut tmp = String::new();
    for _ in 0..len {
        tmp.push(i.next().unwrap());
    }
    tmp.trim().parse().unwrap()
}

#[derive(Clone, Debug)]

pub struct Game {
    pub winning_numbers: Vec<u32>,
    pub my_numbers: Vec<u32>
}

fn get_games (input: &str) -> (HashMap<u32, Game>, u32) {
    let mut games = HashMap::new();
    let mut last_id = 1;

    for line in input.lines() {
        let mut chars = line.chars();
        advance_iter(&mut chars, 5);

        let id = get_t(&mut chars, 3);
        advance_iter(&mut chars, 2);

        let winning_numbers = {
            let mut list = vec![];
            for _ in 0..10 {
                list.push(get_t(&mut chars, 2));
                advance_iter(&mut chars, 1);
            }
            list
        };
        advance_iter(&mut chars, 2);
        let my_numbers = {
            let mut list = vec![];
            for _ in 0..25 {
                list.push(get_t(&mut chars, 2));
                advance_iter(&mut chars, 1);
            }
            list
        };

        let game = Game { winning_numbers, my_numbers };
        games.insert(id, game);
        last_id = id;
    }

    (games, last_id)
}