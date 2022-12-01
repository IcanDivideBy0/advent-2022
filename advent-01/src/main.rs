use std::ops::Add;
use std::str::FromStr;

fn main() {
    let content = std::fs::read_to_string("./input.txt").unwrap();

    let mut elves_calories = content
        .split("\n\n")
        .map(|s| {
            s.split("\n")
                .map(u32::from_str)
                .map(Result::unwrap_or_default)
                .reduce(u32::add)
                .unwrap_or_default()
        })
        .collect::<Vec<_>>();

    elves_calories.sort();
    elves_calories.reverse();

    // part1
    dbg!(elves_calories[0]);

    // part2
    dbg!(elves_calories[0] + elves_calories[1] + elves_calories[2]);
}
