# Wordle Solver
This is a Wordle solver that is not based on a decision tree. It should work when new words are added and does not precomputer paths between words.

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

### Results
The results of the first attempt were surprisingly good for many words, but unacceptable for many with 223 words going unsolved in the 6 guess maximum considered as _winning_. The average number of guesses to solve was 4.66.

Here is the full tally of words solved in number of guesses:
Guess Count | Number of Words Solved
------------|-----------------------
1| 1
2| 57
3| 376
4| 754
5| 612
6| 292
7| 129
8| 49
9| 29
10| 11
11| 3
12| 2

## Second Attempt
The improvement here was to start using the guess words as a way to reduce the remaining answers more quickly, particularly for the words above 6 guesses.

Words in the guess list are sorted with a score that is calculated from the overall letter score. 'OATER' has the highest score based on these frequencies.
