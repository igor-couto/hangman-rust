mod game_progress;
mod graphics;
mod input;
mod words;

use self::game_progress::GameProgress;

const MAX_GUESSES: u8 = 6;

pub fn run() {
    let word: String = words::pick_a_word();
    let mut display: Vec<char> = "_".repeat(word.len()).chars().collect::<Vec<_>>();
    let mut guesses: Vec<char> = vec![];
    let mut incorrect_guesses: u8 = 0;

    loop {
        show_display(&display, &guesses, &incorrect_guesses);

        let guess = match input::get_player_input() {
            Ok(value) => value,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        guesses.push(guess);

        let mut is_match = false;
        for (index, letter) in word.chars().enumerate() {
            if letter == guess {
                is_match = true;
                display[index] = guess;
            }
        }
        if !is_match {
            incorrect_guesses += 1;
        }

        match check_game_progress(&word, &guesses, incorrect_guesses as u8) {
            GameProgress::Won => {
                println!("\nNice! You won!");
                break;
            }
            GameProgress::Lost => {
                println!("\nYou lose! The word was {}.\n", word);
                break;
            }
            _ => {}
        }
    }
}

fn check_game_progress(word: &str, guesses: &[char], incorrect_guesses: u8) -> GameProgress {
    if incorrect_guesses == MAX_GUESSES {
        return GameProgress::Lost;
    }

    for letter in word.chars() {
        if !guesses.contains(&letter) {
            return GameProgress::InProgress;
        }
    }
    return GameProgress::Won;
}

fn show_display(display: &[char], guesses: &[char], incorrect_guesses: &u8) {
    println!("\n\n\n*******************\n\n\n");

    print!("{}   ", graphics::HANGMAN[*incorrect_guesses as usize]);

    for item in display.iter() {
        print!("{} ", item);
    }

    print!("\n\nAlready used letters: ");
    for item in guesses.iter() {
        print!("{} ", item);
    }
}
