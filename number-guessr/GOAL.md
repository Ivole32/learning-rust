# Goal: Number Guesser

A small game where the program picks a secret random number and the player has to guess it. After every guess, the program tells the player whether the secret number is higher or lower.

## What it should do

1. Generate a secret whole number between 1 and 100 (both included).
2. Ask the player to enter a guess.
3. Compare the guess with the secret number:
   - If the guess is too low, print a hint that the number is higher.
   - If the guess is too high, print a hint that the number is lower.
   - If the guess is correct, congratulate the player and end the game.
4. Keep asking for guesses until the player gets it right.

Example:

```
Guess the number between 1 and 100!
Enter your guess: 50
Higher!
Enter your guess: 75
Lower!
Enter your guess: 63
Correct! The number was 63.
```

## What I want to learn

- Combining what I learned in earlier projects (reading input, generating random numbers)
- Repeating code with a loop and stopping it at the right moment
- Converting text input into a number
- Comparing two values and reacting to each possible result
- Using `if` / `else if` / `else` to handle different cases
- Handling invalid input without crashing the program

## Done when

- [ ] The secret number is different each time I play
- [ ] I get a "higher" or "lower" hint after every wrong guess
- [ ] The game ends as soon as I guess correctly
- [ ] Typing something that isn't a number (like `abc`) shows a message and lets me try again instead of crashing
