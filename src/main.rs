mod days;

pub use crate::days::day1;
pub use crate::days::day2;


fn main() {
    println!("Advent of code - 2022");
    day1::run();
    day2::run();
}
