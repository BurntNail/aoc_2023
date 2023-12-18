use nom::character::complete::u32 as nom_u32;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::{branch::alt, bytes::complete::tag, combinator::map, IResult};
use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Colour {
    Red,
    Green,
    Blue,
}

#[derive(Clone)]
pub struct Round(pub HashMap<Colour, u32>);
impl Round {
    pub fn parse(i: &str) -> IResult<&str, Self> {
        let (i, v) = many0(tuple((
            nom_u32,
            alt((
                map(tag(" red"), |_| Colour::Red),
                map(tag(" green"), |_| Colour::Green),
                map(tag(" blue"), |_| Colour::Blue),
            )),
            alt((tag(", "), tag(""))),
        )))(i)?;

        let mut map = HashMap::new();

        for (count, el, _) in v {
            *map.entry(el).or_insert(0) += count;
        }

        Ok((i, Self(map)))
    }
}

#[derive(Clone)]
pub struct Game(pub u32, pub Vec<Round>);
impl Game {
    pub fn parse(i: &str) -> IResult<&str, Self> {
        let (i, (_, id, _)) = tuple((tag("Game "), nom_u32, tag(": ")))(i)?;

        let mut rounds = vec![];
        for round in i.split("; ") {
            let (_, round) = Round::parse(round)?;
            rounds.push(round);
        }

        Ok(("", Self(id, rounds)))
    }
}

fn parse_games(input: &str) -> Vec<Game> {
    input.lines().map(|l| Game::parse(l).unwrap().1).collect()
}

fn part2(games: Vec<Game>) -> u32 {
    let mut sum = 0;

    for game in games {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for round in game.1.into_iter().map(|x| x.0) {
            max_red = max_red.max(round.get(&Colour::Red).copied().unwrap_or_default());
            max_green = max_green.max(round.get(&Colour::Green).copied().unwrap_or_default());
            max_blue = max_blue.max(round.get(&Colour::Blue).copied().unwrap_or_default());
        }

        let power = max_red * max_green * max_blue;
        if power == 0 {
            panic!()
        }

        sum += power;
    }

    sum
}

fn part1(games: Vec<Game>) -> u32 {
    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;
    const MAX_BLUE: u32 = 14;

    let mut sum = 0;
    for game in games {
        let mut failed = false;
        for round in game.1.into_iter().map(|x| x.0) {
            if round.get(&Colour::Red).copied().unwrap_or_default() > MAX_RED
                || round.get(&Colour::Green).copied().unwrap_or_default() > MAX_GREEN
                || round.get(&Colour::Blue).copied().unwrap_or_default() > MAX_BLUE
            {
                failed = true;
                break;
            }
        }

        if !failed {
            sum += game.0;
        }
    }

    sum
}

fn main() {
    let input = include_str!("input.txt");
    let games = parse_games(input);

    // let sum = part1(games);
    let sum = part2(games);
    println!("{sum}");
}
