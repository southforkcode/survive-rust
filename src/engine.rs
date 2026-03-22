use std::str::FromStr;

/// Represents the player's overall state and vitals
#[derive(Debug)]
pub struct Player {
    /// The player's current health points. Player dies if this reaches 0. Max is 100.
    health: u32,
    inventory: Inventory,
}

impl Player {
    pub fn new() -> Self {
        Self { 
            health: 100, 
            inventory: Inventory::new(),
        }
    }
}

#[derive(Debug, Default)]
pub struct Inventory {
    pub wood: u32, // lb
    pub water: u32, // liters
    pub food: u32, // lb
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            wood: 0,
            water: 0,
            food: 0,
        }
    }
}


// Command variants
pub enum Command {
    Help,
    Rest,
    Quit,
    Gather(Resource),
    Unknown,
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().split_whitespace();
        let command: &str = parts.next().unwrap_or("");
        let args: Vec<&str> = parts.collect();

        match command {
            "help" => Ok(Command::Help),
            "rest" => Ok(Command::Rest),
            "quit" => Ok(Command::Quit),
            "gather" => {
                if args.len() > 0 {
                    Ok(Command::Gather(Resource::from_str(args[0]).unwrap_or_else(|_| Resource::Unknown)))
                } else {
                    Ok(Command::Gather(Resource::Unknown))
                }
            },
            _ => {
                Ok(Command::Unknown)
            }
        }
    }
}

// Resource variants
#[derive(Debug, Default)]
pub enum Resource {
    Wood,
    Water,
    Food,
    #[default]
    Unknown,
}

impl FromStr for Resource {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "wood" => Ok(Resource::Wood),
            "water" => Ok(Resource::Water),
            "food" => Ok(Resource::Food),
            _ => Ok(Resource::Unknown)
        }
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

    fn next_turn(&mut self) -> () {
        self.day_count += 1;
    }

    /// Processes a single string command from the user and updates state.
    ///
    /// # Arguments
    /// * `command` - The text command from the terminal
    pub fn process_command(&mut self, raw: &str) -> String {
        let cmd: Command = Command::from_str(raw).unwrap_or_else(|_| Command::Unknown);
        let output = match cmd {
            Command::Help => "Available commands: help, rest, quit, rest, gather".to_string(),
            Command::Rest => {
                self.player.health = (self.player.health + 20).min(100);
                self.next_turn();
                "You gained +20 health back.".to_string()
            }
            Command::Quit => {
                self.is_running = false;
                "Exiting game...".to_string()
            }
            Command::Gather(resource) => {
                let random_amount: u32 = 100;
                match resource {
                    Resource::Wood => {
                        self.player.inventory.wood += random_amount;
                        format!("Gathered {random_amount} lbs. of wood!")
                    }
                    Resource::Water => {
                        self.player.inventory.water += random_amount;
                        format!("Gathered {random_amount} liters of water!")
                    }
                    Resource::Food => {
                        self.player.inventory.food += random_amount;
                        format!("Gathered {random_amount} lbs. of water!")
                    }
                    Resource::Unknown | _ => {
                        format!("Couldn't gather unknown resource!")
                    }
                }

                
            }
            Command::Unknown | _ => "Unknown command!".to_string()
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
