# Learning Rust

This is my personal repository for learning Rust. Every project in here is something I built while working through the language, so expect beginner code that improves over time.

## How it's organized

Each project lives in its own folder and is a standalone Cargo project:

```
learning-rust/
├── hello-world/
│   ├── GOAL.md
│   ├── Cargo.toml
│   └── src/
└── ...
```

Every project has a `GOAL.md` file that describes what the project is supposed to do. I write it before I start coding, so it shows what I set out to build and what I wanted to learn from it.

## Using this as a reference

I'm writing this code for myself, but you're welcome to use it if you're learning Rust too:

1. Read a project's `GOAL.md` to see what it's meant to do.
2. Try building it yourself first.
3. Compare your solution with mine, or look at mine when you get stuck.

My solutions aren't necessarily the best or most idiomatic way to do things. They're just the way I did it at that point in my learning.

## Running a project

You need [Rust](https://www.rust-lang.org/tools/install) installed. Then run a project from its folder:

```bash
cd hello-world
cargo run
```

---

Made with ❤️ by [Ivole32](https://github.com/Ivole32)