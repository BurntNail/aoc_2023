fn main() {
    let input = include_str!("input.txt");
    //let r = part1(input);
    let r = part2(input);

    println!("{r}");
}


fn part1 (input: &str) -> u32{
    input.lines().fold(0, |acc, line| {
        println!("Working on {line}");
        let numbers =  line.chars().filter_map(|c| c.is_ascii_digit().then_some(c as u32 - b'0' as u32)).collect::<Vec<_>>();
        let first = numbers.first().unwrap();

        acc + (first * 10 + numbers.last().unwrap_or(first))
    })
}


fn part2 (input: &str) -> u32{
    let search_targets = [
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let mut sum = 0;

    for line in input.lines().map(|x| x.chars().collect::<Vec<_>>()) {
        let len = line.len();

        let mut first = None;
        let mut last = None;

        for i in 0..len {
            for (target, result) in search_targets {

                let works = if target.len() > 1 {

                    let end = i+target.len();
                    if end > len {
                        continue;
                    }
        
                    (line[i..end]).iter().zip(target.chars()).all(|(a, b)| *a == b)
                } else {
                    target.chars().next().unwrap() == line[i]
                };                

                if works {
                    if first.is_none() {
                        first = Some(result);
                    }
                    
                    last = Some(result);
                }
            }
        }

        // println!("{first:?} & {last:?}");

        let line_count = match (first, last) {
            (Some(a), Some(b)) => a * 10 + b,
            _ => panic!("Failed at {line:?}")
        };

        sum += line_count;
    }

    sum
}