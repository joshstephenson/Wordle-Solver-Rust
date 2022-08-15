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

pub fn load_words(guessables: bool) -> Vec<String> {
    if guessables {
        let mut buf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        buf.push("data/nyt-guesses.txt");
        let path = Path::new(&buf);
        let guesses = lines_from_file(path);
//        let mut answer_clone = answers.clone();
//        guesses.append(&mut answer_clone);
        guesses.iter().map(|a| a.to_uppercase()).collect()
    }else {
        let mut buf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        buf.push("data/nyt-answers.txt");
        let path = Path::new(&buf);
        let answers = lines_from_file(path);
        answers.iter().map(|a| a.to_uppercase()).collect()
    }
}
