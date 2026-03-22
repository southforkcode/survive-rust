# Gemini Agent Guidelines

## General Workflow
- **Designing** When you are asked to design something (given an GH issue). You must not make code changes. You are only meant to post comments to the ticket referenced in the request.
- **Branching Restrictions:** You must ONLY make changes to `feature/` or `bug/` branches. Note: Modifying `main` is strictly prohibited after the initial setup, **unless explicitly instructed by the user** (the goal is to enforce review cycles, not to prevent merges when asked).
- **Testing Requirements:** Ensure code is covered by tests.
  - Prioritize BDD style tests when appropriate.
  - Use Unit Tests when appropriate.
  - UI tests should cover user interaction.

## Rust Best Practices
When writing, modifying, or reviewing Rust code, enforce the following standard best practices:
- **Idiomatic Code:** Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) and [The Rust Book](https://doc.rust-lang.org/book/). Write clean and expressive iterators, pattern matching, and error handling.
- **Error Handling:** Use `Result` and `Option` properly. Avoid using `.unwrap()` or `.expect()` unless absolutely necessary for a well-justified panic. Return errors to the caller where appropriate.
- **Immutability by Default:** Keep variables immutable unless they mathematically or logically require mutation.
- **Borrowing & Lifetimes:** Minimize data cloning (`.clone()`). Leverage Rust's borrowing rules effectively to manage memory and ownership.
- **Documentation:** Document all public functions, structs, enums, and modules using standard `///` documentation comments.
- **Code Quality:** Ensure code formatting passes `cargo fmt` and clean static analysis via `cargo clippy`.
- **Modularity:** Keep modules focused. Strictly separate core game logic from terminal/UI rendering code so the game engine remains highly testable.
