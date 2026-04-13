use crate::status::StatusProvider;
use rand::RngExt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;

pub const WAKE_UP_HOUR: u32 = 7;
pub const SLEEP_HOUR: u32 = 21; // 9 PM
pub const TIME_COST_STATUS: u32 = 1;
pub const GATHER_WOOD_COST_MIN: u32 = 1;
pub const GATHER_WOOD_COST_MAX: u32 = 3;
pub const GATHER_WATER_COST_MIN: u32 = 1;
pub const GATHER_WATER_COST_MAX: u32 = 2;
pub const GATHER_FOOD_COST_MIN: u32 = 1;
pub const GATHER_FOOD_COST_MAX: u32 = 2;
/// Represents the player's overall state and vitals
#[derive(Debug, Serialize, Deserialize)]
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
#[derive(Debug, Default, Serialize, Deserialize)]
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
    /// Saves game state to optional file
    Save(Option<String>),
    /// Loads game state from file
    Load(String),
    // Outputs the player's current inventory 
    Inventory,
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
            "save" => {
                let arg = parts.next().map(|s| s.to_string());
                Ok(Command::Save(arg))
            }
            "load" => {
                if let Some(arg) = parts.next() {
                    Ok(Command::Load(arg.to_string()))
                } else {
                    Ok(Command::Unknown)
                }
            },
            "inventory" | "inv" => {
                Ok(Command::Inventory)
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

/// Time tracking for the game engine
#[derive(Debug, Serialize, Deserialize)]
pub struct GameTime {
    pub day_count: u32,
    pub hour: u32,
}

impl GameTime {
    pub fn new() -> Self {
        Self {
            day_count: 1,
            hour: WAKE_UP_HOUR,
        }
    }

    pub fn next_turn(&mut self) {
        self.day_count += 1;
        self.hour = WAKE_UP_HOUR;
    }

    pub fn advance_time(&mut self, hours: u32) {
        self.hour += hours;
        while self.hour >= 24 {
            self.day_count += 1;
            self.hour -= 24;
        }
        if self.hour >= SLEEP_HOUR || self.hour < WAKE_UP_HOUR {
            if self.hour >= SLEEP_HOUR {
                self.day_count += 1;
            }
            self.hour = WAKE_UP_HOUR;
        }
    }
}

impl Default for GameTime {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents the core state of the game engine
#[derive(Debug, Serialize, Deserialize)]
pub struct GameEngine {
    is_running: bool,
    pub time: GameTime,
    pub player: Player,
    #[serde(skip)]
    status_providers: HashMap<String, Box<dyn StatusProvider>>,
}

impl GameEngine {
    /// Creates a new, initialized GameEngine
    pub fn new() -> Self {
        Self {
            is_running: true,
            time: GameTime::new(),
            player: Player::new(),
            status_providers: HashMap::new(),
        }
    }

    pub fn save_to_file(&self, file_name: &str) -> Result<(), String> {
        if let Some(parent) = std::path::Path::new(file_name)
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
        {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        }
        let yaml = serde_yaml::to_string(self).map_err(|e| e.to_string())?;
        std::fs::write(file_name, yaml).map_err(|e| e.to_string())
    }

    pub fn load_from_file(file_name: &str) -> Result<Self, String> {
        let yaml = std::fs::read_to_string(file_name).map_err(|e| e.to_string())?;
        serde_yaml::from_str(&yaml).map_err(|e| e.to_string())
    }

    pub fn register_status_provider(&mut self, provider: Box<dyn StatusProvider>) {
        self.status_providers
            .insert(provider.name().to_string().to_lowercase(), provider);
    }

    /// Checks if the game is still running
    pub fn is_running(&self) -> bool {
        self.is_running
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
                "Available commands: help, quit, exit, rest, gather, status, inventory".to_string()
            }
            Command::Rest => {
                self.player.health = (self.player.health + 20).min(100);
                self.time.next_turn();
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
            Command::Save(file_opt) => {
                let file_name = file_opt.as_deref().unwrap_or("survive_game.yaml");
                match self.save_to_file(file_name) {
                    Ok(_) => format!("Game saved to {}", file_name),
                    Err(e) => format!("Failed to save game: {}", e),
                }
            }
            Command::Load(file_name) => match GameEngine::load_from_file(&file_name) {
                Ok(loaded_engine) => {
                    self.is_running = loaded_engine.is_running;
                    self.time = loaded_engine.time;
                    self.player = loaded_engine.player;
                    format!("Game state restored from {}", file_name)
                }
                Err(e) => format!("Failed to load game: {}", e),
            },
            Command::Gather(resource) => {
                let random_amount: u32 = 100;
                match resource {
                    Resource::Wood => {
                        time_cost =
                            rand::rng().random_range(GATHER_WOOD_COST_MIN..=GATHER_WOOD_COST_MAX);
                        self.player.inventory.wood += random_amount;
                        format!(
                            "Gathered {} lbs. of wood! (Took {} hours)",
                            random_amount, time_cost
                        )
                    }
                    Resource::Water => {
                        time_cost =
                            rand::rng().random_range(GATHER_WATER_COST_MIN..=GATHER_WATER_COST_MAX);
                        self.player.inventory.water += random_amount;
                        format!(
                            "Gathered {} liters of water! (Took {} hours)",
                            random_amount, time_cost
                        )
                    }
                    Resource::Food => {
                        time_cost =
                            rand::rng().random_range(GATHER_FOOD_COST_MIN..=GATHER_FOOD_COST_MAX);
                        self.player.inventory.food += random_amount;
                        format!(
                            "Gathered {} lbs. of food! (Took {} hours)",
                            random_amount, time_cost
                        )
                    }
                    _ => "Couldn't gather unknown resource!".to_string(),
                }
            },
            Command::Inventory => {
                let player_inventory = &self.player.inventory;
                format!("~ {} lb. of firewood\n~ {} L of water\n~ {} lb. of food", player_inventory.wood, player_inventory.water, player_inventory.food)
            }
            _ => "Unknown command!".to_string(),
        };

        if time_cost > 0 {
            self.time.advance_time(time_cost);
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
        assert_eq!(engine.time.day_count, 2);
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
    fn test_engine_inventory_command_output() {
        let mut engine = GameEngine::new();
        let output = engine.process_command("inventory");
        assert!(output.contains("L of water") && output.contains("lb. of firewood") && output.contains("lb. of food"));
    }

    #[test]
    fn test_engine_inventory_command_shorthand() {
        let mut engine = GameEngine::new();
        let output = engine.process_command("inv");
        assert!(output.contains("~"));
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
        assert_eq!(engine.time.hour, WAKE_UP_HOUR);
        engine.time.advance_time(10);
        assert_eq!(engine.time.hour, 17);
        assert_eq!(engine.time.day_count, 1);

        // Push past sleep hour (21)
        engine.time.advance_time(5); // 17 + 5 = 22
        assert_eq!(engine.time.hour, WAKE_UP_HOUR);
        assert_eq!(engine.time.day_count, 2);
    }

    #[test]
    fn test_engine_time_costs() {
        let mut engine = GameEngine::new();
        let initial_hour = engine.time.hour;

        // Status command costs 1 hour
        engine.process_command("status");
        assert_eq!(engine.time.hour, initial_hour + 1);

        // Gather commands have variable costs, but always at least 1
        let mut engine2 = GameEngine::new();
        engine2.process_command("gather wood");
        assert!(engine2.time.hour > WAKE_UP_HOUR && engine2.time.hour <= WAKE_UP_HOUR + 3);

        // Rest command jumps to next day WAKE_UP_HOUR
        engine2.process_command("rest");
        assert_eq!(engine2.time.day_count, 2);
        assert_eq!(engine2.time.hour, WAKE_UP_HOUR);
    }

    #[test]
    fn test_engine_save_load() {
        let mut engine = GameEngine::new();
        engine.player.inventory.wood = 500;
        let file_name = "test_engine_save.yaml";
        assert!(engine.save_to_file(file_name).is_ok());

        let loaded_engine = GameEngine::load_from_file(file_name).unwrap();
        assert_eq!(loaded_engine.player.inventory.wood, 500);
        assert!(loaded_engine.is_running());

        // cleanup
        let _ = std::fs::remove_file(file_name);
    }
}
