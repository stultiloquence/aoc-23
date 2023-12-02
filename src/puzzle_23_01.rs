use core::panic;

use crate::InputError;

use regex::Regex;

fn calibration_value(line: &str) -> Result<i32, InputError> {
    let first_digit = line.chars().find(|c: &char| c.is_ascii_digit());
    let last_digit = line.chars().rev().find(|c: &char| c.is_ascii_digit());

    if first_digit.is_none() || last_digit.is_none() {
        return Err(InputError {
            details: format!("Cannot find digits in line {}.", line),
        });
    }

    let number: i32 = String::from_iter(vec![first_digit.unwrap(), last_digit.unwrap()])
        .parse()
        .unwrap();

    return Ok(number);
}

pub fn one(input: &String) -> Result<i32, InputError> {
    let values = input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(calibration_value)
        .collect::<Result<Vec<i32>, _>>()?;

    return Ok(values.iter().sum());
}

fn digit_to_value(digit: &str) -> i32 {
    match digit {
        "0" => 0,
        "eno" | "one" | "1" => 1,
        "owt" | "two" | "2" => 2,
        "eerht" | "three" | "3" => 3,
        "ruof" | "four" | "4" => 4,
        "evif" | "five" | "5" => 5,
        "xis" | "six" | "6" => 6,
        "neves" | "seven" | "7" => 7,
        "thgie" | "eight" | "8" => 8,
        "enin" | "nine" | "9" => 9,
        _ => panic!(
            "The regex match {} should have been convertible to a digit value.",
            digit
        ),
    }
}

fn calibration_value2(line: &str) -> Result<i32, InputError> {
    let first_match = Regex::new(r"one|two|three|four|five|six|seven|eight|nine|\d")
        .unwrap()
        .captures(line);

    let line_reversed = line.chars().rev().collect::<String>();
    let last_match = Regex::new(r"eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|\d")
        .unwrap()
        .captures(line_reversed.as_str());

    if first_match.is_none() || last_match.is_none() {
        return Err(InputError {
            details: format!("Did not find a digit in line {}.", line),
        });
    }

    let first_digit = digit_to_value(first_match.unwrap().get(0).unwrap().as_str());
    let last_digit = digit_to_value(last_match.unwrap().get(0).unwrap().as_str());

    return Ok(10 * first_digit + last_digit);
}
pub fn two(input: &String) -> Result<i32, InputError> {
    let values = input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(calibration_value2)
        .collect::<Result<Vec<i32>, _>>()?;

    return Ok(values.iter().sum());
}
