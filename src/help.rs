use std::{
    fs::{File},
    io::{prelude::*, BufReader},
    path::Path,
};
use std::path::PathBuf;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn load_words(include_guesses: bool) -> Vec<String> {
    let mut buf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    buf.push("data/nyt-answers.txt");
    let mut path = Path::new(&buf);
    let answers = lines_from_file(path);
    if include_guesses {
        buf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        buf.push("data/nyt-guesses.txt");
        path = Path::new(&buf);
        let mut guesses = lines_from_file(path);
//        let mut answer_clone = answers.clone();
//        guesses.append(&mut answer_clone);
        return guesses.iter().map(|a| a.to_uppercase()).collect();
    }
    answers.iter().map(|a| a.to_uppercase()).collect()
}
