use std::error;
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn get_line_from_keyboard() -> Result<String> {
    let mut input = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut input)?;
    let trimmed = input.trim();
    Ok(trimmed.to_string())
}

fn get_int_from_keyboard() -> Result<i32> {
    let text = get_line_from_keyboard()?;
    Ok(text.parse()?)
}

fn main() {
    println!("Enter your name:");
    let name = get_line_from_keyboard().unwrap();
    println!("{name}");
}
