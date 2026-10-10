# Module 3: What happens when the input is garbage?

Errors as values, collections, iterators, and streaming huge files.

## By the end of this module you can

- Handle absence and failure with Option and Result instead of panics
- Propagate errors with ? through custom error types
- Aggregate data with Vec, HashMap and iterator chains
- Stream a multi-gigabyte file in bounded memory and benchmark it

## Lessons

1. **Option and Result: no null, no exceptions** — Malformed log lines stop being crashes and become values you handle.
2. **The ? operator and custom errors** — One ParseError enum, thiserror for the library, anyhow for main.
3. **Vec, HashMap, iterators and closures** — Top-10 slowest endpoints in a single iterator chain.
4. **Streaming 4 GB with BufReader: Rust vs Python** — Process a huge log in constant memory and benchmark against the 11-minute Python script.

## Module test

Module 3 check

Cumulative and closed-book. You need 70% to unlock the next module.
Already know this material? Ask to test out — same test, cold, at 85%.
