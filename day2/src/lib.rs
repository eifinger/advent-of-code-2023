use std::error::Error;
use std::fmt;

static CUBES: Cubes = Cubes {red: 12, green: 13, blue: 14};

pub fn solution1(contents: &str) -> Result<i32, Box<dyn Error>> {
    let mut total: i32 = 0;
    for line in contents.lines() {
        let result : i32 = evaluate_game(line, &CUBES);
        total += result;
    }
    Ok(total)
}

pub fn solution2(contents: &str) -> Result<i32, Box<dyn Error>> {
    let mut total: i32 = 0;
    for line in contents.lines() {
        let result : i32 = power_of_minimum_cubes(line);
        total += result;
    }
    Ok(total)
}

struct Cubes {
    red: i32,
    green: i32,
    blue: i32,
}

impl fmt::Display for Cubes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(red: {}, green: {}, blue: {})", self.red, self.green, self.blue)
    }
}

fn power_of_minimum_cubes(game: &str) -> i32 {
    let mut max_cubes = Cubes {red: 0, green: 0, blue: 0};
    let revealed = game.split(":").nth(1).unwrap().split(";").collect::<Vec<&str>>();
    for reveal in revealed {
        let cubes = get_cubes(reveal);
        if cubes.red > max_cubes.red {
            max_cubes.red = cubes.red;
        }
        if cubes.green > max_cubes.green {
            max_cubes.green = cubes.green;
        }
        if cubes.blue > max_cubes.blue {
            max_cubes.blue = cubes.blue;
        }
    }
    max_cubes.red * max_cubes.green * max_cubes.blue
}

fn evaluate_game(game: &str, possible_cubes: &Cubes) -> i32 {
    let game_number = get_game_number(game);
    let mut result = 0;
    let revealed = game.split(":").nth(1).unwrap().split(";").collect::<Vec<&str>>();
    for reveal in revealed {
        let cubes = get_cubes(reveal);
        if is_game_possible(&cubes, &possible_cubes) {
            result = game_number;
        } else {
            return 0;
        }
    }
    result
}

fn get_cubes(revealed: &str) -> Cubes {
    let cubes = Cubes {red: get_cube(revealed, "red"), green: get_cube(revealed, "green"), blue: get_cube(revealed, "blue")};

    cubes
}

fn get_cube(revealed: &str, color: &str) -> i32 {
    let mut cube = 0;
    if revealed.contains(color) == false {
        return cube;
    }
    let before_color = revealed
        .split(color)
        .nth(0)
        .unwrap();
    if before_color.contains(",") {
        cube = before_color
            .split(",")
            .last()
            .unwrap()
            .trim()
            .parse::<i32>().unwrap();
    } else {
        cube = before_color
            .trim()
            .parse::<i32>().unwrap();
    }
    cube
}

fn get_game_number(game: &str) -> i32 {
    game.split(":").nth(0).unwrap()
        .split("Game ").nth(1).unwrap()
        .parse::<i32>().unwrap()
}

fn is_game_possible(cubes: &Cubes, possible_cubes: &Cubes) -> bool {
    cubes.red <= possible_cubes.red && cubes.green <= possible_cubes.green && cubes.blue <= possible_cubes.blue
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_game() {
        assert_eq!(1, evaluate_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", &CUBES));
        assert_eq!(2, evaluate_game("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", &CUBES));
        assert_eq!(0, evaluate_game("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", &CUBES));
        assert_eq!(0, evaluate_game("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", &CUBES));
        assert_eq!(5, evaluate_game("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", &CUBES));
    }

    #[test]
    fn test_power_of_minimum_cubes() {
        assert_eq!(48, power_of_minimum_cubes("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"));
        assert_eq!(12, power_of_minimum_cubes("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"));
        assert_eq!(1560, power_of_minimum_cubes("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"));
        assert_eq!(630, power_of_minimum_cubes("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"));
        assert_eq!(36, power_of_minimum_cubes("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"));
    }
}