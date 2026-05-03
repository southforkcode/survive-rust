use crate::engine as Engine;
use Engine::{Resource, GameEngine};
use rand::prelude::*;
use std::str::FromStr;

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

// Parse the command input from the player
pub fn parse_input(raw: &str) -> Command {
    Command::from_str(raw).ok().unwrap_or(Command::Unknown)
}

// Handle parsed command
pub fn handle_command(cmd: Command, engine: &mut GameEngine) -> String {
    let mut time_cost = 0;
    let output = match cmd {
        Command::Help => {
            "Available commands: help, quit, exit, rest, gather, status, inventory".to_string()
        }
        Command::Rest => {
            engine.player.health = (engine.player.health + 20).min(100);
            engine.time.next_turn();
            "You gained +20 health back.".to_string()
        }
        Command::Status(target) => {
            time_cost = Engine::TIME_COST_STATUS;
            match target {
                Some(name) => {
                    if let Some(provider) = engine.status_providers().get(&name.to_lowercase()) {
                        provider.status(engine)
                    } else {
                        "Cannot get status of unknown target.".to_string()
                    }
                }
                None => {
                    let mut statuses: Vec<String> = engine
                        .status_providers()
                        .values()
                        .map(|p| p.status(engine))
                        .collect();
                    statuses.sort(); // Optional: keeps output deterministic
                    statuses.join("\n\n")
                }
            }
        }
        Command::Quit => {
            engine.quit();
            "Exiting game...".to_string()
        }
        Command::Save(file_opt) => {
            let file_name = file_opt.as_deref().unwrap_or("survive_game.yaml");
            match engine.save_to_file(file_name) {
                Ok(_) => format!("Game saved to {}", file_name),
                Err(e) => format!("Failed to save game: {}", e),
            }
        }
        Command::Load(file_name) => match GameEngine::load_from_file(&file_name) {
            Ok(loaded_engine) => {
                engine.load_new_engine(loaded_engine);
                format!("Game state restored from {}", file_name)
            }
            Err(e) => format!("Failed to load game: {}", e),
        },
        Command::Gather(resource) => {
            let random_amount: u32 = 100;
            match resource {
                Resource::Wood => {
                    time_cost =
                        rand::rng().random_range(Engine::GATHER_WOOD_COST_MIN..=Engine::GATHER_WOOD_COST_MAX);
                    engine.player.inventory.set_wood(engine.player.inventory.wood() + random_amount);
                    format!(
                        "Gathered {} lbs. of wood! (Took {} hours)",
                        random_amount, time_cost
                    )
                }
                Resource::Water => {
                    time_cost =
                        rand::rng().random_range(Engine::GATHER_WATER_COST_MIN..=Engine::GATHER_WATER_COST_MAX);
                    engine.player.inventory.set_water(engine.player.inventory.water() + random_amount);
                    format!(
                        "Gathered {} liters of water! (Took {} hours)",
                        random_amount, time_cost
                    )
                }
                Resource::Food => {
                    time_cost =
                        rand::rng().random_range(Engine::GATHER_FOOD_COST_MIN..=Engine::GATHER_FOOD_COST_MAX);
                    engine.player.inventory.set_food(engine.player.inventory.food() + random_amount);
                    format!(
                        "Gathered {} lbs. of food! (Took {} hours)",
                        random_amount, time_cost
                    )
                }
                _ => "Couldn't gather unknown resource!".to_string(),
            }
        },
        Command::Inventory => {
            let player_inventory = &engine.player.inventory;
            format!("~ {} lb. of firewood\n~ {} L of water\n~ {} lb. of food", player_inventory.wood(), player_inventory.water(), player_inventory.food())
        }
        _ => "Unknown command!".to_string(),
    };

    if time_cost > 0 {
        engine.time.advance_time(time_cost);
    }

    if engine.player.health <= 0 && engine.is_running() {
        engine.quit();
        return if output.is_empty() {
            "You died.".to_string()
        } else {
            format!("{}\nYou died.", output)
        };
    }

    output
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

