use std::io::Read;

trait Priority {
    fn priority(self) -> u32;
}

impl Priority for char {
    fn priority(self) -> u32 {
        match self {
            'a'..='z' => (self as u32) - 96,
            'A'..='Z' => (self as u32) - 64 + 26,
            _ => panic!("Invalid char"),
        }
    }
}

fn letter_match(s1: &str, s2: &str) -> Option<char> {
    for c in s1.chars() {
        if s2.find(c).is_some() {
            return Some(c);
        }
    }

    None
}

fn letter_match3(s1: &str, s2: &str, s3: &str) -> Option<char> {
    for c in s1.chars() {
        if s2.find(c).is_some() & s3.find(c).is_some() {
            return Some(c);
        }
    }

    None
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let res = input
        .lines()
        .map(|s| {
            let mid = s.len() / 2;
            let comp1 = &s[0..mid];
            let comp2 = &s[mid..];

            letter_match(comp1, comp2).unwrap().priority()
        })
        .sum::<u32>();

    println!("part1: {}", res);

    let res = input
        .lines()
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .map(|chunks| {
            let [s1, s2, s3] = <&[&str; 3]>::try_from(chunks).unwrap();
            letter_match3(s1, s2, s3).unwrap().priority()
        })
        .sum::<u32>();

    println!("part2: {:?}", res);
}
