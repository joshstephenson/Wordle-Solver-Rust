# Wordle Answers Lower Bound
This is a Wordle solver that is finds the lowest possible word path for all 2315 Wordle answers from a static starting word.

## What is Wordle?
[Wordle](https://www.nytimes.com/games/wordle/index.html) is a five letter English word guessing game where the player must guess a target word in six guesses. For each attempt, the player enters a five letter word and receives feedback for each letter:
- Correct letters not in their correct positions will be marked yellow.
- Correct letters in their correct positions will be marked green.
- Letters that are not included in the target word will be marked gray.

![KHAKI](https://user-images.githubusercontent.com/11002/184548002-2f5cc825-9ec6-47df-a703-7490a4eb593a.png)

## Focus
The focus of this project is to find the lower bound average solution rate for all 2315 answer words using dynamic programming. This is different from another solver I wrote which takes a more human-like approach. That project does not use backtracking or dynamic programming and includes an interactive solver that does not use any precomputed or cached data.
* [Python Wordle Solver](https://github.com/joshstephenson/Wordle-Solver)

## Results
The lower bound of the overwhelming majority of words is 3 guesses per word. 28 can be resolved in 2, 23 in 4, and 5 require 5 guesses (`EATER`, `PENAL`, `STILL`, `TRUSS`, `UNTIL`). Slate which is the starting word is resolved in 1 of course.
![results-SLATE](https://user-images.githubusercontent.com/11002/184703834-306f5251-9ce4-49f4-a564-0bf62d1b7ade.png)


## How it Works
This algorithm uses:
1. A list of 2315 possible answer words.
2. A list of 10637 guess words which are valid Wordle guesses but won't be used as answers to Wordle daily puzzles.

It starts with a single word choice that you must provide. The current version uses `SLATE` which has a very high letter score. The game state after this word guess is put into a priority queue (`BinaryHeap` from std::collections) which then pops the item with the lowest number of guesses from the queue and then:
* Checks to see if the game has been solved. If it has not been solved, it checks the number of remaining answers based on the greens, yellows and grays from previous guesses:
  * If there is still more than one answer remaining it then duplicates the gameplay once per guess (of 10637 initial guesses) and submits that guess to the gameplay and pushes that gameplay onto the priority queue.
  * If there is only one answer remaining, then it submits that answer to the gameplay and pushes that to the priority queue. This is not guaranteed to be the shortest path, it's only the final path (leaf) in the current branch.

This process finds a decision-tree with every possible move made after the initial guess, as well as every possible remaining guess from there, and so on, until the target solution is reached. The gameplay of the shortest path is what is returned. There is no breaking of ties, because it's not important to the nature of the game.

### Understanding the Lower Bound
It's worth stating that this is a very specific concept of lower bound. Technically, the lower bound of the game is 1. Any puzzle can be solved in only 1 guess. Any puzzle can also be solved in 2 guesses, but those would be lucky guesses and we're not interested in lucky guesses.

What I am doing here in this project is never landing on the final answer until all other possible answers have been filtered out by making guesses. Aside from `SLATE`, a word will never accidentally be solved from a lucky guess. All answers have to be revealed by guesses that partion the remaining answers into the most possible subsets. This ensures we are playing by the rules of the game and then revealing a word path for each puzzle that is guaranteed to land on the target word through the filtering of words based on green, yellow and gray letters.

### Example Word
Here is a path calculated by this project for `BLINK`: `SLATE, KOMBU, BLINK`. Let's break this down. Before `SLATE` the number of possible answers is 2315. - After `SLATE` (1 green: [L], 0 yellow, 4 gray: [S,A,T,E]) there are only 39 possible words:
- `['GLORY', 'CLINK', 'BLIND', 'BLINK', 'BLOND', 'CLING', 'BLOCK', 'FLOUR', 'FLICK', 'FLOCK', 'CLOUD', 'FLING', 'PLUNK', 'CLOWN', 'FLUNK', 'FLOOR', 'CLUNG', 'CLICK', 'FLUID', 'BLOWN', 'PLUCK', 'BLIMP', 'CLOCK', 'CLIMB', 'BLOOD', 'FLOWN', 'FLUNG', 'CLIFF', 'FLOOD', 'BLURB', 'CLUMP', 'CLUCK', 'BLOOM', 'GLOOM', 'PLUMB', 'PLUMP', 'BLUFF', 'GLYPH', 'FLUFF']`
Out of those words, we see the following letters (ignoring letters we've already guessed in `SLATE`):
- `{'U': 15, 'C': 15, 'O': 15, 'F': 13, 'N': 13, 'B': 12, 'I': 11, 'K': 11, 'M': 7, 'G': 7, 'P': 7, 'D': 6, 'R': 4, 'W': 3, 'Y': 2, 'H': 1}`
We see 15 `U`s, 15 `C`s and so on. So we know the next best word needs to have as many of these letters as words from the available guesses can have. If you look at [my other solver](https://github.com/joshstephenson/Wordle-Solver), what I do here is find a word with the most of these letters, favoring letters with the higher numbers, which leads to guessing `BUNCO`. But that's not what we do here. Instead, we just run through all possible choices, and the word this algorithm finds to guess next is `KOMBU`.
- After guessing `KOMBU` (0 green, 2 yellow: [B,K], 3 gray: [O,M,U]) there is only word remaining that can be matched: `BLINK`.

**Here is a worst case example:**
The path for `EATER` is `SLATE, BUMPH, ZINCO, WATER, EATER`. If we walk through this one by one:
- After `SLATE` (0 green, 3 yellow, 2 gray) there are 25 possible words.
- After `BUMPH` (0 green, 0 yellow, 5 gray) there are 17 possible words.
- After `ZINCO` (0 green, 0 yellow, 5 gray) there are still 11 possible words(!).
- After `WATER` (4 green, 0 yellow, 1 gray) there is only one possible answer: `EATER`.

## Usage
Run it on a single word:
```
$ cargo run --release -s rider
(3) SLATE, RINDY, RIDER
```

Run it on all words in the answer file:
```
$ cargo run --release -s 
(3.0000) SLATE, BOODY, ABACK
(3.0000) SLATE, NOBBY, ABASE
(3.0000) SLATE, GOBBY, ABATE
(3.0000) SLATE, BOOGY, ABBEY
(3.0000) SLATE, DOBRO, ABBOT
(3.0000) SLATE, RHOMB, ABHOR
(3.0000) SLATE, BOOMY, ABIDE
(3.0000) SLATE, DOBRO, ABLED
(3.0000) SLATE, DIMBO, ABODE
(3.0000) SLATE, DOORN, ABORT
(3.0000) SLATE, BUNJY, ABOUT
(3.0000) SLATE, DROOB, ABOVE
(3.0000) SLATE, BICCY, ABUSE
(3.0000) SLATE, BROCK, ABYSS
(3.0000) SLATE, YCOND, ACORN
(3.0000) SLATE, KIRRI, ACRID
(3.0000) SLATE, CRUDY, ACTOR
(3.0000) SLATE, MOHUR, ACUTE
(3.0000) SLATE, RIDGY, ADAGE
(3.0000) SLATE, BOORD, ADAPT
...
```

## Contributions
If you use this or would like to contribute, feel free to fork, contact me or submit a PR.
