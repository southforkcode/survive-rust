/// Represents the core state of the game engine
#[derive(Debug)]
pub struct GameEngine {
    is_running: bool,
}

impl GameEngine {
    /// Creates a new, initialized GameEngine
    pub fn new() -> Self {
        Self { is_running: true }
    }

    /// Checks if the game is still running
    pub fn is_running(&self) -> bool {
        self.is_running
    }

    /// Processes a single string command from the user and updates state.
    ///
    /// # Arguments
    /// * `command` - The text command from the terminal
    pub fn process_command(&mut self, command: &str) -> String {
        let cmd = command.trim().to_lowercase();
        match cmd.as_str() {
            "help" => "Available commands: help, quit".to_string(),
            "quit" | "exit" => {
                self.is_running = false;
                "Exiting game...".to_string()
            }
            "" => "".to_string(),
            _ => format!("Unknown command: {}", cmd),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_initialization() {
        let engine = GameEngine::new();
        assert!(engine.is_running());
    }

    #[test]
    fn test_engine_help_command() {
        let mut engine = GameEngine::new();
        let output = engine.process_command("help");
        assert!(output.contains("help"));
        assert!(engine.is_running());
    }

    #[test]
    fn test_engine_quit_command() {
        let mut engine = GameEngine::new();
        let output = engine.process_command("quit");
        assert!(output.contains("Exiting"));
        assert!(!engine.is_running());
    }
}
