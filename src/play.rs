use std::{
    collections::HashMap,
    fmt,
    cmp::Ordering,
};

#[derive(Debug)]
pub struct Gameplay {
    pub target:     String,
    pub guesses:    Vec<String>,
    pub guess_count: usize,
    green:      HashMap<u8, char>,
    yellow:     HashMap<u8, char>,
    gray:       Vec<char>,
    used:       Vec<char>,
    pub answers:    Vec<String>,
    pub guessables: Vec<String>,
//    frequencies:[char; 26],
//    positional: [[char; 26]; 5],
}

impl Gameplay {

    pub fn clone_for_next_guess(&self, answer: &String) -> Gameplay {
        let mut clone = Gameplay {
            target:     self.target.clone(),
            guesses:    self.guesses.clone(),
            guess_count: self.guess_count,
            green:      self.green.clone(),
            yellow:     self.yellow.clone(),
            gray:       self.gray.clone(),
            used:       self.used.clone(),
            answers:    self.answers.clone(),
            guessables: self.guessables.clone(),
        };
        clone.add_guess(answer);
        clone
    }

    pub fn new(target: String, answers: &Vec<String>, guessables: &Vec<String>) -> Gameplay {

        // Initialize frequencies which stores each of the 26 English letters and their frequency
        let mut frequencies: [(u16, char); 26] = Default::default();
        for (i,c) in frequencies.iter_mut().zip('A'..='Z') {
            *i = (0, c);
        }

        // Initialize positional frequencies which stores each of the 26 English letters
        // and their frequency relative to each of the 5 indices of Wordle answers
        let mut positional_frequencies: [[(u16, char); 26]; 5] = Default::default();
        for outer in positional_frequencies.iter_mut() {
            for (inner, c) in outer.iter_mut().zip('A'..='Z') {
                *inner = (0, c);
            }
        }

        // Now calculate those frequencies from the answer list
        for word in answers {
            for (position,c) in word.chars().enumerate() {
                let index = (c as usize)-(b'A' as usize);
                frequencies[index].0 += 1;
                positional_frequencies[position][index].0 += 1;
            }
        }

        // Sort them in case we ever want to see which chars are most frequent
        // or if we log and want to make sure its right. Unstable because
        // there really is no reason why we should preserve alphabetic order
        frequencies.sort_unstable_by(|a,b| b.0.cmp(&a.0));
        let letter_frequencies = frequencies.map(|a| a.1);

        let mut positional_letters:[[char;26];5] = Default::default();
        for index in 0..5 {
            let mut position = positional_frequencies[index];
            // Sorting: see note above
            position.sort_unstable_by(|a,b| b.0.cmp(&a.0));
            positional_letters[index] = position.map(|a| a.1);
        }
//        println!("{:?}", positional_frequencies);

        // Calculate answer scores based on positional letter frequencies
        let mut answers_with_score: Vec<(u16, String)> = Vec::new();
        for word in answers {
            let mut score = 0;
            let mut letter_scores_in_word: HashMap<char, u16> = HashMap::new();
            for (index, c) in word.chars().enumerate() {
                let tup = positional_frequencies[index].iter().find(|&f| f.1 == c).unwrap();
                // If a word has duplicate letters, select the highest scoring duplicate letter
                if letter_scores_in_word.contains_key(&c) {
                    if &letter_scores_in_word.get(&c).unwrap() < &&tup.0 {
                        letter_scores_in_word.remove(&c);
                        letter_scores_in_word.insert(c, tup.0);
                    }
                }else{
                    letter_scores_in_word.insert(c, tup.0);
                    score += tup.0;
                }
            }
            answers_with_score.push((score, word.to_string()));
        }
        // Sort by score. Unstable because we have no other order to care about
        answers_with_score.sort_unstable_by(|a,b| b.0.cmp(&a.0));
//        println!("{:#?}", answers_with_score);

        // Calculate guess word score using overall letter frequencies (not positional)
        let mut guesses_with_score: Vec<(u16, String)> = Vec::new();
        for word in guessables {
            let mut score = 0;
            let mut letter_scores_in_word: HashMap<char, u16> = HashMap::new();
            for c in word.chars() {
                let tup = frequencies.iter().find(|&f| f.1 == c).unwrap();
                // If a word has duplicate letters, select the highest scoring duplicate letter
                if letter_scores_in_word.contains_key(&c) {
                    if &letter_scores_in_word.get(&c).unwrap() < &&tup.0 {
                        letter_scores_in_word.remove(&c);
                        letter_scores_in_word.insert(c, tup.0);
                    }
                }else{
                    letter_scores_in_word.insert(c, tup.0);
                    score += tup.0;
                }
            }
            guesses_with_score.push((score, word.to_string()));
        }
        // Sort by score. Unstable because we have no other order to care about
        guesses_with_score.sort_unstable_by(|a,b| b.0.cmp(&a.0));
        Gameplay {
            target:     target.to_uppercase(),
            guesses:    Vec::new(),
            guess_count: 0,
            green:      HashMap::new(),
            yellow:     HashMap::new(),
            gray:       Vec::new(),
            used:       Vec::new(),
            answers: answers_with_score.into_iter().map(|a| a.1).collect(),
            guessables: guesses_with_score.into_iter().map(|a| a.1).collect(),
//            frequencies: letter_frequencies,
//            positional: positional_letters,
        }
    }

