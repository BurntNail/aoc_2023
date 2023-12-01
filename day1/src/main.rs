fn main() {
    let input = include_str!("input.txt");
    part1(input);
}


fn part1 (input: &'static str) {
    let max = input.lines().fold(0, |acc, line| {
        let numbers =  line.chars().filter_map(|c| c.is_ascii_digit().then_some(c as u32 - b'0' as u32)).collect::<Vec<_>>();
        let first = numbers.first().unwrap();

        acc + (first * 10 + numbers.last().unwrap_or(first))
    });

    println!("Part 1: {max}");
}