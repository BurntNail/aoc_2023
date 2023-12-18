fn main() {
    let input = include_str!("input.txt");
    // let races = parse_all_races(input);
    let races = parse_one_race(input);
    let total = races.iter().map(Race::get_no_ways_to_beat).product::<u64>();
    println!("{total:#?}");
}

#[derive(Copy, Clone, Debug)]
pub struct Race {
    pub time: u64,
    pub record: u64,
}

impl Race {
    pub fn get_no_ways_to_beat(&self) -> u64 {
        let time = self.time as f64;
        let record = self.record as f64;

        let min = time / 2.0 - (time.powi(2) / 4.0 - record).sqrt();
        let max = time / 2.0 + (time.powi(2) / 4.0 - record).sqrt();
        let ways = (max.floor() - min.ceil()) as u64 + 1;

        println!("Time = {time}, Record = {record}");
        println!("\tMin: {min}");
        println!("\tMax: {max}");
        println!("\tWays: {ways}");

        ways
    }
}

pub fn parse_all_races(i: &str) -> Vec<Race> {
    let mut lines = i.lines().map(|i| {
        let mut list = vec![];

        let mut accumulator = String::new();
        for ch in i.chars() {
            if ch.is_ascii_digit() {
                accumulator.push(ch);
            } else if !accumulator.is_empty() {
                list.push(accumulator.parse().unwrap());
                accumulator.clear();
            }
        }

        if !accumulator.is_empty() {
            list.push(accumulator.parse().unwrap());
        }

        list
    });

    let times = lines.next().unwrap();
    let records = lines.next().unwrap();

    times
        .into_iter()
        .zip(records.into_iter())
        .map(|(time, record)| Race { time, record })
        .collect()
}

//returns a vec just to get it to work with the p1 machinery
fn parse_one_race(i: &str) -> Vec<Race> {
    let mut nums = i.lines().map(|x| {
        let chars = x.chars();
        let numbers = chars.filter(|c| c.is_ascii_digit());
        numbers.collect::<String>().parse().unwrap()
    });

    let time = nums.next().unwrap();
    let record = nums.next().unwrap();

    vec![Race { time, record }]
}
