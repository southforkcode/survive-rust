use crate::status::StatusProvider;
use rand::RngExt;
use std::collections::HashMap;
use std::str::FromStr;

pub const WAKE_UP_HOUR: u32 = 7;
pub const SLEEP_HOUR: u32 = 21; // 9 PM
pub const TIME_COST_STATUS: u32 = 1;
/// Represents the player's overall state and vitals
#[derive(Debug)]
pub struct Player {
    /// The player's current health points. Player dies if this reaches 0 or below. Max is 100.
    pub health: i32,
    pub inventory: Inventory,
}

impl Player {
    /// Creates a new player with full health and an empty inventory.
    pub fn new() -> Self {
        Self {
            health: 100,
            inventory: Inventory::new(),
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

/// Tracks all gatherable resources carried by the player.
#[derive(Debug, Default)]
pub struct Inventory {
    /// Amount of wood in pounds.
    pub wood: u32, // lb
    /// Amount of water in liters.
    pub water: u32, // liters
    /// Amount of food in pounds.
    pub food: u32, // lb
}

impl Inventory {
    /// Creates a new empty inventory.
    pub fn new() -> Self {
        Self {
            wood: 0,
            water: 0,
            food: 0,
        }
    }
}

// Command variants
/// Parsed command values accepted by the game engine.
pub enum Command {
    /// Displays the list of available commands.
    Help,
    /// Advances one turn and restores player health.
    Rest,
    /// Stops the game loop.
    Quit,
    /// Gathers the requested resource.
    Gather(Resource),
    /// Gets status of the environment, player, or camp.
    Status(Option<String>),
    /// Represents an unrecognized command.
    Unknown,
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let command: &str = parts.next().unwrap_or("");

        match command {
            "help" => Ok(Command::Help),
            "rest" => Ok(Command::Rest),
            "quit" | "exit" => Ok(Command::Quit),
            "gather" => {
                if let Some(arg) = parts.next() {
                    Ok(Command::Gather(
                        Resource::from_str(arg).ok().unwrap_or(Resource::Unknown),
                    ))
                } else {
                    Ok(Command::Gather(Resource::Unknown))
                }
            }
            "status" => {
                if let Some(arg) = parts.next() {
                    Ok(Command::Status(Some(arg.to_string())))
                } else {
                    Ok(Command::Status(None))
                }
            }
            _ => Ok(Command::Unknown),
        }
    }
}

// Resource variants
/// Supported resource kinds for gather actions.
#[derive(Debug)]
pub enum Resource {
    /// Wood resource.
    Wood,
    /// Water resource.
    Water,
    /// Food resource.
    Food,
    /// Unrecognized resource input.
    Unknown,
}

impl FromStr for Resource {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            s if s.eq_ignore_ascii_case("wood") => Ok(Resource::Wood),
            s if s.eq_ignore_ascii_case("water") => Ok(Resource::Water),
            s if s.eq_ignore_ascii_case("food") => Ok(Resource::Food),
            _ => Ok(Resource::Unknown),
        }
    }
}

/// Represents the core state of the game engine
#[derive(Debug)]
pub struct GameEngine {
    is_running: bool,
    /// The current day of survival
    pub day_count: u32,
    /// The current hour of the day
    pub hour: u32,
    /// The player associated with the game
    pub player: Player,
    status_providers: HashMap<String, Box<dyn StatusProvider>>,
}

impl GameEngine {
    /// Creates a new, initialized GameEngine
    pub fn new() -> Self {
        Self {
            is_running: true,
            day_count: 1,
            hour: WAKE_UP_HOUR,
            player: Player::new(),
            status_providers: HashMap::new(),
        }
    }

    pub fn register_status_provider(&mut self, provider: Box<dyn StatusProvider>) {
        self.status_providers
            .insert(provider.name().to_string().to_lowercase(), provider);
    }

    /// Checks if the game is still running
    pub fn is_running(&self) -> bool {
        self.is_running
    }

    fn next_turn(&mut self) {
        self.day_count += 1;
        self.hour = WAKE_UP_HOUR;
    }

    /// Advances time by the specified number of hours. If time exceeds SLEEP_HOUR, start a new day.
    pub fn advance_time(&mut self, hours: u32) {
        self.hour += hours;
        if self.hour >= SLEEP_HOUR {
            self.next_turn();
        }
    }

