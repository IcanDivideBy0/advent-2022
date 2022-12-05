use nom::{
    bytes::complete::tag,
    character::complete::{anychar, char, u32},
    multi::separated_list1,
    sequence::delimited,
    IResult,
};

fn parse_crate(s: &str) -> IResult<&str, Option<char>> {
    let result: IResult<&str, char> = delimited(char('['), anychar, char(']'))(s);
    Ok((&s[3..], result.map(|(_, c)| c).ok()))
}

fn parse_crates(s: &str) -> IResult<&str, Vec<Option<char>>> {
    separated_list1(char(' '), parse_crate)(s)
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    count: u32,
    from: usize,
    to: usize,
}

fn parse_instruction(s: &str) -> IResult<&str, Instruction> {
    let (s, _) = tag("move ")(s)?;
    let (s, count) = u32(s)?;
    let (s, _) = tag(" from ")(s)?;
    let (s, from) = u32(s)?;
    let (s, _) = tag(" to ")(s)?;
    let (s, to) = u32(s)?;

    Ok((
        s,
        Instruction {
            count,
            from: (from - 1) as usize,
            to: (to - 1) as usize,
        },
    ))
}

fn main() {
    use std::io::Read;
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let [start, instructions] =
        <[&str; 2]>::try_from(input.split("\n\n").collect::<Vec<&str>>()).unwrap();

    let mut stacks: Vec<Vec<char>> = vec![];
    for line in start.lines() {
        let (_, v) = parse_crates(line).unwrap();
        stacks.resize_with(v.len(), Vec::new);

        for (idx, c) in v.iter().enumerate() {
            if let Some(c) = c {
                stacks[idx].push(*c)
            }
        }
    }
    for c in &mut stacks {
        c.reverse();
    }

    let instructions = instructions
        .lines()
        .map(|s| parse_instruction(s).unwrap().1)
        .collect::<Vec<_>>();

    // part 1
    {
        let mut stacks = stacks.clone();

        for i in &instructions {
            for _ in 0..(i.count) {
                let c = stacks[i.from].pop().unwrap();
                stacks[i.to].push(c)
            }
        }

        let mut res = "".to_owned();
        for s in &mut stacks {
            res.push(s.pop().unwrap())
        }

        println!("part1: {res}");
    }

    // part 2
    {
        let mut stacks = stacks.clone();

        for i in &instructions {
            let mut temp = vec![];
            for _ in 0..(i.count) {
                let c = stacks[i.from].pop().unwrap();
                temp.push(c)
            }
            temp.reverse();
            stacks[i.to].append(&mut temp);
        }

        let mut res = "".to_owned();
        for s in &mut stacks {
            res.push(s.pop().unwrap())
        }

        println!("part2: {res}");
    }
}
