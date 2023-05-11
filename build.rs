use std::env;
use std::fs;
use std::path::Path;

fn read_wordle_data(s: &str) -> Vec<[u8; 5]> {
    fs::read_to_string(s)
        .unwrap()
        .split("\n")
        .filter(|row| row.len() == 5)
        .map(|row| {
            let m: Vec<u8> = row.chars().map(|c| c as u8 - 'a' as u8).collect();
            [m[0], m[1], m[2], m[3], m[4]]
        })
        .collect()
}

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("data.rs");

    let answers = read_wordle_data("real-wordle-answers.txt");
    let answers_size = answers.len();

    let dict = read_wordle_data("5-letter-list-insane.txt");
    let dict_size = dict.len();

    fs::write(
        &dest_path,
        format!(
            r###"
pub const ANSWERS_S: [[u8; 5]; {answers_size}] = {answers:?};
pub const DICT_S: [[u8; 5]; {dict_size}] = {dict:?};
"###
        ),
    )
    .unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=5-letter-list-insane.txt");
    println!("cargo:rerun-if-changed=real-wordle-answers.txt");
}
