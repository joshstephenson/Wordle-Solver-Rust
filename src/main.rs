use wordle::play::Gameplay;
use wordle::help::load_words;
use std::collections::HashMap;

fn main() {
    let target = "RIPER".to_string();
    let mut gameplay = Gameplay::new(target.clone(), &load_words(false), &load_words(true));
    let mut guess = "SLATE".to_string();
    while !gameplay.is_solved() {
        gameplay.add_guess(&guess);
        if gameplay.is_solved() {
            println!("{}({}): {:?}", target.clone(), gameplay.guess_count(), gameplay.guesses);

            break;
        }
        guess = gameplay.next_guess();
    }
}

fn run_all() {
    let answers = load_words(false);
    let guesses = load_words(true);
    let mut avg:f64 = 0.0;
    let mut answer_count: usize = 0;
    let mut guess_count_total = 0;
    let mut results: HashMap<usize, Vec<String>> = HashMap::new();
    let mut stats: HashMap<usize, usize> = HashMap::new();
    for answer in answers.clone() {
        answer_count += 1;
        let mut gameplay = Gameplay::new(answer.clone(), &answers, &guesses);

        let mut guess = "SLATE".to_string();
        while !gameplay.is_solved() {
            gameplay.add_guess(&guess);
            if gameplay.is_solved() {
                let guess_count = gameplay.guess_count();
                guess_count_total += guess_count;

                // Keep track of words solved in various guess counts
                let result = results.entry(guess_count).or_insert(vec![]);
                result.push(answer.clone());
                let stats = stats.entry(guess_count).or_insert(0);
                *stats += 1;

                avg = guess_count_total as f64 / answer_count as f64;
                println!("{:.4} {}({}): {:?}", avg, answer.clone(), guess_count, gameplay.guesses);

                break;
            }
            guess = gameplay.next_guess();
        }
    }
    avg = guess_count_total as f64 / answer_count as f64;
    println!("Results:\n{:#?}", results);
    println!("Stats:\n{:#?}", stats);
    println!("Solved all {} words in {} (AVG).", answer_count, avg);
    println!("Total Words: {}, Total Guess Count: {}", answer_count, guess_count_total);
}

