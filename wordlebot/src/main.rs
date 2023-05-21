use rand::prelude::*;
use std::io::{self, Write};

use wordle::TurnResult::*;
use wordle::{data::ANSWERS_S, Game, BLANK};

fn main() {
    let mut game = Game::new(*ANSWERS_S.choose(&mut thread_rng()).unwrap());

    loop {
        println!("{game}");
        print!("Guess a 5-letter word: ");
        io::stdout().flush().unwrap();
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        guess = guess.trim().to_string();

        if guess.len() != 5 {
            println!("Please enter a 5-letter word");
            continue;
        }

        let mut guessed_word = [BLANK; 5];
        for (i, chr) in guess.to_lowercase().chars().enumerate() {
            let guessed_char = chr as u8 - 'a' as u8;
            guessed_word[i] = guessed_char;
        }

        match game.guess(guessed_word) {
            Some(g2) => game = g2,
            Lose(end) => {
                println!("You lose!\n{end}");
                break;
            }
            Win(end) => {
                println!("You win!\n{}", end.result_mojis());
                break;
            }
            Error(m) => println!("ERROR: {m}"),
        }
    }
}
