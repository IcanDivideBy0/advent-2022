use std::io::Read;
use std::str::FromStr;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut elves_calories = input
        .split("\n\n")
        .map(|s| {
            s.lines()
                .map(u32::from_str)
                .map(Result::unwrap_or_default)
                .sum::<u32>()
        })
        .collect::<Vec<_>>();

    elves_calories.sort();
    elves_calories.reverse();

    // part1
    println!("part1: {}", elves_calories[0]);

    // part2
    println!("part2: {}", elves_calories.iter().take(3).sum::<u32>());
}
