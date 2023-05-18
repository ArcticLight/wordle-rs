use std::fmt::{self, Display};

mod generated;
pub use generated::data;

#[derive(Clone, Copy)]
pub struct Row(pub [u8; 5]);
pub const BLANK: u8 = u8::MAX;

#[derive(Clone, Copy)]
pub struct Game {
    pub grid: [Row; 6],
    pub answer: Row,
}

pub enum TurnResult {
    Some(Game),
    Lose(Row),
    Win(Row),
    Error(String),
}

impl Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..5 {
            write!(f, "{}", (self.0[i] + 'a' as u8) as char)?
        }
        return Ok(());
    }
}

impl Game {
    pub fn new(answer: [u8; 5]) -> Game {
        Game {
            grid: [
                Row([BLANK; 5]),
                Row([BLANK; 5]),
                Row([BLANK; 5]),
                Row([BLANK; 5]),
                Row([BLANK; 5]),
                Row([BLANK; 5]),
            ],
            answer: Row(answer),
        }
    }

    pub fn turn_no(&self) -> u8 {
        match self
            .grid
            .iter()
            .position(|row| row.0.iter().all(|c| *c == BLANK))
        {
            Some(t) => t as u8,
            None => 6,
        }
    }

    pub fn guess(self, guess: [u8; 5]) -> TurnResult {
        if !data::DICT_S.contains(&guess) {
            return TurnResult::Error(format!("{} is not a word in the dictionary.", Row(guess)));
        }

        let turn = self.turn_no();
        if turn >= 6 {
            return TurnResult::Lose(self.answer);
        }

        if guess.eq(&self.answer.0) {
            return TurnResult::Win(self.answer);
        }

        let mut new_grid = self.grid;
        new_grid[turn as usize] = Row(guess);
        return TurnResult::Some(Game {
            grid: new_grid,
            ..self
        });
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\x1b[37m┌──────────┐")?;
        for (_i, row) in self.grid.iter().enumerate() {
            write!(f, "│")?;
            for (j, &color) in color_word(row, &self.answer).iter().enumerate() {
                if row.0[j] == BLANK {
                    write!(f, "{}", gray('▢'))?;
                } else {
                    let c = char::from_u32(row.0[j] as u32 + '🄰' as u32).unwrap();
                    match color {
                        Color::Green => write!(f, "{}", green(c))?,
                        Color::Yellow => write!(f, "{}", yellow(c))?,
                        Color::Gray => write!(f, "{}", gray(c))?,
                    }
                }
            }
            writeln!(f, "│")?;
        }
        write!(f, "└──────────┘")
    }
}

fn gray(c: char) -> String {
    format!("\x1b[37;40m{} \x1b[0m", c)
}
fn green(c: char) -> String {
    format!("\x1b[30;42m{} \x1b[0m", c)
}
fn yellow(c: char) -> String {
    format!("\x1b[30;43m{} \x1b[0m", c)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    Gray,
    Green,
    Yellow,
}

fn color_word(guess: &Row, correct: &Row) -> [Color; 5] {
    let mut colors = [Color::Gray; 5];

    let mut unmatched: [u8; u8::MAX as usize + 1] = [0; u8::MAX as usize + 1];

    // Color matched letters and count unmatched letters
    for i in 0..5 {
        if guess.0[i] == correct.0[i] {
            colors[i] = Color::Green
        } else {
            unmatched[correct.0[i] as usize] += 1
        }
    }

    // Color unmatched letters left-to-right
    for i in 0..5 {
        if guess.0[i] != correct.0[i] {
            if unmatched[guess.0[i] as usize] > 0 {
                colors[i] = Color::Yellow;
                unmatched[guess.0[i] as usize] -= 1;
            }
        }
    }

    colors
}

#[cfg(test)]
mod tests {
    use crate::color_word;
    use crate::Color::*;
    use crate::Row;

    #[test]
    fn test_colors() {
        assert_eq!(
            color_word(&Row([1, 2, 3, 4, 5]), &Row([0, 1, 0, 0, 0])),
            [Yellow, Gray, Gray, Gray, Gray]
        );
        assert_eq!(
            color_word(&Row([1, 1, 3, 4, 5]), &Row([0, 1, 0, 0, 0])),
            [Gray, Green, Gray, Gray, Gray]
        );
        assert_eq!(
            color_word(&Row([3, 1, 1, 4, 5]), &Row([0, 1, 0, 0, 0])),
            [Gray, Green, Gray, Gray, Gray]
        );
        assert_eq!(
            color_word(&Row([3, 2, 1, 1, 5]), &Row([0, 1, 0, 0, 0])),
            [Gray, Gray, Yellow, Gray, Gray]
        );
        assert_eq!(
            color_word(&Row([3, 2, 1, 1, 5]), &Row([1, 1, 0, 0, 0])),
            [Gray, Gray, Yellow, Yellow, Gray]
        );
        assert_eq!(
            color_word(&Row([3, 1, 1, 3, 5]), &Row([1, 1, 0, 0, 0])),
            [Gray, Green, Yellow, Gray, Gray]
        );
        assert_eq!(
            color_word(&Row([1, 1, 3, 3, 5]), &Row([0, 1, 1, 0, 0])),
            [Yellow, Green, Gray, Gray, Gray]
        );
        assert_eq!(
            color_word(&Row([0, 1, 0, 1, 5]), &Row([0, 1, 1, 0, 0])),
            [Green, Green, Yellow, Yellow, Gray]
        );
    }
}
