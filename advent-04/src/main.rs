use std::ops::RangeInclusive;
use std::str::FromStr;

fn parse_range(s: &str) -> RangeInclusive<u32> {
    let v = s
        .split("-")
        .map(u32::from_str)
        .map(Result::unwrap_or_default)
        .collect::<Vec<_>>();

    v[0]..=v[1]
}

fn parse_pair(s: &str) -> (RangeInclusive<u32>, RangeInclusive<u32>) {
    let pair = s.split(",").collect::<Vec<_>>();
    (parse_range(pair[0]), parse_range(pair[1]))
}

trait FullyContains {
    fn fully_contains(&self, other: &Self) -> bool;
}
impl FullyContains for RangeInclusive<u32> {
    fn fully_contains(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }
}

trait Overlap {
    fn overlap(&self, other: &Self) -> bool;
}
impl Overlap for RangeInclusive<u32> {
    fn overlap(&self, other: &Self) -> bool {
        self.contains(other.start()) || self.contains(other.end())
    }
}

fn main() {
    use std::io::Read;
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let res = input
        .lines()
        .map(parse_pair)
        .filter(|(r1, r2)| r1.fully_contains(&r2) || r2.fully_contains(&r1))
        .collect::<Vec<_>>()
        .len();

    println!("part1: {}", res);

    let res = input
        .lines()
        .map(parse_pair)
        .filter(|(r1, r2)| r1.overlap(&r2) || r2.overlap(&r1))
        .collect::<Vec<_>>()
        .len();

    println!("part2: {}", res);
}
