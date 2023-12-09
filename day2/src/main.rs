
use day2::{solution1, solution2};

fn main() {
    let sol1 = solution1(include_str!("../input/input.txt")).unwrap();
    println!("The solution to part1 is: {sol1}");

    let sol2 = solution2(include_str!("../input/input.txt")).unwrap();
    println!("The solution to part2 is: {sol2}");
}