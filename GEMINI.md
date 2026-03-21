# Gemini Agent Guidelines

## General Workflow
- **Branching Restrictions:** You must ONLY make changes to `feature/` or `bug/` branches. Note: Modifying `main` is strictly prohibited after the initial setup.
- **Testing Requirements:** All logic changes MUST be accompanied by tests.
  - Features should be thoroughly covered by one or more BDD tests and/or unit tests.
  - BDD tests are the preferred methodology for feature verification and should be used as much as possible.
  - UX tests must exercise the application in the terminal, as the terminal serves as our primary UI.

## Rust Best Practices
When writing, modifying, or reviewing Rust code, enforce the following standard best practices:
- **Idiomatic Code:** Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) and [The Rust Book](https://doc.rust-lang.org/book/). Write clean and expressive iterators, pattern matching, and error handling.
- **Error Handling:** Use `Result` and `Option` properly. Avoid using `.unwrap()` or `.expect()` unless absolutely necessary for a well-justified panic. Return errors to the caller where appropriate.
- **Immutability by Default:** Keep variables immutable unless they mathematically or logically require mutation.
- **Borrowing & Lifetimes:** Minimize data cloning (`.clone()`). Leverage Rust's borrowing rules effectively to manage memory and ownership.
- **Documentation:** Document all public functions, structs, enums, and modules using standard `///` documentation comments.
- **Code Quality:** Ensure code formatting passes `cargo fmt` and clean static analysis via `cargo clippy`.
- **Modularity:** Keep modules focused. Strictly separate core game logic from terminal/UI rendering code so the game engine remains highly testable.
