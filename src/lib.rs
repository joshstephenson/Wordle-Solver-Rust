pub mod help;
pub mod play;

#[cfg(test)]
mod tests {
    use crate::play::Gameplay;
    use crate::help::load_words;

    fn score_for(word: &str) -> usize {
        let target = word.to_string();
        let mut gameplay = Gameplay::new(target.clone(), &load_words(false), &load_words(true));
        let mut guess = Default::default();
        while !gameplay.is_solved() {
            guess = gameplay.next_guess();
            gameplay.add_guess(&guess);
        }
        gameplay.guess_count()
    }

    #[test]
    fn abort_takes_3_guesses() {
        assert_eq!(score_for("abort"), 3);
    }
    
    #[test]
    fn rider_takes_6_guesses() {
        assert_eq!(score_for("rider"), 6);
    }
}
