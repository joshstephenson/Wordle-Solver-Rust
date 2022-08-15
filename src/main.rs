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

