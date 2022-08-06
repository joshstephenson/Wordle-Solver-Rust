use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    collections::HashMap
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
        return guesses.iter().map(|a| a.to_uppercase()).collect();
    }
    answers.iter().map(|a| a.to_uppercase()).collect()
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
    fn new(answers: &Vec<String>, guesses: &Vec<String>) -> WordleData {
        let mut frequencies: [(u16, char); 26] = Default::default();
        for (i,c) in frequencies.iter_mut().zip('A'..='Z') {
            *i = (0, c);
        }
        let mut positional_frequencies: [[(u16, char); 26]; 5] = Default::default();
        for outer in positional_frequencies.iter_mut() {
            for (inner, c) in outer.iter_mut().zip('A'..='Z') {
                *inner = (0, c);
            }
        }

        // Calculate letter frequency and letter frequency by letter index
        for word in answers {
            for (position,c) in word.chars().enumerate() {
                let index = (c as usize)-(b'A' as usize);
                frequencies[index].0 += 1;
                positional_frequencies[position][index].0 += 1;
            }
        }
        frequencies.sort_unstable_by(|a,b| b.0.cmp(&a.0));
        let letter_frequencies = frequencies.map(|a| a.1);

        let mut positional_letters:[[char;26];5] = Default::default();
        for index in 0..5 {
            let mut position = positional_frequencies[index];
            position.sort_unstable_by(|a,b| b.0.cmp(&a.0));
            positional_letters[index] = position.map(|a| a.1);
        }

        // Calculate word scores
        let mut answers_with_score: Vec<(u16, String)> = Vec::new();
        for word in answers {
            let mut score = 0;
            let mut letters_in_word: Vec<char> = Vec::new();
            let mut letter_scores_in_word: HashMap<char, u16> = HashMap::new();
            for (index, c) in word.chars().enumerate() {
                let tup = positional_frequencies[index].iter().find(|&f| f.1 == c).unwrap();
                if letter_scores_in_word.contains_key(&c) {
                    // maintain the score from the highest scoring letter
                    if &letter_scores_in_word.get(&c).unwrap() < &&tup.0 {
                        letter_scores_in_word.remove(&c);
                        letter_scores_in_word.insert(c, tup.0);
                    }
                }else{
                    letters_in_word.push(c);
                    letter_scores_in_word.insert(c, tup.0);
                    score += tup.0;
                }
            }
            answers_with_score.push((score, word.to_string()));
//            answers_with_score.insert(score, word.to_string());
        }
        answers_with_score.sort_unstable_by(|a,b| b.0.cmp(&a.0));

        // Calculate word score of guesses using overall letter scores (not positional)
        let mut guesses_with_score: Vec<(u16, String)> = Vec::new();
        for word in guesses {
            let mut score = 0;
            let mut letters_in_word: Vec<char> = Vec::new();
            for c in word.chars() {
                if !letters_in_word.contains(&c) {
                    letters_in_word.push(c);
                    let tup = frequencies.iter().find(|&f| f.1 == c).unwrap();
                    score += tup.0;
                }
            }
            guesses_with_score.push((score, word.to_string()));
        }
        guesses_with_score.sort_unstable_by(|a,b| b.0.cmp(&a.0));

        WordleData {
            answers: answers_with_score.into_iter().map(|a| a.1).collect(),
            guesses: guesses_with_score.into_iter().map(|a| a.1).collect(),
            frequencies: letter_frequencies,
            positional: positional_letters,
        }
    }

}

fn main() {

    let mut data = WordleData::new(&load_words(false), &load_words(true));

//    println!("{:#?}", data.guesses);
}

