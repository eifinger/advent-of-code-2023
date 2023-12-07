
use day1::solution1;

fn main() {
    let sol1 = solution1(include_str!("../input/input.txt"), false).unwrap();
    println!("The solution to part1 is: {sol1}");

    let sol2 = solution1(include_str!("../input/input.txt"), true).unwrap();
    println!("The solution to part2 is: {sol2}");
}