    pub fn add_guess(&mut self, guess: &String) {
        self.guesses.push(guess.to_string());
        self.guess_count += 1;
        self.process_last_guess();
        self.remove_answer(&guess);
        self.filter_from_gameplay();
    }

    fn process_last_guess(&mut self) {
        let last_guess = self.guesses.last().unwrap();
        for (index, letter) in last_guess.chars().enumerate() {
            if !self.used.contains(&letter) {
                self.used.push(letter);
            }

            if self.target.contains(letter) {
                if self.target.chars().nth(index).unwrap() == letter {
                    self.green.insert(index as u8, letter);
                }else {
                    self.yellow.insert(index as u8, letter);
                }
            }else {
                self.gray.push(letter);
                self.gray.sort_unstable();
            }
        }
    }

    pub fn is_solved(&self) -> bool {
        self.guesses.len() > 0 && self.target.eq(self.guesses.last().unwrap())
    }

    fn filter_from_gameplay(&mut self) {
        self.answers.retain( |answer| {
            for index in self.yellow.keys() {
                let letter = self.yellow[index];
                // if the potential answer doesn't have a yellow char or has a yellow char in the same spot we tried it already
                if !answer.contains(&letter.to_string())  || letter.eq(&answer.chars().nth((*index).into()).unwrap()) {
                    return false
                }
            } 
            for letter in &self.gray {
                if answer.contains(&letter.to_string()) {
                    return false
                }
            }
            // Do this last because it's more expensive
            for key in self.green.keys() {
                if answer.chars().nth(*key as usize) != Some(self.green[key]) {
                    return false
                }
            }
            return true
        });

        self.guessables.retain( |guess| {
            for letter in &self.used {
                if guess.contains(&letter.to_string()) {
                    return false
                }
            }
            return true
        });
    }

    fn remove_answer(&mut self, answer: &String) {
        self.guessables.retain( |word| {
            return !word.eq(&answer.to_string())
        });
        self.answers.retain( |word| {
            return !word.eq(&answer.to_string())
        });
    }

    pub fn next_guess(&mut self) -> String {
        self.answers.first().unwrap().clone()
    }
}

impl fmt::Display for Gameplay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut word = String::new();
        for i in 0..5 {
            if self.green.contains_key(&i) {
                word.push_str(&self.green[&i].to_string());
            }else{
                word.push_str("_");
            }
        }
        write!(f, "WORD: {} +[{}] -[{}]", 
               word, 
               self.yellow.values().collect::<String>(), 
               self.gray.iter().collect::<String>()
               )
    }
}

impl Ord for Gameplay {
    fn cmp(&self, other: &Self) -> Ordering {
        other.guess_count.cmp(&self.guess_count)
            .then_with(|| self.answers.len().cmp(&other.answers.len()))
    }
}

impl PartialOrd for Gameplay {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Gameplay {
    fn eq(&self, other: &Self) -> bool {
        self.guess_count == other.guess_count
    }
}
impl Eq for Gameplay {
}
