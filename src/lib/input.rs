use std::error::Error;
use std::io::{self, Write};

pub fn get_player_input() -> Result<char, Box<dyn Error>> {
    let stdin = io::stdin();

    print!("\nTake a guess: ");
    io::stdout().flush()?;

    let mut buffer = String::new();
    stdin.read_line(&mut buffer)?;

    let character: &char = &buffer.trim_end().parse::<char>()?.to_ascii_lowercase();

    if !character.is_ascii_alphabetic() {
        return Err("What you typed is not a valid letter, try again".into());
    }

    Ok(character.to_owned())
}
