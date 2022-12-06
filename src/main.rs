mod days;

pub use crate::days::day1;
pub use crate::days::day2;
pub use crate::days::day3;

fn main() {
    println!("Advent of code - 2022");
    day1::run();
    day2::run();
    day3::run();
}
