/// Represents the player's overall state and vitals
#[derive(Debug)]
pub struct Player {
    /// The player's current health points. Player dies if this reaches 0. Max is 100.
    pub health: i32,
}

impl Player {
    pub fn new() -> Self {
        Self { health: 100 }
    }
}

/// Represents the core state of the game engine
#[derive(Debug)]
pub struct GameEngine {
    is_running: bool,
    /// The current day of survival
    pub day_count: u32,
    /// The player associated with the game
    pub player: Player,
}

impl GameEngine {
    /// Creates a new, initialized GameEngine
    pub fn new() -> Self {
        Self {
            is_running: true,
            day_count: 1,
            player: Player::new(),
        }
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
        let output = match cmd.as_str() {
            "help" => "Available commands: help, rest, quit".to_string(),
            "rest" => {
                self.player.health = (self.player.health + 20).min(100);
                self.day_count += 1;
                "You gained +20 health back.".to_string()
            }
            "quit" | "exit" => {
                self.is_running = false;
                "Exiting game...".to_string()
            }
            "" => "".to_string(),
            _ => format!("Unknown command: {}", cmd),
        };

        if self.player.health <= 0 && self.is_running {
            self.is_running = false;
            return if output.is_empty() {
                "You died.".to_string()
            } else {
                format!("{}\nYou died.", output)
            };
        }

        output
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

    #[test]
    fn test_engine_rest_command() {
        let mut engine = GameEngine::new();
        engine.player.health = 50;
        let output = engine.process_command("rest");
        assert_eq!(output, "You gained +20 health back.");
        assert_eq!(engine.player.health, 70);
        assert_eq!(engine.day_count, 2);
    }

    #[test]
    fn test_engine_rest_health_cap() {
        let mut engine = GameEngine::new();
        engine.player.health = 90;
        engine.process_command("rest");
        assert_eq!(engine.player.health, 100);
    }

    #[test]
    fn test_engine_death() {
        let mut engine = GameEngine::new();
        engine.player.health = 0;
        let output = engine.process_command("");
        assert!(output.contains("You died."));
        assert!(!engine.is_running());
    }
}
