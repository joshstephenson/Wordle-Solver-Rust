use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn load_words(include_guesses: bool) -> Vec<String> {
    let mut answers = lines_from_file("../data/nyt-answers.txt");
    if include_guesses {
        let mut guesses = lines_from_file("../data/nyt-guesses.txt");
        let mut answer_clone = answers.clone();
        guesses.append(&mut answer_clone);
        return guesses
    }
    answers
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct WordLetter {
    frequency: u16,
    letter: char,
}

#[derive(Debug)]
struct WordleData {
    answers: Vec<String>,
    guesses: Vec<String>,
    frequencies: [char; 26],
    positional: [[char; 26]; 5],
}

impl WordleData {
    fn new(answers: &Vec<String>, guesses: Vec<String>) -> WordleData {
        let mut frequencies: [(u16, char); 26] = Default::default();
        for i in 0..26 {
            frequencies[i] = (0, (b'A' + i as u8) as char);
        }
        let mut positional_frequencies: [[(u16, char); 26]; 5] = Default::default();
        for i in 0..5 {
            for j in 0..26 {
                positional_frequencies[i][j] = (0, (b'A' + j as u8) as char);
            }
        }

        for word in answers {
            for (position,c) in word.to_uppercase().chars().enumerate() {
                let index = (c as usize)-(b'A' as usize);
                frequencies[index].0 += 1;

                positional_frequencies[position][index].0 += 1;
            }
        }

        frequencies.sort_by(|a,b| b.0.cmp(&a.0));
        let l = frequencies.map(|a| a.1);

        let mut positional_letters:[[char;26];5] = Default::default();
        for index in 0..5 {
            let mut position = positional_frequencies[index];
            position.sort_by(|a,b| b.0.cmp(&a.0));
            positional_letters[index] = position.map(|a| a.1);
        }

        WordleData {
            answers: answers.to_vec(),
            guesses: guesses,
            frequencies: l,
            positional: positional_letters,
        }
    }

}

fn main() {

    let mut data = WordleData::new(&load_words(false),load_words(true));

    println!("{:?}", data.positional);
}

