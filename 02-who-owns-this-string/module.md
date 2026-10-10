# Module 2: Who owns this string?

Ownership, moves and borrowing: the rules that replace a garbage collector.

## By the end of this module you can

- Predict when a value is moved, copied or cloned
- Borrow immutably and mutably within the aliasing-XOR-mutation rule
- Parse text with slices instead of allocating new Strings
- Diagnose and fix common borrow-checker errors from their error codes

## Lessons

1. **Moves, clone and Copy** — Why let b = a broke your code, and what Python was doing behind your back.
2. **Borrowing: shared and mutable references** — & and &mut, and the one rule the borrow checker enforces.
3. **Slices: &str vs String and zero-copy parsing** — Split log lines into fields without allocating a single new String.
4. **Bug hunt: reading borrow-checker errors** — Five broken crumbs functions, five error codes, zero unnecessary clones.

## Module test

Module 2 check

Cumulative and closed-book. You need 70% to unlock the next module.
Already know this material? Ask to test out — same test, cold, at 85%.
