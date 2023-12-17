fn main() {
    let input = include_str!("input.txt");
    let races = parse_all_races(input);
    let total = races.iter().map(Race::get_no_ways_to_beat).product::<u32>();
    println!("{total:#?}");
}

#[derive(Copy, Clone, Debug)]
pub struct Race {
    pub time: u32,
    pub record: u32
}

impl Race {
    pub fn get_no_ways_to_beat (&self) -> u32 {
        let time = self.time as f32;
        let record = self.record as f32;

        let min = time / 2.0 - (time.powi(2) / 4.0 - record).sqrt();
        let max = time / 2.0 + (time.powi(2) / 4.0 - record).sqrt();
        let ways = (max.floor() - min.ceil()) as u32 + 1;

        println!("Time = {time}, Record = {record}");
        println!("\tMin: {min}");
        println!("\tMax: {max}");
        println!("\tWays: {ways}");

        ways
    }
}

pub fn parse_all_races (i: &str) -> Vec<Race> {
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

    times.into_iter().zip(records.into_iter()).map(|(time, record)| Race { time, record }).collect()
}
