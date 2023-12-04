use std::collections::HashMap;
use nom::{IResult, bytes::complete::tag, combinator::map, branch::alt};
use nom::character::complete::u32 as nom_u32;
use nom::multi::many0;
use nom::sequence::tuple;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Colour {
    Red,
    Green,
    Blue
}

#[derive(Clone)]
pub struct Round(pub HashMap<Colour, u32>);
impl Round {
    pub fn parse (i: &str) -> IResult<&str, Self> {
        let (i, v) = many0(tuple((
            nom_u32,
            alt((
                map(tag(" red"), |_| Colour::Red),
                map(tag(" green"), |_| Colour::Green),
                map(tag(" blue"), |_| Colour::Blue),
            )),
            alt((tag(", "), tag("")))
        )))(i)?;

        let mut map = HashMap::new();

        for (count, el, _) in v {
            *map.entry(el).or_insert(0) += count;
        }


        Ok((i, Self(map)))
    }
}

#[derive(Clone)]
pub struct Game (pub u32, pub Vec<Round>);
impl Game {
    pub fn parse (i: &str) -> IResult<&str, Self> {
        let (i, (_, id, _)) = tuple((tag("Game "), nom_u32, tag(": ")))(i)?;

        let mut rounds = vec![];
        for round in i.split("; ") {
            let (_, round) = Round::parse(round)?;
            rounds.push(round);
        }

        Ok(("", Self(id, rounds)))
    }
}

fn part1(input: &str) -> u32 {
    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;
    const MAX_BLUE: u32 = 14;

    let mut sum = 0;
    for line in input.lines() {
        let (_, game) = Game::parse(line).unwrap();

        println!("Game {}", game.0);

        let mut failed = false;
        for round in game.1.into_iter().map(|x| x.0) {
            println!("\t{round:?}");
            if round.get(&Colour::Red).copied().unwrap_or_default() > MAX_RED || round.get(&Colour::Green).copied().unwrap_or_default() > MAX_GREEN || round.get(&Colour::Blue).copied().unwrap_or_default() > MAX_BLUE {

                println!("\t\tSTINKY");

                failed = true;
                break;
            }
        }

        if failed {
            sum += game.0;
        }
    }

    sum
}


fn main () {
    let input = include_str!("input.txt");

    let sum = part1(input);
    println!("{sum}");
}