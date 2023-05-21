use std::fmt::{self, Display};

mod generated;
pub use generated::data;

#[derive(Clone, Copy)]
pub struct Row(pub [u8; 5]);
pub const BLANK: u8 = u8::MAX;
pub const BLANK_ROW: Row = Row([BLANK, BLANK, BLANK, BLANK, BLANK]);

#[derive(Clone, Copy)]
pub struct Game {
    pub grid: [Row; 6],
    pub answer: Row,
}

pub enum TurnResult {
    Some(Game),
    Lose(Game),
    Win(Game),
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
                BLANK_ROW,
                BLANK_ROW,
                BLANK_ROW,
                BLANK_ROW,
                BLANK_ROW,
                BLANK_ROW,
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

    pub fn result_mojis(&self) -> String {
        const BLK: char = '⬛';
        const YLW: char = '🟨';
        const GRN: char = '🟩';
        let mut buf: String = String::with_capacity(30);
        for row in self.grid.iter() {
            if row.0.iter().all(|c| *c == BLANK) {
                continue
            }
            for col in color_word(row, &self.answer) {
                buf.push(match col {
                    Color::White | Color::Gray => BLK,
                    Color::Green => GRN,
                    Color::Yellow => YLW,
                })
            }
            buf.push('\n')
        }

        buf
    }

    pub fn guess(self, guess: [u8; 5]) -> TurnResult {
        let dict_word = data::DICT_S.iter().find(|x| **x == guess);
        return match dict_word {
            None => TurnResult::Error(format!("{} is not a word in the dictionary.", Row(guess))),
            Some(word) => {
                let turn = self.turn_no();

                let mut new_grid = self.grid;
                if turn <= 5 {
                    new_grid[turn as usize] = Row(*word);
                }

                if guess == self.answer.0 {
                    return TurnResult::Win(Game {
                        grid: new_grid,
                        ..self
                    });
                }

                if turn >= 5 {
                    return TurnResult::Lose(Game {
                        grid: new_grid,
                        ..self
                    });
                }

                TurnResult::Some(Game {
                    grid: new_grid,
                    ..self
                })
            }
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const LETTERMAP: [&[u8]; 6] = [
            &[16, 22, 4, 17, 19, 24],              // "qwerty"
            &[BLANK, BLANK, 20, 8, 14, 15],        // "uiop"
            &[0, 18, 3, 5, 6],                     // "asdfg"
            &[BLANK, BLANK, 7, 9, 10, 11],         // "hjkl"
            &[25, 23, 2, 21, 1],                   // "zxcvb"
            &[BLANK, BLANK, BLANK, BLANK, 13, 12], // "nm"
        ];
        let mut colors: [[Color; 5]; 6] = [[Color::White; 5]; 6];
        let mut letters = [Color::White; 26];
        for (i, row) in self.grid.iter().enumerate() {
            for (j, &color) in color_word(row, &self.answer).iter().enumerate() {
                colors[i][j] = color;
                if row.0[j] <= 25 && letters[row.0[j] as usize] < color {
                    letters[row.0[j] as usize] = color;
                }
            }
        }

        writeln!(f, "\x1b[37m┌──────────┐")?;
        for (i, row) in self.grid.iter().enumerate() {
            write!(f, "│")?;
            for (j, color) in colors[i].iter().enumerate() {
                if row.0[j] == BLANK {
                    write!(f, "{}", gray('▢'))?;
                } else {
                    let c = char::from_u32(row.0[j] as u32 + '🄰' as u32).unwrap();
                    match color {
                        Color::White => write!(f, "  ")?,
                        Color::Green => write!(f, "{}", green(c))?,
                        Color::Yellow => write!(f, "{}", yellow(c))?,
                        Color::Gray => write!(f, "{}", gray(c))?,
                    }
                }
            }
            write!(f, "│ ")?;
            for &t in LETTERMAP[i].iter() {
                if t == BLANK {
                    write!(f, "  ")?;
                    continue
                }
                let c = char::from_u32(t as u32 + '🄰' as u32).unwrap();
                write!(f, "{}", chalk(c, letters[t as usize]))?;
            }
            writeln!(f, "")?;
        }
        write!(f, "└──────────┘")
    }
}

fn chalk(c: char, color: Color) -> String {
    match color {
        Color::White => white(c),
        Color::Gray => gray(c),
        Color::Green => green(c),
        Color::Yellow => yellow(c),
    }
}

fn white(c: char) -> String {
    format!("\x1b[30;47m{} \x1b[0m", c)
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

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Color {
    // TODO:
    // White was introduced for, and only ever used for,
    // the "unused" color for displaying the alphabet.
    //
    // Is there a better way of having a "white" or "nil"
    // color just for that one purpose, rather than muddying
    // the whole enum for that effect..?
    White = 0,
    Gray = 1,
    Yellow = 2,
    Green = 3,
}

fn color_word(guess: &Row, correct: &Row) -> [Color; 5] {
    let mut colors = [Color::Gray; 5];
    let mut unmatched: [u8; 26] = [0; 26];

    // Color matched letters and count unmatched letters
    for i in 0..5 {
        if guess.0[i] == correct.0[i] {
            colors[i] = Color::Green
        } else if correct.0[i] <= 25 {
            unmatched[correct.0[i] as usize] += 1
        }
    }

    // Color unmatched letters left-to-right
    for i in 0..5 {
        if guess.0[i] != correct.0[i] {
            if guess.0[i] <= 25 && unmatched[guess.0[i] as usize] > 0 {
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
