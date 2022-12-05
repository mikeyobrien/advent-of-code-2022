use std::{fs, collections::HashMap};
use std::cmp::Ordering::{Less, Equal, Greater};

pub fn run() {
    println!("Day2");
    let contents = fs::read_to_string("day2.txt")
        .expect("Should have been able to read the file");
    part1(contents.clone());
}

pub fn part1(contents: String) {
    let mut score = 0;
    let values = HashMap::from([
        ("A", 1),
        ("X", 1),
        ("B", 2),
        ("Y", 2),
        ("C", 3),
        ("Z", 3)
    ]);
    for line in contents.split("\n") {
        if line.is_empty() { continue; }
        let res: Vec<&str> = line.split(" ").collect();
        let opponent = values.get(res[0]).unwrap();
        let me = values.get(res[1]).unwrap();
        let round_score = match me.cmp(&opponent) {
            Equal => me + 3, // draw
            Less => if opponent == &3 && me == &1 { me + 6 } else { *me },
            Greater =>  if opponent == &1 && me == &3 { *me } else { me + 6 },
        };
        score += round_score;
    }
    println!("Total: {score}")
}