    /// Processes a single string command from the user and updates state.
    ///
    /// # Arguments
    /// * `command` - The text command from the terminal
    pub fn process_command(&mut self, raw: &str) -> String {
        let cmd: Command = Command::from_str(raw).ok().unwrap_or(Command::Unknown);
        let mut time_cost = 0;

        let output = match cmd {
            Command::Help => {
                "Available commands: help, quit, exit, rest, gather, status".to_string()
            }
            Command::Rest => {
                self.player.health = (self.player.health + 20).min(100);
                self.next_turn();
                "You gained +20 health back.".to_string()
            }
            Command::Status(target) => {
                time_cost = TIME_COST_STATUS;
                match target {
                    Some(name) => {
                        if let Some(provider) = self.status_providers.get(&name.to_lowercase()) {
                            provider.status(self)
                        } else {
                            "Cannot get status of unknown target.".to_string()
                        }
                    }
                    None => {
                        let mut statuses: Vec<String> = self
                            .status_providers
                            .values()
                            .map(|p| p.status(self))
                            .collect();
                        statuses.sort(); // Optional: keeps output deterministic
                        statuses.join("\n\n")
                    }
                }
            }
            Command::Quit => {
                self.is_running = false;
                "Exiting game...".to_string()
            }
            Command::Gather(resource) => {
                let random_amount: u32 = 100;
                match resource {
                    Resource::Wood => {
                        time_cost = rand::rng().random_range(1..=3);
                        self.player.inventory.wood += random_amount;
                        format!(
                            "Gathered {} lbs. of wood! (Took {} hours)",
                            random_amount, time_cost
                        )
                    }
                    Resource::Water => {
                        time_cost = rand::rng().random_range(1..=2);
                        self.player.inventory.water += random_amount;
                        format!(
                            "Gathered {} liters of water! (Took {} hours)",
                            random_amount, time_cost
                        )
                    }
                    Resource::Food => {
                        time_cost = rand::rng().random_range(1..=2);
                        self.player.inventory.food += random_amount;
                        format!(
                            "Gathered {} lbs. of food! (Took {} hours)",
                            random_amount, time_cost
                        )
                    }
                    _ => "Couldn't gather unknown resource!".to_string(),
                }
            }
            _ => "Unknown command!".to_string(),
        };

        if time_cost > 0 {
            self.advance_time(time_cost);
        }

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

impl Default for GameEngine {
    fn default() -> Self {
        Self::new()
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
    fn test_engine_status_command() {
        use crate::status::{CampStatusProvider, PlayerStatusProvider, WeatherStatusProvider};
        let mut engine = GameEngine::new();
        engine.register_status_provider(Box::new(WeatherStatusProvider));
        engine.register_status_provider(Box::new(CampStatusProvider));
        engine.register_status_provider(Box::new(PlayerStatusProvider));

        let output = engine.process_command("status");
        assert!(output.contains("The sun is high in the sky."));
        assert!(output.contains("A sleeping spot"));
        assert!(output.contains("rested and healthy"));

        let weather_output = engine.process_command("status weather");
        assert!(weather_output.contains("The sun is high in the sky."));

        let camp_output = engine.process_command("status camp");
        assert!(camp_output.contains("A sleeping spot"));

        engine.process_command("gather wood");
        let item_output = engine.process_command("status camp");
        assert!(item_output.contains("Some firewood"));

        engine.player.health = 40;
        let p_status = engine.process_command("status player");
        assert!(p_status.contains("weak and injured"));
    }

    #[test]
    fn test_engine_gather_command_valid_resources() {
        let mut engine: GameEngine = GameEngine::new();

        for resource in ["wood", "water", "food"] {
            let command: String = "gather ".to_string() + resource;
            let output = engine.process_command(&command);
            assert!(!output.contains("Couldn't"));
            match resource {
                "wood" => {
                    assert_eq!(engine.player.inventory.wood, 100);
                    assert_eq!(engine.player.inventory.water, 0);
                    assert_eq!(engine.player.inventory.food, 0)
                }
                "water" => {
                    assert_eq!(engine.player.inventory.wood, 100);
                    assert_eq!(engine.player.inventory.water, 100);
                    assert_eq!(engine.player.inventory.food, 0)
                }
                "food" => {
                    assert_eq!(engine.player.inventory.wood, 100);
                    assert_eq!(engine.player.inventory.water, 100);
                    assert_eq!(engine.player.inventory.food, 100)
                }
                _ => {}
            }
        }
    }

    #[test]
    fn test_engine_gather_command_invalid_resources() {
        let mut engine = GameEngine::new();
        let invalid_inputs = ["", " ", "something"];
        for input in invalid_inputs {
            let command: String = "gather ".to_string() + input;
            let output = engine.process_command(&command);
            assert!(output.contains("Couldn't gather unknown resource!"));
            assert_eq!(engine.player.inventory.wood, 0);
            assert_eq!(engine.player.inventory.water, 0);
            assert_eq!(engine.player.inventory.food, 0);
        }
    }

    #[test]
    fn test_engine_death() {
        let mut engine = GameEngine::new();
        engine.player.health = 0;
        let output = engine.process_command("");
        assert!(output.contains("You died."));
        assert!(!engine.is_running());
    }

    #[test]
    fn test_engine_time_advance() {
        let mut engine = GameEngine::new();
        assert_eq!(engine.hour, WAKE_UP_HOUR);
        engine.advance_time(10);
        assert_eq!(engine.hour, 17);
        assert_eq!(engine.day_count, 1);

        // Push past sleep hour (21)
        engine.advance_time(5); // 17 + 5 = 22
        assert_eq!(engine.hour, WAKE_UP_HOUR);
        assert_eq!(engine.day_count, 2);
    }

    #[test]
    fn test_engine_time_costs() {
        let mut engine = GameEngine::new();
        let initial_hour = engine.hour;

        // Status command costs 1 hour
        engine.process_command("status");
        assert_eq!(engine.hour, initial_hour + 1);

        // Gather commands have variable costs, but always at least 1
        let mut engine2 = GameEngine::new();
        engine2.process_command("gather wood");
        assert!(engine2.hour >= WAKE_UP_HOUR + 1 && engine2.hour <= WAKE_UP_HOUR + 3);

        // Rest command jumps to next day WAKE_UP_HOUR
        engine2.process_command("rest");
        assert_eq!(engine2.day_count, 2);
        assert_eq!(engine2.hour, WAKE_UP_HOUR);
    }
}
