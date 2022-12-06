fn has_duplicates(msg: &[u8]) -> bool {
    for (idx, c) in msg.iter().enumerate() {
        if msg[..idx].contains(c) || msg[(idx + 1)..].contains(c) {
            return true;
        }
    }

    return false;
}

fn main() {
    use std::io::Read;
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let line = input.lines().next().unwrap();

    let mut res = 4;
    for bytes in line.as_bytes().windows(4) {
        if has_duplicates(&bytes) {
            res += 1;
        } else {
            break;
        }
    }

    println!("part1: {res}");

    let mut res = 14;
    for bytes in line.as_bytes().windows(14) {
        if has_duplicates(&bytes) {
            res += 1;
        } else {
            break;
        }
    }

    println!("part1: {res}");
}
