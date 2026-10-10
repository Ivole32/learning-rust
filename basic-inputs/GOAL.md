# Goal: Basic Inputs

A small program that reads a line of text from the user and prints it back to the terminal.

## What it should do

1. Ask the user to type something.
2. Read the line they enter.
3. Print it back.

Example:

```
Type something: hello rust
You typed: 'hello rust'
```

## What I want to learn

- Reading input from the terminal
- Creating an empty `String` to store the input
- Why the variable needs `mut` to be changed
- Passing a mutable reference (`&mut`) to a function
- Handling a `Result` that might fail
- Removing the trailing newline and whitespace from the input
- Bringing things from the standard library into scope with `use`

## Done when

- [ ] The program waits for input and prints back exactly what I typed
- [ ] The output has no extra blank line after the text
- [ ] I can explain why `read_line` takes `&mut` and returns a `Result`
