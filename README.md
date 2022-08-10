# Wordle Solver
This is a Wordle solver that is not based on a decision tree. It should work when new words are added and does not precompute paths between words.

## Goals
The goals here are:
1. To solve wordle programmatically without the use of a decision tree. The point is to have the computer make guesses like a human word, only with more knowledge available, as a benchmark for human performance in Wordle games.
2. To solve all words in 6 guesses or under, making all possible games _wins_.
3. To determine whether it is worth it to consider position in letter frequency when calculating word scores.
4. To understand the tradeoffs between pruning guesses and answer guesses.

## First Attempt

The algorithm is written to work like a human would with perfect knowledge of the words involved. This knowledge includes:
1. A list of 2315 possible answer words.
2. A list of 10637 guess words which are valid Wordle guesses but won't be used as answers to Wordle daily puzzles.

(NOTE: in this first attempt, the guess word list was ignored to get a baseline.)

Before it starts, the letter frequency of the answer words is tallied based on a positional letter score. 'S' is the most common starting letter, 'A' is the most common letter in the second position, followed by 'A' again, then 'E' in positions 4 and 5. Words in the answer list are sorted with a score that is calculated from these positional letter scores. 'S' appears as the first letter in 366 words, so the individual letter score for 'S' is 366. 'SLATE' has the highest score based on these frequencies with 366 for S(1), 201 for L(2), 307 for A(3), 139 for T(4) and 424 for E(5). The score for 'SLATE' is then: `366+201+307+139+424=1437`. Letters with duplicates will not receive the lowest score for any duplicate letters. For example: 'SLATS' would not receive points for the final 'S' because it is worth less than the leading 'S': `366+201+307+139=1013`. This is because guessing words with duplicate letters is not as helpful in terms of reducing the remaining guesses (in this case, answers) as quickly.

While a word remains unsolved, the algorithm will offer up the next highest scoring word ('SLATE' will be the starting word) and the green, yellow, and gray letters will be identified and tracked (see `Gameplay` struct). This will then reduce the answer list accordingly:
- words with gray letters will be removed
- words with yellow letters will be retained
- words with green letters in the right position will be retained

### Results from First Attempt
The results of the first attempt were surprisingly good for many words, but unacceptable for many with 223 words going unsolved in the 6 guess maximum considered as _winning_. The average number of guesses to solve was 4.66.

To be certain that positional frequencies were important, I decided to simplify this and only calculate letter frequency overall. This led to 'ALERT' being the highest scoring word, and the results were much better with the average going down to 4.23.

Average: 4.23 guesses per puzzle
Guess Count | # Words Sorted by Positional Frequency | # Words Sorted by Overall Frequency
------------|-----------------------|---------------------
1| 1 | 1
2| 57 | 92
3| 376 | 526
4| 754 | 846
5| 612 | 527
6| 292 | 208
7| 129 | 77
8| 49 | 31
9| 29 | 5
10| 11 | 1
11| 3 | 1
12| 2 | 0

As you can see, we still have 100 words not being solved in 6 words or under.

## First Improvement



### Results
Average: 4.42 guesses per puzzle
Guess Count | # Words Sorted by Overall Frequency| Applicable Words
------------|------------------------------------|-------------
3| 1 | CHUMP
4| 1570| 
5| 561| 
6| 146| 
7| 30| 
8| 5 | TEASE, STATE, ROVER, JUNTA, TAUNT
9| 1 | WOOER
11| 1 | ERROR

As you can see here, we are solving more words in the 6 guess limit (only 37 not being solved), but the distribution of guesses has been squashed, mostly from the left side. Prior to using the guess list, we solved *619 words* in 3 guesses or fewer, which is outstanding. Now we are only solving *1 word* in 3 guesses or fewer. That's a huge change that is really hurting the average. Clearly, we need a way to get the good and not the bad from guesses. So we need a way to decide when to skip the guesses and go straight to the answers, and clearly we can't do this before the first guess.
