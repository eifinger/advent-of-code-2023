use std::error::Error;

pub fn solution1(contents: &str, convert: bool) -> Result<i32, Box<dyn Error>> {
    let mut total: i32 = 0;
    for line in contents.lines() {
        let digits : i32 = first_last_digit(line, convert);
        total += digits;
    }
    Ok(total)
}

pub fn first_last_digit(contents: &str, convert: bool) -> i32 {
    let mut first: i32 = 0;
    let mut last: i32 = 0;
    let mut contents = contents;
    let transformed;

    println!("contents: {}", contents);
    if convert {
        transformed = replace_names(contents);
        contents = &transformed;
    }

    println!("contents: {}", contents);

    for c in contents.chars() {
        if c.is_digit(10) {
            if first == 0 {
                first = c.to_digit(10).unwrap() as i32;
            }
            else {
                last = c.to_digit(10).unwrap() as i32;
                
            }
        }
    }
    if last == 0 {
        last = first;
    }
    let result: i32 = first *10 + last;
    println!("result: {}", result);
    result
}

pub fn replace_names(name: &str) -> String{

    let res = name.replace("one", "o1e")
    .replace("two", "t2o")
    .replace("three", "t3e")
    .replace("four", "f4r")
    .replace("five", "f5e")
    .replace("six", "s6x")
    .replace("seven", "s7n")
    .replace("eight", "e8t")
    .replace("nine", "n9e");
    res
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_first_and_last_number_in_line() {
        assert_eq!(12, first_last_digit("1abc2", false));
        assert_eq!(38, first_last_digit("pqr3stu8vwx", false));
        assert_eq!(15, first_last_digit("a1b2c3d4e5f", false));
        assert_eq!(77, first_last_digit("treb7uchet", false));

        assert_eq!(29, first_last_digit("two1nine", true));
        assert_eq!(83, first_last_digit("eightwothree", true));
        assert_eq!(13, first_last_digit("abcone2threexyz", true));
        assert_eq!(24, first_last_digit("xtwone3four", true));
        assert_eq!(42, first_last_digit("4nineeightseven2", true));
        assert_eq!(14, first_last_digit("zoneight234", true));
        assert_eq!(76, first_last_digit("7pqrstsixteen", true));
        assert_eq!(71, first_last_digit("7one718onegfqtdbtxfcmd", true));
        assert_eq!(63, first_last_digit("vxzzvdhfqfsix83c1ttvbbstxgdrkfcnmm3", true));
        assert_eq!(62, first_last_digit("dmhkvgbc6four6eightwofkk", true));
        assert_eq!(87, first_last_digit("eightsevensixtxvxlqtdjfivebnvpdhvfsqbbssfkplzmkvvxh7", true));
        assert_eq!(38, first_last_digit("3oneight", true));
    }

}