use crate::InputError;
use regex::Regex;
use std::todo;

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

fn calibration_value2(_line: &str) -> Result<i32, InputError> {
    todo!();
}
pub fn two(input: &String) -> Result<i32, InputError> {
    let values = input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(calibration_value2)
        .collect::<Result<Vec<i32>, _>>()?;

    return Ok(values.iter().sum());
}
