mod utils;

use wasm_bindgen::prelude::*;
use wordle::{Game, Row, TurnResult, BLANK};

#[wasm_bindgen]
pub struct JSGame(Game);

#[wasm_bindgen]
pub struct JSBoard(String, String, String, String, String, String);

#[wasm_bindgen]
impl JSGame {
  #[wasm_bindgen(constructor)]
  pub fn new(word: &str) -> Result<JSGame, String> {
    utils::set_panic_hook();
    if word.len() != 5 {
      return Err("Word must be 5 letters".into());
    }
    let mut answer: [u8; 5] = [0; 5];
    for (i, chr) in word.to_lowercase().chars().enumerate() {
      answer[i] = chr as u8 - 'a' as u8;
    }
    return Ok(JSGame(Game::new(answer)))
  }

  pub fn guess(&self, word: &str) -> Result<JSGame, JsValue> {
    let maybe_guess = str_to_guess(word);
    match maybe_guess {
      Some(guess) =>
        match self.0.guess(guess) {
          TurnResult::Some(game) => Ok(JSGame(game)),
          TurnResult::Win(word) => Err(format!("You win! the word was {word}").into()),
          TurnResult::Lose(word) => Err(format!("You lose, the word was {word}").into()),
          TurnResult::Error(message) => Err(message.into()),
        }
      None => Err(format!("{word} is not a 5 letter word").into())
    }
  }

  pub fn get_board(&self) -> String {
    format!(
      "{}\n{}\n{}\n{}\n{}\n{}",
      row_to_string(self.0.grid[0]),
      row_to_string(self.0.grid[1]),
      row_to_string(self.0.grid[2]),
      row_to_string(self.0.grid[3]),
      row_to_string(self.0.grid[4]),
      row_to_string(self.0.grid[5]),
    )
  }
}

fn str_to_guess(word: &str) -> Option<[u8; 5]> {
  if word.len() != 5 {
    return None
  }
  let mut guess = [BLANK; 5];
  for (i, chr) in word.to_lowercase().chars().enumerate() {
    guess[i] = chr as u8 - 'a' as u8;
  }

  Some(guess)
}

fn row_to_string(r: Row) -> String {
  let mut ret = String::new();
  for i in 0..5 {
    if r.0[i] == wordle::BLANK {
      ret.push(' ');
    } else {
      ret.push((r.0[i] + 'a' as u8) as char);
    }
  }
  return ret
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
