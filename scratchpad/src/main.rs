//use anyhow::Result;
use thiserror::Error;

const MY_CONST: usize = 12;
const MY_CONST2: usize = MY_CONST + 12;

#[cfg(target_family = "unix")]
const A: usize = 3;

#[cfg(not(target_family = "unix"))]
const A: usize = 4;

const fn add(a: usize, b:usize) -> usize {
    a + b
}

const MAIN: &str = include_str!("main.rs");

const fn loopy() -> usize {
    let mut n = 3.2;
    

    let mut n = 1;
    let mut i = 0;
    while i < 20 {
        n += i;
        i += 1;
    }
    n
}

const C:usize = add(4,6) + loopy();

#[derive(Error, Debug)]
enum InputError {
    #[error("Standard input is unavailable")]
    StdIn,

    #[error("Cannot parse integer from text")]
    NotAnInteger,
}

fn get_line_from_keyboard() -> Result<String, InputError> {
    let mut input = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut input).map_err(|_| InputError::StdIn)?;
    let trimmed = input.trim();
    Ok(trimmed.to_string())
}

fn get_int_from_keyboard() -> Result<i32, InputError> {
    let text = get_line_from_keyboard()?;
    Ok(text.parse().map_err(|_| InputError::NotAnInteger))?
}

fn main() {
    loop {
        println!("Enter an integer:");
        let number = get_int_from_keyboard();
        match number {
            Ok(n)  => { println!("You entered {n}"); break; },
            Err(InputError::StdIn) => panic!("Input doesn't work"),
            Err(InputError::NotAnInteger) => println!("Please try again"),
        }
    }
}
