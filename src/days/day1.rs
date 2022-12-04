use std::fs;

pub fn run() {
    println!("Day1");
    let contents = fs::read_to_string("day1.txt")
            .expect("Should have been able to read the file");
    part1(contents.clone());
    part2(contents.clone());
}

pub fn part1(contents: String) {
    let calories = contents.split("\n\n");
    let mut max = 0;
    for c in calories {
        let summed_calories : i32 = c.split("\n").filter(|&x| !x.is_empty()).map(|x| x.to_string().parse::<i32>().unwrap()).sum();
        if summed_calories > max {
            max = summed_calories
        }
    }
    println!("\tpart1: {max}")
}

pub fn part2(contents: String) {
    let calories = contents.split("\n\n");
    let mut top_three: Vec<i32> = vec![0,0,0];

    for c in calories {
        let summed_calories : i32 = c.split("\n")
                                    .filter(|&x| !x.is_empty())
                                    .map(|x| x.to_string().parse::<i32>().unwrap())
                                    .sum();
        for i in 0..top_three.len() {
            if summed_calories > top_three[i] {
                top_three[i] = summed_calories;
                break;
            }
        }
        top_three.sort();
    }
    let total: i32 = top_three.iter().sum();
    println!("\tpart2: {total}");
}
