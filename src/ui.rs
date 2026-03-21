use std::io::{self, Write};

/// Manages interaction with the terminal UI
pub struct TerminalUi;

impl TerminalUi {
    /// Constructs a new TerminalUi manager
    pub fn new() -> Self {
        Self
    }

    /// Prompts the user and reads a line from standard input.
    pub fn read_command(&self) -> String {
        print!("> ");
        let _ = io::stdout().flush();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading input");
        }
        input
    }

    /// Displays output text to the user.
    pub fn display_output(&self, output: &str) {
        if !output.is_empty() {
            println!("{}", output);
        }
    }
}
