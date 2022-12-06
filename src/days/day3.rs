use std::collections::HashSet;

static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e',
    'f', 'g', 'h', 'i', 'j',
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't',
    'u', 'v', 'w', 'x', 'y',
    'z',
];

pub fn run() {
    println!("Day3");
    let contents = include_str!("input/day3.txt").to_string();
    part1(contents.clone());
    part2(contents.clone());
}

pub fn part1(contents: String) {
    let lines = contents.lines();
    let mut total = 0;
    for line in lines {
        let mid = line.len() / 2;
        let mut shared: HashSet<char> = HashSet::new();
        let compartment_one = &line[0..mid];
        let compartment_two = &line[mid..line.len()];
        for c in compartment_one.chars() {
            if compartment_two.contains(c) { shared.insert(c); };
        }
        for s in shared {
            let index = ASCII_LOWER.iter().position(|&r| r == s.to_ascii_lowercase()).unwrap() + 1;
            total += if s.is_ascii_uppercase() { index + 26 } else { index };
        }
    }
    println!("Total: {total}")
}

pub fn part2(contents: String) {
    let lines : Vec<&str>= contents.lines().collect();
    let mut total = 0;
    for chunk in lines.chunks(3) {
        let mut shared: HashSet<char> = HashSet::new();
        for c in chunk[0].chars() {
            if chunk[1].contains(c) && chunk[2].contains(c) {
                shared.insert(c);
            }
        }
        for s in shared {
            let index = ASCII_LOWER.iter().position(|&r| r == s.to_ascii_lowercase()).unwrap() + 1;
            total += if s.is_ascii_uppercase() { index + 26 } else { index };
        }
    }
    println!("Total: {total}")
}

