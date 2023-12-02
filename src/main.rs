use std::env;
use std::error::Error;

use std::fmt::Debug;
use std::fs;

use std::fmt;

mod puzzle_23_01;

#[derive(Debug)]
pub struct InputError {
    details: String,
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.details)
    }
}

impl Error for InputError {}

fn puzzle23(puzzle_nr: i32, sub_nr: i32, contents: &String) -> Result<i32, InputError> {
    match (puzzle_nr, sub_nr) {
        (1, 1) => puzzle_23_01::one(contents),
        (1, 2) => puzzle_23_01::two(contents),
        _ => Err(InputError {
            details: format!("Unknown puzzle number {}.", puzzle_nr),
        }),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let puzzle_nr = args[1]
        .parse::<i32>()
        .expect("The first argument should be a number");
    let sub_nr = args[2]
        .parse::<i32>()
        .expect("The second argument should be a number");
    let input_file_path = &args[3];
    let contents = fs::read_to_string(input_file_path)
        .expect("The third argument should be a valid file name.");

    dbg!(puzzle23(puzzle_nr, sub_nr, &contents).unwrap());

    return Ok(());
}
