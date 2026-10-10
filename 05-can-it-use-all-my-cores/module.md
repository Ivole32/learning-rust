# Module 5: Can it use all my cores?

Lifetimes, smart pointers, threads and the async mental model.

## By the end of this module you can

- Annotate lifetimes where the compiler needs them and explain what they mean
- Choose between Box, Rc, Arc and interior mutability
- Parallelise work safely with threads and rayon
- Explain how async/await and tokio schedule work

## Lessons

1. **Lifetimes: what 'a actually means** — A struct that borrows its log line, and the annotation that makes it compile.
2. **Box, Rc, Arc and interior mutability** — When single ownership isn't enough.
3. **Threads, Send/Sync and rayon** — Parse the 4 GB log on every core with no data races.
4. **async/await and tokio** — The mental model you need before writing a server.

## Module test

Module 5 check

Cumulative and closed-book. You need 70% to unlock the next module.
Already know this material? Ask to test out — same test, cold, at 85%.
