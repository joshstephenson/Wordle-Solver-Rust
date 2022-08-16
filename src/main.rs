use wordle::play::Gameplay;
use wordle::help::load_words;
use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
 
    // -s option is to find lower bound for all words in answer file
    if args.len() == 2 {
        if args[1].eq("-s") {
            find_lower_bound();
        }else if args[1].eq("-f") {
            find_best_starting_word();
        }
    }else if args.len() > 2 {
  
        // -w option is for running basic solution where top of answers is used as guess. No
        // shortest path analysis run
        if args[1].eq("-w") {
            run_one(args[2].to_uppercase().clone());

        // -s with a word following runs shortest path on just one word
        } else if args[1].eq("-s") {
            if let Some(gameplay) = find_shortest_for(args[2].to_uppercase().clone()) {
                println!("({}) {}", gameplay.guess_count, gameplay.guesses.join(", "));
            }
        }
    }else {
        // like -w but for all words
        run_all();
    }
}

fn find_shortest_for(word: String) -> Option<Gameplay> {
    let mut start = Gameplay::new(word, &load_words(false), &load_words(true));
    // Start with SLATE
    start.add_guess(&"SLATE".to_string());

    let mut heap: BinaryHeap<Gameplay> = BinaryHeap::new();
    heap.push(start);

    while let Some(gameplay) = heap.pop() {
        if gameplay.is_solved() {
            return Some(gameplay);
        }
        // Until the answer list is pruned to 1, we will use guesses to narrow it down
        // Otherwise we would go straight to the answer in 1 guess every time
        // The last move should be from the answer list
        if gameplay.answers.len() == 1 || gameplay.guessables.len() == 0 {
            let nextmove = gameplay.clone_for_next_guess(&gameplay.answers[0]);
//            println!("Last move for {}: {}", gameplay.guesses.last().unwrap(), nextmove.guesses.last().unwrap());
            heap.push(nextmove);
        }else {
            for (_, answer) in gameplay.guessables.iter().enumerate() {
                let nextmove = gameplay.clone_for_next_guess(&answer);
//                println!("{} move from {}: {}", index, gameplay.guesses.last().unwrap(), nextmove.guesses.last().unwrap());
                heap.push(nextmove);
            }
        }
    }
    None
}

fn find_lower_bound() {
    let answers = load_words(false);
    let mut count = 0;
    let mut guess_count = 0;
    for answer in answers {
        if let Some(gameplay) = find_shortest_for(answer.clone()) {
            guess_count += gameplay.guess_count;
            count += 1;
            let avg: f64 = (guess_count as f64) / (count as f64);
            println!("({:.4}) {}", avg, gameplay.guesses.join(", "));
        }else {
            println!("NO SOLUTION FOUND FOR {}", answer.clone());
        }
    }
}

fn find_best_starting_word() {
    let answers = load_words(false);
    let mut answer_clone = answers.clone();
    let mut guessables = load_words(true);
    guessables.append(&mut answer_clone);

//    let mut words: Vec<String> = Vec::new();
//    let mut avg: Vec<f64> = Vec::new();
//    let mut best: Vec<f64> = Vec::new();
//    let mut best_word: Vec<String> = Vec::new();
//    let mut worst: Vec<f64> = Vec::new();
//    let mut worst_word: Vec<String> = Vec::new();

    let total_answer_count = &answers.len();
    for starting_word in guessables {

        let mut after_guess_count: usize = 0;
        let mut starting_count: usize = 0;
//        words.append(&starting_word);

        let mut this_avg: f64 = Default::default();
        let mut this_best: f64 = 1.0;
        let mut this_best_word: String = Default::default();
        let mut this_worst:f64 = 0.0;
        let mut this_worst_word: String = Default::default();

        for answer in &answers {
            if starting_word.eq(answer) {
                continue;
            }
            let mut green:HashMap<u8, char> = HashMap::new();
            let mut yellow:HashMap<u8, char> = HashMap::new();
            let mut gray:Vec<char> = Vec::new();
            for (index, letter) in starting_word.chars().enumerate() {
                if answer.contains(letter) {
                    if answer.chars().nth(index).unwrap() == letter {
                        green.insert(index as u8, letter);
                    }else {
                        yellow.insert(index as u8, letter);
                    }
                }else {
                    gray.push(letter);
                }
            }

            let mut clone = answers.clone();
            clone.retain(|a| {
                for index in yellow.keys() {
                    let letter = yellow[index];
                    if !a.contains(&letter.to_string()) || letter.eq(&a.chars().nth((*index).into()).unwrap()) {
                        return false
                    }
                }
                for letter in &gray {
                    if a.contains(&letter.to_string()) {
                        return false
                    }
                }
                for key in green.keys() {
                    if a.chars().nth(*key as usize) != Some(green[key]) {
                        return false
                    }
                }
                return true
            });

            starting_count += total_answer_count;
            let new_answer_count = clone.len();
            after_guess_count += new_answer_count;
            let partition_amount:f64 = new_answer_count as f64 / *total_answer_count as f64;

            this_avg = after_guess_count as f64 / starting_count as f64;
            if partition_amount < this_best {
                this_best = partition_amount;
                this_best_word = answer.clone();
            }
            if partition_amount > this_worst {
                this_worst = partition_amount;
                this_worst_word = answer.clone();
            }
        }

        println!("{},{},{},{},{},{}", starting_word, this_avg, this_best, this_best_word, this_worst, this_worst_word);
    }
}

fn run_one(target: String) {
    let mut gameplay = Gameplay::new(target.clone(), &load_words(false), &load_words(true));
    while !gameplay.is_solved() {
        let guess = gameplay.next_guess();
        gameplay.add_guess(&guess);
    }
    println!("{}({}): {:?}", target.clone(), gameplay.guess_count, gameplay.guesses);
}

fn run_all() {
    let answers = load_words(false);
    let guesses = load_words(true);
    let mut answer_count: usize = 0;
    let mut guess_count_total = 0;
    let mut results: HashMap<usize, Vec<String>> = HashMap::new();
    let mut stats: HashMap<usize, usize> = HashMap::new();
    for answer in answers.clone() {
        answer_count += 1;
        let mut gameplay = Gameplay::new(answer.clone(), &answers, &guesses);

        while !gameplay.is_solved() {
            let guess = gameplay.next_guess();
            gameplay.add_guess(&guess);
        }
        let guess_count = gameplay.guess_count;
        guess_count_total += guess_count;

        // Keep track of words solved in various guess counts
        let result = results.entry(guess_count).or_insert(vec![]);
        result.push(answer.clone());
        let stats = stats.entry(guess_count).or_insert(0);
        *stats += 1;

        let avg = guess_count_total as f64 / answer_count as f64;
        println!("{:.4} {}({}): {:?}", avg, answer.clone(), guess_count, gameplay.guesses);
    }
    let avg = guess_count_total as f64 / answer_count as f64;
    println!("Results:\n{:#?}", results);
    println!("Stats:\n{:#?}", stats);
    println!("Solved all {} words in {} (AVG).", answer_count, avg);
    println!("Total Words: {}, Total Guess Count: {}", answer_count, guess_count_total);
}

